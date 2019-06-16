
#[repr(u8)]
enum c_void {
    __variant1,
    __variant2,
}

extern {
    fn pvPortMalloc(size: u32) -> *mut c_void;
    fn vPortFree(p: *mut c_void);	
}

#[alloc_error_handler]
fn foo(_: core::alloc::Layout) -> ! {
    panic!("OOM!");
}

use core::alloc::{GlobalAlloc, Layout};

pub struct FreeRtosAllocator;

unsafe impl GlobalAlloc for FreeRtosAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        pvPortMalloc(layout.size() as u32) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        vPortFree(ptr as *mut c_void)
    }
}