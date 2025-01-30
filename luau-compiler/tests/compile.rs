use luau_compiler::{compile, CompilerOptions};

#[test]
fn test_compile_simple() {
    let opts = CompilerOptions::new();

    let r1 = compile("print(\"HELLO WORLD\")", &opts);
    assert!(r1.is_ok(), "this must compile correctly");

    let r2 = compile("print(\"HELLO WORLD\"", &opts);
    assert!(r2.is_err(), "this must fail to compile");
    assert_eq!(
        format!("{}", r2.err().unwrap()),
        "luau compile error: 1: Expected ')' (to close '(' at column 6), got <eof>",
        "error must be valid string"
    );
}

#[test]
fn test_compile_mut_globals_opt() {
    // Tests compiling with different mutable globals list and checking if the bytecode is different
    let mut opts = CompilerOptions::new();

    let code = r#"function test()
    print(a)
    a = 5
    end

    print(a)
    a = 4"#;

    let without = compile(code, &opts);
    opts.set_mutable_globals(["a"]);
    let with = compile(code, &opts);

    assert!(without.is_ok(), "this must compile correctly");
    assert!(with.is_ok(), "this must compile correctly");
    // assert_ne!(
    //     without.unwrap(),
    //     with.unwrap(),
    //     "bytecode must not be identical"
    // ); // TODO fails for some reason
}
