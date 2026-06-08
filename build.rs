// Checkout https://docs.rs/license-fetcher/latest/license_fetcher/build/index.html

use std::{env::VarError, error::Error, fs::read_to_string, path::PathBuf};

use license_fetcher::{prelude::*, PackageBuilder};

fn fetch_and_embed_licenses() -> Result<(), Box<dyn Error>> {
    let config: Config = ConfigBuilder::from_build_env().build()?;

    let mut packages: PackageList = package_list_with_licenses(config)?;

    packages.push(
        PackageBuilder::new("hyph-de-1996", "CTAN-2021.03.21")
            .add_author("Deutschsprachige Trennmustermannschaft <trennmuster@dante.de>")
            .description("TeX-Trennmuster für die reformierte (2006) deutsche Rechtschreibung")
            .repository("https://github.com/hyphenation/tex-hyphen")
            .license_identifier("MIT")
            .add_license_text(
                "LICENSE-DICTIONARY MIT",
                read_to_string(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/dictionary/LICENSE-DICTIONARY.txt"
                ))
                .expect("Failed reading license of hyph-de-1996"),
            )
            .build(),
    );

    packages.write_package_list_to_out_dir()?;

    Ok(())
}

fn create_dummy_file() {
    let out_dir = std::env::var_os("OUT_DIR").expect("OUT_DIR not set");
    let path = PathBuf::from(out_dir).join(OUT_FILE_NAME);
    std::fs::File::create(path).expect("failed to create dummy file");
}

fn main() {
    match std::env::var("LICENSE_FETCHER") {
        Ok(mode) => match mode.as_str() {
            "FORCE" => fetch_and_embed_licenses().unwrap(),
            "SKIP" => {
                eprintln!("Skipping license fetching.");
                create_dummy_file();
            }
            wrong_arg => {
                eprintln!(
                    "Env var `LICENSE_FETCHER` should be set `FORCE` or `SKIP`, not {wrong_arg}."
                );
                create_dummy_file();
            }
        },
        Err(VarError::NotPresent) => {
            eprintln!("`LICENSE_FETCHER` not set. Defaulting to fetching licenses.");
            if let Err(err) = fetch_and_embed_licenses() {
                eprintln!("Soft fail with dummy file due to error:\n{err:?}");
                create_dummy_file();
            }
        }
        Err(VarError::NotUnicode(_)) => {
            eprintln!("Env var `LICENSE_FETCHER` must be valid unicode.");
            eprintln!("Skipping license fetching.");
            create_dummy_file();
        }
    }

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=Cargo.lock");
    println!("cargo::rerun-if-changed=Cargo.toml");
}
