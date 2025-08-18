# IIS2DULPX FIFO Data Streaming on STM32F401RE Nucleo-64 Using Embassy Framework

This example demonstrates how to stream accelerometer data with timestamps from the **IIS2DULPX** sensor using FIFO buffering on an **STM32F401RE** microcontroller board. The sensor is configured to generate interrupts on FIFO watermark, and the program reads and outputs FIFO data over UART.

The project uses the [Embassy](https://embassy.dev/) framework for peripheral initialization and interrupt handling, leveraging its async runtime to efficiently wait for sensor interrupts. UART output is performed using blocking writes.

---

## Hardware Setup

- **Microcontroller Board:** STM32F401RE Nucleo-64
- **Sensor:** IIS2DULPX Accelerometer with FIFO support
- **Communication Interface:** I2C1 at 100 kHz Standard Mode with DMA
- **UART:** USART2 for serial output at 115200 baud with DMA
- **Interrupt Pin:** PA4 configured as input with external interrupt for FIFO watermark

### Default Pin Configuration

| Signal       | STM32F401RE Pin | Description                      |
|--------------|-----------------|---------------------------------|
| I2C1_SCL     | PB8             | I2C clock line (open-drain)     |
| I2C1_SDA     | PB9             | I2C data line (open-drain)      |
| USART2_TX    | PA2             | UART transmit for debug output  |
| EXTI4 (INT)  | PA4             | External interrupt from FIFO watermark |

The IIS2DULPX sensor is connected via I2C1 on PB8/PB9. The FIFO watermark interrupt line is connected to PA4, configured to trigger an external interrupt on rising edge. UART output is routed through PA2.

---

## Code Description

### Initialization

- The program initializes microcontroller peripherals including clocks, GPIO pins, I2C with DMA, UART with DMA, and a delay abstraction.
- I2C1 is configured at 100 kHz Standard Mode with DMA channels for efficient data transfer.
- UART is configured on USART2 (PA2) with DMA at 115200 baud for serial output.
- PA4 is configured as an input pin with interrupt on rising edge for FIFO watermark signaling.
- The external interrupt is enabled in the NVIC and linked to the EXTI4 interrupt handler.
- The interrupt pin is wrapped in Embassy's async `ExtiInput` to await rising edges asynchronously.

### Sensor Configuration

- The IIS2DULPX sensor is initialized over I2C with the high I2C address.
- The device ID is read and verified; if mismatched, the program panics.
- The sensor is reset to default configuration and waits until reset completes.
- Block Data Update and interface increment are enabled for safe driver usage.
- FIFO is configured in stream mode with watermark set to 33 entries (32 samples + timestamp).
- Timestamping is enabled with 32-sample decimation.
- Interrupt routing is configured to output FIFO watermark events on INT1 pin (PA4).
- Output data rate is set to 25 Hz with ±4g full scale and low-pass filter.

### Data Acquisition Loop

- The main async task waits for rising edge interrupts on PA4 signaling FIFO watermark.
- When a watermark interrupt occurs, the program reads the number of FIFO entries.
- It reads FIFO data entries and processes them based on their tag:
  - `XlOnly2xTag`: prints two accelerometer samples.
  - `XlTempTag`: prints accelerometer data and temperature if enabled.
  - `TimestampTag`: prints timestamp in milliseconds.
  - Unknown tags are reported.
- Data is printed over UART.

---

## Usage

1. Connect the IIS2DULPX sensor to the STM32F401RE Nucleo board via I2C1 (PB8/PB9).
2. Connect the sensor's FIFO watermark interrupt output to PA4 on the STM32F401RE.
3. Build and flash the firmware onto the STM32F401RE board.
4. Open a serial terminal at 115200 baud on the USART2 TX line.
5. Observe FIFO buffered accelerometer data and timestamps printed over UART.

---

## Notes

- The example uses Embassy's async runtime to efficiently wait for GPIO interrupts.
- UART output uses blocking writes without DMA.
- The environment is `#![no_std]` and `#![no_main]` for embedded Rust applications.
- Panic behavior is set to halt on panic using `defmt` and `panic_probe`.

---

## References

- [STM32F401RE Nucleo-64 Board](https://www.st.com/en/evaluation-tools/nucleo-f401re.html)
- [IIS2DULPX Datasheet](https://www.st.com/resource/en/datasheet/iis2dulpx.pdf)
- [Embassy STM32 HAL](https://docs.rs/embassy-stm32)

---

*This README provides a detailed explanation of the embedded Rust program for FIFO-based accelerometer data streaming on the IIS2DULPX sensor using STM32F401RE and the Embassy framework.*
