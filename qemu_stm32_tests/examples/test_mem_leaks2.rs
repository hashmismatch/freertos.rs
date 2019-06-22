#![no_main]
#![no_std]

#[macro_use]
extern crate qemu_stm32_tests;

use qemu_stm32_tests::prelude::v1::*;

freertos_rs_test!(TestMemLeaks2);

pub struct TestMemLeaks2;
impl Test for TestMemLeaks2 {
	fn run<T: Testbed>(tb: &T) {
		let main_task = Task::new().name("main").start(|| {
			let start_memory_usage = T::heap_allocated_memory();
			let mut end_memory_usage = 0;


            T::debug_print("main.1");
            let here = "hello";
            T::debug_print(&here);
            
            let task = Task::new().name("i1").start(move || {
                T::debug_print("i1.1");
                T::debug_print(&here);
                T::debug_print("i1.2");
            }).unwrap();
            T::debug_print("main.3");

			CurrentTask::delay(Duration::ms(200));

            T::debug_print("main.4");

			end_memory_usage = T::heap_allocated_memory();
			assert_eq!(start_memory_usage, end_memory_usage, "Mem usage #1");
			T::ok();
		}).unwrap();

		T::start_kernel();
	}
}
