pub mod telemetry_ingester {
    include!(concat!(env!("OUT_DIR"), "/my_proto.rs"));
    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("my_proto_service");
}
