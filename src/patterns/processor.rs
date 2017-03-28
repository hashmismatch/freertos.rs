use prelude::v1::*;
use base::*;
use mutex::*;
use queue::*;
use units::*;

pub type SharedClientWithReplyQueue<T> = Arc<ClientWithReplyQueue<T>>;
pub type Client<T> = ProcessorClient<T, ()>;

pub trait ReplyableMessage {
    fn reply_to_client_id(&self) -> Option<usize>;
}

#[derive(Copy, Clone)]
pub enum Message<T> where T: Copy {
    Request { val: T },
    RequestWithReply { val: T, client_id: usize }
}

impl<T> Message<T> where T: Copy {
    pub fn request(val: T) -> Self {
        Message::Request { val: val }
    }

    pub fn request_with_reply(val: T, client_id: usize) -> Self {
        Message::RequestWithReply { val: val, client_id: client_id }
    }

    pub fn get_val(&self) -> T {
        match *self {
            Message::Request { val } => val,
            Message::RequestWithReply { val, .. } => val
        }
    }
}

impl<T> ReplyableMessage for Message<T> where T: Copy {
    fn reply_to_client_id(&self) -> Option<usize> {
        match *self {
            Message::RequestWithReply { client_id, .. } => Some(client_id),
            _ => None
        }
    }
}

pub struct Processor<T, R> where T: ReplyableMessage + Copy, R: Copy {
    queue: Arc<Queue<T>>,
    inner: Arc<Mutex<ProcessorInner<R>>>,
}

impl<T, R> Processor<T, R> where T: ReplyableMessage + Copy, R: Copy {
    pub fn new(queue_size: usize) -> Result<Self, FreeRtosError> {
        let p = ProcessorInner {
            clients: Vec::new(), 
            next_client_id: 1
        };        
        let p = Arc::new(try!(Mutex::new(p)));
        let p = Processor {
            queue: Arc::new(try!(Queue::new(queue_size))),
            inner: p
        };
        Ok(p)
    }

    pub fn new_client(&self) -> Result<Client<T>, FreeRtosError> {
        let c = ProcessorClient {
            processor_queue: Arc::downgrade(&self.queue),
            client_reply: ()
        };

        Ok(c)
    }

    
    pub fn new_client_with_reply(&self, client_receive_queue_size: usize, max_wait: Duration) -> Result<ProcessorClient<T, SharedClientWithReplyQueue<R>>, FreeRtosError> {        
        if client_receive_queue_size == 0 {
            return Err(FreeRtosError::InvalidQueueSize);
        }

        let client_reply = {
            let mut processor = try!(self.inner.lock(max_wait));

            let c = ClientWithReplyQueue {
                id: processor.next_client_id,
                processor_inner: self.inner.clone(),
                receive_queue: try!(Queue::new(client_receive_queue_size))
            };            

            let c = Arc::new(c);
            processor.clients.push((c.id, Arc::downgrade(&c)));

            processor.next_client_id += 1;

            c
        };

        let c = ProcessorClient {
            processor_queue: Arc::downgrade(&self.queue),
            client_reply: client_reply
        };

        Ok(c)
    }

    pub fn get_receive_queue(&self) -> &Queue<T> {
        &*self.queue
    }

    pub fn reply(&self, received_message: T, reply: R, max_wait: Duration) -> Result<bool, FreeRtosError> {
        if let Some(client_id) = received_message.reply_to_client_id() {            
            let inner = try!(self.inner.lock(max_wait));
            if let Some(client) = inner.clients.iter().flat_map(|ref x| x.1.upgrade().into_iter()).find(|x| x.id == client_id) {
                try!(client.receive_queue.send(reply, max_wait));
                return Ok(true);
            }
        }

        Ok(false)
    }
}

impl<T, R> Processor<Message<T>, R> where T: Copy, R: Copy {
    pub fn reply_val(&self, received_message: Message<T>, reply: R, max_wait: Duration) -> Result<bool, FreeRtosError> {
        self.reply(received_message, reply, max_wait)
    }
}

struct ProcessorInner<R> where R: Copy {
    clients: Vec<(usize, Weak<ClientWithReplyQueue<R>>)>,
    next_client_id: usize
}

impl<R> ProcessorInner<R> where R: Copy {
    fn remove_client_reply(&mut self, client: &ClientWithReplyQueue<R>) {
        self.clients.retain(|ref x| x.0 != client.id)
    }
}



pub struct ProcessorClient<T, C> where T: ReplyableMessage + Copy {    
    processor_queue: Weak<Queue<T>>,
    client_reply: C
}

impl<T, R> ProcessorClient<T, R> where T: ReplyableMessage + Copy {
    pub fn send(&self, message: T, max_wait: Duration) -> Result<(), FreeRtosError> {
        let processor_queue = try!(self.processor_queue.upgrade().ok_or(FreeRtosError::ProcessorHasShutDown));
        try!(processor_queue.send(message, max_wait));
        Ok(())   
    }
}

impl<T> ProcessorClient<Message<T>, ()> where T: Copy {
    pub fn send_val(&self, val: T, max_wait: Duration) -> Result<(), FreeRtosError> {
        self.send(Message::request(val), max_wait)
    }
}

impl<T, R> ProcessorClient<T, SharedClientWithReplyQueue<R>> where T: ReplyableMessage + Copy, R: Copy {
    pub fn call(&self, message: T, max_wait: Duration) -> Result<R, FreeRtosError> {
        try!(self.send(message, max_wait));
        self.client_reply.receive_queue.receive(max_wait)
    }

    pub fn get_receive_queue(&self) -> &Queue<R> {
        &self.client_reply.receive_queue
    }
}

impl<T, R> ProcessorClient<Message<T>, SharedClientWithReplyQueue<R>> where T: Copy, R: Copy {
    pub fn send_val(&self, val: T, max_wait: Duration) -> Result<(), FreeRtosError> {
        self.send(Message::request(val), max_wait)
    }
    
    pub fn call_val(&self, val: T, max_wait: Duration) -> Result<R, FreeRtosError> {
        let reply = try!(self.call(Message::request_with_reply(val, self.client_reply.id), max_wait));
        Ok(reply)
    }
}

impl<T, C> Clone for ProcessorClient<T, C> where T: ReplyableMessage + Copy, C: Clone {
    fn clone(&self) -> Self {
        ProcessorClient {
            processor_queue: self.processor_queue.clone(),
            client_reply: self.client_reply.clone()
        }
    }
}



pub struct ClientWithReplyQueue<R> where R: Copy {
    id: usize,
    processor_inner: Arc<Mutex<ProcessorInner<R>>>,
    receive_queue: Queue<R>
}

impl<R> Drop for ClientWithReplyQueue<R> where R: Copy {
    fn drop(&mut self) {
        if let Ok(mut p) = self.processor_inner.lock(Duration::ms(1000)) {
            p.remove_client_reply(&self);
        }
    }
}
