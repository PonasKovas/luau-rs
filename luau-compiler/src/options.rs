use luau_sys::common::bytecode::LuauBytecodeType;
use std::{collections::HashMap, ffi::CString};

#[derive(Debug, Clone, PartialEq)]
pub struct CompilerOptions {
    pub optimization: OptLevel,
    pub debug: DebugLevel,
    /// type information is used to guide native code generation decisions
    /// information includes testable types for function arguments, locals, upvalues and some temporaries
    pub generate_type_info_for_all: bool,
    pub coverage: CoverageLevel,

    pub(crate) alt_vector: Option<VectorOptions>,
    pub(crate) mutable_globals: Vec<CString>,
    pub(crate) userdata_types: Vec<CString>,
    pub(crate) known_libraries: Vec<LibraryWithKnownMembersC>,
    pub(crate) disabled_builtins: Vec<CString>,
}

impl CompilerOptions {
    pub fn new() -> Self {
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
    pub fn set_alt_vector(
        &mut self,
        vector_lib: impl AsRef<str>,
        vector_constructor: impl AsRef<str>,
        vector_type: impl AsRef<str>,
    ) -> &mut Self {
        self.alt_vector = Some(VectorOptions {
            library_name: CString::new(vector_lib.as_ref().to_owned()).unwrap(),
            constructor: CString::new(vector_constructor.as_ref().to_owned()).unwrap(),
            type_name: CString::new(vector_type.as_ref().to_owned()).unwrap(),
        });

        self
    }
    pub fn set_mutable_globals(
        &mut self,
        globals: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> &mut Self {
        self.mutable_globals = globals
            .into_iter()
            .map(|g| CString::new(g.as_ref().to_owned()).unwrap())
            .collect();

        self
    }
    pub fn set_userdata_types(
        &mut self,
        userdata_types: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> &mut Self {
        self.userdata_types = userdata_types
            .into_iter()
            .map(|g| CString::new(g.as_ref().to_owned()).unwrap())
            .collect();

        self
    }
    pub fn set_disabled_builtins(
        &mut self,
        builtins: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> &mut Self {
        self.disabled_builtins = builtins
            .into_iter()
            .map(|g| CString::new(g.as_ref().to_owned()).unwrap())
            .collect();

        self
    }
    pub fn add_known_library(&mut self, library: LibraryWithKnownMembers) -> &mut Self {
        self.known_libraries.push(library.into());

        self
    }
}

impl Default for CompilerOptions {
    fn default() -> Self {
        Self::new()
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
pub(crate) struct VectorOptions {
    pub(crate) library_name: CString,
    pub(crate) constructor: CString,
    pub(crate) type_name: CString,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct LibraryWithKnownMembersC {
    pub(crate) name: CString,
    pub(crate) types: HashMap<String, LuauBytecodeType>,
    pub(crate) constants: HashMap<String, Constant>,
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
impl From<LibraryWithKnownMembers> for LibraryWithKnownMembersC {
    fn from(value: LibraryWithKnownMembers) -> Self {
        Self {
            name: CString::new(value.name).unwrap(),
            types: value.types,
            constants: value.constants,
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
