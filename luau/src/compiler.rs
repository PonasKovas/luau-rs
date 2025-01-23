struct Compiler {
    pub optimization: OptLevel,
    pub debug: DebugLevel,
    /// type information is used to guide native code generation decisions
    /// information includes testable types for function arguments, locals, upvalues and some temporaries
    pub generate_type_info_for_all: bool,
    pub coverage: CoverageLevel,
}

/// 0 - no optimization
/// 1 - baseline optimization level that doesn't prevent debuggability
/// 2 - includes optimizations that harm debuggability such as inlining
pub enum OptLevel {
    NoOptimization = 0,
    Baseline = 1,
    MaxOptimization = 2,
}

/// 0 - no debugging support
/// 1 - line info & function names only; sufficient for backtraces
/// 2 - full debug info with local & upvalue names; necessary for debugger
pub enum DebugLevel {
    NoDebug = 0,
    Backtrace = 1,
    Full = 2,
}

pub enum CoverageLevel {
    NoCoverage = 0,
    Statement = 1,
    StatementExpression = 2,
}
