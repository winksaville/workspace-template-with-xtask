use std::{
    env,
    path::{Path, PathBuf},
    process::{exit, Command},
};

type DynError = Box<dyn std::error::Error>;

fn main() -> Result<(), DynError> {
    let mut env_iter = env::args();
    let task = env_iter.nth(1);
    let remaining_args: Vec<String> = env_iter.collect();
    //let remaining_args: Vec<String> = env_iter.map(|a| a).collect();
    match task.as_deref() {
        Some("fmt") => cargo_cmd("fmt", &remaining_args)?,
        Some("test") => cargo_cmd("test", &remaining_args)?,
        Some("clippy") => cargo_cmd("clippy", &remaining_args)?,
        Some("pre-commit") => pre_commit(&remaining_args)?,
        Some("gen-phl") => gen_phl()?,
        Some("gen-profraw") => gen_profraw()?,
        Some("gen-html") => gen_html()?,
        Some("gen-lcov") => gen_lcov()?,
        Some("gen-covdir") => gen_covdir()?,
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        r#"Tasks:
    pre-commit:    Runs `cargo fmt`, `cargo clippy` and `cargo test` in <proj-root>
    gen-phl:       Removes <proj-root>/coverage/ then generates coverage data in <proj-root>/coverage/
                   using gen-profraw, gen-html and gen-lcov.

Tasks for testing the above tasks:
    clippy:        Runs `cargo clippy` in current directory
    fmt:           Runs `cargo fmt` in current directory
    test:          Runs `cargo test` in current directory
    gen-profraw:   Runs `cargo test` with `-Cinstrument-coverage` generating `<proj-root>/coverage/*.profraw` files
    gen-html:      Runs `grcov` generating html files in `<proj-root>/coverage/html/`
    gen-lcov:      Rust `grcov` generating `<proj-root>/coverage/tests.lcov`
    gen-covdir:    Rust `grcov` generating `<proj-root>/coverage/tests.covdir.json`"#
    );
}

fn pre_commit(remaining_args: &Vec<String>) -> Result<(), DynError> {
    cargo_cmd_prj_root("test", remaining_args)?;
    cargo_cmd_prj_root("clippy", remaining_args)?;
    cargo_cmd_prj_root("fmt", remaining_args)?;

    Ok(())
}

fn gen_phl() -> Result<(), DynError> {
    std::fs::remove_dir_all(project_root().join("coverage"))?;
    std::fs::create_dir_all(project_root().join("coverage"))?;
    gen_profraw()?;
    gen_lcov()?;
    gen_html()?;
    gen_covdir()?;

    Ok(())
}

fn gen_profraw() -> Result<(), DynError> {
    eprintln!("Create profraw data");

    let coverage_dir = project_coverage_root()?;

    let status = Command::new("cargo")
        .env("CARGO_INCREMENTAL", "0")
        .env("RUSTFLAGS", "-Cinstrument-coverage")
        .env("TMPDIR", coverage_dir)
        .env("LLVM_PROFILE_FILE", "%t/cargo-test-%p-%m.profraw")
        .arg("test")
        //.args(["-p", "sub", "-p", "add"]) // All packages if none, else choose specific packages
        //.args(remaining_args)
        .status()?;

    if !status.success() {
        Err("cargo test with code-coverage {remaining_args:?} Failed")?;
    }

    Ok(())
}

fn gen_html() -> Result<(), DynError> {
    println!("gen_html");
    let output_path_buf = project_root().join("coverage").join("html");
    gen_coverage("html", &output_path_buf)
}

fn gen_lcov() -> Result<(), DynError> {
    let output_path_buf = project_root().join("coverage").join("tests.lcov");
    gen_coverage("lcov", &output_path_buf)
}

fn gen_covdir() -> Result<(), DynError> {
    let output_path_buf = project_root().join("coverage").join("tests.covdir.json");
    gen_coverage("covdir", &output_path_buf)
}

fn gen_coverage(output_type: &str, output_path_buf: &Path) -> Result<(), DynError> {
    let output_path = output_path_buf.to_string_lossy();
    eprintln!("Create {output_path}");

    let pb = project_root().join("target").join("debug").join("deps");
    let binary_path = pb.to_string_lossy();

    //let pb = Path::new("/home/wink/prgs/rust/clones/grcov/target/debug/grcov");
    //let grcov = pb.to_string_lossy().to_string();
    let grcov = "grcov".to_string();
    let status = Command::new(&grcov)
        .current_dir(project_root())
        .args([
            ".",
            "--binary-path",
            &binary_path,
            "--branch",
            "--ignore-not-existing",
            "--source-dir",
            ".",
            // All --ignore options are releative to --source-dir
            "--ignore",
            "xtask/*",
            //"--ignore",
            //"*/src/tests/*",
            //"--ignore",
            //"../*", // Ignore all explicitly relative paths
            //"--ignore",
            //"/*", // Ignore all absolute paths
            "--output-type",
            output_type,
            "--output-path",
            &output_path,
        ])
        //.args(remaining_args)
        .status()?;

    if !status.success() {
        Err(format!("Creating {output_path} Failed"))?;
    }

    Ok(())
}

fn cargo_cmd(cmd: &str, remaining_args: &Vec<String>) -> Result<(), DynError> {
    eprintln!("Run cargo {cmd} {remaining_args:?}");

    let status = Command::new(cargo_string())
        .arg(cmd)
        .args(remaining_args)
        .status()?;

    if !status.success() {
        Err("cargo {cmd} {remaining_args:?} Failed")?;
    }
    Ok(())
}

fn cargo_cmd_prj_root(cmd: &str, remaining_args: &Vec<String>) -> Result<(), DynError> {
    eprintln!("Run cargo {cmd} {remaining_args:?}");

    let status = Command::new(cargo_string())
        .current_dir(project_root())
        .arg(cmd)
        .args(remaining_args)
        .status()?;

    if !status.success() {
        Err("cargo {cmd} {remaining_args:?} Failed")?;
    }
    Ok(())
}

fn cargo_string() -> String {
    match env::var("CARGO") {
        Ok(cs) => cs,
        Err(_) => {
            eprintln!("No CARGO environment variable, is cargo installed?");
            exit(2);
        }
    }
}

fn project_coverage_root() -> Result<String, DynError> {
    let pb = project_root().join("coverage");
    let coverage_dir = match pb.to_str() {
        Some(dir) => dir,
        None => return Err("Unable to create coverage dir".into()),
    };

    Ok(coverage_dir.to_owned())
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

//#[cfg(test)]
//mod test {
//    use super::*; //project_root;
//
//    #[test]
//    fn test_project_root() {
//        let pr = project_root();
//        assert!(pr.starts_with("/"));
//    }
//}
