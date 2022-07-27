fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/metrics.proto")?;
    tonic_build::compile_protos("proto/helloworld.proto")?;

    Ok(())
}