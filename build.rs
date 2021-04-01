fn main() -> Result<(), Box<dyn std::error::Error>> {
    // compiling protos using path on build time
    tonic_build::compile_protos("proto/grpc_service.proto")?;
    // tonic_build::compile_protos("proto/model_config.proto")?;
    Ok(())
    // tonic_build::configure()
    //     .out_dir("src/")
    //     .compile(
    //         &[
    //             "proto/summarize.proto",
    //         ],
    //         &[".."],
    //     )?;
    // Ok(())
}