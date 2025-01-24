use core::str;
use luau_sys::compiler::{lua_CompileConstant, luau_set_compile_constant_nil};
use malloced::Malloced;
use std::error::Error;
use std::fmt::Debug;
use std::{
    ffi::{c_char, c_int},
    fmt::Display,
    ptr::null,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CompilerOptions {
    pub optimization: OptLevel,
    pub debug: DebugLevel,
    /// type information is used to guide native code generation decisions
    /// information includes testable types for function arguments, locals, upvalues and some temporaries
    pub generate_type_info_for_all: bool,
    pub coverage: CoverageLevel,
}

impl Default for CompilerOptions {
    fn default() -> Self {
        Self {
            optimization: Default::default(),
            debug: Default::default(),
            generate_type_info_for_all: false,
            coverage: Default::default(),
        }
    }
}

/// 0 - no optimization
/// 1 - baseline optimization level that doesn't prevent debuggability
/// 2 - includes optimizations that harm debuggability such as inlining
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OptLevel {
    None = 0,
    Baseline = 1,
    Max = 2,
}

impl Default for OptLevel {
    fn default() -> Self {
        Self::Baseline
    }
}

/// 0 - no debugging support
/// 1 - line info & function names only; sufficient for backtraces
/// 2 - full debug info with local & upvalue names; necessary for debugger
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DebugLevel {
    NoDebug = 0,
    Backtrace = 1,
    Full = 2,
}

impl Default for DebugLevel {
    fn default() -> Self {
        Self::Backtrace
    }
}

/// 0 - no code coverage support
/// 1 - statement coverage
/// 2 - statement and expression coverage (verbose)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CoverageLevel {
    NoCoverage = 0,
    Statement = 1,
    StatementExpression = 2,
}

impl Default for CoverageLevel {
    fn default() -> Self {
        Self::NoCoverage
    }
}

pub struct CompileError {
    buffer: Malloced<[u8]>,
}
impl CompileError {
    /// Panics if not valid utf8
    fn get_message(&self) -> &str {
        match str::from_utf8(&self.buffer[2..]) {
            Ok(s) => s,
            Err(e) => panic!("Compile error not valid utf8: {e}"),
        }
    }
}
impl Debug for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "luau compile error: {}", self.get_message())
    }
}
impl Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "luau compile error: {}", self.get_message())
    }
}
impl Error for CompileError {}

pub fn compile(code: &str, opts: CompilerOptions) -> Result<Malloced<[u8]>, CompileError> {
    unsafe extern "C" fn lib_member_type_cb(_lib: *const c_char, _member: *const c_char) -> c_int {
        luau_sys::common::bytecode::LuauBytecodeType::LBC_TYPE_NIL.0 as c_int
    }
    unsafe extern "C" fn lib_member_const_cb(
        _lib: *const c_char,
        _member: *const c_char,
        constant: *mut lua_CompileConstant,
    ) {
        unsafe {
            luau_set_compile_constant_nil(constant);
        }
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
        vectorLib: null(),
        vectorCtor: null(),
        vectorType: null(),
        mutableGlobals: &null(),
        userdataTypes: &null(),
        librariesWithKnownMembers: &null(),
        libraryMemberTypeCb: Some(lib_member_type_cb),
        libraryMemberConstantCb: Some(lib_member_const_cb),
        disabledBuiltins: &null(),
    };

    let mut outsize = 0;
    let buffer = unsafe {
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
