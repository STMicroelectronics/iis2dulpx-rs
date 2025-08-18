# IIS2DULPX Accelerometer and QVAR Data Acquisition on STM32F401RE Nucleo-64

This example demonstrates how to read acceleration and QVAR (angular velocity variance) data from the **IIS2DULPX** sensor using an **STM32F401RE** microcontroller board. The sensor is configured to generate data-ready interrupts, and the program outputs sensor readings over UART.

---

## Hardware Setup

- **Microcontroller Board:** STM32F401RE Nucleo-64
- **Sensor:** IIS2DULPX Accelerometer with QVAR feature
- **Communication Interface:** I2C1 at 100 kHz Standard Mode
- **UART:** USART2 for serial output at 115200 baud
- **Interrupt Pin:** PA10 configured as input with external interrupt for data-ready signaling

### Default Pin Configuration

| Signal       | STM32F401RE Pin | Description                      |
|--------------|-----------------|---------------------------------|
| I2C1_SCL     | PB8             | I2C clock line (open-drain)     |
| I2C1_SDA     | PB9             | I2C data line (open-drain)      |
| USART2_TX    | PA2             | UART transmit for debug output  |
| EXTI15_10    | PA10            | External interrupt from sensor data-ready signal |

The IIS2DULPX sensor is connected via I2C1 on PB8/PB9. The data-ready interrupt line is connected to PA10, configured to trigger an external interrupt on rising edge. UART output is routed through PA2.

---

## Code Description

### Initialization

- The program initializes microcontroller peripherals including clocks, GPIO pins, I2C, UART, and a timer for delays.
- I2C1 is configured at 100 kHz Standard Mode on pins PB8 (SCL) and PB9 (SDA).
- UART is configured on USART2 (PA2) at 115200 baud for serial output.
- PA10 is configured as an input pin with interrupt on rising edge for data-ready signaling.
- The external interrupt is enabled in the NVIC and linked to the `EXTI15_10` interrupt handler.
- The interrupt pin is stored in a mutex-protected static for safe interrupt flag clearing.
- An atomic flag `MEMS_EVENT` is used to signal the main loop when new data is ready.

### Sensor Configuration

- The IIS2DULPX sensor is initialized over I2C with the low I2C address.
- The sensor is taken out of deep power down mode.
- The device ID is read and verified; if mismatched, the program halts.
- The sensor is reset to default configuration and waits until reset completes.
- Block Data Update and interface increment are enabled for safe driver usage.
- Output data rate is set to 25 Hz with ±4g full scale and low-pass filter.
- QVAR feature is enabled with specific gain and input impedance settings.
- Interrupt routing is configured to output data-ready and interrupt-on-reset signals on INT1 pin (PA10).

### Data Acquisition Loop

- The program waits for the `MEMS_EVENT` flag to be set by the interrupt handler.
- When new data is ready, it reads acceleration and QVAR data from the sensor.
- Sensor readings are printed over UART.

### Interrupt Handler

- The `EXTI15_10` interrupt handler clears the interrupt pending bit on PA10 and sets the `MEMS_EVENT` flag to notify the main loop.

---

## Usage

1. Connect the IIS2DULPX sensor to the STM32F401RE Nucleo board via I2C1 (PB8/PB9).
2. Connect the sensor's data-ready interrupt output to PA10 on the STM32F401RE.
3. Build and flash the firmware onto the STM32F401RE board.
4. Open a serial terminal at 115200 baud on the USART2 TX line.
5. Observe acceleration and QVAR data printed over UART.

---

## Notes

- The example uses hardware interrupts and an atomic flag to efficiently handle sensor data readiness.
- UART output uses blocking writes without DMA.
- The environment is `#![no_std]` and `#![no_main]` for embedded Rust applications.
- Panic behavior is set to halt on panic using `panic_halt`.

---

## References

- [STM32F401RE Nucleo-64 Board](https://www.st.com/en/evaluation-tools/nucleo-f401re.html)
- [IIS2DULPX Datasheet](https://www.st.com/resource/en/datasheet/lis2dulpx.pdf)
- [stm32f4xx-hal Rust crate](https://docs.rs/stm32f4xx-hal)

---

*This README explains the embedded Rust program for accelerometer and QVAR data acquisition on the IIS2DULPX sensor using STM32F401RE.*
