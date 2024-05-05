use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let output = Command::new("salloc")
        .arg("-V")
        .output();

    let (major, minor, micro) = if let Ok(output) = output {
        let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
        let version_str = stdout.trim().split_whitespace().last().expect("Invalid version string");
        let version_parts: Vec<_> = version_str.split('.').collect();

        let version_numbers: Vec<i32> = version_parts
            .into_iter()
            .map(|part| part.parse().expect("Invalid version number"))
            .collect();

        if version_numbers.len() != 3 {
            panic!("Invalid version number");
        } else {
            (version_numbers[0], version_numbers[1], version_numbers[2])
        }
    } else {
        (23, 11, 1)
    };


    if env::var("DOCS_RS").is_err() {
        let slurm_dir = format!(concat!(env!("CARGO_MANIFEST_DIR"), "/include/slurm_{:02}_{:02}"), major, minor);
        let bindings = bindgen::Builder::default()
            .clang_arg(format!("-I{}", slurm_dir))
            .header("wrapper.h")
            // We define spank_option manually to indicate that string pointers are const
            .blocklist_item("SLURM_VERSION_NUMBER")
            .blocklist_type("spank_option")
            .generate()
            .expect("Unable to generate bindings");

        let spank_file = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("src")
            .join("spank")
            .join(format!("spank_{:02}_{:02}.rs", major, minor));

        bindings
            .write_to_file(spank_file)
            .expect("Couldn't write bindings!");
    }

    let compat_content = match (major, minor) {
        (20, 11) => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/compat/compat_20_11.rs")),
        (21, 08) | (22, 5) | (23, 02) => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/compat/compat_21_08-23_02.rs")),
        _ => ""
    };
    let slurm_version = ((major as u32) << 16) | ((minor as u32) << 8) | (micro as u32);
    // process spank bindings
    let content = format!(r#"
pub const SLURM_VERSION_NUMBER: u32 = 0x{slurm_version:x};
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/spank/spank_{major:02}_{minor:02}.rs"));
{compat_content}
"#);

    let generated_file = PathBuf::from(env::var("OUT_DIR").unwrap()).join("generated.rs");
    std::fs::write(generated_file, content).expect("Couldn't write version file");
}
