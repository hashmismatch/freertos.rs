#![no_main]
#![no_std]

#[macro_use]
extern crate qemu_stm32_tests;

use qemu_stm32_tests::prelude::v1::*;

freertos_rs_test!(TestDelay);

pub struct TestDelay;
impl Test for TestDelay {
	fn run<T: Testbed>(tb: &T) {
		let main_task = Task::new().name("main").start(|| {
			let start = FreeRtosUtils::get_tick_count();

			let counter = Arc::new(Mutex::new(0).unwrap());

			{
				let counter = counter.clone();
				let delay_task = Task::new().name("delay").start(move || {
					for _ in 0..10 {
						CurrentTask::delay(Duration::ms(100));

						// increase the counter and immediately release it
						{
							let mut counter = counter.lock(Duration::infinite()).unwrap();
							*counter += 1;
						}
					}
				}).unwrap();
			}
			
			CurrentTask::delay(Duration::ms(550));

			{
				let counter = counter.lock(Duration::infinite()).unwrap();
				assert_eq!(*counter, 5);
			}

			CurrentTask::delay(Duration::ms(500));

			{
				let counter = counter.lock(Duration::infinite()).unwrap();
				assert_eq!(*counter, 10);
			}

			// negative test: the counter should never increment
			{
				let counter = Arc::new(Mutex::new(0).unwrap());
				{
					let counter = counter.clone();
					let task = Task::new().name("delay_long").start(move || {
						for _ in 0..10 {
							CurrentTask::delay(Duration::ms(1000));

							// increase the counter and immediately release it
							{
								let mut counter = counter.lock(Duration::infinite()).unwrap();
								*counter += 1;
							}
						}
					});
				}

				CurrentTask::delay(Duration::ms(500));
				let counter = counter.lock(Duration::infinite()).unwrap();
				assert_eq!(*counter, 0);
			}

			T::exit_test(0);
		});

		let main_task = main_task.unwrap();
		T::start_kernel();
	}
}
