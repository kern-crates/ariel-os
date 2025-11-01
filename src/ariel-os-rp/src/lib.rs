//! Items specific to the Raspberry Pi RP MCUs.

#![no_std]
#![cfg_attr(nightly, feature(doc_cfg))]
#![deny(missing_docs)]

pub mod gpio;

#[doc(hidden)]
pub mod peripheral {
    pub use embassy_rp::Peripheral;
}

#[cfg(context = "rp235xa")]
mod picotool;

#[cfg(feature = "wifi")]
mod wifi;

#[cfg(feature = "ble")]
#[doc(hidden)]
pub mod ble;

#[cfg(any(feature = "ble-cyw43", feature = "wifi-cyw43"))]
#[doc(hidden)]
pub mod cyw43;

#[cfg(feature = "hwrng")]
#[doc(hidden)]
pub mod hwrng;

#[cfg(feature = "i2c")]
pub mod i2c;

#[doc(hidden)]
pub mod identity {
    use ariel_os_embassy_common::identity;

    pub type DeviceId = identity::NoDeviceId<identity::NotImplemented>;
}

#[cfg(feature = "spi")]
pub mod spi;

#[cfg(feature = "uart")]
pub mod uart;

#[cfg(feature = "storage")]
#[doc(hidden)]
pub mod storage;

#[cfg(feature = "usb")]
#[doc(hidden)]
pub mod usb;

#[doc(hidden)]
pub use embassy_rp::OptionalPeripherals;

pub use embassy_rp::peripherals;

#[cfg(feature = "executor-interrupt")]
#[doc(hidden)]
pub use embassy_executor::InterruptExecutor as Executor;
#[cfg(feature = "executor-interrupt")]
#[doc(hidden)]
pub use embassy_rp::interrupt;

#[cfg(feature = "executor-interrupt")]
ariel_os_embassy_common::executor_swi!(SWI_IRQ_1);

#[cfg(feature = "executor-interrupt")]
#[doc(hidden)]
pub static EXECUTOR: Executor = Executor::new();

#[doc(hidden)]
#[must_use]
pub fn init() -> OptionalPeripherals {
    #[cfg(feature = "executor-interrupt")]
    {
        // SWI & DMA priority need to match. DMA is hard-coded to P3 by upstream.
        use embassy_rp::interrupt::{InterruptExt as _, Priority};
        SWI.set_priority(Priority::P3);
    }

    let peripherals = embassy_rp::init(embassy_rp::config::Config::default());
    OptionalPeripherals::from(peripherals)
}
