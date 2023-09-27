use std::{error::Error, process::Command};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    proto: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let proto_file = format!("{}.proto", args.proto);

    // NOTE: install grpcui beforehand
    // go install github.com/fullstorydev/grpcui/cmd/grpcui@latest
    let _cmd = Command::new("grpcui")
        .args([
            "-import-path",
            "./proto",
            "-plaintext",
            "-proto",
            &proto_file,
            "0.0.0.0:5005",
        ])
        .spawn()
        .expect("grpcui is not installed, install grpcui here:\ngo install github.com/fullstorydev/grpcui/cmd/grpcui@latest");

    Ok(())
}
