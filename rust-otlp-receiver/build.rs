fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/metrics.proto")?;

    // tonic_build::compile_protos("proto/opentelemetry/proto/collector/logs/v1/logs_service.proto")?;
    // tonic_build::compile_protos("proto/opentelemetry/proto/collector/metrics/v1/metrics_service.proto")?;
    // tonic_build::compile_protos("proto/opentelemetry/proto/collector/traces/v1/traces_service.proto")?;

    // tonic_build::compile_protos("proto/opentelemetry/proto/common/v1/common.proto")?;

    // tonic_build::compile_protos("proto/opentelemetry/proto/resource/v1/resource.proto")?;
    // tonic_build::compile_protos("proto/opentelemetry/proto/logs/v1/logs.proto")?;
    // tonic_build::compile_protos("proto/opentelemetry/proto/traces/v1/traces.proto")?;
    // tonic_build::compile_protos("proto/opentelemetry/proto/metrics/v1/metrics.proto")?;


    Ok(())
}