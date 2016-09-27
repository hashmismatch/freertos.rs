use super::*;
use prelude::v1::*;

use freertos_rs::*;

lazy_static! {
    static ref QUEUE: Queue<u32> = Queue::new(15).unwrap();    
}

#[no_mangle]
pub extern fn test_isr_timer4_queue() -> i8 {

	let main = Task::new().start(|| {

		// trigger init of the queue object (lazy_static)
		let ref q = QUEUE;

		start_timer4_50ms();
		
		for i in 1..15 {
			match QUEUE.receive(Duration::ms(1000)) {
				Ok(received) => {
					debug_print(&format!("Received queue item '{}'", received));

					if received == 10 {
						exit_test(0);
					}
				},
				Err(e) => {
					debug_print(&format!("Error receiving item: {:?}", e));
				}
			}
		}

		exit_test(1);

	}).unwrap();

	start_kernel();

	1
}

static mut COUNTER: u32 = 0;

#[no_mangle]
pub extern fn test_isr_timer4_queue_timer4_isr() {
	let mut context = InterruptContext::new();
	let c = unsafe {
		COUNTER += 1;
		COUNTER
	};
	QUEUE.send_from_isr(&mut context, c).unwrap();
}
