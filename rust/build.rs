use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let protos_dir = "./protos";
    let types_proto = format!("{}/TTSTypes.proto", protos_dir);
    let services_proto = format!("{}/TTSServices.proto", protos_dir);

    tonic_prost_build::configure()
        .build_server(false)
        .compile_protos(&[&types_proto, &services_proto], &[&protos_dir.to_string()])?;

    println!("cargo:rerun-if-changed={}", types_proto);
    println!("cargo:rerun-if-changed={}", services_proto);
    Ok(())
}
