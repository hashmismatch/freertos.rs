
pub trait Testbed {
    fn debug_print(s: &str);
    fn start_kernel() -> !;
    fn exit_test(return_code: i8) -> !;
    fn heap_allocated_memory() -> u32;
    fn start_timer4_50ms();

    fn ok() -> ! {
        Self::exit_test(0)
    }
}

pub struct QemuTestbed;

extern {
	fn testbed_println(line: *const u8, line_len: u16);
	fn testbed_start_kernel();
	fn testbed_return(return_code: i8);
	fn testbed_allocated_memory() -> u32;
	fn testbed_init_timer4_50ms_isr();
}

impl Testbed for QemuTestbed {
    fn debug_print(s: &str) {
        let s = s.as_bytes();	
        unsafe {
            testbed_println(s.as_ptr(), s.len() as u16);
        }
    }

    fn start_kernel() -> ! {
        unsafe {
            testbed_start_kernel();
        }

        unreachable!()
    }

    fn exit_test(return_code: i8) -> ! {
        unsafe {
            testbed_return(return_code);
        }

        unreachable!()
    }

    fn heap_allocated_memory() -> u32 {
        unsafe {
            testbed_allocated_memory()
        }
    }

    fn start_timer4_50ms() {
        unsafe {
            testbed_init_timer4_50ms_isr();
        }
    }    
}
