use core::str;
use luau_sys::{
    common::bytecode::LuauBytecodeType,
    compiler::{
        lua_CompileConstant, luau_set_compile_constant_boolean, luau_set_compile_constant_nil,
        luau_set_compile_constant_number, luau_set_compile_constant_string,
        luau_set_compile_constant_vector,
    },
};
use malloced::Malloced;
use std::{
    cell::Cell,
    ffi::{c_char, c_int, CStr},
    iter::once,
    ptr::null,
};

mod error;
mod options;

pub use error::CompileError;
pub use options::*;

pub fn compile(code: &str, opts: &CompilerOptions) -> Result<Malloced<[u8]>, CompileError> {
    #[allow(non_snake_case)]
    let (vectorLib, vectorCtor, vectorType) = opts
        .alt_vector
        .as_ref()
        .map(
            |VectorOptions {
                 library_name,
                 constructor,
                 type_name,
             }| {
                (
                    library_name.as_ptr(),
                    constructor.as_ptr(),
                    type_name.as_ptr(),
                )
            },
        )
        .unwrap_or((null(), null(), null()));

    macro_rules! cstr_ptr_from_string {
        ($var:ident = $strings:expr) => {
            let temp: Vec<_> = $strings
                .iter()
                .map(|s| s.as_ptr())
                .chain(once(null()))
                .collect();

            #[allow(non_snake_case)]
            let $var = temp.as_ptr();
        };
    }
    cstr_ptr_from_string!(mutableGlobals = opts.mutable_globals);
    cstr_ptr_from_string!(userdataTypes = opts.userdata_types);
    cstr_ptr_from_string!(disabledBuiltins = opts.disabled_builtins);

    let temp: Vec<_> = opts
        .known_libraries
        .iter()
        .map(|l| l.name.as_ptr())
        .chain(once(null()))
        .collect();
    #[allow(non_snake_case)]
    let librariesWithKnownMembers = temp.as_ptr();

    // we call the luau::compile function
    // it will call our callbacks and then its gonna be done, all in this scope
    // so we can keep the data in thread local storage to make it accessible for the callbacks
    thread_local! {
        static LIBS_WITH_KNOWN_MEMBERS: Cell<Vec<LibraryWithKnownMembersC>> = Cell::new(Vec::new());
    }
    LIBS_WITH_KNOWN_MEMBERS.set(opts.known_libraries.clone());

    unsafe extern "C" fn lib_member_type_cb(lib: *const c_char, member: *const c_char) -> c_int {
        let lib_name = unsafe { CStr::from_ptr(lib) };
        let member = unsafe { CStr::from_ptr(member) };

        let libs_list = LIBS_WITH_KNOWN_MEMBERS.take();

        let ty = libs_list.iter().find_map(|lib| {
            if lib.name.as_bytes() == lib_name.to_bytes() {
                lib.types.iter().find_map(|(name, ty)| {
                    if name.as_bytes() == member.to_bytes() {
                        Some(ty)
                    } else {
                        None
                    }
                })
            } else {
                None
            }
        });

        let ty = match ty {
            Some(ty) => *ty,
            None => {
                eprintln!("Internal error: libraryMemberTypeCb called with library member that was not found: {lib_name:?} {member:?}");
                LuauBytecodeType::LBC_TYPE_NIL
            }
        };

        LIBS_WITH_KNOWN_MEMBERS.set(libs_list);

        ty.0 as c_int
    }
    unsafe extern "C" fn lib_member_const_cb(
        lib: *const c_char,
        member: *const c_char,
        constant: *mut lua_CompileConstant,
    ) {
        let lib_name = unsafe { CStr::from_ptr(lib) };
        let member = unsafe { CStr::from_ptr(member) };

        let libs_list = LIBS_WITH_KNOWN_MEMBERS.take();

        let constant_val = libs_list.iter().find_map(|lib| {
            if lib.name.as_bytes() == lib_name.to_bytes() {
                lib.constants.iter().find_map(|(name, constant)| {
                    if name.as_bytes() == member.to_bytes() {
                        Some(constant)
                    } else {
                        None
                    }
                })
            } else {
                None
            }
        });

        match constant_val {
            Some(&Constant::Nil) => unsafe { luau_set_compile_constant_nil(constant) },
            Some(&Constant::Bool(b)) => unsafe {
                luau_set_compile_constant_boolean(constant, b as c_int)
            },
            Some(&Constant::Number(n)) => unsafe { luau_set_compile_constant_number(constant, n) },
            Some(&Constant::Vector(x, y, z, w)) => unsafe {
                luau_set_compile_constant_vector(constant, x, y, z, w)
            },
            Some(Constant::String(s)) => unsafe {
                luau_set_compile_constant_string(constant, s.as_ptr() as *const i8, s.len())
            },

            None => {
                eprintln!("Internal error: libraryMemberConstantCb called with library member that was not found: {lib_name:?} {member:?}");
                unsafe { luau_set_compile_constant_nil(constant) };
            }
        };

        LIBS_WITH_KNOWN_MEMBERS.set(libs_list);
    }

    let options = luau_sys::compiler::lua_CompileOptions {
        optimizationLevel: opts.optimization as c_int,
        debugLevel: opts.debug as c_int,
        typeInfoLevel: if opts.generate_type_info_for_all {
            1
        } else {
            0
        },
        coverageLevel: opts.coverage as c_int,
        vectorLib,
        vectorCtor,
        vectorType,
        mutableGlobals,
        userdataTypes,
        librariesWithKnownMembers,
        libraryMemberTypeCb: Some(lib_member_type_cb),
        libraryMemberConstantCb: Some(lib_member_const_cb),
        disabledBuiltins,
    };

    let buffer = unsafe {
        let mut outsize = 0;

        let bytecode_ptr = luau_sys::compiler::luau_compile(
            code.as_ptr().cast(),
            code.len(),
            &options as *const _ as *mut _,
            &mut outsize,
        ) as *mut u8;

        Malloced::slice_from_raw_parts(bytecode_ptr, outsize)
    };

    assert!(buffer.len() > 0, "bytecode compile result cant be 0 length");

    if buffer[0] == 0 {
        // null byte means there was an error compiling
        assert!(
            buffer.len() > 1,
            "bytecode compile result starts with 0 but has no error message"
        );
        assert_eq!(
            buffer[1], ':' as u8,
            "bytecode compile result error must start with `:`"
        );

        return Err(CompileError { buffer });
    }

    Ok(buffer)
}
