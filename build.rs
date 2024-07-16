use std::{env, io::Result, path::PathBuf};

fn main() -> Result<()> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .file_descriptor_set_path(out_dir.join("my_proto_service.bin"))
        .compile(&["src/my_proto.proto"], &["src"])?;
    Ok(())
}
