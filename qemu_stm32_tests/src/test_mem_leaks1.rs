use super::*;
use prelude::v1::*;

use freertos_rs::*;
use freertos_rs::patterns::compute_task::*;
use freertos_rs::patterns::processor::*;
use freertos_rs::patterns::pub_sub::*;


#[no_mangle]
pub fn test_mem_leaks1() -> i8 {

	let main_task = Task::new().name("main").stack_size(2048).start(|| {
		let start_memory_usage = heap_allocated_memory();
		let mut end_memory_usage = 0;

		// simple task spawn		
		for i in 0..100 {			
			Task::new().name(&format!("t_{}", i)).stack_size(128).start(move || {
				let a = i;
				let s: String = "Hello world".into();
			}).unwrap();

			CurrentTask::delay(Duration::ms(1));
		}
		

		// simple mutexes
		for i in 0..100 {
			let m = Mutex::new(0).unwrap();
			let mut v = m.lock(Duration::ms(50)).unwrap();
			*v += 1;
		}

		// recursive mutexes
		for i in 0..100 {
			let m = Mutex::new_recursive(0).unwrap();
			let mut v = m.lock(Duration::ms(50)).unwrap();
			*v += 1;
		}

		// deconstructing a mutex
		{
			let n = 50;

			let test_str = "Hello Heap World";

			let mutexes = {
				let mut v = vec![];
				for _ in 0..n {
					let mutex = {
						let m = Mutex::new(test_str.to_string()).unwrap();
						{
							let l = m.lock(Duration::ms(50)).unwrap();
							assert_eq!(test_str, *l);
						}
						m
					};

					v.push(mutex);
				}

				v
			};

			for mutex in mutexes.into_iter() {
				let inner: String = mutex.into_inner();
				assert_eq!(test_str, inner);
			}			
		}

		// simple queues
		for i in 0..100 {
			let q = Queue::new(10).unwrap();
			q.send(10, Duration::ms(5)).unwrap();
			q.receive(Duration::ms(100)).unwrap();
		}

		// compute tasks
		{
			let n = 12;
			let res: u64 = 42;

			let tasks = {
				let mut v = vec![];

				for i in 0..n {
					let name = format!("comp_{}", i);
					let t = Task::new().name(&name).stack_size(256).compute(|| {
						CurrentTask::delay(Duration::ms(200));
						42 as u64
					}).unwrap();
					v.push(t);
				}

				v
			};

			for task in tasks.into_iter() {
				let result = task.into_result(Duration::ms(200)).unwrap();
				assert_eq!(res, result);
			}

		}

		// pub sub
		{
			let w = Duration::ms(1);

			let publisher = QueuePublisher::new().unwrap();			
			let sub1 = publisher.subscribe(10, w).unwrap();
			assert_eq!(1, publisher.send("A", w));
			let sub2 = publisher.subscribe(10, w).unwrap();			
			let publisher2 = publisher.clone();
			assert_eq!(2, publisher2.send("B", w));			
			
			assert_eq!("A", sub1.receive(w).unwrap());
			assert_eq!("B", sub1.receive(w).unwrap());
			assert_eq!(Result::Err(FreeRtosError::QueueReceiveTimeout), sub1.receive(w));
			drop(sub1);

			assert_eq!("B", sub2.receive(w).unwrap());
			assert_eq!(Result::Err(FreeRtosError::QueueReceiveTimeout), sub2.receive(w));
		}
		
		// timers		
		{
			let timer = Timer::new()
                .set_period(Duration::ms(50))
                .set_auto_reload(false)
                .create(|mut timer| {                    
                    let a = 1;
                }).unwrap();

			timer.start(Duration::infinite()).unwrap();

			CurrentTask::delay(Duration::ms(100))
		}

		// processor
		{
			#[derive(PartialEq, Copy, Clone, Debug)]
			enum ProcessorMsg {
				Val(usize),
				Shutdown
			}

			let processor: Processor<Message<ProcessorMsg>> = Processor::new(5).unwrap();
			let client_1 = processor.new_client().unwrap();
			let client_2 = processor.new_client_with_reply(5, Duration::ms(5)).unwrap();
			let client_3 = processor.new_client().unwrap();
			let client_4 = processor.new_client_with_reply(15, Duration::ms(5)).unwrap();

			let processor_task = Task::new().name("processor").start(move || {
				loop {
					if let Ok(m) = processor.get_receive_queue().receive(Duration::ms(10)) {
						match m.get_val() {
							ProcessorMsg::Val(v) => {
								let processed = v + 1;
								let r = processor.reply_val(m, ProcessorMsg::Val(processed), Duration::ms(10)).unwrap();
							},
							ProcessorMsg::Shutdown => { break; }
						}
					}
				}
				drop(processor);
				debug_print("Processor shutting down");
			}).unwrap();
			
			client_1.send_val(ProcessorMsg::Val(5), Duration::ms(5));
			client_2.send_val(ProcessorMsg::Val(6), Duration::ms(5));
			client_2.send_val(ProcessorMsg::Val(7), Duration::ms(5));
			client_2.call_val(ProcessorMsg::Val(8), Duration::ms(5));
			
			client_3.send_val(ProcessorMsg::Shutdown, Duration::ms(5));

			CurrentTask::delay(Duration::ms(50));
						
			assert_eq!(Err(FreeRtosError::ProcessorHasShutDown), client_4.call_val(ProcessorMsg::Val(2), Duration::ms(5)));
		}

		CurrentTask::delay(Duration::ms(100));		

		end_memory_usage = heap_allocated_memory();
		assert_eq!(start_memory_usage, end_memory_usage);

		exit_test(0);
	}).unwrap();


	start_kernel();

	1
}
