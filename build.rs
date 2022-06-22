fn main() -> std::io::Result<()> {
    prost_build::Config::new()
        .out_dir("src/ws_client")
        .compile_protos(&["src/ws_client/abi.proto"], &["."])?;

    Ok(())
}
