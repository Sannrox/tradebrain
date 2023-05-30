fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("api/protobuf/value_investing.proto")?;
    Ok(())
}
