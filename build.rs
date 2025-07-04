fn main() {
    tonic_build::configure()
        .compile_well_known_types(true)
        .extern_path(".google.protobuf.Timestamp", "prost_types::Timestamp")
        .compile_protos(
            &[
                "proto/routes/routes.proto",
                "proto/routes/types.proto",
                "proto/votes/votes.proto",
                "proto/votes/types.proto",
            ],
            &["proto"],
        )
        .unwrap();
}
