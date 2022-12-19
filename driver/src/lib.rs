#![no_std]

use kernel::log::KernelLogger;
use kernel::{NTSTATUS, STATUS_SUCCESS};

#[no_mangle]
pub extern "system" fn driver_entry(_driver: usize, _: usize) -> NTSTATUS {
    KernelLogger::init(::log::LevelFilter::Info).expect("Failed to initialize logger.");

    log::warn!("hello world");

    STATUS_SUCCESS
}
