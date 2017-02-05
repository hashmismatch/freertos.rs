/// Basic error type for the library.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FreeRtosError {
    OutOfMemory,
    Timeout,
    QueueFull,
    StringConversionError,
    TaskNotFound,
}

unsafe impl Send for CVoid {}
#[repr(u32)]
pub enum CVoid {
    _Variant1,
    _Variant2,
}

pub type FreeRtosVoidPtr = *const CVoid;
pub type FreeRtosMutVoidPtr = *mut CVoid;
pub type FreeRtosCharPtr = *const u8;
pub type FreeRtosChar = u8;

pub type FreeRtosBaseType = i32;
pub type FreeRtosUBaseType = u32;
pub type FreeRtosTickType = u32;
pub type FreeRtosBaseTypeMutPtr = *mut FreeRtosBaseType;

pub type FreeRtosTaskHandle = *const CVoid;
pub type FreeRtosMutTaskHandle = *mut CVoid;
pub type FreeRtosQueueHandle = *const CVoid;
pub type FreeRtosSemaphoreHandle = *const CVoid;
pub type FreeRtosTaskFunction = *const CVoid;
pub type FreeRtosTimerHandle = *const CVoid;
pub type FreeRtosTimerCallback = *const CVoid;