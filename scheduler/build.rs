fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(&["proto/scheduler.proto"], &["proto"])?;
    tonic_build::configure().compile(&["proto/executor.proto"], &["proto"])?;
    Ok(())
}
