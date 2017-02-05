use shim::*;

/// Time unit used by FreeRTOS, passed to the scheduler as ticks.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Duration {
    ticks: u32
}

impl Duration {
    /// Milliseconds constructor
    pub fn ms(milliseconds: u32) -> Duration {
        Self::ticks(unsafe { milliseconds / freertos_rs_get_portTICK_PERIOD_MS() })
    }

    pub fn ticks(ticks: u32) -> Duration {
        Duration { ticks: ticks }
    }

    /// An infinite duration
    pub fn infinite() -> Duration {
        Self::ticks(unsafe { freertos_rs_max_wait() })
    }

    /// A duration of zero, for non-blocking calls
    pub fn zero() -> Duration {
        Self::ticks(0)
    }

    /// Smallest unit of measurement, one tick
    pub fn eps() -> Duration {
        Self::ticks(1)
    }

    /// Convert to ticks, the internal time measurement unit of FreeRTOS
    pub fn to_ticks(&self) -> u32 {
        self.ticks
    }

    pub fn to_ms(&self) -> u32 {
        unsafe { self.ticks * freertos_rs_get_portTICK_PERIOD_MS() } 
    }
}
