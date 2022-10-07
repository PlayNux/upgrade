fn main() {
    pkg_config::Config::new().probe("nux_system_updater_gtk").unwrap();
    println!("cargo:rerun-if-changed=build.rs")
}
