use st_mems_reg_config_conv::parser;
use std::path::Path;

fn main() {
    // Source file:
    // https://github.com/STMicroelectronics/st-mems-finite-state-machine/blob/main/examples/fourd_orientation_detection/iis2dulpx/iis2dulpx_fourd_orientation.json
    let input_file = Path::new("iis2dulpx_fourd_orientation.json");
    let output_file = Path::new("src/fsm_config.rs");
    parser::generate_rs_from_json(input_file, output_file, "FOUR_D", "IIS2DULPX", false);
    println!("cargo:rerun-if-changed=iis2dulpx_fourd_orientation.json");

    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}
