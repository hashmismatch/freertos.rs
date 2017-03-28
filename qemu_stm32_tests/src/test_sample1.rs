use super::*;
use prelude::v1::*;

use freertos_rs::*;
use freertos_rs::patterns::compute_task::*;

#[no_mangle]
pub extern fn test_sample1() -> i8 {
	
	let main_task = Task::new().name("main").start(|| {

		// A shared queue
		let queue = Arc::new(Queue::new(10).unwrap());

		// Task that consumes integers from the shared queue. Returns the
		// summed value when a new integer hasn't been posted for 100 ms.
		let sum_task = {
			let queue = queue.clone();

			Task::new().name("sum").compute(move || {				
				let mut sum = 0;

				loop {
					if let Ok(val) = queue.receive(Duration::ms(100)) {
						sum += val;
					} else {
						break;
					}
				}

				sum
			}).unwrap()
		};

		// Send the integers to the shared queue
		for i in 1..11 {
			queue.send(i, Duration::ms(15)).unwrap();
		}

		// Wait for the compute task to finish summing
		let sum = sum_task.into_result(Duration::infinite()).unwrap();

		// Check the result
		assert_eq!(55, sum);

		exit_test(0);

	}).unwrap();

	start_kernel();
	
	1
}