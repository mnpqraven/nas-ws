use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let list = std::fs::read_dir("proto").unwrap();
    let list: Vec<PathBuf> = list.into_iter().map(|e| e.unwrap().path()).collect();

    // tonic_build::configure()
    //     .build_server(true)
    //     .compile(list.iter().as_slice(), &["proto"])
    //     .unwrap();
    for path in list {
        tonic_build::compile_protos(path)?;
    }
    Ok(())
}
