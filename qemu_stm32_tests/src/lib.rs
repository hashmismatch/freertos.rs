#![no_std]

#![feature(alloc_error_handler)]

#[global_allocator]
static GLOBAL: freertos_alloc::FreeRtosAllocator = freertos_alloc::FreeRtosAllocator;

use core::panic::PanicInfo;
use testbed::Testbed;

#[panic_handler]
#[inline(never)]
fn panic(info: &PanicInfo) -> ! {
	use core::fmt;
	use core::fmt::Write;
	use alloc::string::*;

	testbed::QemuTestbed::debug_print("Panicked!");

	{
		testbed::QemuTestbed::debug_print(&format!("{:}", info));
	}

	testbed::QemuTestbed::exit_test(98);
	loop {}
}


#[macro_use]
#[macro_export]
pub extern crate alloc;

pub extern crate freertos_rs;

#[macro_use]
#[macro_export]
pub extern crate lazy_static;

pub mod testbed;
pub mod freertos_alloc;
pub mod prelude;
pub mod utils;


pub trait Test {
	fn run<T: testbed::Testbed>(tb: &T);
}

pub fn run_test<T: Test>() {
	let tb = testbed::QemuTestbed;
	T::run(&tb);
}

#[macro_export]
macro_rules! freertos_rs_test {
	($test: ty) => {
		#[no_mangle]
		pub extern "C" fn testbed_main() -> i8 {
			run_test::<$test>();
			0
		}
	};
}