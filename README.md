# luau rust bindings

Safe bindings to the Luau compiler and VM in Rust.

## Why not mlua

`mlua` works and is usable, but I found it's design to be incredibly stupid.

- Why support multiple Lua flavors in the same library when they are so different?
- Why use mutually exclusive cargo features for that?!

I decided to just make my own bindings instead of dealing with that shit.

