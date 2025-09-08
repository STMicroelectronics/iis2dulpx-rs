use st_mems_reg_config_conv::parser;
use std::path::Path;

fn main() {
    // Source file:
    // https://github.com/STMicroelectronics/st-mems-machine-learning-core/blob/main/examples/vibration_monitoring/iis2dulpx/iis2dulpx_vibration_monitoring.json
    let file_name = "iis2dulpx_vibration_monitoring.json";
    let input_file = Path::new(file_name);
    let output_file = Path::new("src/mlc_config.rs");
    parser::generate_rs_from_json(&input_file, &output_file, "VIBRATION", "IIS2DULPX", false);
    println!("cargo:rerun-if-changed={file_name}");

    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}
