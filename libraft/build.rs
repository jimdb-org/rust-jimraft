use cmake::Config;
use std::env;
use std::fs;
use std::path::PathBuf;
extern crate bindgen;
extern crate cmake;

fn fail_on_empty_directory(name: &str) {
    if fs::read_dir(name).unwrap().count() == 0 {
        println!(
            "The `{}` directory is empty, did you forget to pull the submodules?",
            name
        );
        panic!();
    }
}

fn get_os_type() -> &'static str {
    if cfg!(target_os = "windows") {
        return "windows";
    } else if cfg!(target_os = "linux") {
        return "linux";
    } else if cfg!(target_os = "macos") {
        return "macos";
    } else {
        return "unknown_os";
    }
}

fn bindgen_raft(jimraft_include_dir: String) {
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(&["-x", "c++", "-std=c++11"])
        .enable_cxx_namespaces()
        .derive_debug(false)
        .opaque_type("std::.*")
        .whitelist_function("jim_.*")
        .whitelist_function("raft_.*")
        .ctypes_prefix("libc")
        .clang_arg(&jimraft_include_dir)
        .size_t_is_usize(true)
        .generate()
        .expect("unable to generate raft bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("unable to write raft bindings");
}

fn build_jimraft() -> String {
    fail_on_empty_directory("jimraft");
    let dst = Config::new("jimraft").build();
    // now - emitting some cargo commands to build and link the lib
    println!("cargo:rustc-link-search=native={}", dst.display());

    dst.into_os_string().into_string().unwrap()
}

fn main() {
    let jimraft_out = build_jimraft();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=jimraft/");
    //create protobuf path
    let proto_dir = get_abs_dir(String::from("./jimraft/.external/protobuf"));
    println!("cargo:rustc-link-search=native={}", proto_dir);
    let os = get_os_type();
    if os == "linux" {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    } else if os == "macos" {
        println!("cargo:rustc-link-lib=dylib=c++");
    } else {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }

    let mut lib_path = String::from("cargo:rustc-link-search=native=");
    lib_path.push_str(&jimraft_out);
    lib_path.push_str("/lib");
    println!("{}", lib_path);

    println!("cargo:rustc-link-lib=static=jim-base");
    println!("cargo:rustc-link-lib=static=jim-common");
    println!("cargo:rustc-link-lib=static=jim-raft");
    println!("cargo:rustc-link-lib=static=jim-net");
    println!("cargo:rustc-link-lib=static=protoc");
    println!("cargo:rustc-link-lib=static=protobuf");
    println!("cargo:rustc-link-lib=static=protobuf-lite");

    let mut include_dir = String::from("-I");
    include_dir.push_str(&jimraft_out);
    include_dir.push_str("/raft");
    bindgen_raft(include_dir);
}

fn get_abs_dir(relative_dir: String) -> String {
    let relative_path = PathBuf::from(relative_dir);
    fs::canonicalize(&relative_path)
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
}
