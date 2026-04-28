// Checkout https://docs.rs/license-fetcher/latest/license_fetcher/build/index.html

use std::concat;
use std::error::Error;
use std::fs::read_to_string;

use license_fetcher::build::config::{Config, ConfigBuilder};
use license_fetcher::build::package_list_with_licenses;
use license_fetcher::{package, PackageList};

fn fetch_and_embed_licenses() -> Result<(), Box<dyn Error>> {
    // Config with environment variables set by cargo, to fetch licenses at build time.
    let config: Config = ConfigBuilder::from_build_env().build()?;

    let mut packages: PackageList = package_list_with_licenses(config)?;

    packages.push(package! {
        name: "hyph-de-1996".to_owned(),
        version: "CTAN-2021.03.21".to_owned(),
        authors: vec!["Deutschsprachige Trennmustermannschaft <trennmuster@dante.de>".to_owned()],
        description: Some(
            "TeX-Trennmuster für die reformierte (2006) deutsche Rechtschreibung".to_owned(),
        ),
        homepage: None,
        repository: Some("https://github.com/hyphenation/tex-hyphen".to_owned()),
        license_identifier: Some("MIT".to_owned()),
        license_text: Some(
            read_to_string(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/dictionary/LICENSE-DICTIONARY.txt"
            ))
            .expect("Failed reading license of hyph-de-1996"),
        ),
    });

    // Write packages to out dir to be embedded.
    packages.write_package_list_to_out_dir()?;

    Ok(())
}

// Create empty dummy file so that the embedding does not fail.
fn dummy_file() {
    let mut path = std::env::var_os("OUT_DIR")
        .expect("Creation of dummy file failed: Environment variable 'OUT_DIR' not set.");
    path.push("/LICENSE-3RD-PARTY.bincode.deflate");
    let _ = std::fs::File::create(path).expect("Creation of dummy file failed: Write failed.");
}

fn main() {
    if let Some(mode) = std::env::var_os("LICENSE_FETCHER") {
        match mode.to_ascii_lowercase().to_string_lossy().as_ref() {
            "production" => fetch_and_embed_licenses().unwrap(),
            "development" => {
                eprintln!("Skipping license fetching.");
                dummy_file();
            }
            &_ => {
                eprintln!("Wrong environment variable `LICENSE_FETCHER`!");
                eprintln!("Expected either ``, `production` or `development`.");

                dummy_file();
            }
        }
    } else {
        if let Err(err) = fetch_and_embed_licenses() {
            eprintln!("An error occurred during license fetch:\n\n");
            eprintln!("{:?}", err);

            dummy_file();
        }
    }

    // Rerun only if one of the following files changed:
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=Cargo.lock");
    println!("cargo::rerun-if-changed=Cargo.toml");
}
