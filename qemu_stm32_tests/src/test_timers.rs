use super::*;
use prelude::v1::*;

use freertos_rs::*;

#[no_mangle]
pub extern fn test_timers() -> i8 {
	
	let main_task = Task::new().name("main").start(|| {

        let counter_a = Arc::new(Mutex::new(0).unwrap());
        let counter_b = Arc::new(Mutex::new(100).unwrap());

        {
            let timer_a = {
                let counter_a = counter_a.clone();        
                
                Timer::new()
                    .set_period(Duration::ms(50))
                    .set_auto_reload(true)
                    .create(move |mut timer| {                    
                        if let Ok(mut counter_a) = counter_a.lock(Duration::ms(5)) {
                            *counter_a += 1;
                        }
                    }).unwrap()
            };

            let timer_b = {
                let counter_b = counter_b.clone();        
                
                Timer::new()
                    .set_period(Duration::ms(100))
                    .set_auto_reload(false)
                    .create(move |mut timer| {                    
                        if let Ok(mut counter_b) = counter_b.lock(Duration::ms(5)) {
                            *counter_b += 1;
                        }
                    }).unwrap()
            };

            timer_a.start(Duration::ms(1)).unwrap();
            timer_b.start(Duration::ms(1)).unwrap();
            
            CurrentTask::delay(Duration::ms(225));

            drop(timer_a);
            drop(timer_b);
        }
        
        if let Ok(counter_a) = counter_a.lock(Duration::infinite()) {
            assert_eq!(4, *counter_a);
        }

        if let Ok(counter_b) = counter_b.lock(Duration::infinite()) {
            assert_eq!(101, *counter_b);
        }


		exit_test(0);

	}).unwrap();

	start_kernel();
	
	1
}