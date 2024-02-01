use std::fs;
use std::path::PathBuf;
use clap::{arg, command, value_parser};

const SRC: &str = r"Q:\Quality System Documentation\Procedures and Forms\QP002-AQP\QP002 F10 Inspection Plan Release.doc";

fn main() {
    let matches = command!()
        .arg(
            arg!(
                <BOM> "BOM Number"
            )
                .required(true)
                .value_parser(value_parser!(String)),
        )
        .arg(
            arg!(
                <Rev> "Revision"
            )
                .required(false)
                .value_parser(value_parser!(String))
                .help("Defaults to Rev Rel"),
        )
        .get_matches();
    if let Some(file_name) = matches.get_one::<String>("BOM") {
        let rev = match matches.get_one::<String>("Rev") {
            Some(rev) => rev,
            None => "Rel"
        };
        let destination = PathBuf::from(
            format!("Inspection Plan for {file_name} Rev{rev}.doc")
        );
        create_inspection_plan_at(&destination);
    } else {
        println!("Failed to create Inspection Plan")
    }
}

fn create_inspection_plan_at(destination: &PathBuf) {
    let src = PathBuf::from(SRC);
    if destination.exists() {
        println!("Error: Destination already exists");
        return
    }
    match fs::copy(src, destination) {
        Ok(_) => println!("Successfully created Inspection Plan at: {}", destination.display()),
        Err(e) => println!("Error: {e}")
    };
}



