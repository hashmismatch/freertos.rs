use super::*;
use prelude::v1::*;

use freertos_rs::*;

#[no_mangle]
pub extern fn test_stats() -> i8 {
	
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
        debug_print(&format!("All tasks {:#?}", tasks));

		exit_test(0);

	}).unwrap();

	start_kernel();
	
	1
}