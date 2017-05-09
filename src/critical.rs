use prelude::v1::*;
use base::*;
use shim::*;

pub struct CriticalRegion;
impl CriticalRegion {
    pub fn enter() -> Self {
        unsafe { freertos_rs_enter_critical(); }

        CriticalRegion
    }
}

impl Drop for CriticalRegion {
    fn drop(&mut self) {
        unsafe { freertos_rs_exit_critical(); }
    }
}

unsafe impl<T: Sync + Send> Send for ExclusiveData<T> {}
unsafe impl<T: Sync + Send> Sync for ExclusiveData<T> {}

/// Data protected with a critical region. Lightweight version of a mutex,
/// intended for simple data structures.
pub struct ExclusiveData<T: ?Sized> {
    data: UnsafeCell<T>
}

impl<T> ExclusiveData<T> {
    pub fn new(data: T) -> Self {
        ExclusiveData {
            data: UnsafeCell::new(data)
        }
    }

    pub fn lock(&self) -> Result<ExclusiveDataGuard<T>, FreeRtosError> {
        Ok(ExclusiveDataGuard {
            __data: &self.data,
            __lock: CriticalRegion::enter()
        })
    }
}

/// Holds the mutex until we are dropped
pub struct ExclusiveDataGuard<'a, T: ?Sized + 'a> {
    __data: &'a UnsafeCell<T>,
    __lock: CriticalRegion
}

impl<'mutex, T: ?Sized> Deref for ExclusiveDataGuard<'mutex, T> {
    type Target = T;

    fn deref<'a>(&'a self) -> &'a T {
        unsafe { &*self.__data.get() }
    }
}

impl<'mutex, T: ?Sized> DerefMut for ExclusiveDataGuard<'mutex, T> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut T {
        unsafe { &mut *self.__data.get() }
    }
}

