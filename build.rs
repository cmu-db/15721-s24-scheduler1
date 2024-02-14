fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile(
            &["proto/substrait/proto/substrait/plan.proto"],
            &["proto/substrait/proto/"],
        )?;
    tonic_build::configure()
        .compile(
            &["proto/scheduler.proto"],
            &["proto", "proto/substrait/proto"],
        )?;
    Ok(())
}
