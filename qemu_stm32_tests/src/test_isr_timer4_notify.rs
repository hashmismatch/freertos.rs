use super::*;
use prelude::v1::*;

use freertos_rs::*;

static mut MAIN_TASK: Option<Task> = None;

#[no_mangle]
pub extern fn test_isr_timer4_notify() -> i8 {
	let main_task = Task::new().start(|| {
		start_timer4_50ms();

		let current_task = Task::current().unwrap();

		loop {
			match current_task.wait_for_notification(0, 0, Duration::ms(1000)) {
				Ok(value) => {
					debug_print(&format!("Received notification value {}", value));

					if value == 10 {
						exit_test(0);
					}
				},
				Err(e) => {
					debug_print(&format!("Error receiving notification: {:?}", e));
					exit_test(1);
				}
			};

		}


	}).unwrap();

	unsafe {
		MAIN_TASK = Some(main_task);
	}

	start_kernel();

	1
}

static mut COUNTER: u32 = 0;

#[no_mangle]
pub extern fn test_isr_timer4_notify_timer4_isr() {
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
