fn main()->Result<(),Box<dyn std::error::Error>>{
    // compiling protos using path on build time
       tonic_build::compile_protos("src/grpc/protos/core.proto")?;
       tonic_build::compile_protos("src/grpc/protos/platform.proto")?;
       tonic_build::compile_protos("src/grpc/protos/transactions_filter_stream.proto")?;
       Ok(())
    }
