#![no_main]
#![no_std]

#[macro_use]
extern crate qemu_stm32_tests;

use qemu_stm32_tests::prelude::v1::*;
use qemu_stm32_tests::freertos_rs::patterns::processor::*;

freertos_rs_test!(TestProcessor);

pub struct TestProcessor;
impl Test for TestProcessor {
	fn run<T: Testbed>(tb: &T) {
        let main_task = Task::new().name("main").start(|| {

            let shutdown = 255;

            let processor: Processor<InputMessage<usize>, usize> = Processor::new(5).unwrap();
            let client_1 = processor.new_client().unwrap();
            let client_2 = processor.new_client_with_reply(1, Duration::ms(100)).unwrap();

            let processor_task = Task::new().name("processor").start(move || {

                loop {
                    if let Ok(msg) = processor.get_receive_queue().receive(Duration::ms(10)) {

                        if msg.get_val() == shutdown {
                            break;
                        }

                        T::debug_print(&format!("Received val {}", msg.get_val()));
                        let processed_message = msg.get_val() + 1;
                        processor.reply_val(msg, processed_message, Duration::ms(10)).expect("Failed to send the reply");
                        T::debug_print("Processed.");
                    }
                }

                T::debug_print("Shutting down.");

            }).unwrap();

                    
            client_1.send_val(1, Duration::ms(100));
            
            let processed = client_2.call_val(2, Duration::ms(100)).expect("Missing the reply from the processor");
            assert_eq!(3, processed);

            client_1.send_val(shutdown, Duration::ms(100));

            CurrentTask::delay(Duration::ms(10));

            assert_eq!(Err(FreeRtosError::ProcessorHasShutDown), client_1.send_val(1, Duration::zero()));
            assert_eq!(Err(FreeRtosError::ProcessorHasShutDown), client_2.send_val(1, Duration::zero()));

            T::ok();

        }).unwrap();

        T::start_kernel();
    }
}
