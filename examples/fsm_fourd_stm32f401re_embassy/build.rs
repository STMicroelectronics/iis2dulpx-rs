use st_mems_reg_config_conv::parser;
use std::path::Path;

fn main() {
    // Parse ucf file
    let input_file = Path::new("iis2dulpx_four_d.ucf");
    let output_file = Path::new("src/fsm_config.rs");
    parser::generate_rs_from_ucf(input_file, output_file, "FOUR_D");
    println!("cargo:rerun-if-changed=iis2dulpx_four_d.ucf");

    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}
