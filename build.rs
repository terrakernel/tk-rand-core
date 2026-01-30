use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let metadata = cargo_metadata::MetadataCommand::new()
        .exec()
        .expect("Failed to fetch cargo metadata");

    let get_version = |name: &str| {
        metadata
            .packages
            .iter()
            .find(|p| p.name == name)
            .map(|p| p.version.to_string())
            .unwrap_or_else(|| "unknown".to_string())
    };

    let rand_ver = get_version("rand");
    let chacha_ver = get_version("rand_chacha");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("audit_info.rs");
    let mut f = File::create(&dest_path).unwrap();

    writeln!(f, "pub const DEP_RAND_VERSION: &str = \"{}\";", rand_ver).unwrap();
    writeln!(f, "pub const DEP_CHACHA_VERSION: &str = \"{}\";", chacha_ver).unwrap();
    println!("cargo:rerun-if-changed=Cargo.toml");
}