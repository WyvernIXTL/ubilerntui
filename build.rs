use license_fetcher::build_script::generate_package_list_with_licenses;

fn main() {
    generate_package_list_with_licenses();
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=Cargo.lock");
    println!("cargo::rerun-if-changed=Cargo.toml");
}