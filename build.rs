use std::concat;
use std::fs::read_to_string;

use license_fetcher::build::config::{Config, ConfigBuilder};
use license_fetcher::build::package_list_with_licenses;
use license_fetcher::{package, PackageList};

fn main() {
    let config: Config = ConfigBuilder::from_build_env()
        .build()
        .expect("Failed to build configuration.");

    let mut packages: PackageList =
        package_list_with_licenses(config).expect("Failed to fetch metadata or licenses.");

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

    packages
        .write_package_list_to_out_dir()
        .expect("Failed to write package list.");

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=Cargo.lock");
    println!("cargo::rerun-if-changed=Cargo.toml");
}
