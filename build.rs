fn main() {
    femtopb_build::compile_protos(&["proto/command.proto", "proto/response.proto"], &["proto"])
        .unwrap();

    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}
