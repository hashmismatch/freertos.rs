use shim::*;

/// Time unit used by FreeRTOS, passed to the scheduler as ticks.
#[derive(Debug, Copy, Clone)]
pub enum Duration {
    Ticks(u32),
    Milliseconds(u32),
    Infinite,
}

impl Duration {
    /// Milliseconds constructor
    pub fn ms(milliseconds: u32) -> Duration {
        Duration::Milliseconds(milliseconds)
    }

    /// An infinite duration
    pub fn infinite() -> Duration {
        Duration::Infinite
    }

    /// Convert to ticks, the internal time measurement unit of FreeRTOS
    pub fn to_ticks(&self) -> u32 {
        match *self {
            Duration::Ticks(t) => t,
            Duration::Milliseconds(ms) => unsafe { ms / freertos_rs_get_portTICK_PERIOD_MS() },
            Duration::Infinite => unsafe { freertos_rs_max_wait() },
        }
    }
}
