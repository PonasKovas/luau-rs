use bindgen::Builder;
use std::{env, fs, path::PathBuf, process::Command};

fn main() {
    // fetch the git submodules
    Command::new("git")
        .arg("submodule")
        .arg("update")
        .arg("--init")
        .arg("--recursive")
        .arg("--depth")
        .arg("1")
        .status()
        .expect("Failed to fetch git submodules");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let mut build_dir = out_dir.clone();
    build_dir.push("build");
    fs::create_dir_all(&build_dir).unwrap();

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut luau_source = PathBuf::from(manifest_dir);
    luau_source.push("..");
    luau_source.push("vendor");
    luau_source.push("luau");

    // generate the build scripts
    Command::new("cmake")
        .arg("-DLUAU_BUILD_CLI=OFF")
        .arg("-DLUAU_BUILD_TESTS=OFF")
        .arg("-DLUAU_STATIC_CRT=ON")
        .arg("-DLUAU_EXTERN_C=ON")
        .arg("-DCMAKE_BUILD_TYPE=RelWithDebInfo")
        .arg("-S")
        .arg(&luau_source)
        .arg("-B")
        .arg(&build_dir)
        .status()
        .expect("failed to generate cmake build scripts");

    // build luau
    Command::new("cmake")
        .arg("--build")
        .arg(&build_dir)
        .arg("--target")
        .arg("Luau.VM")
        .arg("Luau.Compiler")
        .arg("--config")
        .arg("RelWithDebInfo")
        .status()
        .expect("failed to build luau");

    // build the rust bindings
    let vm_bindings = Builder::default()
        .header(
            luau_source
                .join("VM")
                .join("include")
                .join("lualib.h")
                .to_str()
                .unwrap(),
        )
        .generate()
        .expect("generating VM bindings");

    let compiler_bindings = Builder::default()
        .header(
            luau_source
                .join("Compiler")
                .join("include")
                .join("luacode.h")
                .to_str()
                .unwrap(),
        )
        .generate()
        .expect("generating Compiler bindings");

    // write the bindings to the OUT_DIR
    vm_bindings
        .write_to_file(out_dir.join("vm_bindings.rs"))
        .unwrap();
    compiler_bindings
        .write_to_file(out_dir.join("compiler_bindings.rs"))
        .unwrap();

    println!("cargo:rerun-if-changed=../vendor/");
    println!("cargo:rustc-link-search=native={}", build_dir.display());
    println!("cargo:rustc-link-lib=static=Luau.VM");
    println!("cargo:rustc-link-lib=static=Luau.Compiler");
}
