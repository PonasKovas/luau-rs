use luau_sys::vm::lua_State;
use std::ptr::NonNull;

pub struct Luau {}

struct RawState {
    raw: NonNull<lua_State>,
}
