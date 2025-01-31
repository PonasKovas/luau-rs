# luau rust bindings

### abandoned for now. learned a lot of new stuff while doing this and realised that rust won't allow me to do what I wanted to have in the end
### mlua is good. except for the cargo feature slop. but its not worth rewriting it from scratch just for that. at least for now. we will see what happens later.
### rn im focusing on other things in my main project and we will see whether i will continue making this or use mlua by then.

Safe bindings to the Luau compiler and VM in Rust.

## Why not mlua

`mlua` works and is usable, but

- Why use mutually exclusive cargo features?!
- Cargo feature slop (see previous point)
- Using C API with `longjmp`s when the C++ API with exceptions fits better with Rust (which is not available to standard Lua, only Luau, since Lua is written in C)
- I needed lower level control for my main project. I needed to integrate rust-side references in luau for example. I could have tried to make patches to `mlua` for this but I still need to understand the internals much more, and the best way is to do it myself.
- luau is different enough from standard lua that it deserves its own crate instead of being slopmaxxed into mlua
