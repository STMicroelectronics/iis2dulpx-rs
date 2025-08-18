# IIS2DULPX Accelerometer and QVAR Data Acquisition on STM32F401RE Nucleo-64 Using Embassy Framework

This example demonstrates how to read acceleration and QVAR (angular velocity variance) data from the **IIS2DULPX** sensor using an **STM32F401RE** microcontroller board. The sensor is configured to generate data-ready interrupts, and the program outputs sensor readings over UART.

The project uses the [Embassy](https://embassy.dev/) framework for peripheral initialization and interrupt handling, leveraging its async runtime to efficiently wait for sensor interrupts. UART output is performed using blocking writes.

---

## Hardware Setup

- **Microcontroller Board:** STM32F401RE Nucleo-64
- **Sensor:** IIS2DULPX Accelerometer with QVAR feature
- **Communication Interface:** I2C1 at 400 kHz Fast Mode
- **UART:** USART2 for serial output at 115200 baud
- **Interrupt Pin:** PA10 configured as input with external interrupt for data-ready signaling

### Default Pin Configuration

| Signal       | STM32F401RE Pin | Description                      |
|--------------|-----------------|---------------------------------|
| I2C1_SCL     | PB8             | I2C clock line (open-drain)     |
| I2C1_SDA     | PB9             | I2C data line (open-drain)      |
| USART2_TX    | PA2             | UART transmit for debug output  |
| EXTI10 (INT) | PA10            | External interrupt from sensor data-ready signal |

The IIS2DULPX sensor is connected via I2C1 on PB8/PB9. The data-ready interrupt line is connected to PA10, configured to trigger an external interrupt on rising edge. UART output is routed through PA2.

---

## Code Description

### Initialization

- The program initializes microcontroller peripherals including clocks, GPIO pins, I2C, UART, and a delay abstraction.
- I2C1 is configured at 400 kHz Fast Mode on pins PB8 (SCL) and PB9 (SDA).
- UART is configured on USART2 (PA2) at 115200 baud for serial output.
- PA10 is configured as an input pin with interrupt on rising edge for data-ready signaling.
- The interrupt pin is wrapped in Embassy's async `ExtiInput` to await rising edges asynchronously.

### Sensor Configuration

- The IIS2DULPX sensor is initialized over I2C with the low I2C address.
- The sensor is taken out of deep power down mode.
- The device ID is read and verified; if mismatched, the program halts.
- The sensor is reset to default configuration and waits until reset completes.
- Block Data Update and interface increment are enabled for safe driver usage.
- Interrupt routing is configured to output data-ready and interrupt-on-reset signals on INT1 pin (PA10).
- Output data rate is set to 25 Hz with ±4g full scale and low-pass filter.
- QVAR feature is enabled with specific gain and input impedance settings.

### Data Acquisition Loop

- The main async task waits for rising edge interrupts on PA10 signaling new sensor data.
- When new data is ready, it reads acceleration and QVAR data from the sensor.
- Sensor readings are printed over UART.

---

## Usage

1. Connect the IIS2DULPX sensor to the STM32F401RE Nucleo board via I2C1 (PB8/PB9).
2. Connect the sensor's data-ready interrupt output to PA10 on the STM32F401RE.
3. Build and flash the firmware onto the STM32F401RE board.
4. Open a serial terminal at 115200 baud on the USART2 TX line.
5. Observe acceleration and QVAR data printed over UART.

---

## Notes

- The example uses Embassy's async runtime to efficiently wait for GPIO interrupts.
- UART output uses blocking writes without DMA.
- The environment is `#![no_std]` and `#![no_main]` for embedded Rust applications.
- Panic behavior is set to halt on panic using `defmt` and `panic_probe`.

---

## References

- [STM32F401RE Nucleo-64 Board](https://www.st.com/en/evaluation-tools/nucleo-f401re.html)
- [IIS2DULPX Datasheet](https://www.st.com/resource/en/datasheet/lis2dulpx.pdf)
- [Embassy STM32 HAL](https://docs.rs/embassy-stm32)
- [lis2dulpx Rust Driver](https://crates.io/crates/lis2dulpx)

---

*This README explains the embedded Rust program for accelerometer and QVAR data acquisition on the IIS2DULPX sensor using STM32F401RE and the Embassy framework.*
