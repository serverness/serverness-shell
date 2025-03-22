use clap::Parser;
use newline_converter;
use progenitor::{GenerationSettings, Generator, TagStyle};
use rustfmt_wrapper;
use std::{fs::File, io::Write, path::PathBuf};

#[derive(Parser)]
#[command(name = "xtask")]
enum Xtask {
    #[command()]
    Generate {},
}

fn main() -> Result<(), String> {
    let args = Xtask::parse();

    match args {
        Xtask::Generate {} => generate(),
    }
}

fn generate() -> Result<(), String> {
    let xtask_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let root_path = xtask_path.parent().unwrap().to_path_buf();
    let mut spec_path = root_path.clone();
    spec_path.push("serverness.json");

    let file = File::open(spec_path).unwrap();
    let spec = serde_json::from_reader(file).unwrap();
    let mut generator = Generator::new(
        GenerationSettings::default()
            .with_interface(progenitor::InterfaceStyle::Builder)
            .with_tag(TagStyle::Separate)
            .with_derive("schemars::JsonSchema")
            .with_map_type("indexmap::IndexMap"),
    );

    let code = generator.nu(&spec, "serverness").unwrap().to_string();
    let contents = format_code(code);

    let mut out_path = root_path.clone();
    out_path.push("shell");
    out_path.push("src");
    out_path.push("generated_nu.rs");

    std::fs::write(out_path, &contents).unwrap();

    Ok(())
}

fn format_code(code: String) -> String {
    let contents = format!(
        "// The contents of this file are generated; do not modify them.\n\n{}",
        code,
    );

    let contents = rustfmt_wrapper::rustfmt_config(
        rustfmt_wrapper::config::Config {
            format_strings: Some(true),
            normalize_doc_attributes: Some(true),
            wrap_comments: Some(true),
            ..Default::default()
        },
        contents,
    )
    .unwrap();

    newline_converter::dos2unix(&contents).to_string()
}
