# IIS2DULPX 4D Orientation Detection on STM32F401RE Nucleo-64

This example demonstrates how to detect 4D orientation events (portrait and landscape positions) using the **IIS2DULPX** accelerometer sensor on an **STM32F401RE** microcontroller board. The sensor's finite state machine (FSM) is configured via a UCF-generated register sequence to recognize these orientation events, which are then output over UART.

---

## Hardware Setup

- **Microcontroller Board:** STM32F401RE Nucleo-64
- **Sensor:** IIS2DULPX Accelerometer with FSM-based 4D orientation detection
- **Communication Interface:** I2C1 at 100 kHz Standard Mode
- **UART:** USART2 for serial output at 115200 baud
- **Interrupt Pin:** PA4 configured as input with external interrupt for FSM event notification

### Default Pin Configuration

| Signal       | STM32F401RE Pin | Description                      |
|--------------|-----------------|---------------------------------|
| I2C1_SCL     | PB8             | I2C clock line (open-drain)     |
| I2C1_SDA     | PB9             | I2C data line (open-drain)      |
| USART2_TX    | PA2             | UART transmit for debug output  |
| EXTI4 (INT)  | PA4             | External interrupt from sensor FSM event |

The IIS2DULPX sensor is connected via I2C1 on PB8/PB9. The FSM event interrupt line is connected to PA4, configured to trigger an external interrupt on rising edge. UART output is routed through PA2.

---

## Code Description

### Initialization

- The program initializes microcontroller peripherals including clocks, GPIO pins, I2C, UART, and a delay abstraction.
- I2C1 is configured at 100 kHz Standard Mode on pins PB8 (SCL) and PB9 (SDA).
- UART is configured on USART2 (PA2) at 115200 baud for serial output.
- PA4 is configured as an input pin with interrupt on rising edge for FSM event detection.
- The external interrupt is enabled in the NVIC and linked to the EXTI4 interrupt handler.
- The interrupt pin is stored in a mutex-protected static for safe interrupt flag clearing.

### Sensor Setup via UCF Configuration

- The IIS2DULPX sensor is initialized over I2C with the high I2C address.
- The device ID is read and verified; if mismatched, the program panics.
- The sensor is reset to default configuration and waits until reset completes.
- The sensor is configured by applying a sequence of register writes and delays defined in the `FOUR_D` array, generated from a UCF file. This programs the sensor's FSM for 4D orientation detection.

### Data Acquisition Loop

- The program enters a low-power wait-for-interrupt (WFI) loop.
- When an FSM event interrupt occurs, the program reads the FSM status.
- If FSM1 event is detected, it reads the FSM output and converts it to a `FourdEvent` enum.
- The detected orientation event (e.g., "Y-axis pointing down") is printed over UART.

### Interrupt Handler

- The `EXTI4` interrupt handler clears the interrupt pending bit on PA4 to allow further interrupts.

---

## Usage

1. Connect the IIS2DULPX sensor to the STM32F401RE Nucleo board via I2C1 (PB8/PB9).
2. Connect the sensor's FSM interrupt output to PA4 on the STM32F401RE.
3. Build the project, which uses the **`ucf-tool`** to generate Rust configuration code from UCF files automatically at build time.
4. Flash the compiled Rust firmware onto the STM32F401RE.
5. Open a serial terminal at 115200 baud on the UART port.
6. Change the device orientation to trigger 4D FSM events.
7. Observe orientation event messages printed over UART.

---

## Notes

- This example uses polling of interrupts and FSM event registers for orientation detection.
- The **`ucf-tool`** enables flexible sensor FSM configuration by converting UCF files into Rust code.
- The sensor driver and UCF-generated code handle low-level register access and FSM programming.
- The environment is `#![no_std]` and `#![no_main]` for embedded Rust applications.
- Panic behavior is set to halt on panic using `panic_itm`.

---

## References

- [STM32F401RE Nucleo-64 Board](https://www.st.com/en/evaluation-tools/nucleo-f401re.html)
- [IIS2DULPX Datasheet](https://www.st.com/resource/en/datasheet/iis2dulpx.pdf)
- [stm32f4xx-hal Rust crate](https://docs.rs/stm32f4xx-hal)

---

*This README provides a detailed explanation of the embedded Rust program for 4D orientation detection on STM32F401RE using the IIS2DULPX sensor and UCF-generated FSM configuration.*
