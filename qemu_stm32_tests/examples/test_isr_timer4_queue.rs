#![no_main]
#![no_std]

#[macro_use]
extern crate qemu_stm32_tests;

use qemu_stm32_tests::prelude::v1::*;

use qemu_stm32_tests::lazy_static::lazy_static;

lazy_static! {
    static ref QUEUE: Queue<u32> = Queue::new(15).unwrap();    
}

freertos_rs_test!(TestIsrTimer4Queue);

pub struct TestIsrTimer4Queue;
impl Test for TestIsrTimer4Queue {
	fn run<T: Testbed>(tb: &T) {
		let main = Task::new().start(|| {

			// trigger init of the queue object (lazy_static)
			let ref q = QUEUE;

			T::start_timer4_50ms();
			
			for i in 1..15 {
				match QUEUE.receive(Duration::ms(1000)) {
					Ok(received) => {
						T::debug_print(&format!("Received queue item '{}'", received));

						if received == 10 {
							T::ok()
						}
					},
					Err(e) => {
						T::debug_print(&format!("Error receiving item: {:?}", e));
					}
				}
			}

			T::exit_test(1);

		}).unwrap();

		T::start_kernel();
	}
}

static mut COUNTER: u32 = 0;

#[no_mangle]
pub extern fn testbed_timer4_isr() {
	let mut context = InterruptContext::new();
	let c = unsafe {
		COUNTER += 1;
		COUNTER
	};
	QUEUE.send_from_isr(&mut context, c).unwrap();
}
