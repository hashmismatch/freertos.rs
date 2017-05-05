use prelude::v1::*;
use base::*;
use units::*;
use shim::*;

unsafe impl<T: Sync + Send> Send for Mutex<T> {}
unsafe impl<T: Sync + Send> Sync for Mutex<T> {}

/// Mutual exclusion access to a contained value. Can be recursive -
/// the current owner of a lock can re-lock it.
pub struct Mutex<T: ?Sized> {
    mutex: FreeRtosSemaphoreHandle,
    recursive: bool,
    data: UnsafeCell<T>,
}

impl<T: ?Sized> fmt::Debug for Mutex<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Mutex address: {:?}, recursive: {:?}",
               self.mutex,
               self.recursive)
    }
}

impl<T> Mutex<T> {
    fn new_mutex(t: T, recursive: bool) -> Result<Mutex<T>, FreeRtosError> {
        unsafe {
            let m = if recursive {
                freertos_rs_create_recursive_mutex()
            } else {
                freertos_rs_create_mutex()
            };
            if m == 0 as *const _ {
                return Err(FreeRtosError::OutOfMemory);
            }

            let mutex = Mutex {
                mutex: m,
                recursive: recursive,
                data: UnsafeCell::new(t),
            };

            Ok(mutex)
        }
    }

    /// Create a new mutex with the given inner value
    pub fn new(t: T) -> Result<Mutex<T>, FreeRtosError> {
        Self::new_mutex(t, false)
    }

    /// Create a new recursive mutex with the given inner value
    pub fn new_recursive(t: T) -> Result<Mutex<T>, FreeRtosError> {
        Self::new_mutex(t, true)
    }

    /// Try to obtain a lock and mutable access to our inner value
    pub fn lock<D: DurationTicks>(&self, max_wait: D) -> Result<MutexGuard<T>, FreeRtosError> {
        unsafe {
            let res = if self.recursive {
                freertos_rs_take_recursive_mutex(self.mutex, max_wait.to_ticks())
            } else {
                freertos_rs_take_mutex(self.mutex, max_wait.to_ticks())
            };

            if res != 0 {
                return Err(FreeRtosError::MutexTimeout);
            }

            Ok(MutexGuard {
                __mutex: self.mutex,
                __recursive: self.recursive,
                __data: &self.data,
            })
        }
    }

    /// Consume the mutex and return its inner value
    pub fn into_inner(self) -> T {
        // Manually deconstruct the structure, because it implements Drop
        // and we cannot move the data value out of it.
        unsafe {
            let (mutex, data) = {
                let Mutex { ref mutex, recursive: _, ref data } = self;
                (ptr::read(mutex), ptr::read(data))
            };
            mem::forget(self);

            freertos_rs_delete_semaphore(mutex);

            data.into_inner()
        }
    }
}

impl<T: ?Sized> Drop for Mutex<T> {
    fn drop(&mut self) {
        unsafe {
            freertos_rs_delete_semaphore(self.mutex);
        }
    }
}

/// Holds the mutex until we are dropped
pub struct MutexGuard<'a, T: ?Sized + 'a> {
    __mutex: FreeRtosSemaphoreHandle,
    __recursive: bool,
    __data: &'a UnsafeCell<T>,
}

impl<'mutex, T: ?Sized> Deref for MutexGuard<'mutex, T> {
    type Target = T;

    fn deref<'a>(&'a self) -> &'a T {
        unsafe { &*self.__data.get() }
    }
}

impl<'mutex, T: ?Sized> DerefMut for MutexGuard<'mutex, T> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut T {
        unsafe { &mut *self.__data.get() }
    }
}

impl<'a, T: ?Sized> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        unsafe {
            if self.__recursive {
                freertos_rs_give_recursive_mutex(self.__mutex);
            } else {
                freertos_rs_give_mutex(self.__mutex);
            }
        }
    }
}
