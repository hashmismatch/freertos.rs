use shim::*;
use base::FreeRtosTickType;

pub trait DurationTicks : Copy + Clone {
    /// Convert to ticks, the internal time measurement unit of FreeRTOS
    fn to_ticks(&self) -> FreeRtosTickType;
}

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

    pub fn to_ms(&self) -> u32 {
        unsafe { self.ticks * freertos_rs_get_portTICK_PERIOD_MS() } 
    }
}

impl DurationTicks for Duration {
    fn to_ticks(&self) -> FreeRtosTickType {
        self.ticks
    }
}