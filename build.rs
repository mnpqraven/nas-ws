use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bin = std::env::var("CARGO_BIN_NAME");
    match bin {
        Ok(value) if value == *"nas-ws" => {
            let list = std::fs::read_dir("proto").unwrap();
            let list: Vec<PathBuf> = list.into_iter().map(|e| e.unwrap().path()).collect();

            for path in list {
                // tonic_build::compile_protos(path)?;

                tonic_build::configure()
                    .build_server(true)
                    .protoc_arg("--experimental_allow_proto3_optional")
                    .compile(
                        &[path.clone()],
                        &[path
                            .parent()
                            .expect("proto file should reside in a directory")],
                    )
                    .unwrap();
            }
            Ok(())
        }
        _ => Ok(()),
    }
}
