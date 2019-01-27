
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
pub extern fn __rust_alloc(size: usize, align: usize, err: *mut u8) -> *mut u8 {
    unsafe { pvPortMalloc(size as u32) as *mut u8 }
}

#[no_mangle]
pub extern fn rust_oom(err: *const u8) -> ! {
    panic!("OOM");
}

#[no_mangle]
pub extern fn __rust_dealloc(ptr: *mut u8, size: usize, align: usize) {
    unsafe { vPortFree(ptr as *mut c_void) }
}

#[no_mangle]
pub extern fn __rust_realloc(ptr: *mut u8,
                    old_size: usize,
                    old_align: usize,
                    new_size: usize,
                    new_align: usize,
                    err: *mut u8) -> *mut u8
{
    unsafe { pvPortRealloc(ptr as *mut c_void, new_size as u32) as *mut u8 }
}

#[no_mangle]
pub extern fn __rust_alloc_zeroed(size: usize, align: usize, err: *mut u8) -> *mut u8 {
    unsafe { 
		let ptr = __rust_alloc(size, align, err);
		if !ptr.is_null() {
			::core::ptr::write_bytes(ptr, 0, size);
		}
		ptr
	}
}