use super::*;
use prelude::v1::*;

use freertos_rs::*;

#[no_mangle]
pub extern fn test_delay() -> i8 {
	let main_task = Task::new().name("main").start(|| {
		let start = CurrentTask::get_tick_count();

		let counter = Arc::new(Mutex::new(0).unwrap());

		{
			let counter = counter.clone();
			let delay_task = Task::new().name("delay").start(move || {
				for _ in 0..10 {
					CurrentTask::delay(Duration::ms(100));

					// increase the counter and immediately release it
					{
						let mut counter = counter.lock(Duration::Infinite).unwrap();
						*counter += 1;
					}
				}
			}).unwrap();
		}
		
		CurrentTask::delay(Duration::ms(550));

		{
			let counter = counter.lock(Duration::Infinite).unwrap();
			assert_eq!(*counter, 5);
		}

		CurrentTask::delay(Duration::ms(500));

		{
			let counter = counter.lock(Duration::Infinite).unwrap();
			assert_eq!(*counter, 10);
		}

		exit_test(0);
	});
	let main_task = main_task.unwrap();

	start_kernel();

	1
}