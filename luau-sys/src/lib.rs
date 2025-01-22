#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod VM {
    include!(concat!(env!("OUT_DIR"), "/vm_bindings.rs"));
}

pub mod Compiler {
    include!(concat!(env!("OUT_DIR"), "/compiler_bindings.rs"));
}
