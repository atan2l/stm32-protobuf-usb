fn main() {
    let mut prost_config = prost_build::Config::new();
    prost_config.btree_map(["."]);
    prost_config.compile_protos(&["proto/command.proto", "proto/response.proto"], &["proto"]).unwrap();
}