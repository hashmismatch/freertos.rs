#![no_main]
#![no_std]

#[macro_use]
extern crate qemu_stm32_tests;

use qemu_stm32_tests::prelude::v1::*;


freertos_rs_test!(TestTimers);

pub struct TestTimers;
impl Test for TestTimers {
	fn run<T: Testbed>(tb: &T) {
        
        let main_task = Task::new().name("main").start(|| {

            {
                let counter_a = Arc::new(Mutex::new(0).unwrap());
                let counter_b = Arc::new(Mutex::new(100).unwrap());

                {
                    let timer_a = {
                        let counter_a = counter_a.clone();        
                        
                        Timer::new(Duration::ms(50))
                            .set_auto_reload(true)
                            .create(move |mut timer| {                    
                                if let Ok(mut counter_a) = counter_a.lock(Duration::ms(5)) {
                                    *counter_a += 1;
                                }
                            }).unwrap()
                    };

                    let timer_b = {
                        let counter_b = counter_b.clone();        
                        
                        Timer::new(Duration::ms(100))
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
                
                let counter_a = counter_a.lock(Duration::infinite()).unwrap();
                assert_eq!(4, *counter_a);

                let counter_b = counter_b.lock(Duration::infinite()).unwrap();
                assert_eq!(101, *counter_b);
            }

            // non-auto reloading timer test
            {
                let counter_a = Arc::new(Mutex::new(0).unwrap());

                let timer_a = {
                    let counter_a = counter_a.clone();        
                    
                    Timer::new(Duration::ms(50))
                        .set_auto_reload(false)
                        .create(move |mut timer| {                    
                            if let Ok(mut counter_a) = counter_a.lock(Duration::ms(5)) {
                                *counter_a += 1;
                            }
                        }).unwrap()
                };

                CurrentTask::delay(Duration::ms(100));

                assert_eq!(0, *counter_a.lock(Duration::infinite()).unwrap());
                
                timer_a.start(Duration::ms(1)).unwrap();

                CurrentTask::delay(Duration::ms(200));
                assert_eq!(1, *counter_a.lock(Duration::infinite()).unwrap());

                timer_a.start(Duration::ms(1)).unwrap();
                CurrentTask::delay(Duration::ms(200));
                
                assert_eq!(2, *counter_a.lock(Duration::infinite()).unwrap());
            }

            T::ok();

        }).unwrap();

        T::start_kernel();
    }
}