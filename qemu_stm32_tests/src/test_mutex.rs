use super::*;
use prelude::v1::*;
use utils::*;

use freertos_rs::*;

#[no_mangle]
pub extern fn test_mutex() -> i8 {
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
						let mut rnd = rnd.lock(Duration::Infinite).unwrap();
						(rnd.next_num(m) as usize, rnd.next_num(5) as u32)
					};

					//debug_print(&format!("Next mutex {}", next_mutex_idx));

					let mutex = {
						let m = mutexes.lock(Duration::Infinite).unwrap();
						m[next_mutex_idx].clone()
					};

					{
						let mut m = mutex.lock(Duration::Infinite).unwrap();
						*m += 1;
						CurrentTask::delay(Duration::ms(delay_ms));
					}

					{
						let mut total = total.lock(Duration::Infinite).unwrap();
						*total += 1;
					}
				}

				//debug_print(&format!("Task {} finished.", i));

				main_task.notify(TaskNotification::Increment);

			}).unwrap();
		}

		let main_task = Task::current().unwrap();
		let mut finished_tasks = 0;
		loop {			
			let nv = main_task.take_notification(true, Duration::Infinite);
			finished_tasks += nv;
			if finished_tasks == t {
				let total = total.lock(Duration::Infinite).unwrap();
				assert_eq!(*total, n * t);

				exit_test(0);
			}
		}		

	}).unwrap();

	start_kernel();

	1
}