#![no_main]
#![no_std]

#[macro_use]
extern crate qemu_stm32_tests;

use qemu_stm32_tests::prelude::v1::*;
use qemu_stm32_tests::utils::Rnd;

freertos_rs_test!(TestMutex);

pub struct TestMutex;
impl Test for TestMutex {
	fn run<T: Testbed>(tb: &T) {

		let n = 3000;
		let m = 32;
		let t = 6;

		let mut mutexes = vec![];
		for i in 0..m {
			let m = Arc::new(Mutex::new(0).unwrap());
			mutexes.push(m);
		}
		let mut mutexes = Arc::new(Mutex::new(mutexes).unwrap());

		let mut total = Arc::new(Mutex::new(0).unwrap());
		let mut rnd = Arc::new(Mutex::new(Rnd::new(100)).unwrap());

		let main_task = Task::new().name("main").start(move || {

			for i in 1..(t+1) {
				let t = format!("task_{}", i);
				let mut mutexes = mutexes.clone();
				let mut rnd = rnd.clone();
				let mut total = total.clone();

				let main_task = Task::current().unwrap();

				Task::new().name(&t).start(move || {

					for _ in 0..n {
						let (next_mutex_idx, delay_ms) = {
							let mut rnd = rnd.lock(Duration::infinite()).unwrap();
							(rnd.next_num(m) as usize, rnd.next_num(5) as u32)
						};

						let mutex = {
							let m = mutexes.lock(Duration::infinite()).unwrap();
							m[next_mutex_idx].clone()
						};

						{
							let mut m = mutex.lock(Duration::infinite()).unwrap();
							*m += 1;
							CurrentTask::delay(Duration::ms(delay_ms));
						}

						{
							let mut total = total.lock(Duration::infinite()).unwrap();
							*total += 1;
						}
					}

					main_task.notify(TaskNotification::Increment);

				}).unwrap();
			}

			let main_task = Task::current().unwrap();
			let mut finished_tasks = 0;
			loop {			
				let nv = main_task.take_notification(true, Duration::infinite());
				finished_tasks += nv;
				if finished_tasks == t {
					let total = total.lock(Duration::infinite()).unwrap();
					assert_eq!(*total, n * t);

					T::exit_test(0);
				}
			}		

		}).unwrap();

		T::start_kernel();
	}
}