use std::env;
use std::process::Command;

fn main() {
    let grammars = [
        "VisitorBasic",
        "VisitorCalc",
        "CSV",
        "ReferenceToATN",
        "XMLLexer",
        "SimpleLR",
        "Labels",
        "FHIRPath",
    ];
    let additional_args = [
        Some("-visitor"),
        Some("-visitor"),
        Some("-visitor"),
        None,
        None,
        None,
        None,
    ];
    let antlr_path = "/home/rrevenantt/dev/antlr4/tool/target/antlr4-4.8-2-SNAPSHOT-complete.jar";

    for (grammar, arg) in grammars
        .iter()
        .copied()
        .zip(additional_args.iter().copied())
    {
        //ignoring error because we do not need to run anything when deploying to crates.io
        let _ = gen_for_grammar(grammar, antlr_path, arg);
    }

    println!("cargo:rerun-if-changed=build.rs");

    println!("cargo:rerun-if-changed={antlr_path}");
}

fn gen_for_grammar(
    grammar_file_name: &str,
    antlr_path: &str,
    additional_arg: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let input = env::current_dir()?.join("grammars");
    let file_name = grammar_file_name.to_owned() + ".g4";

    Command::new("java")
        .current_dir(input)
        .arg("-cp")
        .arg(antlr_path)
        .arg("org.antlr.v4.Tool")
        .arg("-Dlanguage=Rust")
        .arg("-o")
        .arg("../tests/gen")
        .arg(&file_name)
        .args(additional_arg)
        .spawn()
        .expect("antlr tool failed to start")
        .wait_with_output()?;

    println!("cargo:rerun-if-changed=grammars/{}", file_name);
    Ok(())
}
