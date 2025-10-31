fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Tell Cargo to re-run this build script if the proto file changes
    // println!("cargo:rerun-if-changed=proto/user_service.proto");

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let proto_dir = std::path::Path::new(&manifest_dir).join("proto");

    let matchmaker_proto_file = proto_dir.join("matchmaker.proto");
    let ranking_proto_file = proto_dir.join("ranking.proto");
    let account_proto_file = proto_dir.join("account.proto");

    let out_dir = std::path::Path::new(&manifest_dir).join("src/generated");
    std::fs::create_dir_all(&out_dir)?;

    tonic_build::configure()
        .out_dir(out_dir)
        .compile(&[matchmaker_proto_file, ranking_proto_file, account_proto_file], &[proto_dir])?;

    Ok(())
}
