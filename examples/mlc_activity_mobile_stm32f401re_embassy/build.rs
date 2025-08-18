use st_mems_reg_config_conv::parser;
use std::path::Path;

fn main() {
    // Parse ucf file
    let file_name = "iis2dulpx_activity_recognition_for_mobile.ucf";
    let input_file = Path::new(file_name);
    let output_file = Path::new("src/mlc_config.rs");
    parser::generate_rs_from_ucf(&input_file, &output_file, "ACTIVITY_REC_FOR_MOBILE");
    println!("cargo:rerun-if-changed={file_name}");

    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}
