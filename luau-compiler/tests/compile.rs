use luau_compiler::{compile, CompilerOptions};

#[test]
fn test_compile() {
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
