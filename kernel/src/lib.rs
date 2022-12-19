#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

// external lib exports
pub use winapi::shared::{ntdef::NTSTATUS, ntstatus::STATUS_SUCCESS};

// public modules
pub mod log;

// private modules
mod allocator;

use allocator::KernelAlloc;

#[global_allocator]
static GLOBAL: KernelAlloc = KernelAlloc;

// fixes compiler errors
#[panic_handler]
#[cfg(not(test))]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// fixes compiler errors
#[used]
#[no_mangle]
pub static _fltused: i32 = 0;

// fixes compiler errors
#[no_mangle]
pub extern "system" fn __CxxFrameHandler3() -> i32 {
    0
}
