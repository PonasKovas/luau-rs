use crate::allocator::{self, LuauAllocator, LuauAllocatorDefault};
use luau_sys::vm::{lua_State, lua_close, lua_newstate};
use std::{ffi::c_void, fmt::Debug, ptr::NonNull};

pub struct LuauState {
    ptr: NonNull<lua_State>,
    allocator_ptr: *mut c_void,
    allocator_drop: unsafe fn(*mut c_void),
}

impl LuauState {
    pub fn new() -> Option<Self> {
        // Default allocator (using rust's allocator) with no limit
        Self::new_with_alloc(LuauAllocatorDefault::new(None))
    }
    pub fn new_with_alloc<A: LuauAllocator>(alloc: A) -> Option<Self> {
        let alloc_raw_f = allocator::raw::<A>();
        let allocator_ptr = Box::into_raw(Box::new(alloc)) as *mut c_void;

        let state_ptr = unsafe { lua_newstate(alloc_raw_f, allocator_ptr) };

        unsafe fn allocator_drop<A: LuauAllocator>(ptr: *mut c_void) {
            let _ = unsafe { Box::from_raw(ptr as *mut A) };
        }

        NonNull::new(state_ptr).map(|ptr| Self {
            ptr,
            allocator_ptr,
            allocator_drop: allocator_drop::<A>,
        })
    }
}
impl Drop for LuauState {
    fn drop(&mut self) {
        unsafe {
            // destroy the state
            lua_close(self.ptr.as_ptr());

            // destroy the allocator
            (self.allocator_drop)(self.allocator_ptr);
        }
    }
}
/// Luau does not implement any synchronization therefore it's not thread safe = !Sync
/// But it is safe to move it to another thread, it doesn't use TLS or anything else that
/// would make it bound to the thread it was created in
///
/// https://github.com/luau-lang/luau/discussions/1628
unsafe impl Send for LuauState {}

impl Debug for LuauState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Luau state {:p}>", self.ptr.as_ptr())
    }
}
