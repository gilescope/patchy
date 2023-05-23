use {
    regex::Regex,
    std::path::Path,
    std::process::Command,
    std::{fs::File, fs::OpenOptions, io::Write},
    walkdir::WalkDir,
};

const TGT: &str = "target";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let re = Regex::new(r#"name\s*=\s*"(.+?)""#)?;
    let is_target = |e: &Path| e.ancestors().any(|f| f.file_name() == Some(TGT.as_ref()));

    let process = |project: &str| -> Result<String, Box<dyn std::error::Error>> {
        let mut out = String::new();
        out.push_str("\n");
        out.push_str(&format!(
            r#"[patch."https://github.com/paritytech/{project}"]"#
        ));
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
                    out.push_str(&format!("\n{name} = {{ path = \"{}\" }}", target.display()));
                }
            }
        }
        Ok(out)
    };

    let is_cumulus = std::env::current_dir().unwrap().file_name() == Some("cumulus".as_ref());
    let mut out = if is_cumulus {
        process("polkadot")?
    } else {
        String::new()
    };
    out.push_str("\n");
    out.push_str(&process("substrate")?);

    println!("{}", &out);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("./Cargo.toml")
        .expect("Failed to open file");
    file.write_all(out.as_bytes()).unwrap();

    let output = Command::new("cargo")
        .args(&["update", "-p", "sp-io"])
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Command executed successfully:\n{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Command failed:\n{}", stderr);
    }

    if is_cumulus {
        let output = Command::new("cargo")
            .args(&["update", "-p", "polkadot-primitives"])
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("Command executed successfully:\n{}", stdout);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("Command failed:\n{}", stderr);
        }
    }

    Ok(())
}
