fn main() {
    femtopb_build::compile_protos(&["proto/command.proto", "proto/response.proto"], &["proto"])
        .unwrap();
}
