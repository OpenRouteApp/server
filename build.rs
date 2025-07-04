fn main() {
    tonic_build::configure()
        .compile_well_known_types(true)
        .extern_path(".google.protobuf.Timestamp", "prost_types::Timestamp")
        .compile_protos(
            &[
                "proto/orroutes/routes.proto",
                "proto/orroutes/types.proto",
                "proto/orvotes/votes.proto",
                "proto/orvotes/types.proto",
            ],
            &["proto"],
        )
        .unwrap();
}
