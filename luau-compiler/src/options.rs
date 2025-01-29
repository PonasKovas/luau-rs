use std::collections::HashMap;

use luau_sys::common::bytecode::LuauBytecodeType;

#[derive(Debug, Clone, PartialEq)]
pub struct CompilerOptions {
    pub optimization: OptLevel,
    pub debug: DebugLevel,
    /// type information is used to guide native code generation decisions
    /// information includes testable types for function arguments, locals, upvalues and some temporaries
    pub generate_type_info_for_all: bool,
    pub coverage: CoverageLevel,

    pub alt_vector: Option<VectorOptions>,
    pub mutable_globals: Vec<String>,
    pub userdata_types: Vec<String>,
    pub known_libraries: Vec<LibraryWithKnownMembers>,
    pub disabled_builtins: Vec<String>,
}

impl Default for CompilerOptions {
    fn default() -> Self {
        Self {
            optimization: Default::default(),
            debug: Default::default(),
            generate_type_info_for_all: false,
            coverage: Default::default(),
            alt_vector: Default::default(),
            mutable_globals: Vec::new(),
            userdata_types: Vec::new(),
            known_libraries: Vec::new(),
            disabled_builtins: Vec::new(),
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

#[derive(Debug, Clone, PartialEq)]
pub struct VectorOptions {
    pub library_name: String,
    pub constructor: String,
    pub type_name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LibraryWithKnownMembers {
    pub name: String,
    pub types: HashMap<String, LuauBytecodeType>,
    pub constants: HashMap<String, Constant>,
}
impl LibraryWithKnownMembers {
    pub fn new(name: impl AsRef<str>) -> Self {
        Self {
            name: name.as_ref().to_owned(),
            types: HashMap::new(),
            constants: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    Nil,
    Bool(bool),
    Number(f64),
    Vector(f32, f32, f32, f32),
    String(String),
}
