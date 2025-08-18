# iis2dulpx-rs
[![Crates.io][crates-badge]][crates-url]
[![BSD 3-Clause licensed][bsd-badge]][bsd-url]

[crates-badge]: https://img.shields.io/crates/v/iis2dulpx-rs
[crates-url]: https://crates.io/crates/iis2dulpx-rs
[bsd-badge]: https://img.shields.io/crates/l/iis2dulpx-rs
[bsd-url]: https://opensource.org/licenses/BSD-3-Clause

This crate provides a platform-agnostic driver for the ST IIS2DULPX digital 3-axis linear accelerometer. 

## Sensor Overview

The IIS2DULPX is an intelligent, digital 3-axis linear accelerometer whose MEMS and ASIC have been expressly designed to combine the lowest supply current
possible with features such as always-on antialiasing filtering, a finite state machine (FSM) and machine learning core (MLC) with adaptive self-configuration (ASC), and
an analog hub / Qvar sensing channel.

The FSM and MLC with ASC deliver outstanding always-on, edge processing capabilities to the IIS2DULPX, while the analog hub / Qvar sensing channel defines a new degree of system optimization. The IIS2DULPX MIPI I3C® target interface and embedded 128-level FIFO buffer complete a set of features that make this
accelerometer a reference in terms of system integration from a standpoint of the bill of materials, processing, or power consumption.

The device has user-selectable full scales of ±2g/±4g/±8g/±16g and is capable of measuring accelerations with output data rates from 1.6 Hz to 800 Hz.

The IIS2DULPX has a dedicated internal engine to process motion and acceleration detection including free-fall, wake-up, single/double/triple-tap recognition, activity/inactivity, and 6D/4D orientation.

The device is available in a small thin plastic, land grid array (LGA) package and it is guaranteed to operate over an extended temperature range from -40°C to +105°C.

This driver was built using the [embedded-hal](https://docs.rs/embedded-hal/1.0.0/embedded_hal/) traits.

For more info, please visit the device page at [https://www.st.com/en/mems-and-sensors/iis2dulpx.html](https://www.st.com/en/mems-and-sensors/iis2dulpx.html)

## Installation

Add the driver to your `Cargo.toml` dependencies:

```toml
[dependencies]
iis2dulpx-rs = "0.1.0"
```

Or, add it directly from the terminal:

```sh
cargo add iis2dulpx-rs
```

## Usage

Include the crate and its prelude
```rust
use iis2dulpx_rs as iis2dulpx;
use iis2dulpx::*;
use iis2dulpx::prelude::*;
```

### Create an instance

Create an instance of the driver with the `new_<bus>` associated function, by passing an I2C (`embedded_hal::i2c::I2c`) instance and I2C address, or an SPI (`embedded_hal::spi::SpiDevice`) instance, along with a timing peripheral.

An example with I2C:

```rust
let mut sensor = Iis2dulpx::new_i2c(i2c, iis2dulpx::I2CAddress::I2cAddH, delay).unwrap();
```

### Check "Who Am I" Register

This step ensures correct communication with the sensor. It returns a unique ID to verify the sensor's identity.

```rust
let whoami = sensor.device_id_get().unwrap();
if whoami != ID {
    panic!("Invalid sensor ID");
}
```

### Configure

See details in specific examples; the following are common api calls:

```rust
// Restore default configuration
sensor.init_set(Init::Reset).unwrap();
loop {
    let status = sensor.status_get().unwrap();
    if status.sw_reset == 0 {
        break;
    }
}

// Set Output Data Rate
let md = Md {
    odr: Odr::_25hzLp,
    fs: Fs::_4g,
    bw: Bw::OdrDiv4,
};
sensor.mode_set(&md).unwrap();
```

## License

Distributed under the BSD-3 Clause license.

More Information: [http://www.st.com](http://st.com/MEMS).

**Copyright (C) 2025 STMicroelectronics**