use std::concat;
use std::fs::read_to_string;

use license_fetcher::{build_script::generate_package_list_with_licenses, Package};

fn main() {
    let mut packages = generate_package_list_with_licenses();

    packages.push(Package {
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

    packages.write();

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=Cargo.lock");
    println!("cargo::rerun-if-changed=Cargo.toml");
}
