#![no_std]
#![feature(lang_items)]

#![feature(alloc)]
#![feature(collections)]
#![feature(fnbox)]

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "eh_unwind_resume"] extern fn eh_unwind_resume() {}

#[lang = "panic_fmt"]
#[inline(never)]
extern fn panic_fmt(msg: core::fmt::Arguments, file_line: &(&'static str, u32)) -> ! {
	use core::fmt;
	use core::fmt::Write;
	use collections::string::*;

	debug_print("Panicked!");
	
	{
		let mut s = String::new();
		s.write_fmt(msg);
		debug_print(&s);
	}
    
	exit_test(98);
	loop {}
}


#[macro_use]
extern crate alloc;
#[macro_use]
extern crate collections;

extern crate freertos_rs;

#[macro_use]
extern crate lazy_static;

extern {
	fn testbed_println(line: *const u8, line_len: u16);
	fn testbed_start_kernel();
	fn testbed_return(return_code: i8);
	fn testbed_allocated_memory() -> u32;
	fn testbed_init_timer4_50ms_isr();
}

pub fn debug_print(s: &str) {
	let s = s.as_bytes();	
	unsafe {
		testbed_println(s.as_ptr(), s.len() as u16);
	}
}

pub fn start_kernel() {
	unsafe {
		testbed_start_kernel();
	}
}

pub fn exit_test(return_code: i8) {
	unsafe {
		testbed_return(return_code);
	}
}

pub fn heap_allocated_memory() -> u32 {
	unsafe {
		testbed_allocated_memory()
	}
}

pub fn start_timer4_50ms() {
	unsafe {
		testbed_init_timer4_50ms_isr();
	}
}


mod prelude;
mod utils;

pub mod test_basics;
pub mod test_delay;
pub mod test_mutex;
pub mod test_mem_leaks1;
pub mod test_isr_timer4_queue;
pub mod test_isr_timer4_notify;
pub mod test_sample1;





#[repr(u8)]
enum c_void {
    __variant1,
    __variant2,
}

extern {
    fn pvPortMalloc(size: u32) -> *mut c_void;
    fn pvPortRealloc(p: *mut c_void, size: u32) -> *mut c_void;
    fn vPortFree(p: *mut c_void);	
}

#[no_mangle]
pub extern fn __rust_allocate(size: usize, align: usize) -> *mut u8 {
	unsafe { pvPortMalloc(size as u32) as *mut u8 }
}

#[no_mangle]
pub extern fn __rust_deallocate(ptr: *mut u8, old_size: usize, align: usize) {
	unsafe { vPortFree(ptr as *mut c_void) }
}

#[no_mangle]
pub extern fn __rust_reallocate(ptr: *mut u8, old_size: usize, size: usize, align: usize) -> *mut u8 {
	unsafe { pvPortRealloc(ptr as *mut c_void, size as u32) as *mut u8 }
}
