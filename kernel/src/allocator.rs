use core::alloc::{GlobalAlloc, Layout};
use winapi::ctypes::c_void;

#[repr(C)]
pub enum PoolType {
    NonPagedPool,
    NonPagedPoolExecute,
}

#[link(name = "ntoskrnl")]
extern "system" {
    pub fn ExAllocatePool(pool_type: PoolType, number_of_bytes: usize) -> *mut c_void;
    pub fn ExFreePool(pool: *mut c_void);
}

/// The global kernel allocator structure.
pub struct KernelAlloc;

unsafe impl GlobalAlloc for KernelAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pool = ExAllocatePool(PoolType::NonPagedPool, layout.size() as _);

        if pool.is_null() {
            panic!("Failed to allocate pool");
        }

        pool as _
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        ExFreePool(ptr as _);
    }
}

#[alloc_error_handler]
#[cfg(not(test))]
fn alloc_error(layout: Layout) -> ! {
    panic!("{:?} alloc memory error", layout);
}
