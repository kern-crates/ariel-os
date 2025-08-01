//! Provides on-board benchmarking facilities.

#![cfg_attr(not(any(test, context = "native")), no_std)]
#![deny(clippy::pedantic)]
#![deny(missing_docs)]

cfg_if::cfg_if! {
    if #[cfg(context = "cortex-m")] {
        mod cortexm;
        use cortexm as bench;
    } else if #[cfg(context = "esp")] {
        mod esp;
        use esp as bench;
    } else if #[cfg(context = "native")] {
        mod native;
        use native as bench;
    } else if #[cfg(context = "ariel-os")] {
        // When run with laze but the MCU family is not supported
        compile_error!("benchmarking is not supported for this MCU family");
    } else {
        // Provide a default bench module, for HAL-independent tooling
        mod bench {
            use crate::Error;

            /// Benchmarks "time" required to run the provided function.
            ///
            /// Runs the provided function `iterations` times, and returns the mean number of system timer
            /// increments per iteration.
            ///
            /// # Errors
            ///
            /// Returns [`Error::SystemTimerWrapped`] if the system timer counter has wrapped when
            /// benchmarking.
            #[allow(unused_variables)]
            pub fn benchmark<F: FnMut()>(iterations: usize, f: F) -> Result<usize, Error> {
                unimplemented!();
            }
        }
    }
}

pub use bench::benchmark;

/// Possible errors happening when benchmarking.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// The system timer wrapped when benchmarking.
    SystemTimerWrapped,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::SystemTimerWrapped => write!(f, "system timer wrapped"),
        }
    }
}

impl core::error::Error for Error {}
