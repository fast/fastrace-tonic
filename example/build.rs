fn main() {
    tonic_build::compile_protos("proto/ping.proto").unwrap();
}
