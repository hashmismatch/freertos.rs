#![no_main]
#![no_std]

#[macro_use]
extern crate qemu_stm32_tests;

use qemu_stm32_tests::prelude::v1::*;

freertos_rs_test!(TestStats);

pub struct TestStats;
impl Test for TestStats {
	fn run<T: Testbed>(tb: &T) {
        let main_task = Task::new().name("main").start(|| {

            let task = Task::new().name("test1").start(|| {
                CurrentTask::delay(Duration::ms(1000))
            }).unwrap();

            let task = Task::new().name("test2").stack_size(1024).priority(TaskPriority(3)).start(|| {
                CurrentTask::delay(Duration::ms(1000))
            }).unwrap();

            CurrentTask::delay(Duration::ms(10));

            let all_tasks_count = FreeRtosUtils::get_number_of_tasks();
            assert_eq!(5, all_tasks_count);
            
            let tasks = FreeRtosUtils::get_all_tasks(Some(10));
            assert_eq!(5, tasks.tasks.len());
            T::debug_print(&format!("{}", tasks));

            T::ok();

        }).unwrap();

        T::start_kernel();
    }
}
