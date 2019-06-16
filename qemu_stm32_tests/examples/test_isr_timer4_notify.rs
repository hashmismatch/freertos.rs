#![no_main]
#![no_std]

#[macro_use]
extern crate qemu_stm32_tests;

use qemu_stm32_tests::prelude::v1::*;

static mut MAIN_TASK: Option<Task> = None;

freertos_rs_test!(TestIsrTimer4Notify);

pub struct TestIsrTimer4Notify;
impl Test for TestIsrTimer4Notify {
	fn run<T: Testbed>(tb: &T) {
		let main_task = Task::new().start(|| {
			T::start_timer4_50ms();

			let current_task = Task::current().unwrap();

			loop {
				match current_task.wait_for_notification(0, 0, Duration::ms(1000)) {
					Ok(value) => {
						T::debug_print(&format!("Received notification value {}", value));

						if value == 10 {
							T::ok()
						}
					},
					Err(e) => {
						T::debug_print(&format!("Error receiving notification: {:?}", e));
						T::exit_test(1);
					}
				};

			}


		}).unwrap();

		unsafe {
			MAIN_TASK = Some(main_task);
		}

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

	unsafe {
		if let Some(ref task) = MAIN_TASK {
			task.notify_from_isr(&context, TaskNotification::SetValue(c)).unwrap();
		}
	}
}
