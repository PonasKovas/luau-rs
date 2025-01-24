#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod vm {
    include!(concat!(env!("OUT_DIR"), "/vm_bindings.rs"));
}

pub mod compiler {
    include!(concat!(env!("OUT_DIR"), "/compiler_bindings.rs"));
}

pub mod common {
    pub mod bytecode {
        include!(concat!(env!("OUT_DIR"), "/common_bytecode_bindings.rs"));
    }
}
