use {regex::Regex, std::path::Path, walkdir::WalkDir};
const TGT: &str = "target";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let re = Regex::new(r#"name\s*=\s*"(.+?)""#)?;
    let is_target = |e: &Path| e.ancestors().any(|f| f.file_name() == Some(TGT.as_ref()));

    let process = |project: &str| -> Result<(), Box<dyn std::error::Error>> {
        println!(r#"[patch."https://github.com/paritytech/{project}"]"#);
        for entry in WalkDir::new(format!("../{project}"))
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_name() == "Cargo.toml" && !is_target(e.path()))
        {
            let target = entry.path().parent().unwrap().canonicalize()?;
            if target.join("src").exists() {
                let contents = std::fs::read_to_string(entry.path())?;
                let name = &re.captures(&contents).unwrap()[1];
                if name != "erasure_coding_fuzzer" {
                    println!("{name} = {{ path = \"{}\" }}", target.display());
                }
            }
        }
        Ok(())
    };
    if std::env::current_dir().unwrap().file_name() == Some("cumulus".as_ref()) {
        process("polkadot")?;
        println!();
    }
    process("substrate")
}
