use luau_sys::vm::lua_State;
use std::{ptr::NonNull, sync::Arc};

use crate::allocator::LuauAllocator;

pub struct Luau {
    raw: Arc<RawState>,
}

struct RawState {
    ptr: NonNull<lua_State>,
}

impl Drop for RawState {
    fn drop(&mut self) {
        // destroy the state
    }
}

impl Luau {
    pub fn new(allocator: impl LuauAllocator) -> Self {
        todo!()
    }
}
