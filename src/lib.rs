#![no_std]

use core::fmt::Debug;
use embedded_hal::{
    delay::DelayNs,
    i2c::{I2c, SevenBitAddress},
    spi::SpiDevice,
};
use prelude::*;
use st_mems_bus::{BusOperation, EmbAdvFunctions, MemBankFunctions};

pub mod prelude;
pub mod register;

/// Driver for the Iis2dulpx sensor.
///
/// The struct takes a bus and a timer hardware object to write to the
/// registers.
/// The bus is generalized over the BusOperation trait, allowing the use
/// of I2C or SPI protocols; this also allows the user to implement sharing
/// techniques to share the underlying bus.
pub struct Iis2dulpx<B: BusOperation, T: DelayNs> {
    pub bus: B,
    pub tim: T,
    pub func_cfg_access_main: FuncCfgAccess,
}

/// Driver errors.
#[derive(Debug)]
pub enum Error<B> {
    Bus(B), // Error at the bus level
    FailedToBoot,
    FailedToSwReset,
    InvalidBwForODR,
    InvalidValue,
    BufferTooSmall,
    FailedToReadMemBank,
    FailedToSetMembank(MemBank),
}

impl<B, T> Iis2dulpx<B, T>
where
    B: BusOperation,
    T: DelayNs,
{
    /// Constructor method using a generic Bus that implements BusOperation
    pub fn from_bus(bus: B, tim: T) -> Self {
        Self {
            bus,
            tim,
            func_cfg_access_main: FuncCfgAccess::new(),
        }
    }
}

impl<P, T> Iis2dulpx<st_mems_bus::i2c::I2cBus<P>, T>
where
    P: I2c,
    T: DelayNs,
{
    /// Constructor method for using the I2C bus.
    pub fn new_i2c(i2c: P, address: I2CAddress, tim: T) -> Self {
        // Initialize the I2C bus with the Iis2dulpx address
        let bus = st_mems_bus::i2c::I2cBus::new(i2c, address as SevenBitAddress);
        Self {
            bus,
            tim,
            func_cfg_access_main: FuncCfgAccess::new(),
        }
    }
}

impl<P, T> Iis2dulpx<st_mems_bus::spi::SpiBus<P>, T>
where
    P: SpiDevice,
    T: DelayNs,
{
    /// Constructor method for using the SPI bus.
    pub fn new_spi(spi: P, tim: T) -> Self {
        // Initialize the SPI bus
        let bus = st_mems_bus::spi::SpiBus::new(spi);
        Self {
            bus,
            tim,
            func_cfg_access_main: FuncCfgAccess::new(),
        }
    }
}

impl<B, T> MemBankFunctions<MemBank> for Iis2dulpx<B, T>
where
    B: BusOperation,
    T: DelayNs,
{
    type Error = Error<B::Error>;

    /// Changes the memory bank.
    ///
    /// # Arguments
    ///
    /// - `val: MemBank`: Specifies the memory bank to set. Possible values include:
    ///   - `MemBank::MainMemBank`: Main memory bank.
    ///   - `MemBank::EmbedFuncMemBank`: Embedded function memory bank.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful memory bank change.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function changes the memory bank by modifying the `FuncCfgAccess` register. It ensures the correct
    /// memory bank is set for subsequent operations.
    fn mem_bank_set(&mut self, val: MemBank) -> Result<(), Self::Error> {
        // load func_cfg_access from stored one
        let mut func_cfg_access = self.func_cfg_access_main;
        func_cfg_access.set_emb_func_reg_access((val as u8) & 0x1);
        let result = func_cfg_access
            .write(self)
            .map_err(|_| Error::FailedToSetMembank(val));

        self.func_cfg_access_main = func_cfg_access;
        result
    }

    /// Retrieves the current memory bank.
    ///
    /// # Returns
    ///
    /// - `Result<MemBank, Error<B::Error>>`:
    ///   - `MemBank`: The current memory bank, either `MainMemBank` or `EmbedFuncMemBank`.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `FuncCfgAccess` register to determine the current memory bank. It returns
    /// the memory bank as a `MemBank` enum.
    fn mem_bank_get(&mut self) -> Result<MemBank, Self::Error> {
        self.func_cfg_access_main = if self.func_cfg_access_main.emb_func_reg_access() == 0 {
            FuncCfgAccess::read(self)?
        } else {
            self.func_cfg_access_main
        };

        let val = self
            .func_cfg_access_main
            .emb_func_reg_access()
            .try_into()
            .unwrap_or_default();
        Ok(val)
    }
}

impl<B, T> EmbAdvFunctions for Iis2dulpx<B, T>
where
    B: BusOperation,
    T: DelayNs,
{
    type Error = Error<B::Error>;

    /// Writes a buffer to a specified page.
    ///
    /// # Arguments
    ///
    /// - `address: u16`: The address of the page register to be written, with the page number in the 8-bit MSB
    ///   and the register address in the 8-bit LSB.
    /// - `buf: &[u8]`: A reference to the data buffer to be written.
    /// - `len: u8`: The length of the buffer.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful writing of the buffer.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///     - `Error::LenGraterThanBufSize`: Indicates that the specified length is greater than the buffer size.
    ///
    /// # Description
    ///
    /// This function writes data from a buffer to a specified page in the embedded function memory bank.
    /// It handles page wrapping and ensures the correct memory bank is set before and after the operation.
    fn ln_pg_write(&mut self, address: u16, buf: &[u8], len: u8) -> Result<(), Self::Error> {
        MemBank::operate_over_emb(self, |state| {
            let [mut lsb, mut msb] = address.to_le_bytes();

            // page write
            let mut page_rw = PageRw::read(state)?;
            page_rw.set_page_read(PROPERTY_DISABLE);
            page_rw.set_page_write(PROPERTY_ENABLE);
            page_rw.write(state)?;

            // set page num
            let mut page_sel = PageSel::default().with_page_sel(msb);
            page_sel.write(state)?;

            // set page addr
            PageAddress::from_bits(lsb).write(state)?;

            if len > buf.len() as u8 {
                return Err(Error::BufferTooSmall);
            }

            for &item in buf.iter().take(len as usize) {
                // read value
                PageValue::from_bits(item).write(state)?;

                lsb = lsb.wrapping_add(1);

                // check if page wrap
                if lsb == 0 {
                    msb += 1;
                    page_sel.set_page_sel(msb);
                    page_sel.write(state)?;
                }
            }

            // Reset page selection
            page_sel.set_page_sel(0);
            page_sel.write(state)?;

            page_rw = PageRw::read(state)?;
            page_rw.set_page_read(PROPERTY_DISABLE);
            page_rw.set_page_write(PROPERTY_DISABLE);
            page_rw.write(state)
        })
    }

    /// Reads a buffer from a specified page.
    ///
    /// # Arguments
    ///
    /// - `address: u16`: The address of the page register to be read, with the page number in the 8-bit MSB
    ///   and the register address in the 8-bit LSB.
    /// - `buf: &mut [u8]`: A mutable reference to the data buffer to store the read data.
    /// - `len: u8`: The length of the buffer.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful reading of the buffer.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///     - `Error::LenGraterThanBufSize`: Indicates that the specified length is greater than the buffer size.
    ///
    /// # Description
    ///
    /// This function reads data into a buffer from a specified page in the embedded function memory bank.
    /// It handles page wrapping and ensures the correct memory bank is set before and after the operation.
    fn ln_pg_read(&mut self, address: u16, buf: &mut [u8], len: u8) -> Result<(), Self::Error> {
        MemBank::operate_over_emb(self, |state| {
            let [mut lsb, mut msb] = address.to_le_bytes();

            // Set page read
            let mut page_rw = PageRw::read(state)?;
            page_rw.set_page_read(PROPERTY_ENABLE);
            page_rw.set_page_write(PROPERTY_DISABLE);
            page_rw.write(state)?;

            // set page num
            let mut page_sel = PageSel::default().with_page_sel(msb);
            page_sel.write(state)?;

            PageAddress::from_bits(lsb).write(state)?;

            if len > buf.len() as u8 {
                return Err(Error::BufferTooSmall);
            }

            for i in 0..len as usize {
                state.read_from_register(EmbReg::PageValue as u8, &mut buf[i..(i + 1)])?;
                lsb += 1;

                // check if page wrap
                if lsb == 0 {
                    msb += 1;
                    page_sel = PageSel::default().with_page_sel(msb);
                    page_sel.write(state)?;
                }
            }

            // Reset page selection
            page_sel.set_page_sel(0);
            page_sel.write(state)?;

            page_rw = PageRw::read(state)?;
            page_rw.set_page_read(PROPERTY_DISABLE);
            page_rw.set_page_write(PROPERTY_DISABLE);
            page_rw.write(state)
        })
    }
}

impl<B: BusOperation, T: DelayNs> Iis2dulpx<B, T> {
    #[inline]
    pub fn read_from_register(&mut self, reg: u8, buf: &mut [u8]) -> Result<(), Error<B::Error>> {
        self.bus.read_from_register(reg, buf).map_err(Error::Bus)
    }

    #[inline]
    pub fn write_to_register(&mut self, reg: u8, buf: &[u8]) -> Result<(), Error<B::Error>> {
        self.bus.write_to_register(reg, buf).map_err(Error::Bus)
    }

    fn reset_priv_data(&mut self) {
        self.func_cfg_access_main = FuncCfgAccess::new();
    }

    /// Retrieves the device ID from the hardware register.
    ///
    /// # Returns
    ///
    /// - `Result<u8, Error<B::Error>>`:
    ///   - `u8`: The device ID.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the device ID from the `WhoAmI` register. It utilizes the `read_from_register`
    /// method to fetch the ID and returns it as a `u8` value. If the read operation encounters any issues,
    /// it returns an error encapsulated in the `Error` enum.
    pub fn device_id_get(&mut self) -> Result<u8, Error<B::Error>> {
        let mut arr: [u8; 1] = [0];
        self.read_from_register(Reg::WhoAmI as u8, &mut arr)?;
        Ok(arr[0])
    }

    /// Perform device reboot (boot time: 25 ms)
    ///
    /// # Result
    /// - `Result<(), Error<B::Error>>`:
    ///     - `()`: Operation completed
    ///     - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///         - `Error::Bus`: Indicates an error at the bus level.
    ///         - `Error::FailedToBoot`: Indicated that the procedure did not completed
    pub fn reboot(&mut self) -> Result<(), Error<B::Error>> {
        let mut ctrl4 = Ctrl4::read(self)?;
        ctrl4.set_boot(PROPERTY_ENABLE);
        ctrl4.write(self)?;

        let mut cnt = 0;
        for _ in 0..BOOT_SWRESET_MAX_ATTEMPTS {
            let ctrl4 = Ctrl4::read(self)?;
            // boot procedure ended correctly
            if ctrl4.boot() == PROPERTY_DISABLE {
                break;
            }
            self.tim.delay_ms(BOOT_TIME_DELAY_MS as u32);
            cnt += 1;
        }

        if cnt + 1 >= BOOT_SWRESET_MAX_ATTEMPTS {
            return Err(Error::FailedToBoot);
        }

        Ok(())
    }

    /// Global reset of the device: power-on reset
    ///
    /// # Result
    /// - `Result<(), Error<B::Error>>`:
    ///     - `()`: Operation completed
    ///     - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///         - `Error::Bus`: Indicates an error at the bus level.
    pub fn sw_por(&mut self) -> Result<(), Error<B::Error>> {
        self.enter_deep_power_down(1)?;
        self.reset_priv_data();
        self.enter_deep_power_down(0)
    }

    /// Software reset: resets configuration registers.
    ///
    /// # Result
    /// - `Result<(), Error<B::Error>>`:
    ///     - `()`: Operation completed
    ///     - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///         - `Error::Bus`: Indicates an error at the bus level.
    ///         - `Error::FailedToSwReset`: Procedure did not completed
    pub fn sw_reset(&mut self) -> Result<(), Error<B::Error>> {
        let mut ctrl1 = Ctrl1::read(self)?;
        ctrl1.set_sw_reset(PROPERTY_ENABLE);
        ctrl1.write(self)?;

        let mut cnt = 0;
        for _ in 0..BOOT_SWRESET_MAX_ATTEMPTS {
            self.tim.delay_us(SW_RESET_DELAY_US as u32);

            let status = self.status_get()?;

            // sw-reset procedure ended correctly
            if status.sw_reset == 0 {
                break;
            }

            cnt += 1;
        }

        if cnt + 1 >= BOOT_SWRESET_MAX_ATTEMPTS {
            return Err(Error::FailedToSwReset);
        }

        Ok(())
    }

    /// Inits the sensor with suggested parameters.
    ///
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// BDU and IF_ADD_INC is activated by this function
    pub fn init_set(&mut self) -> Result<(), Error<B::Error>> {
        let mut ctrl1 = Ctrl1::read(self)?;
        let mut ctrl4 = Ctrl4::read(self)?;

        ctrl4.set_bdu(1);
        ctrl1.set_if_add_inc(1);

        ctrl4.write(self)?;
        ctrl1.write(self)?;

        self.reset_priv_data();

        Ok(())
    }

    /// Retrieves the current status of the device.
    ///
    /// # Returns
    ///
    /// - `Result<Status, Error<B::Error>>`:
    ///   - `Status`: Contains the current status of the device, including fields like `sw_reset`, `boot`,
    ///     `drdy`, and `power_down`.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads multiple registers to gather the current status of the device. It constructs a
    /// `Status` object containing various status indicators such as software reset status, boot status,
    /// data-ready status, and power-down status. The function ensures accurate status retrieval by reading
    /// from the `Status`, `Ctrl1`, and `Ctrl4` registers.
    pub fn status_get(&mut self) -> Result<Status, Error<B::Error>> {
        let status_register = StatusRegister::read(self)?;
        let ctrl1 = Ctrl1::read(self)?;
        let ctrl4 = Ctrl4::read(self)?;

        let val = Status {
            sw_reset: ctrl1.sw_reset(),
            boot: ctrl4.boot(),
            drdy: status_register.drdy(),
        };

        Ok(val)
    }

    /// FSM capability to write CTRL regs.
    ///
    /// # Arguments
    ///
    /// - `val: u8`: Enable or disable the FSM_WR_CTRL bit
    pub fn fsm_wr_ctrl_en_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut func_cfg_access = if self.func_cfg_access_main.emb_func_reg_access() == 0 {
            FuncCfgAccess::read(self)?
        } else {
            self.func_cfg_access_main
        };

        func_cfg_access.set_fsm_wr_ctrl_en(val & 0x1);
        let result = func_cfg_access.write(self);
        self.func_cfg_access_main = func_cfg_access;

        result
    }

    /// FSM capability to write CTRL regs.
    ///
    /// # Returns
    /// - `Result<u8, Error<B::Error>>`:
    ///   - `u8`: Value of FSM_WR_CTRL bit.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    pub fn fsm_wr_ctrl_en_get(&mut self) -> Result<u8, Error<B::Error>> {
        let func_cfg_access_main = if self.func_cfg_access_main.emb_func_reg_access() == 0 {
            FuncCfgAccess::read(self)?
        } else {
            self.func_cfg_access_main
        };

        let bit = func_cfg_access_main.fsm_wr_ctrl_en();
        self.func_cfg_access_main = func_cfg_access_main;

        Ok(bit)
    }

    /// Retrieves the status of the embedded functions.
    ///
    /// # Returns
    ///
    /// - `Result<EmbeddedStatus, Error<B::Error>>`:
    ///   - `EmbeddedStatus`: Contains the status of the embedded functions, including:
    ///     - `is_step_det`: Indicates if a step has been detected.
    ///     - `is_tilt`: Indicates if a tilt has been detected.
    ///     - `is_sigmot`: Indicates if significant motion has been detected.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function switches to the embedded function memory bank to read the status of embedded functions.
    /// It then switches back to the main memory bank. The status is encapsulated in the `EmbeddedStatus` struct.
    pub fn embedded_status_get(&mut self) -> Result<EmbeddedStatus, Error<B::Error>> {
        let status = EmbFuncStatusMainpage::read(self)?;

        Ok(EmbeddedStatus {
            is_step_det: status.is_step_det(),
            is_tilt: status.is_tilt(),
            is_sigmot: status.is_sigmot(),
        })
    }

    /// Enables pulsed data-ready mode (~75 us).
    ///
    /// # Arguments
    ///
    /// - `val: DataReadyMode`: Specifies the data-ready mode. Possible values include:
    ///   - `DataReadyMode::DrdyLatched`: Latched data-ready mode.
    ///   - `DataReadyMode::DrdyPulsed`: Pulsed data-ready mode.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration of the data-ready mode.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function sets the data-ready mode by modifying the `Ctrl1` register. It allows switching between
    /// latched and pulsed modes for data-ready signals.
    pub fn data_ready_mode_set(&mut self, val: &DataReadyMode) -> Result<(), Error<B::Error>> {
        let mut ctrl1 = Ctrl1::read(self)?;
        ctrl1.set_drdy_pulsed((*val as u8) & 0x1);
        ctrl1.write(self)
    }

    /// Retrieves the current data-ready mode.
    ///
    /// # Returns
    ///
    /// - `Result<DataReadyMode, Error<B::Error>>`:
    ///   - `DataReadyMode`: The current data-ready mode, either `DrdyLatched` or `DrdyPulsed`.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `Ctrl1` register to determine the current data-ready mode. It returns the mode
    /// as a `DataReadyMode` enum.
    pub fn data_ready_mode_get(&mut self) -> Result<DataReadyMode, Error<B::Error>> {
        let val = Ctrl1::read(self)?
            .drdy_pulsed()
            .try_into()
            .unwrap_or_default();
        Ok(val)
    }

    /// Sets the sensor mode, including full-scale (FS) and output data rate (ODR).
    ///
    /// # Arguments
    ///
    /// - `val: &Md`: Specifies the sensor mode configuration, including FS and ODR.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration of the sensor mode.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///     - `Error::InvalidBwForODR`: Indicates an invalid bandwidth value for the selected ODR.
    ///
    /// # Description
    ///
    /// This function configures the sensor mode by setting the FS and ODR in the `Ctrl5` register. It also
    /// adjusts the bandwidth and high-pass filter settings based on the selected ODR.
    pub fn mode_set(&mut self, val: &Md) -> Result<(), Error<B::Error>> {
        let mut ctrl5 = Ctrl5::read(self)?;

        ctrl5.set_odr(val.odr as u8 & 0xF);
        ctrl5.set_fs(val.fs as u8);

        match val.odr {
            // no anti-aliasing filter present
            Odr::Off | Odr::_1_6hzUlp | Odr::_3hzUlp | Odr::_25hzUlp => {
                ctrl5.set_bw(0x0);
            }
            // low-power mode with ODR < 50 Hz
            Odr::_6hzLp => match val.bw {
                Bw::OdrDiv2 | Bw::OdrDiv4 | Bw::OdrDiv8 => return Err(Error::InvalidBwForODR),
                Bw::OdrDiv16 => ctrl5.set_bw(Bw::OdrDiv16 as u8),
            },
            Odr::_12_5hzLp => match val.bw {
                Bw::OdrDiv2 | Bw::OdrDiv4 => return Err(Error::InvalidBwForODR),
                Bw::OdrDiv8 => ctrl5.set_bw(Bw::OdrDiv8 as u8),
                Bw::OdrDiv16 => ctrl5.set_bw(Bw::OdrDiv16 as u8),
            },
            Odr::_25hzLp => match val.bw {
                Bw::OdrDiv2 => return Err(Error::InvalidBwForODR),
                Bw::OdrDiv4 => ctrl5.set_bw(Bw::OdrDiv4 as u8),
                Bw::OdrDiv8 => ctrl5.set_bw(Bw::OdrDiv8 as u8),
                Bw::OdrDiv16 => ctrl5.set_bw(Bw::OdrDiv16 as u8),
            },
            // Standard cases
            _ => ctrl5.set_bw(val.bw as u8),
        }

        let mut ctrl3 = Ctrl3::read(self)?;

        ctrl3.set_hp_en(if (val.odr as u8 & 0x30) == 0x10 {
            PROPERTY_ENABLE
        } else {
            PROPERTY_DISABLE
        });

        ctrl5.write(self)?;
        ctrl3.write(self)
    }

    /// Retrieves the current sensor mode, including FS and ODR.
    ///
    /// # Returns
    ///
    /// - `Result<Md, Error<B::Error>>`:
    ///   - `Md`: The current sensor mode configuration, including FS and ODR.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `Ctrl5` and `Ctrl3` registers to determine the current sensor mode. It returns
    /// the mode as an `Md` struct, which includes FS, ODR, and bandwidth settings.
    pub fn mode_get(&mut self) -> Result<Md, Error<B::Error>> {
        let ctrl5 = Ctrl5::read(self)?;
        let ctrl3 = Ctrl3::read(self)?;

        let odr = Odr::new(ctrl5.odr(), ctrl3.hp_en());
        let fs = ctrl5.fs().try_into().unwrap_or_default();
        let bw = ctrl5.bw().try_into().unwrap_or_default();

        Ok(Md { odr, fs, bw })
    }

    /// Disable/Enable temperature (or AH_QVAR) sensor acquisition.
    ///
    /// # Arguments
    ///
    /// - `val: u8`:
    ///   - `1`: Disable temperature acquisition.
    ///   - `0`: Enable temperature acquisition.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function sets the acquisition state of the temperature (or AH_QVAR) sensor by modifying the
    /// `SelfTest` register. It allows enabling or disabling the sensor acquisition.
    pub fn t_ah_qvar_dis_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut st = SelfTest::read(self)?;
        st.set_t_ah_qvar_dis(val);
        st.write(self)
    }

    /// Disable/Enable temperature (or AH_QVAR) sensor acquisition.
    ///
    /// # Returns
    ///
    /// - `Result<u8, Error<B::Error>>`:
    ///   - `u8`:
    ///     - `1`: Temperature acquisition is disabled.
    ///     - `0`: Temperature acquisition is enabled.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function retrieves the current acquisition state of the temperature (or AH_QVAR) sensor from
    /// the `SelfTest` register.
    pub fn t_ah_qvar_dis_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ok(SelfTest::read(self)?.t_ah_qvar_dis())
    }

    /// Enter deep power down.
    ///
    /// # Arguments
    ///
    /// - `val: u8`: Enter deep power down.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful entry into deep power down.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function sets the device into deep power down mode by modifying the `Sleep` register.
    pub fn enter_deep_power_down(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut sleep = Sleep::read(self)?;
        sleep.set_deep_pd(val);
        sleep.write(self)
    }

    /// Enter soft power down in SPI case.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful exit from deep power down.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function exits the deep power down mode by enabling the soft power down in the `EnDeviceConfig`
    /// register. It includes a delay to ensure proper transition.
    pub fn exit_deep_power_down(&mut self) -> Result<(), Error<B::Error>> {
        let mut val = EnDeviceConfig::from_bits(0);
        val.set_soft_pd(PROPERTY_ENABLE);
        val.write(self)?;
        self.tim.delay_ms(25); // See AN5812 - paragraphs 3.1.1.1 and 3.1.1.2
        Ok(())
    }

    /// Disable hard-reset from CS.
    ///
    /// # Arguments
    ///
    /// - `val: u8`:
    ///   - `0`: Enable hard-reset from CS.
    ///   - `1`: Disable hard-reset from CS.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function configures the hard-reset capability from the chip select (CS) line by modifying the
    /// `FifoCtrl` register. It allows enabling or disabling the hard-reset feature.
    pub fn disable_hard_reset_from_cs_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut fifo_ctrl = FifoCtrl::read(self)?;
        fifo_ctrl.set_dis_hard_rst_cs(if val == 1 {
            PROPERTY_ENABLE
        } else {
            PROPERTY_DISABLE
        });
        fifo_ctrl.write(self)
    }

    /// Disable hard-reset from CS.
    ///
    /// # Returns
    ///
    /// - `Result<u8, Error<B::Error>>`:
    ///   - `u8`:
    ///     - `0`: Hard-reset from CS is enabled.
    ///     - `1`: Hard-reset from CS is disabled.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function retrieves the current configuration of the hard-reset capability from the chip select
    /// (CS) line from the `FifoCtrl` register.
    pub fn disable_hard_reset_from_cs_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ok(FifoCtrl::read(self)?.dis_hard_rst_cs())
    }

    /// Software trigger for One-Shot.
    ///
    /// # Arguments
    ///
    /// - `md: &Md`: The sensor conversion parameters.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful triggering of the One-Shot conversion.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function triggers a software One-Shot conversion if the output data rate (ODR) is set to
    /// `TrigSw`. It modifies the `Ctrl4` register to enable the start of conversion (SOC) property.
    pub fn trigger_sw(&mut self, md: &Md) -> Result<(), Error<B::Error>> {
        if md.odr == Odr::TrigSw {
            let mut ctrl4 = Ctrl4::read(self)?;
            ctrl4.set_soc(PROPERTY_ENABLE);
            ctrl4.write(self)?;
        }
        Ok(())
    }

    /// Retrieves all source information from the device.
    ///
    /// # Returns
    ///
    /// - `Result<AllSources, Error<B::Error>>`:
    ///   - `AllSources`: Contains various source information, including:
    ///     - `drdy`: Data-ready status.
    ///     - `timestamp`: Timestamp (assumed 0).
    ///     - `free_fall`: Free-fall detection status.
    ///     - `wake_up`: Wake-up detection status.
    ///     - `wake_up_z`: Wake-up detection on Z-axis.
    ///     - `wake_up_y`: Wake-up detection on Y-axis.
    ///     - `wake_up_x`: Wake-up detection on X-axis.
    ///     - `single_tap`: Single tap detection status.
    ///     - `double_tap`: Double tap detection status.
    ///     - `triple_tap`: Triple tap detection status.
    ///     - `six_d`: 6D detection status.
    ///     - `six_d_xl`: 6D detection on X-axis low.
    ///     - `six_d_xh`: 6D detection on X-axis high.
    ///     - `six_d_yl`: 6D detection on Y-axis low.
    ///     - `six_d_yh`: 6D detection on Y-axis high.
    ///     - `six_d_zl`: 6D detection on Z-axis low.
    ///     - `six_d_zh`: 6D detection on Z-axis high.
    ///     - `sleep_change`: Sleep change status (assumed 0).
    ///     - `sleep_state`: Sleep state status (assumed 0).
    ///     - `tilt`: Tilt detection status (assumed 0).
    ///     - `fifo_bdr`: FIFO batch data rate status (assumed 0).
    ///     - `fifo_full`: FIFO full status (assumed 0).
    ///     - `fifo_ovr`: FIFO overrun status (assumed 0).
    ///     - `fifo_th`: FIFO threshold status (assumed 0).
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads multiple registers to gather comprehensive source information from the device.
    /// It constructs an `AllSources` object containing various detection and status indicators.
    pub fn all_sources_get(&mut self) -> Result<AllSources, Error<B::Error>> {
        let status = StatusRegister::read(self)?;
        let sixd_src = SixdSrc::read(self)?;
        let wu_src = WakeUpSrc::read(self)?;
        let tap_src = TapSrc::read(self)?;

        let val = AllSources {
            drdy: status.drdy(),
            free_fall: wu_src.ff_ia(),
            wake_up: wu_src.wu_ia(),
            wake_up_z: wu_src.z_wu(),
            wake_up_y: wu_src.y_wu(),
            wake_up_x: wu_src.x_wu(),
            single_tap: tap_src.single_tap_ia(),
            double_tap: tap_src.double_tap_ia(),
            triple_tap: tap_src.triple_tap_ia(),
            six_d: sixd_src.d6d_ia(),
            six_d_xl: sixd_src.xl(),
            six_d_xh: sixd_src.xh(),
            six_d_yl: sixd_src.yl(),
            six_d_yh: sixd_src.yh(),
            six_d_zl: sixd_src.zl(),
            six_d_zh: sixd_src.zh(),
            sleep_change: wu_src.sleep_change_ia(),
            sleep_state: wu_src.sleep_state(),
        };

        Ok(val)
    }

    /// Retrieves accelerometer data.
    ///
    /// # Arguments
    ///
    /// - `md: &Md`: The sensor conversion parameters.
    ///
    /// # Returns
    ///
    /// - `Result<XlData, Error<B::Error>>`:
    ///   - `XlData`: Contains the accelerometer data, including:
    ///     - `mg`: Converted acceleration values in milli-g.
    ///     - `raw`: Raw acceleration data.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads accelerometer data from the `OutXL` register and converts the raw data into
    /// milli-g values based on the full-scale (FS) setting. It returns the data encapsulated in an `XlData` struct.
    pub fn xl_data_get(&mut self, md: &Md) -> Result<XlData, Error<B::Error>> {
        let raw = [
            OutX::read(self)?.outx().cast_signed(),
            OutY::read(self)?.outy().cast_signed(),
            OutZ::read(self)?.outz().cast_signed(),
        ];
        let mg = raw.map(|r| match md.fs {
            Fs::_2g => from_fs2g_to_mg(r),
            Fs::_4g => from_fs4g_to_mg(r),
            Fs::_8g => from_fs8g_to_mg(r),
            Fs::_16g => from_fs16g_to_mg(r),
        });
        Ok(XlData { raw, mg })
    }

    /// Retrieves OUTT data.
    ///
    /// # Returns
    ///
    /// - `Result<OuttData, Error<B::Error>>`:
    ///   - `OuttData`: Contains the temperature data, including:
    ///     - `heat`: A `Heat` struct with:
    ///       - `deg_c`: Temperature in degrees Celsius.
    ///       - `raw`: Raw temperature data.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads temperature data from the `OutTAhQvarL` register and converts the raw data
    /// into degrees Celsius. It returns the data encapsulated in an `OuttData` struct.
    pub fn outt_data_get(&mut self) -> Result<OuttData, Error<B::Error>> {
        let raw = OutTAhQvar::read(self)?.outt().cast_signed();
        let deg_c = from_lsb_to_celsius(raw);

        Ok(OuttData {
            heat: Heat { raw, deg_c },
        })
    }

    /// Retrieves AH_QVAR data.
    ///
    /// # Returns
    ///
    /// - `Result<AhQvarData, Error<B::Error>>`:
    ///   - `AhQvarData`: Contains the AH_QVAR data, including:
    ///     - `mv`: Converted voltage in millivolts.
    ///     - `raw`: Raw AH_QVAR data.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads AH_QVAR data from the `OutTAhQvarL` register and converts the raw data into
    /// millivolts. It returns the data encapsulated in an `AhQvarData` struct.
    pub fn ah_qvar_data_get(&mut self) -> Result<AhQvarData, Error<B::Error>> {
        // Read and discard also `OUT_Z_H` reg to clear drdy
        let _ = OutZ::read(self)?;

        let raw = OutTAhQvar::read(self)?.outt().cast_signed();
        let mv = from_lsb_to_mv(raw);

        Ok(AhQvarData { mv, raw })
    }

    /// Configures the self-test mode.
    ///
    /// # Arguments
    ///
    /// - `val: XlSelfTest`: Specifies the self-test mode. Possible values include:
    ///   - `XlSelfTest::Positive`: Positive self-test.
    ///   - `XlSelfTest::Negative`: Negative self-test.
    ///   - `XlSelfTest::Disable`: Disable self-test (returns an error).
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration of the self-test mode.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///     - `Error::InvalidValue`: Indicates an invalid self-test mode value.
    ///
    /// # Description
    ///
    /// This function configures the self-test mode by modifying the `Ctrl3` and `WakeUpDur` registers.
    /// It allows setting positive or negative self-test modes, or returns an error if disabling is attempted.
    pub fn self_test_sign_set(&mut self, val: XlSelfTest) -> Result<(), Error<B::Error>> {
        let mut ctrl3 = Ctrl3::read(self)?;
        let mut wkup_dur = WakeUpDur::read(self)?;

        match val {
            XlSelfTest::Positive => {
                ctrl3.set_st_sign_x(1);
                ctrl3.set_st_sign_y(1);
                wkup_dur.set_st_sign_z(0);
            }
            XlSelfTest::Negative => {
                ctrl3.set_st_sign_x(0);
                ctrl3.set_st_sign_y(0);
                wkup_dur.set_st_sign_z(1);
            }
            XlSelfTest::Disable => {
                return Err(Error::InvalidValue);
            }
        }

        ctrl3.write(self)?;
        wkup_dur.write(self)
    }

    /// Starts the self-test procedure.
    ///
    /// # Arguments
    ///
    /// - `val: u8`: Valid values are `2` (1st step) or `1` (2nd step).
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful start of the self-test.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::InvalidValue`: Indicates an invalid value for the self-test step.
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function initiates the self-test procedure by setting the appropriate value in the `SelfTest` register.
    /// It validates the input to ensure it is either `1` or `2`, corresponding to the self-test steps.
    pub fn self_test_start(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        if val != 1 && val != 2 {
            return Err(Error::InvalidValue);
        }

        let mut self_test = SelfTest::read(self)?;
        self_test.set_st(val);
        self_test.write(self)
    }

    /// Stops the self-test procedure.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful stop of the self-test.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function stops the self-test procedure by resetting the self-test value in the `SelfTest`
    /// register.
    pub fn self_test_stop(&mut self) -> Result<(), Error<B::Error>> {
        let mut self_test = SelfTest::read(self)?;
        self_test.set_st(0);
        self_test.write(self)
    }

    /// Configures the I3C bus settings.
    ///
    /// # Arguments
    ///
    /// - `val: &I3cCfg`: Configuration parameters for the I3C bus.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration of the I3C bus.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function configures the I3C bus by setting parameters in the `I3cIfCtrl` register. It allows
    /// customization of bus activity selection, dynamic address assignment, and asynchronous frame support.
    pub fn i3c_configure_set(&mut self, val: &I3cCfg) -> Result<(), Error<B::Error>> {
        let mut i3c_cfg = I3cIfCtrl::read(self)?;
        i3c_cfg.set_bus_act_sel(val.bus_act_sel as u8);
        i3c_cfg.set_dis_drstdaa(val.drstdaa_en);
        i3c_cfg.set_asf_on(val.asf_on);
        i3c_cfg.write(self)
    }

    /// Retrieves the current I3C bus configuration.
    ///
    /// # Returns
    ///
    /// - `Result<I3cCfg, Error<B::Error>>`:
    ///   - `I3cCfg`: Contains the current configuration parameters for the I3C bus.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `I3cIfCtrl` register to retrieve the current I3C bus configuration. It returns
    /// the configuration encapsulated in an `I3cCfg` struct.
    pub fn i3c_configure_get(&mut self) -> Result<I3cCfg, Error<B::Error>> {
        let i3c_cfg = I3cIfCtrl::read(self)?;

        let val = I3cCfg {
            drstdaa_en: i3c_cfg.dis_drstdaa(),
            asf_on: i3c_cfg.asf_on(),
            bus_act_sel: i3c_cfg.bus_act_sel().try_into().unwrap_or_default(),
        };

        Ok(val)
    }

    /// Enables or disables the external clock on the INT pin.
    ///
    /// # Arguments
    ///
    /// - `val: u8`:
    ///   - `0`: Disable external clock.
    ///   - `1`: Enable external clock.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function configures the external clock enable/disable setting on the INT pin by modifying
    /// the `ExtClkCfg` register.
    pub fn ext_clk_en_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut clk = ExtClkCfg::read(self)?;
        clk.set_ext_clk_en(val);
        clk.write(self)
    }

    /// Retrieves the external clock enable/disable status on the INT pin.
    ///
    /// # Returns
    ///
    /// - `Result<u8, Error<B::Error>>`:
    ///   - `u8`:
    ///     - `0`: External clock is disabled.
    ///     - `1`: External clock is enabled.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `ExtClkCfg` register to determine the current external clock enable/disable
    /// status on the INT pin.
    pub fn ext_clk_en_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ok(ExtClkCfg::read(self)?.ext_clk_en())
    }

    /// Configures the electrical settings for the configurable pins.
    ///
    /// # Arguments
    ///
    /// - `val: &PinConf`: The electrical settings for the configurable pins.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function configures the electrical settings for the configurable pins by modifying the `PinCtrl`
    /// register. It allows customization of pull-up and pull-down settings for various pins.
    pub fn pin_conf_set(&mut self, val: &PinConf) -> Result<(), Error<B::Error>> {
        let mut pin_ctrl = PinCtrl::read(self)?;

        pin_ctrl.set_cs_pu_dis(!val.cs_pull_up);
        pin_ctrl.set_pd_dis_int1(!val.int1_pull_down);
        pin_ctrl.set_pd_dis_int2(!val.int2_pull_down);
        pin_ctrl.set_sda_pu_en(val.sda_pull_up);
        pin_ctrl.set_sdo_pu_en(val.sdo_pull_up);
        pin_ctrl.set_pp_od(!val.int1_int2_push_pull);

        pin_ctrl.write(self)
    }

    /// Retrieves the electrical settings for the configurable pins.
    ///
    /// # Returns
    ///
    /// - `Result<PinConf, Error<B::Error>>`:
    ///   - `PinConf`: Contains the electrical settings for the configurable pins.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `PinCtrl` register to retrieve the current electrical settings for the
    /// configurable pins. It returns the settings encapsulated in a `PinConf` struct.
    pub fn pin_conf_get(&mut self) -> Result<PinConf, Error<B::Error>> {
        let pin_ctrl = PinCtrl::read(self)?;
        let pin_conf = PinConf {
            cs_pull_up: !pin_ctrl.cs_pu_dis(),
            int1_pull_down: !pin_ctrl.pd_dis_int1(),
            int2_pull_down: !pin_ctrl.pd_dis_int2(),
            sda_pull_up: pin_ctrl.sda_pu_en(),
            sdo_pull_up: pin_ctrl.sdo_pu_en(),
            int1_int2_push_pull: !pin_ctrl.pp_od(),
        };
        Ok(pin_conf)
    }

    /// Sets the interrupt activation level.
    ///
    /// # Arguments
    ///
    /// - `val: IntPinPolarity`: Specifies the interrupt activation level. Possible values include:
    ///   - `IntPinPolarity::ActiveHigh`: Active high.
    ///   - `IntPinPolarity::ActiveLow`: Active low.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function sets the interrupt activation level by modifying the `PinCtrl` register.
    pub fn int_pin_polarity_set(&mut self, val: IntPinPolarity) -> Result<(), Error<B::Error>> {
        let mut pin_ctrl = PinCtrl::read(self)?;
        pin_ctrl.set_h_lactive(val as u8);
        pin_ctrl.write(self)
    }

    /// Retrieves the interrupt activation level.
    ///
    /// # Returns
    ///
    /// - `Result<IntPinPolarity, Error<B::Error>>`:
    ///   - `IntPinPolarity`: The current interrupt activation level, either `ActiveHigh` or `ActiveLow`.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `PinCtrl` register to determine the current interrupt activation level.
    pub fn int_pin_polarity_get(&mut self) -> Result<IntPinPolarity, Error<B::Error>> {
        let pin_ctrl = PinCtrl::read(self)?;
        let val = pin_ctrl.h_lactive().try_into().unwrap_or_default();
        Ok(val)
    }

    /// Sets the SPI mode.
    ///
    /// # Arguments
    ///
    /// - `val: SpiMode`: Specifies the SPI mode. Possible values include:
    ///   - `SpiMode::Spi4Wire`: 4-wire SPI mode.
    ///   - `SpiMode::Spi3Wire`: 3-wire SPI mode.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function sets the SPI mode by modifying the `PinCtrl` register.
    pub fn spi_mode_set(&mut self, val: &SpiMode) -> Result<(), Error<B::Error>> {
        let mut pin_ctrl = PinCtrl::read(self)?;
        pin_ctrl.set_sim(*val as u8);
        pin_ctrl.write(self)
    }

    /// Retrieves the SPI mode.
    ///
    /// # Returns
    ///
    /// - `Result<SpiMode, Error<B::Error>>`:
    ///   - `SpiMode`: The current SPI mode, either `Spi4Wire` or `Spi3Wire`.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `PinCtrl` register to determine the current SPI mode.
    pub fn spi_mode_get(&mut self) -> Result<SpiMode, Error<B::Error>> {
        let pin_ctrl = PinCtrl::read(self)?;
        let val = pin_ctrl.sim().try_into().unwrap_or_default();
        Ok(val)
    }

    /// Routes interrupt signals on the INT1 pin.
    ///
    /// # Arguments
    ///
    /// - `val: &PinIntRoute`: Contains the interrupt signals routing configuration for the INT1 pin.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function configures the routing of interrupt signals on the INT1 pin by modifying the `Ctrl1`,
    /// `Ctrl2`, and `Md1Cfg` registers.
    pub fn pin_int1_route_set(&mut self, val: &PinInt1Route) -> Result<(), Error<B::Error>> {
        let mut ctrl1 = Ctrl1::read(self)?;
        ctrl1.set_int1_on_res(val.int_on_res);
        ctrl1.write(self)?;

        let mut ctrl2 = Ctrl2::read(self)?;
        ctrl2.set_int1_drdy(val.drdy);
        ctrl2.set_int1_fifo_ovr(val.fifo_ovr);
        ctrl2.set_int1_fifo_th(val.fifo_th);
        ctrl2.set_int1_fifo_full(val.fifo_full);
        ctrl2.set_int1_boot(val.boot);
        ctrl2.write(self)?;

        let mut md1_cfg = Md1Cfg::read(self)?;
        md1_cfg.set_int1_ff(val.free_fall);
        md1_cfg.set_int1_6d(val.six_d);
        md1_cfg.set_int1_tap(val.tap);
        md1_cfg.set_int1_wu(val.wake_up);
        md1_cfg.set_int1_sleep_change(val.sleep_change);
        md1_cfg.set_int1_emb_func(val.emb_function);
        md1_cfg.set_int1_timestamp(val.timestamp);
        md1_cfg.write(self)
    }

    /// Retrieves the interrupt signals routing configuration on the INT1 pin.
    ///
    /// # Returns
    ///
    /// - `Result<PinIntRoute, Error<B::Error>>`:
    ///   - `PinIntRoute`: Contains the interrupt signals routing configuration for the INT1 pin.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `Ctrl1`, `Ctrl2`, and `Md1Cfg` registers to retrieve the current interrupt
    /// signals routing configuration on the INT1 pin.
    pub fn pin_int1_route_get(&mut self) -> Result<PinInt1Route, Error<B::Error>> {
        let ctrl1 = Ctrl1::read(self)?;
        let ctrl2 = Ctrl2::read(self)?;
        let md1_cfg = Md1Cfg::read(self)?;

        Ok(PinInt1Route {
            int_on_res: ctrl1.int1_on_res(),
            drdy: ctrl2.int1_drdy(),
            fifo_ovr: ctrl2.int1_fifo_ovr(),
            fifo_th: ctrl2.int1_fifo_th(),
            fifo_full: ctrl2.int1_fifo_full(),
            boot: ctrl2.int1_boot(),
            free_fall: md1_cfg.int1_ff(),
            six_d: md1_cfg.int1_6d(),
            tap: md1_cfg.int1_tap(),
            wake_up: md1_cfg.int1_wu(),
            sleep_change: md1_cfg.int1_sleep_change(),
            emb_function: md1_cfg.int1_emb_func(),
            timestamp: md1_cfg.int1_timestamp(),
        })
    }

    /// Routes embedded function interrupt signals on the INT1 pin.
    ///
    /// # Arguments
    ///
    /// - `val: &EmbPinIntRoute`: Contains the embedded function interrupt signals routing configuration for the INT1 pin.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function configures the routing of embedded function interrupt signals on the INT1 pin by modifying
    /// the `EmbFuncInt1` register. It ensures the correct memory bank is set before and after the operation.
    pub fn emb_pin_int1_route_set(&mut self, val: &EmbPinIntRoute) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_emb(self, |state| {
            let mut emb_func_int1 = EmbFuncInt1::read(state)?;
            emb_func_int1.set_int1_tilt(val.tilt);
            emb_func_int1.set_int1_sig_mot(val.sig_mot);
            emb_func_int1.set_int1_step_det(val.step_det);
            emb_func_int1.set_int1_fsm_lc(val.fsm_lc);
            emb_func_int1.write(state)
        })?;

        let mut md1_cfg = Md1Cfg::read(self)?;
        md1_cfg.set_int1_emb_func(1);
        md1_cfg.write(self)
    }

    /// Retrieves the embedded function interrupt signals routing configuration on the INT1 pin.
    ///
    /// # Returns
    ///
    /// - `Result<EmbPinIntRoute, Error<B::Error>>`:
    ///   - `EmbPinIntRoute`: Contains the embedded function interrupt signals routing configuration for the INT1 pin.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `EmbFuncInt1` register to retrieve the current embedded function interrupt signals
    /// routing configuration on the INT1 pin. It ensures the correct memory bank is set before and after the operation.
    pub fn emb_pin_int1_route_get(&mut self) -> Result<EmbPinIntRoute, Error<B::Error>> {
        MemBank::operate_over_emb(self, |state| {
            let emb_func_int1 = EmbFuncInt1::read(state)?;
            let val = EmbPinIntRoute {
                tilt: emb_func_int1.int1_tilt(),
                sig_mot: emb_func_int1.int1_sig_mot(),
                step_det: emb_func_int1.int1_step_det(),
                fsm_lc: emb_func_int1.int1_fsm_lc(),
            };
            Ok(val)
        })
    }

    /// Routes interrupt signals on the INT2 pin.
    ///
    /// # Arguments
    ///
    /// - `val: &PinIntRoute`: Contains the interrupt signals routing configuration for the INT2 pin.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function configures the routing of interrupt signals on the INT2 pin by modifying the `Ctrl3` and
    /// `Md2Cfg` registers.
    pub fn pin_int2_route_set(&mut self, val: &PinInt2Route) -> Result<(), Error<B::Error>> {
        let mut ctrl3 = Ctrl3::read(self)?;
        ctrl3.set_int2_drdy(val.drdy);
        ctrl3.set_int2_fifo_ovr(val.fifo_ovr);
        ctrl3.set_int2_fifo_th(val.fifo_th);
        ctrl3.set_int2_fifo_full(val.fifo_full);
        ctrl3.set_int2_boot(val.boot);
        ctrl3.write(self)?;

        let mut md2_cfg = Md2Cfg::read(self)?;
        md2_cfg.set_int2_ff(val.free_fall);
        md2_cfg.set_int2_6d(val.six_d);
        md2_cfg.set_int2_tap(val.tap);
        md2_cfg.set_int2_wu(val.wake_up);
        md2_cfg.set_int2_sleep_change(val.sleep_change);
        md2_cfg.set_int2_emb_func(val.emb_function);
        md2_cfg.set_int2_timestamp(val.timestamp);
        md2_cfg.write(self)
    }

    /// Retrieves the interrupt signals routing configuration on the INT2 pin.
    ///
    /// # Returns
    ///
    /// - `Result<PinIntRoute, Error<B::Error>>`:
    ///   - `PinIntRoute`: Contains the interrupt signals routing configuration for the INT2 pin.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `Ctrl3` and `Md2Cfg` registers to retrieve the current interrupt signals routing
    /// configuration on the INT2 pin.
    pub fn pin_int2_route_get(&mut self) -> Result<PinInt2Route, Error<B::Error>> {
        let ctrl3 = Ctrl3::read(self)?;
        let md2_cfg = Md2Cfg::read(self)?;

        Ok(PinInt2Route {
            drdy: ctrl3.int2_drdy(),
            fifo_ovr: ctrl3.int2_fifo_ovr(),
            fifo_th: ctrl3.int2_fifo_th(),
            fifo_full: ctrl3.int2_fifo_full(),
            boot: ctrl3.int2_boot(),
            free_fall: md2_cfg.int2_ff(),
            six_d: md2_cfg.int2_6d(),
            tap: md2_cfg.int2_tap(),
            wake_up: md2_cfg.int2_wu(),
            sleep_change: md2_cfg.int2_sleep_change(),
            emb_function: md2_cfg.int2_emb_func(),
            timestamp: md2_cfg.int2_timestamp(),
        })
    }

    /// Routes embedded function interrupt signals on the INT2 pin.
    ///
    /// # Arguments
    ///
    /// - `val: &EmbPinIntRoute`: Contains the embedded function interrupt signals routing configuration for the INT2 pin.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function configures the routing of embedded function interrupt signals on the INT2 pin by modifying
    /// the `EmbFuncInt2` register. It ensures the correct memory bank is set before and after the operation.
    pub fn emb_pin_int2_route_set(&mut self, val: &EmbPinIntRoute) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_emb(self, |state| {
            let mut emb_func_int2 = EmbFuncInt2::read(state)?;

            emb_func_int2.set_int2_tilt(val.tilt);
            emb_func_int2.set_int2_sig_mot(val.sig_mot);
            emb_func_int2.set_int2_step_det(val.step_det);
            emb_func_int2.set_int2_fsm_lc(val.fsm_lc);

            emb_func_int2.write(state)
        })?;

        let mut md2_cfg = Md2Cfg::read(self)?;
        md2_cfg.set_int2_emb_func(1);
        md2_cfg.write(self)
    }

    /// Retrieves the embedded function interrupt signals routing configuration on the INT2 pin.
    ///
    /// # Returns
    ///
    /// - `Result<EmbPinIntRoute, Error<B::Error>>`:
    ///   - `EmbPinIntRoute`: Contains the embedded function interrupt signals routing configuration for the INT2 pin.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `EmbFuncInt2` register to retrieve the current embedded function interrupt signals
    /// routing configuration on the INT2 pin. It ensures the correct memory bank is set before and after the operation.
    pub fn emb_pin_int2_route_get(&mut self) -> Result<EmbPinIntRoute, Error<B::Error>> {
        MemBank::operate_over_emb(self, |state| {
            let emb_func_int2 = EmbFuncInt2::read(state)?;
            let val = EmbPinIntRoute {
                tilt: emb_func_int2.int2_tilt(),
                sig_mot: emb_func_int2.int2_sig_mot(),
                step_det: emb_func_int2.int2_step_det(),
                fsm_lc: emb_func_int2.int2_fsm_lc(),
            };
            Ok(val)
        })
    }

    /// Sets the interrupt configuration mode.
    ///
    /// # Arguments
    ///
    /// - `val: &IntConfig`: Contains the interrupt configuration settings.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function configures the interrupt mode by modifying the `InterruptCfg` register. It allows
    /// setting the interrupt mode to disabled, level-sensitive, or latched, and configures additional
    /// settings such as disabling reset on latch interrupt and sleep status on interrupt.
    pub fn int_config_set(&mut self, val: &IntConfig) -> Result<(), Error<B::Error>> {
        let mut interrupt_cfg = InterruptCfg::read(self)?;

        match val.int_cfg {
            IntCfg::Disabled => {
                interrupt_cfg.set_interrupts_enable(0);
            }
            IntCfg::Level => {
                interrupt_cfg.set_interrupts_enable(1);
                interrupt_cfg.set_lir(0);
            }
            IntCfg::Latched => {
                interrupt_cfg.set_interrupts_enable(1);
                interrupt_cfg.set_lir(1);
            }
        }

        interrupt_cfg.set_dis_rst_lir_all_int(val.dis_rst_lir_all_int);
        interrupt_cfg.set_sleep_status_on_int(val.sleep_status_on_int);

        interrupt_cfg.write(self)
    }

    /// Retrieves the interrupt configuration mode.
    ///
    /// # Returns
    ///
    /// - `Result<IntConfig, Error<B::Error>>`:
    ///   - `IntConfig`: Contains the current interrupt configuration settings, including:
    ///     - `int_cfg`: The interrupt mode (disabled, level-sensitive, or latched).
    ///     - `sleep_status_on_int`: Sleep status on interrupt.
    ///     - `dis_rst_lir_all_int`: Disable reset on latch interrupt.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `InterruptCfg` register to retrieve the current interrupt configuration
    /// settings. It returns the settings encapsulated in an `IntConfig` struct.
    pub fn int_config_get(&mut self) -> Result<IntConfig, Error<B::Error>> {
        let interrupt_cfg = InterruptCfg::read(self)?;

        let int_cfg = if interrupt_cfg.interrupts_enable() == 0 {
            IntCfg::Disabled
        } else if interrupt_cfg.lir() == 0 {
            IntCfg::Level
        } else {
            IntCfg::Latched
        };

        Ok(IntConfig {
            int_cfg,
            sleep_status_on_int: interrupt_cfg.sleep_status_on_int(),
            dis_rst_lir_all_int: interrupt_cfg.dis_rst_lir_all_int(),
        })
    }

    /// Sets the embedded interrupt configuration mode.
    ///
    /// # Arguments
    ///
    /// - `val: EmbeddedIntConfig`: Specifies the embedded interrupt configuration mode. Possible values include:
    ///   - `EmbeddedIntConfig::Level`: Level-sensitive.
    ///   - `EmbeddedIntConfig::Latched`: Latched.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function configures the embedded interrupt mode by modifying the `PageRw` register. It ensures
    /// the correct memory bank is set before and after the operation.
    pub fn embedded_int_cfg_set(&mut self, val: EmbeddedIntConfig) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_emb(self, |state| {
            let mut page_rw = PageRw::read(state)?;
            match val {
                EmbeddedIntConfig::Level => page_rw.set_emb_func_lir(0),
                EmbeddedIntConfig::Latched => page_rw.set_emb_func_lir(1),
            }
            page_rw.write(state)
        })
    }

    /// Retrieves the embedded interrupt configuration mode.
    ///
    /// # Returns
    ///
    /// - `Result<EmbeddedIntConfig, Error<B::Error>>`:
    ///   - `EmbeddedIntConfig`: The current embedded interrupt configuration mode (level-sensitive or latched).
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `PageRw` register to retrieve the current embedded interrupt configuration
    /// mode. It ensures the correct memory bank is set before and after the operation.
    pub fn embedded_int_cfg_get(&mut self) -> Result<EmbeddedIntConfig, Error<B::Error>> {
        MemBank::operate_over_emb(self, |state| {
            let page_rw = PageRw::read(state)?;
            let val = if page_rw.emb_func_lir() == 0 {
                EmbeddedIntConfig::Level
            } else {
                EmbeddedIntConfig::Latched
            };
            Ok(val)
        })
    }

    /// Sets the FIFO mode configuration.
    ///
    /// # Arguments
    ///
    /// - `val: FifoMode`: The desired FIFO mode settings.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration of the FIFO mode.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function configures the FIFO mode by setting various parameters such as operation mode,
    /// storage depth, accelerometer only and batch in FIFO. It modifies the `Ctrl4`, `FifoCtrl`,
    /// and `FifoWtm` registers to apply the settings.
    pub fn fifo_mode_set(&mut self, val: &FifoMode) -> Result<(), Error<B::Error>> {
        let mut ctrl4 = Ctrl4::read(self)?;
        let mut fifo_ctrl = FifoCtrl::read(self)?;
        let mut fifo_wtm = FifoWtm::read(self)?;

        // Set FIFO mode
        if val.operation != FifoOperation::FifoOff {
            ctrl4.set_fifo_en(1);
            fifo_ctrl.set_fifo_mode(val.operation as u8 & 0x7);
        } else {
            ctrl4.set_fifo_en(0);
        }

        // Set FIFO depth (1X/2X)
        fifo_ctrl.set_fifo_depth(val.store as u8);

        // Set xl_only_fifo
        fifo_wtm.set_xl_only_fifo(val.xl_only);

        fifo_ctrl.set_cfg_chg_en(val.cfg_change_in_fifo);

        fifo_wtm.write(self)?;
        fifo_ctrl.write(self)?;
        ctrl4.write(self)
    }

    /// Retrieves the current FIFO mode configuration.
    ///
    /// # Returns
    ///
    /// - `Result<FifoMode, Error<B::Error>>`:
    ///   - `FifoMode`: The current FIFO mode settings.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `Ctrl4`, `FifoCtrl`, `FifoBatchDec`, and `FifoWtm` registers to retrieve
    /// the current FIFO mode configuration. It returns the settings encapsulated in a `FifoMode` struct.
    pub fn fifo_mode_get(&mut self) -> Result<FifoMode, Error<B::Error>> {
        let ctrl4 = Ctrl4::read(self)?;
        let fifo_ctrl = FifoCtrl::read(self)?;
        let fifo_wtm = FifoWtm::read(self)?;

        // Get FIFO mode
        let operation = if ctrl4.fifo_en() == 0 {
            FifoOperation::FifoOff
        } else {
            fifo_ctrl.fifo_mode().try_into().unwrap_or_default()
        };

        let store = fifo_ctrl.fifo_depth().try_into().unwrap_or_default();

        Ok(FifoMode {
            operation,
            store,
            xl_only: fifo_wtm.xl_only_fifo(),
            cfg_change_in_fifo: fifo_ctrl.cfg_chg_en(),
        })
    }

    /// Sets the FIFO watermark level.
    ///
    /// The FIFO watermark is a programmable threshold that determines when a FIFO threshold interrupt is generated.
    /// When the number of unread samples in the FIFO buffer reaches or exceeds this level, the device can trigger an interrupt
    /// (if enabled via the INT1_FIFO_TH or INT2_FIFO_TH bits in CTRL2/CTRL3).
    ///
    /// Registers used:
    /// - Writes to the `FIFO_WTM` (0x16) register, FTH[6:0] field. (See datasheet Table 44, Section 8.11)
    ///
    /// # Parameters
    /// - `val`: Watermark threshold value (0..=127). Each unit corresponds to one FIFO sample (1 sample = 7 bytes: 1 TAG + 6 DATA).
    ///
    /// # Returns
    /// - `Ok(())` on success.
    /// - `Err`: If the value is out of range or a bus error occurs.
    ///
    /// # Panics
    /// - Panics if `val >= 128` (assertion).
    ///
    /// # Example
    /// ```rust
    /// sensor.fifo_watermark_set(32)?;
    /// ```
    pub fn fifo_watermark_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        assert!(val < 128);

        let mut fifo_wtm = FifoWtm::read(self)?;
        fifo_wtm.set_fth(val);
        fifo_wtm.write(self)
    }

    /// Retrieves the current FIFO watermark threshold value.
    ///
    /// This function reads the FIFO_WTM register and returns the current watermark threshold (FTH[6:0]).
    /// The watermark determines when the FIFO threshold interrupt is triggered (if enabled).
    ///
    /// Registers used:
    /// - Reads from the `FIFO_WTM` (0x16) register, FTH[6:0] field. (See datasheet Table 44, Section 8.11)
    ///
    /// # Returns
    /// - `Ok(u8)`: The current FIFO watermark threshold (0..=127).
    /// - `Err`: If a bus or register access error occurs.
    ///
    /// # Example
    /// ```rust
    /// let wtm = sensor.fifo_watermark_get()?;
    /// ```
    pub fn fifo_watermark_get(&mut self) -> Result<u8, Error<B::Error>> {
        FifoWtm::read(self).map(|reg| reg.fth())
    }

    /// Configures FIFO batching for timestamp and accelerometer data.
    ///
    /// This function sets the decimation rate for timestamp batching and the batch data rate (BDR) for accelerometer data
    /// in the FIFO. Batching allows the device to store data at a reduced rate, optimizing memory usage and power consumption.
    ///
    /// Registers used:
    /// - Writes to the `FIFO_BATCH_DEC` (0x47) register:
    ///   - DEC_TS_BATCH[1:0]: Timestamp decimation (see Table 117)
    ///   - BDR_XL[2:0]: Accelerometer batch data rate (see Table 118)
    ///
    /// # Parameters
    /// - `val`: Reference to a `Batch` struct containing the desired decimation and batch data rate settings.
    ///
    /// # Returns
    /// - `Ok(())` on success.
    /// - `Err`: If a bus or register access error occurs.
    ///
    /// # Example
    /// ```rust
    /// let batch = Batch { dec_ts: 1, bdr_xl: 2 };
    /// sensor.fifo_batch_set(&batch)?;
    /// ```
    pub fn fifo_batch_set(&mut self, val: &Batch) -> Result<(), Error<B::Error>> {
        let mut fifo_batch = FifoBatchDec::read(self)?;

        // Set batching info
        fifo_batch.set_dec_ts_batch(val.dec_ts as u8);
        fifo_batch.set_bdr_xl(val.bdr_xl as u8);

        fifo_batch.write(self)
    }

    /// Reads the current FIFO batching configuration.
    ///
    /// This function retrieves the current decimation rate for timestamp batching and the batch data rate (BDR)
    /// for accelerometer data from the FIFO_BATCH_DEC register.
    ///
    /// Registers used:
    /// - Reads from the `FIFO_BATCH_DEC` (0x47) register:
    ///   - DEC_TS_BATCH[1:0]: Timestamp decimation
    ///   - BDR_XL[2:0]: Accelerometer batch data rate
    ///
    /// # Returns
    /// - `Ok(Batch)`: Struct containing the current batching configuration.
    /// - `Err`: If a bus or register access error occurs.
    ///
    /// # Example
    /// ```rust
    /// let batch = sensor.fifo_batch_get()?;
    /// ```
    pub fn fifo_batch_get(&mut self) -> Result<Batch, Error<B::Error>> {
        let reg = FifoBatchDec::read(self)?;
        Ok(Batch {
            dec_ts: reg.dec_ts_batch().try_into().unwrap_or_default(),
            bdr_xl: reg.bdr_xl().try_into().unwrap_or_default(),
        })
    }

    /// Enables or disables the FIFO stop-on-watermark feature.
    ///
    /// When enabled, the FIFO buffer stops collecting new data once the number of unread samples reaches the watermark threshold.
    /// This is useful for applications that require precise control over the number of samples collected in the FIFO.
    /// When disabled, the FIFO continues to collect data, potentially overwriting the oldest samples (depending on FIFO mode).
    ///
    /// Registers used:
    /// - Writes to the `FIFO_CTRL` (0x15) register, STOP_ON_FTH bit (bit 4) (See datasheet Table 41, Section 8.10)
    ///
    /// # Parameters
    /// - `fth`: `FifoEvent` enum value indicating whether to enable or disable the stop-on-watermark feature.
    ///
    /// # Returns
    /// - `Ok(())` on success.
    /// - `Err`: If a bus or register access error occurs.
    ///
    /// # Example
    /// ```rust
    /// sensor.fifo_stop_on_wtm_set(FifoEvent::Enable)?;
    /// ```
    pub fn fifo_stop_on_wtm_set(&mut self, fth: FifoEvent) -> Result<(), Error<B::Error>> {
        let mut fifo_ctrl = FifoCtrl::read(self)?;
        fifo_ctrl.set_stop_on_fth(fth as u8);
        fifo_ctrl.write(self)
    }

    /// Reads the current status of the FIFO stop-on-watermark feature.
    ///
    /// This function checks whether the FIFO is configured to stop collecting data when the watermark threshold is reached.
    /// Returns the current setting as a `FifoEvent` enum.
    ///
    /// Registers used:
    /// - Reads from the `FIFO_CTRL` (0x15) register, STOP_ON_FTH bit (bit 4) (See datasheet Table 41, Section 8.10)
    ///
    /// # Returns
    /// - `Ok(FifoEvent)`: Current stop-on-watermark configuration.
    /// - `Err`: If a bus or register access error occurs.
    ///
    /// # Example
    /// ```rust
    /// let stop_on_wtm = sensor.fifo_stop_on_wtm_get()?;
    /// ```
    pub fn fifo_stop_on_wtm_get(&mut self) -> Result<FifoEvent, Error<B::Error>> {
        let ctrl = FifoCtrl::read(self)?;
        let evt = FifoEvent::try_from(ctrl.stop_on_fth()).unwrap_or_default();
        Ok(evt)
    }

    /// Retrieves the number of unread sensor data entries stored in the FIFO.
    ///
    /// # Returns
    ///
    /// - `Result<u16, Error<B::Error>>`:
    ///   - `u16`: The number of unread sensor data entries (TAG + 6 bytes) in the FIFO.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `FifoStatus2` register to determine the number of unread sensor data
    /// entries stored in the FIFO.
    pub fn fifo_data_level_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ok(FifoStatus2::read(self)?.fss())
    }

    /// Retrieves the FIFO watermark flag status.
    ///
    /// # Returns
    ///
    /// - `Result<u8, Error<B::Error>>`:
    ///   - `u8`: The FIFO watermark flag status.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `FifoStatus1` register to determine the FIFO watermark flag status.
    pub fn fifo_wtm_flag_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ok(FifoStatus1::read(self)?.fifo_wtm_ia())
    }

    /// Retrieves the sensor tag from the FIFO data output.
    ///
    /// # Returns
    ///
    /// - `Result<FifoSensorTag, Error<B::Error>>`:
    ///   - `FifoSensorTag`: The sensor tag from the FIFO data output.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `FifoDataOutTag` register to retrieve the sensor tag from the FIFO data output.
    pub fn fifo_sensor_tag_get(&mut self) -> Result<FifoSensorTag, Error<B::Error>> {
        let fifo_tag = FifoDataOutTag::read(self)?;
        let val = FifoSensorTag::try_from(fifo_tag.tag_sensor()).unwrap_or_default();
        Ok(val)
    }

    /// Retrieves raw data from the FIFO output.
    ///
    /// # Returns
    ///
    /// - `Result<[u8; 6], Error<B::Error>>`:
    ///   - `[u8; 6]`: The raw data from the FIFO output.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `FifoDataOutXL` register to retrieve raw data from the FIFO output.
    pub fn fifo_out_raw_get(&mut self) -> Result<[u8; 6], Error<B::Error>> {
        let mut buff: [u8; 6] = [0; 6];
        self.read_from_register(Reg::FifoDataOutXL as u8, &mut buff)?;
        Ok(buff)
    }

    /// Retrieves and processes FIFO data based on the sensor and FIFO mode configurations.
    ///
    /// # Arguments
    ///
    /// - `md: &Md`: The sensor mode configuration.
    /// - `f_md: &FifoMode`: The FIFO mode configuration.
    ///
    /// # Returns
    ///
    /// - `Result<FifoData, Error<B::Error>>`:
    ///   - `FifoData`: The processed FIFO data.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function retrieves and processes FIFO data based on the provided sensor and FIFO mode configurations.
    /// It reads the `FifoDataOutTag` and `FifoDataOutXL` registers to obtain raw data and processes it according
    /// to the sensor tag and mode settings.
    pub fn fifo_data_get(&mut self, md: &Md, f_md: &FifoMode) -> Result<FifoData, Error<B::Error>> {
        let fifo_tag = FifoDataOutTag::read(self)?;

        let tag_sensor = FifoSensorTag::try_from(fifo_tag.tag_sensor()).unwrap_or_default();

        let fifo_raw: [u8; 6] = self.fifo_out_raw_get()?;

        let mut data = FifoData {
            tag: fifo_tag.tag_sensor(),
            ..Default::default()
        };

        match tag_sensor {
            FifoSensorTag::XlOnly2xTag | FifoSensorTag::XlOnly2xTag2nd => {
                // A FIFO sample consists of 2X 8-bits 3-axis XL at ODR/2
                for i in 0..3 {
                    data.xl[0].raw[i] = i16::from_le_bytes([0, fifo_raw[i]]);
                    data.xl[1].raw[i] = i16::from_le_bytes([0, fifo_raw[3 + i]]);
                }
            }
            FifoSensorTag::XlAndQvar | FifoSensorTag::XlTempTag => {
                if f_md.xl_only == 0 {
                    // A FIFO sample consists of 12-bits 3-axis XL + T at ODR
                    data.xl[0].raw[0] = ((fifo_raw[0] as i16) + ((fifo_raw[1] as i16) << 8)) << 4;
                    data.xl[0].raw[1] =
                        (((fifo_raw[1] as i16) >> 4) + ((fifo_raw[2] as i16) << 4)) << 4;
                    data.xl[0].raw[2] = ((fifo_raw[3] as i16) + ((fifo_raw[4] as i16) << 8)) << 4;

                    data.heat.raw = (fifo_raw[4] as i16) >> 4;

                    if tag_sensor == FifoSensorTag::XlTempTag {
                        data.heat.deg_c = from_lsb_to_celsius(data.heat.raw);
                    } else {
                        data.ah_qvar.raw = data.heat.raw;
                        data.ah_qvar.mv = from_lsb_to_mv(data.ah_qvar.raw);
                    }
                } else {
                    // A FIFO sample consists of 16-bits 3-axis XL at ODR
                    data.xl[0].raw[0] = i16::from_le_bytes([fifo_raw[0], fifo_raw[1]]);
                    data.xl[0].raw[1] = i16::from_le_bytes([fifo_raw[2], fifo_raw[3]]);
                    data.xl[0].raw[2] = i16::from_le_bytes([fifo_raw[4], fifo_raw[5]]);
                }
            }
            FifoSensorTag::TimestampTag => {
                data.cfg_chg.cfg_change = fifo_raw[0] >> 7;
                data.cfg_chg.odr = (fifo_raw[0] >> 3) & 0xF;
                data.cfg_chg.bw = (fifo_raw[0] >> 1) & 0x3;
                data.cfg_chg.lp_hp = fifo_raw[0] & 0x1;
                data.cfg_chg.qvar_en = fifo_raw[1] >> 7;
                data.cfg_chg.fs = (fifo_raw[1] >> 5) & 0x3;
                data.cfg_chg.dec_ts = (fifo_raw[1] >> 3) & 0x3;
                data.cfg_chg.odr_xl_batch = fifo_raw[1] & 0x7;

                data.cfg_chg.timestamp =
                    u32::from_le_bytes([fifo_raw[2], fifo_raw[3], fifo_raw[4], fifo_raw[5]]);
            }
            FifoSensorTag::StepCounterTag => {
                data.pedo.steps = u32::from_le_bytes([fifo_raw[0], fifo_raw[1], 0, 0]);

                data.pedo.timestamp =
                    u32::from_le_bytes([fifo_raw[2], fifo_raw[3], fifo_raw[4], fifo_raw[5]]);
            }
            _ => {}
        }

        for i in 0..3 {
            match md.fs {
                Fs::_2g => {
                    data.xl[0].mg[i] = from_fs2g_to_mg(data.xl[0].raw[i]);
                    data.xl[1].mg[i] = from_fs2g_to_mg(data.xl[1].raw[i]);
                }
                Fs::_4g => {
                    data.xl[0].mg[i] = from_fs4g_to_mg(data.xl[0].raw[i]);
                    data.xl[1].mg[i] = from_fs4g_to_mg(data.xl[1].raw[i]);
                }
                Fs::_8g => {
                    data.xl[0].mg[i] = from_fs8g_to_mg(data.xl[0].raw[i]);
                    data.xl[1].mg[i] = from_fs8g_to_mg(data.xl[1].raw[i]);
                }
                Fs::_16g => {
                    data.xl[0].mg[i] = from_fs16g_to_mg(data.xl[0].raw[i]);
                    data.xl[1].mg[i] = from_fs16g_to_mg(data.xl[1].raw[i]);
                }
            }
        }

        Ok(data)
    }

    /// Enables and configures the AH_QVAR chain.
    ///
    /// # Arguments
    ///
    /// - `val: AhQvarMode`: The configuration settings for the AH_QVAR chain.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration of the AH_QVAR chain.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function configures the AH_QVAR chain by setting parameters such as gain, input impedance,
    /// notch filter cutoff, and enabling the AH_QVAR chain. It modifies the `AhQvarCfg` register to apply
    /// the settings.
    pub fn ah_qvar_mode_set(&mut self, val: &AhQvarMode) -> Result<(), Error<B::Error>> {
        let mut ah_qvar_cfg = AhQvarCfg::read(self)?;

        ah_qvar_cfg.set_ah_qvar_gain(val.ah_qvar_gain as u8);
        ah_qvar_cfg.set_ah_qvar_c_zin(val.ah_qvar_zin as u8);
        ah_qvar_cfg.set_ah_qvar_notch_cutoff(val.ah_qvar_notch as u8);
        ah_qvar_cfg.set_ah_qvar_notch_en(val.ah_qvar_notch_en);
        ah_qvar_cfg.set_ah_qvar_en(val.ah_qvar_en);

        ah_qvar_cfg.write(self)
    }

    /// Retrieves the current configuration of the AH_QVAR chain.
    ///
    /// # Returns
    ///
    /// - `Result<AhQvarMode, Error<B::Error>>`:
    ///   - `AhQvarMode`: The current configuration settings for the AH_QVAR chain.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `AhQvarCfg` register to retrieve the current configuration of the AH_QVAR chain.
    /// It returns the settings encapsulated in an `AhQvarMode` struct.
    pub fn ah_qvar_mode_get(&mut self) -> Result<AhQvarMode, Error<B::Error>> {
        let ah_qvar_cfg = AhQvarCfg::read(self)?;

        let val = AhQvarMode {
            ah_qvar_en: ah_qvar_cfg.ah_qvar_en(),
            ah_qvar_notch_en: ah_qvar_cfg.ah_qvar_notch_en(),
            ah_qvar_notch: ah_qvar_cfg
                .ah_qvar_notch_cutoff()
                .try_into()
                .unwrap_or_default(),
            ah_qvar_zin: ah_qvar_cfg.ah_qvar_c_zin().try_into().unwrap_or_default(),
            ah_qvar_gain: ah_qvar_cfg.ah_qvar_gain().try_into().unwrap_or_default(),
        };

        Ok(val)
    }

    /// Sets the step counter mode.
    ///
    /// # Arguments
    ///
    /// - `val: StpcntMode`: The configuration settings for the step counter mode.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration of the step counter mode.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function configures the step counter mode by setting parameters such as false step rejection,
    /// step counter enable, and step counter FIFO inclusion. It modifies the `EmbFuncEnA`, `EmbFuncFifoEn`,
    /// and `PedoCmdReg` registers to apply the settings.
    pub fn stpcnt_mode_set(&mut self, val: &StpcntMode) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_emb(self, |state| {
            let mut emb_func_en_a = EmbFuncEnA::read(state)?;
            let emb_func_en_b = EmbFuncEnB::read(state)?;
            let mut emb_func_fifo_en = EmbFuncFifoEn::read(state)?;

            if val.false_step_rej == PROPERTY_ENABLE
                && (emb_func_en_a.mlc_before_fsm_en() & emb_func_en_b.mlc_en()) == PROPERTY_DISABLE
            {
                emb_func_en_a.set_mlc_before_fsm_en(PROPERTY_ENABLE);
            }

            emb_func_fifo_en.set_step_counter_fifo_en(val.step_counter_in_fifo);
            emb_func_fifo_en.write(state)?;

            emb_func_en_a.set_pedo_en(val.step_counter_enable);
            emb_func_en_a.write(state)
        })?;

        let mut pedo_cmd_reg = PedoCmdReg::read(self)?;
        pedo_cmd_reg.set_fp_rejection_en(val.false_step_rej);
        pedo_cmd_reg.write(self)
    }

    /// Retrieves the current step counter mode configuration.
    ///
    /// # Returns
    ///
    /// - `Result<StpcntMode, Error<B::Error>>`:
    ///   - `StpcntMode`: The current configuration settings for the step counter mode.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `EmbFuncEnA` and `PedoCmdReg` registers to retrieve the current step counter
    /// mode configuration. It returns the settings encapsulated in a `StpcntMode` struct.
    pub fn stpcnt_mode_get(&mut self) -> Result<StpcntMode, Error<B::Error>> {
        let (step_counter_enable, step_counter_in_fifo) =
            MemBank::operate_over_emb(self, |state| {
                let stpcnt_en = EmbFuncEnA::read(state)?.pedo_en();
                let stpcnt_in_fifo = EmbFuncFifoEn::read(state)?.step_counter_fifo_en();
                Ok((stpcnt_en, stpcnt_in_fifo))
            })?;
        let pedo_cmd_reg = PedoCmdReg::read(self)?;

        let val = StpcntMode {
            false_step_rej: pedo_cmd_reg.fp_rejection_en(),
            step_counter_enable,
            step_counter_in_fifo,
        };

        Ok(val)
    }

    /// Retrieves the number of detected steps from the step counter.
    ///
    /// # Returns
    ///
    /// - `Result<u16, Error<B::Error>>`:
    ///   - `u16`: The number of detected steps.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `StepCounterL` register to retrieve the number of detected steps from the
    /// step counter.
    pub fn stpcnt_steps_get(&mut self) -> Result<u16, Error<B::Error>> {
        MemBank::operate_over_emb(self, |state| Ok(StepCounter::read(state)?.step()))
    }

    /// Resets the step counter.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful reset of the step counter.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function resets the step counter by modifying the `EmbFuncSrc` register.
    pub fn stpcnt_rst_step_set(&mut self) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_emb(self, |state| {
            let mut emb_func_src = EmbFuncSrc::read(state)?;
            emb_func_src.set_pedo_rst_step(1);
            emb_func_src.write(state)
        })
    }

    /// Configures the pedometer debounce setting.
    ///
    /// # Arguments
    ///
    /// - `val: u8`: The pedometer debounce configuration value.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration of the pedometer debounce setting.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function configures the pedometer debounce setting by writing to the `PedoDebStepsConf` register.
    pub fn stpcnt_debounce_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        PedoDebStepsConf::from_bits(val).write(self)
    }

    /// Retrieves the pedometer debounce configuration.
    ///
    /// # Returns
    ///
    /// - `Result<u8, Error<B::Error>>`:
    ///   - `u8`: The current pedometer debounce configuration value.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `PedoDebStepsConf` register to retrieve the current pedometer debounce configuration
    pub fn stpcnt_debounce_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ok(PedoDebStepsConf::read(self)?.deb_step())
    }

    /// Sets the time period for step detection based on delta time.
    ///
    /// # Arguments
    ///
    /// - `val: u16`: The time period value for step detection on delta time.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration of the time period for step detection.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function configures the time period for step detection based on delta time by writing to the
    /// `PedoScDeltatL` register.
    pub fn stpcnt_period_set(&mut self, val: u16) -> Result<(), Error<B::Error>> {
        PedoScDeltat::from_bits(val).write(self)
    }

    /// Retrieves the time period for step detection based on delta time.
    ///
    /// # Returns
    ///
    /// - `Result<u16, Error<B::Error>>`:
    ///   - `u16`: The current time period value for step detection on delta time.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `PedoScDeltatL` register to retrieve the current time period for step detection
    /// based on delta time.
    pub fn stpcnt_period_get(&mut self) -> Result<u16, Error<B::Error>> {
        Ok(PedoScDeltat::read(self)?.pd_sc())
    }

    /// Configures the smart power functionality.
    ///
    /// # Arguments
    ///
    /// - `val: SmartPowerCfg`: The configuration settings for the smart power functionality.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration of the smart power functionality.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function configures the smart power functionality by setting parameters such as enable status,
    /// window, and duration. It modifies the `Ctrl1` and `SmartPowerCtrl` registers to apply the settings.
    pub fn smart_power_set(&mut self, val: SmartPowerCfg) -> Result<(), Error<B::Error>> {
        let mut ctrl1 = Ctrl1::read(self)?;
        ctrl1.set_smart_power_en(val.enable);
        ctrl1.write(self)?;

        if val.enable == 0 {
            // If disabling smart_power no need to set win/dur fields
            return Ok(());
        }

        let mut smart_power_ctrl = SmartPowerCtrl::default();
        smart_power_ctrl.set_smart_power_ctrl_win(val.window);
        smart_power_ctrl.set_smart_power_ctrl_dur(val.duration);
        smart_power_ctrl.write(self)
    }

    /// Retrieves the current smart power configuration.
    ///
    /// # Returns
    ///
    /// - `Result<SmartPowerCfg, Error<B::Error>>`:
    ///   - `SmartPowerCfg`: The current configuration settings for the smart power functionality.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `Ctrl1` and `SmartPowerCtrl` registers to retrieve the current smart power
    /// configuration. It returns the settings encapsulated in a `SmartPowerCfg` struct.
    pub fn smart_power_get(&mut self) -> Result<SmartPowerCfg, Error<B::Error>> {
        let ctrl1 = Ctrl1::read(self)?;

        let mut val = SmartPowerCfg {
            enable: ctrl1.smart_power_en(),
            window: 0,
            duration: 0,
        };

        let smart_power_ctrl = SmartPowerCtrl::read(self)?;
        val.window = smart_power_ctrl.smart_power_ctrl_win();
        val.duration = smart_power_ctrl.smart_power_ctrl_dur();

        Ok(val)
    }

    /// Configures the tilt calculation mode.
    ///
    /// # Arguments
    ///
    /// - `val: u8`: The configuration value for enabling or disabling tilt calculation.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration of the tilt calculation mode.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function configures the tilt calculation mode by enabling or disabling it in the `EmbFuncEnA` register.
    /// It ensures the correct memory bank is set before and after the operation.
    pub fn tilt_mode_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_emb(self, |state| {
            let mut emb_func_en_a = EmbFuncEnA::read(state)?;
            emb_func_en_a.set_tilt_en(val);
            emb_func_en_a.write(state)
        })
    }

    /// Retrieves the current tilt calculation mode.
    ///
    /// # Returns
    ///
    /// - `Result<u8, Error<B::Error>>`:
    ///   - `u8`: The current configuration value for tilt calculation.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `EmbFuncEnA` register to retrieve the current configuration of the tilt calculation mode.
    /// It ensures the correct memory bank is set before and after the operation.
    pub fn tilt_mode_get(&mut self) -> Result<u8, Error<B::Error>> {
        MemBank::operate_over_emb(self, |state| Ok(EmbFuncEnA::read(state)?.tilt_en()))
    }

    /// Enables or disables the significant motion detection function.
    ///
    /// # Arguments
    ///
    /// - `val: u8`: The configuration value for enabling or disabling significant motion detection.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration of the significant motion detection function.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function configures the significant motion detection function by enabling or disabling it in the
    /// `EmbFuncEnA` register. It ensures the correct memory bank is set before and after the operation.
    pub fn sigmot_mode_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_emb(self, |state| {
            let mut emb_func_en_a = EmbFuncEnA::read(state)?;
            emb_func_en_a.set_sign_motion_en(val);
            emb_func_en_a.write(state)
        })
    }

    /// Retrieves the current configuration of the significant motion detection function.
    ///
    /// # Returns
    ///
    /// - `Result<u8, Error<B::Error>>`:
    ///   - `u8`: The current configuration value for significant motion detection.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `EmbFuncEnA` register to retrieve the current configuration of the significant
    /// motion detection function. It ensures the correct memory bank is set before and after the operation.
    pub fn sigmot_mode_get(&mut self) -> Result<u8, Error<B::Error>> {
        MemBank::operate_over_emb(self, |state| Ok(EmbFuncEnA::read(state)?.sign_motion_en()))
    }

    /// Configures the time window for Free Fall detection.
    ///
    /// # Arguments
    ///
    /// - `val: u8`: The time window configuration value for Free Fall detection (1 LSB = 1/ODR_XL time).
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration of the Free Fall detection time window.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function configures the time window for Free Fall detection by setting the appropriate values in
    /// the `WakeUpDur` and `FreeFall` registers.
    pub fn ff_duration_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut wake_up_dur = WakeUpDur::read(self)?;
        wake_up_dur.set_ff_dur((val >> 5) & 0x1);
        wake_up_dur.write(self)?;

        let mut free_fall = FreeFall::read(self)?;
        free_fall.set_ff_dur(val & 0x1F);
        free_fall.write(self)
    }

    /// Retrieves the current time window configuration for Free Fall detection.
    ///
    /// # Returns
    ///
    /// - `Result<u8, Error<B::Error>>`:
    ///   - `u8`: The current time window configuration value for Free Fall detection (1 LSB = 1/ODR_XL time).
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `WakeUpDur` and `FreeFall` registers to retrieve the current time window
    /// configuration for Free Fall detection.
    pub fn ff_duration_get(&mut self) -> Result<u8, Error<B::Error>> {
        let wake_up_dur = WakeUpDur::read(self)?;
        let free_fall = FreeFall::read(self)?;

        let val = (wake_up_dur.ff_dur() << 5) | free_fall.ff_dur();
        Ok(val)
    }

    /// Sets the Free Fall threshold.
    ///
    /// # Arguments
    ///
    /// - `val: FfThreshold`: The threshold value for Free Fall detection.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration of the Free Fall threshold.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function sets the Free Fall threshold by configuring the `FreeFall` register with the specified
    /// threshold value.
    pub fn ff_thresholds_set(&mut self, val: FfThreshold) -> Result<(), Error<B::Error>> {
        let mut free_fall = FreeFall::read(self)?;
        free_fall.set_ff_ths((val as u8) & 0x7);
        free_fall.write(self)
    }

    /// Retrieves the current Free Fall threshold setting.
    ///
    /// # Returns
    ///
    /// - `Result<FfThreshold, Error<B::Error>>`:
    ///   - `FfThreshold`: The current threshold value for Free Fall detection.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `FreeFall` register to retrieve the current Free Fall threshold setting.
    pub fn ff_thresholds_get(&mut self) -> Result<FfThreshold, Error<B::Error>> {
        let free_fall = FreeFall::read(self)?;
        Ok(free_fall.ff_ths().try_into().unwrap_or_default())
    }

    /// Configures the 4D/6D detection function.
    ///
    /// # Arguments
    ///
    /// - `val: SixdConfig`: The configuration settings for the 4D/6D detection function.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration of the 4D/6D detection function.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function configures the 4D/6D detection function by setting the mode (4D or 6D) and the threshold
    /// in the `Sixd` register.
    pub fn sixd_config_set(&mut self, val: SixdConfig) -> Result<(), Error<B::Error>> {
        let mut sixd = Sixd::read(self)?;
        sixd.set_d4d_en(val.mode as u8);
        sixd.set_d6d_ths(val.threshold as u8);
        sixd.write(self)
    }

    /// Retrieves the current configuration of the 4D/6D detection function.
    ///
    /// # Returns
    ///
    /// - `Result<SixdConfig, Error<B::Error>>`:
    ///   - `SixdConfig`: The current configuration settings for the 4D/6D detection function.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `Sixd` register to retrieve the current configuration of the 4D/6D detection
    /// function. It returns the settings encapsulated in a `SixdConfig` struct.
    pub fn sixd_config_get(&mut self) -> Result<SixdConfig, Error<B::Error>> {
        let sixd = Sixd::read(self)?;

        let mode = sixd.d4d_en().try_into().unwrap_or_default();
        let threshold = sixd.d6d_ths().try_into().unwrap_or_default();

        Ok(SixdConfig { mode, threshold })
    }

    /// Configures the wakeup function.
    ///
    /// # Arguments
    ///
    /// - `val: WakeupConfig`: The configuration settings for the wakeup function.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration of the wakeup function.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function configures the wakeup function by setting parameters such as wake duration, sleep duration,
    /// wake threshold, and inactivity ODR. It modifies the `WakeUpThs`, `WakeUpDur`, `WakeUpDurExt`, `InterruptCfg`,
    /// `Ctrl1`, and `Ctrl4` registers to apply the settings.
    pub fn wakeup_config_set(&mut self, val: WakeupConfig) -> Result<(), Error<B::Error>> {
        let mut wup_ths = WakeUpThs::read(self)?;
        let mut wup_dur = WakeUpDur::read(self)?;
        let mut wup_dur_ext = WakeUpDurExt::read(self)?;
        let mut int_cfg = InterruptCfg::read(self)?;
        let mut ctrl1 = Ctrl1::read(self)?;
        let mut ctrl4 = Ctrl4::read(self)?;

        wup_dur.set_wake_dur(val.wake_dur.wake_dur());
        wup_dur_ext.set_wu_dur_extended(val.wake_dur.wake_up_dur_ext());
        wup_dur.set_sleep_dur(val.sleep_dur);

        int_cfg.set_wake_ths_w(val.wake_ths_weight);
        wup_ths.set_wk_ths(val.wake_ths);
        wup_ths.set_sleep_on(val.wake_enable as u8);
        ctrl4.set_inact_odr(val.inact_odr as u8);

        if val.wake_enable == WakeEnable::SleepOn {
            ctrl1.set_wu_x_en(1);
            ctrl1.set_wu_y_en(1);
            ctrl1.set_wu_z_en(1);
        } else {
            ctrl1.set_wu_x_en(0);
            ctrl1.set_wu_y_en(0);
            ctrl1.set_wu_z_en(0);
        }

        wup_ths.write(self)?;
        wup_dur.write(self)?;
        wup_dur_ext.write(self)?;
        int_cfg.write(self)?;
        ctrl1.write(self)?;
        ctrl4.write(self)
    }

    /// Retrieves the current configuration of the wakeup function.
    ///
    /// # Returns
    ///
    /// - `Result<WakeupConfig, Error<B::Error>>`:
    ///   - `WakeupConfig`: The current configuration settings for the wakeup function.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `WakeUpThs`, `WakeUpDur`, `WakeUpDurExt`, `InterruptCfg`, and `Ctrl4` registers
    /// to retrieve the current configuration of the wakeup function. It returns the settings encapsulated in
    /// a `WakeupConfig` struct.
    pub fn wakeup_config_get(&mut self) -> Result<WakeupConfig, Error<B::Error>> {
        let wup_ths = WakeUpThs::read(self)?;
        let wup_dur = WakeUpDur::read(self)?;
        let wup_dur_ext = WakeUpDurExt::read(self)?;
        let int_cfg = InterruptCfg::read(self)?;
        let ctrl4 = Ctrl4::read(self)?;

        Ok(WakeupConfig {
            wake_dur: WakeDur::new(wup_dur_ext.wu_dur_extended(), wup_dur.wake_dur()),
            sleep_dur: wup_dur.sleep_dur(),
            wake_ths: wup_ths.wk_ths(),
            wake_ths_weight: int_cfg.wake_ths_w(),
            wake_enable: wup_ths.sleep_on().try_into().unwrap_or_default(),
            inact_odr: ctrl4.inact_odr().try_into().unwrap_or_default(),
        })
    }

    /// Configures the tap detection settings.
    ///
    /// # Arguments
    ///
    /// - `val: TapConfig`: The configuration settings for tap detection.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration of the tap detection settings.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function configures the tap detection settings by writing to multiple registers, including
    /// `TapCfg0`, `TapCfg1`, `TapCfg2`, `TapCfg3`, `TapCfg4`, `TapCfg5`, and `TapCfg6`. It allows customization
    /// of parameters such as axis selection, thresholds, timings, and enabling single, double, or triple tap detection.
    pub fn tap_config_set(&mut self, val: TapConfig) -> Result<(), Error<B::Error>> {
        let mut tap_cfg0 = TapCfg0::read(self)?;
        let mut tap_cfg1 = TapCfg1::read(self)?;
        let mut tap_cfg2 = TapCfg2::read(self)?;
        let mut tap_cfg3 = TapCfg3::read(self)?;
        let mut tap_cfg4 = TapCfg4::read(self)?;
        let mut tap_cfg5 = TapCfg5::read(self)?;
        let mut tap_cfg6 = TapCfg6::read(self)?;

        tap_cfg0.set_axis(val.axis as u8);
        tap_cfg0.set_invert_t(val.inverted_peak_time);
        tap_cfg1.set_pre_still_ths(val.pre_still_ths);
        tap_cfg3.set_post_still_ths(val.post_still_ths);
        tap_cfg1.set_post_still_t(val.post_still_time & 0xF);
        tap_cfg2.set_post_still_t(val.post_still_time >> 4);
        tap_cfg2.set_wait_t(val.shock_wait_time);
        tap_cfg3.set_latency_t(val.latency);
        tap_cfg4.set_wait_end_latency(val.wait_end_latency);
        tap_cfg4.set_peak_ths(val.peak_ths);
        tap_cfg5.set_rebound_t(val.rebound);
        tap_cfg5.set_single_tap_en(val.single_tap_on);
        tap_cfg5.set_double_tap_en(val.double_tap_on);
        tap_cfg5.set_triple_tap_en(val.triple_tap_on);
        tap_cfg6.set_pre_still_st(val.pre_still_start);
        tap_cfg6.set_pre_still_n(val.pre_still_n);

        tap_cfg0.write(self)?;
        tap_cfg1.write(self)?;
        tap_cfg2.write(self)?;
        tap_cfg3.write(self)?;
        tap_cfg4.write(self)?;
        tap_cfg5.write(self)?;
        tap_cfg6.write(self)
    }

    /// Retrieves the current tap detection configuration.
    ///
    /// # Returns
    ///
    /// - `Result<TapConfig, Error<B::Error>>`:
    ///   - `TapConfig`: The current configuration settings for tap detection.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads multiple registers, including `TapCfg0`, `TapCfg1`, `TapCfg2`, `TapCfg3`, `TapCfg4`,
    /// `TapCfg5`, and `TapCfg6`, to retrieve the current tap detection configuration. It returns the settings
    /// encapsulated in a `TapConfig` struct.
    pub fn tap_config_get(&mut self) -> Result<TapConfig, Error<B::Error>> {
        let tap_cfg0 = TapCfg0::read(self)?;
        let tap_cfg1 = TapCfg1::read(self)?;
        let tap_cfg2 = TapCfg2::read(self)?;
        let tap_cfg3 = TapCfg3::read(self)?;
        let tap_cfg4 = TapCfg4::read(self)?;
        let tap_cfg5 = TapCfg5::read(self)?;
        let tap_cfg6 = TapCfg6::read(self)?;

        Ok(TapConfig {
            axis: tap_cfg0.axis().try_into().unwrap_or_default(),
            inverted_peak_time: tap_cfg0.invert_t(),
            pre_still_ths: tap_cfg1.pre_still_ths(),
            post_still_ths: tap_cfg3.post_still_ths(),
            post_still_time: (tap_cfg2.post_still_t() << 4) | tap_cfg1.post_still_t(),
            shock_wait_time: tap_cfg2.wait_t(),
            latency: tap_cfg3.latency_t(),
            wait_end_latency: tap_cfg4.wait_end_latency(),
            peak_ths: tap_cfg4.peak_ths(),
            rebound: tap_cfg5.rebound_t(),
            pre_still_start: tap_cfg6.pre_still_st(),
            pre_still_n: tap_cfg6.pre_still_n(),
            single_tap_on: tap_cfg5.single_tap_en(),
            double_tap_on: tap_cfg5.double_tap_en(),
            triple_tap_on: tap_cfg5.triple_tap_en(),
        })
    }

    /// Enables the timestamp counter.
    ///
    /// # Arguments
    ///
    /// - `val: u8`: The value to enable or disable the timestamp counter.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration of the timestamp counter.
    ///   - `Err`: Returns an error if the operation fails.
    ///
    /// # Description
    ///
    /// This function enables or disables the timestamp counter by modifying the `timestamp_en` field
    /// in the `INTERRUPT_CFG` register.
    pub fn timestamp_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut int_cfg = InterruptCfg::read(self)?;
        int_cfg.set_timestamp_en(val);
        int_cfg.write(self)
    }

    /// Retrieves the current state of the timestamp enable/disable counter field.
    ///
    /// # Returns
    ///
    /// - `Result<u8, Error<B::Error>>`:
    ///   - `u8`: The value of the `timestamp_en` field in the `INTERRUPT_CFG` register.
    ///   - `Err`: Returns an error if the operation fails.
    ///
    /// # Description
    ///
    /// This function reads the `INTERRUPT_CFG` register to retrieve the current state of the timestamp enable/disable counter field.
    pub fn timestamp_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ok(InterruptCfg::read(self)?.timestamp_en())
    }

    /// Retrieves the raw timestamp value.
    ///
    /// # Returns
    ///
    /// - `Result<u32, Error<B::Error>>`:
    ///   - `u32`: The raw timestamp value expressed as a 32-bit word with a resolution of 10 µs.
    ///   - `Err`: Returns an error if the operation fails.
    ///
    /// # Description
    ///
    /// This function reads the `Timestamp0` register to retrieve the raw timestamp value.
    pub fn timestamp_raw_get(&mut self) -> Result<u32, Error<B::Error>> {
        Ok(Timestamp::read(self)?.timestamp())
    }

    /// Retrieves the FSM long counter timeout interrupt status.
    ///
    /// # Returns
    ///
    /// - `Result<u8, Error<B::Error>>`:
    ///   - `u8`: The value of the `is_fsm_lc` field in the `EMB_FUNC_STATUS` register.
    ///   - `Err`: Returns an error if the operation fails.
    ///
    /// # Description
    ///
    /// This function retrieves the FSM long counter timeout interrupt status by reading the `EMB_FUNC_STATUS` register.
    pub fn long_cnt_flag_data_ready_get(&mut self) -> Result<u8, Error<B::Error>> {
        MemBank::operate_over_emb(self, |state| Ok(EmbFuncStatus::read(state)?.is_fsm_lc()))
    }

    /// Configures the embedded final state machine functions mode.
    ///
    /// # Arguments
    ///
    /// - `val: u8`: The value to configure the `fsm_en` field in the `EMB_FUNC_EN_B` register.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration of the FSM mode.
    ///   - `Err`: Returns an error if the operation fails.
    ///
    /// # Description
    ///
    /// This function configures the FSM mode by modifying the `fsm_en` field in the `EMB_FUNC_EN_B` register.
    pub fn emb_fsm_en_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_emb(self, |state| {
            let mut emb_func_en_b = EmbFuncEnB::read(state)?;
            emb_func_en_b.set_fsm_en(val);
            emb_func_en_b.write(state)
        })
    }

    /// Retrieves the current FSM mode configuration.
    ///
    /// # Returns
    ///
    /// - `Result<u8, Error<B::Error>>`:
    ///   - `u8`: The value of the `fsm_en` field in the `EMB_FUNC_EN_B` register.
    ///   - `Err`: Returns an error if the operation fails.
    ///
    /// # Description
    ///
    /// This function reads the `EMB_FUNC_EN_B` register to retrieve the current FSM mode configuration.
    pub fn emb_fsm_en_get(&mut self) -> Result<u8, Error<B::Error>> {
        MemBank::operate_over_emb(self, |state| Ok(EmbFuncEnB::read(state)?.fsm_en()))
    }

    /// Configures the FSM enable registers.
    ///
    /// # Arguments
    ///
    /// - `val: &EmbFsmEnable`: The structure containing the FSM enable configuration.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration of the FSM enable registers.
    ///   - `Err`: Returns an error if the operation fails.
    ///
    /// # Description
    ///
    /// This function configures the FSM enable registers by writing to the `FSM_ENABLE_A` and `FSM_ENABLE_B` registers.
    pub fn fsm_enable_set(&mut self, val: &FsmEnable) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_emb(self, |state| {
            val.write(state)?;
            let mut emb_func_en_b = EmbFuncEnB::read(state)?;

            emb_func_en_b.set_fsm_en(
                if (val.fsm1_en()
                    | val.fsm2_en()
                    | val.fsm3_en()
                    | val.fsm4_en()
                    | val.fsm5_en()
                    | val.fsm6_en()
                    | val.fsm7_en()
                    | val.fsm8_en())
                    != PROPERTY_DISABLE
                {
                    PROPERTY_ENABLE
                } else {
                    PROPERTY_DISABLE
                },
            );

            emb_func_en_b.write(state)
        })
    }

    /// Retrieves the current FSM enable configuration.
    ///
    /// # Returns
    ///
    /// - `Result<EmbFsmEnable, Error<B::Error>>`:
    ///   - `EmbFsmEnable`: The structure containing the FSM enable configuration.
    ///   - `Err`: Returns an error if the operation fails.
    ///
    /// # Description
    ///
    /// This function reads the `FSM_ENABLE_A` and `FSM_ENABLE_B` registers to retrieve the current FSM enable configuration.
    pub fn fsm_enable_get(&mut self) -> Result<FsmEnable, Error<B::Error>> {
        MemBank::operate_over_emb(self, |state| FsmEnable::read(state))
    }

    /// Configures the FSM long counter value.
    ///
    /// # Arguments
    ///
    /// - `val: u16`: The long counter value to configure.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration of the FSM long counter.
    ///   - `Err`: Returns an error if the operation fails.
    ///
    /// # Description
    ///
    /// This function configures the FSM long counter value by writing to the `FSM_LONG_COUNTER_L` register.
    pub fn long_cnt_set(&mut self, val: u16) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_emb(self, |state| FsmLongCounter::from_bits(val).write(state))
    }

    /// Retrieves the current FSM long counter value.
    ///
    /// # Returns
    ///
    /// - `Result<u16, Error<B::Error>>`:
    ///   - `u16`: The current FSM long counter value.
    ///   - `Err`: Returns an error if the operation fails.
    ///
    /// # Description
    ///
    /// This function reads the `FSM_LONG_COUNTER_L` register to retrieve the current FSM long counter value.
    pub fn long_cnt_get(&mut self) -> Result<u16, Error<B::Error>> {
        MemBank::operate_over_emb(self, |state| Ok(FsmLongCounter::read(state)?.fsm_lc()))
    }

    /// Retrieves the FSM status.
    ///
    /// # Returns
    ///
    /// - `Result<FsmStatusMainpage, Error<B::Error>>`:
    ///   - `FsmStatusMainpage`: The FSM status from the `FSM_STATUS_MAINPAGE` register.
    ///   - `Err`: Returns an error if the operation fails.
    ///
    /// # Description
    ///
    /// This function reads the `FSM_STATUS_MAINPAGE` register to retrieve the FSM status.
    pub fn fsm_status_get(&mut self) -> Result<FsmStatusMainpage, Error<B::Error>> {
        FsmStatusMainpage::read(self)
    }

    /// Retrieves the FSM output registers.
    ///
    /// # Returns
    ///
    /// - `Result<[u8; 8], Error<B::Error>>`:
    ///   - `[u8; 8]`: The FSM output values from the `FSM_OUTS1` to `FSM_OUTS16` registers.
    ///   - `Err`: Returns an error if the operation fails.
    ///
    /// # Description
    ///
    /// This function reads the FSM output registers to retrieve the FSM output values.
    pub fn fsm_out_get(&mut self) -> Result<[u8; 8], Error<B::Error>> {
        MemBank::operate_over_emb(self, |state| {
            let mut val: [u8; 8] = [0; 8];
            FsmOuts::read_more(state, &mut val)?;
            Ok(val)
        })
    }

    /// Configures the FSM output data rate (ODR).
    ///
    /// # Arguments
    ///
    /// - `val: FsmValOdr`: The ODR value to configure in the `EMB_FUNC_ODR_CFG_B` register.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration of the FSM ODR.
    ///   - `Err`: Returns an error if the operation fails.
    ///
    /// # Description
    ///
    /// This function configures the FSM ODR by modifying the `fsm_odr` field in the `EMB_FUNC_ODR_CFG_B` register.
    pub fn fsm_data_rate_set(&mut self, val: FsmValOdr) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_emb(self, |state| {
            let mut fsm_odr_reg = FsmOdr::read(state)?;
            fsm_odr_reg.set_fsm_odr(val as u8);
            fsm_odr_reg.write(state)
        })
    }

    /// Retrieves the current FSM output data rate (ODR).
    ///
    /// # Returns
    ///
    /// - `Result<FsmValOdr, Error<B::Error>>`:
    ///   - `FsmValOdr`: The current FSM ODR value.
    ///   - `Err`: Returns an error if the operation fails.
    ///
    /// # Description
    ///
    /// This function reads the `EMB_FUNC_ODR_CFG_B` register to retrieve the current FSM ODR value.
    pub fn fsm_data_rate_get(&mut self) -> Result<FsmValOdr, Error<B::Error>> {
        MemBank::operate_over_emb(self, |state| {
            let val = FsmOdr::read(state)?.fsm_odr();
            Ok(FsmValOdr::try_from(val).unwrap_or_default())
        })
    }

    /// Configures the FSM initialization request.
    ///
    /// # Arguments
    ///
    /// - `val: u8`: The value to configure the `fsm_init` field in the `FSM_INIT` register.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration of the FSM initialization request.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function configures the FSM initialization request by modifying the `fsm_init` field in the `FSM_INIT` register.
    pub fn fsm_init_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_emb(self, |state| {
            let mut emb_func_init_b = EmbFuncInitB::read(state)?;
            emb_func_init_b.set_fsm_init(val);
            emb_func_init_b.write(state)
        })
    }

    /// Retrieves the current FSM initialization request configuration.
    ///
    /// # Returns
    ///
    /// - `Result<u8, Error<B::Error>>`:
    ///   - `u8`: The value of the `fsm_init` field in the `FSM_INIT` register.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `FSM_INIT` register to retrieve the current FSM initialization request configuration.
    pub fn fsm_init_get(&mut self) -> Result<u8, Error<B::Error>> {
        MemBank::operate_over_emb(self, |state| Ok(EmbFuncInitB::read(state)?.fsm_init()))
    }

    /// Configures the FSM FIFO enable bit.
    ///
    /// # Arguments
    ///
    /// - `val: u8`: The value to configure the `fsm_fifo_en` field in the `EMB_FUNC_FIFO_EN` register.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration of the FSM FIFO enable bit.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function configures the FSM FIFO enable bit by modifying the `fsm_fifo_en` field in the `EMB_FUNC_FIFO_EN` register.
    pub fn fsm_fifo_en_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_emb(self, |state| {
            let mut fifo_reg = EmbFuncFifoEn::read(state)?;
            fifo_reg.set_fsm_fifo_en(val);
            fifo_reg.write(state)
        })
    }

    /// Retrieves the current FSM FIFO enable bit configuration.
    ///
    /// # Returns
    ///
    /// - `Result<u8, Error<B::Error>>`:
    ///   - `u8`: The value of the `fsm_fifo_en` field in the `EMB_FUNC_FIFO_EN` register.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `EMB_FUNC_FIFO_EN` register to retrieve the current FSM FIFO enable bit configuration.
    pub fn fsm_fifo_en_get(&mut self) -> Result<u8, Error<B::Error>> {
        MemBank::operate_over_emb(self, |state| Ok(EmbFuncFifoEn::read(state)?.fsm_fifo_en()))
    }

    /// Configures the FSM long counter timeout value.
    ///
    /// # Arguments
    ///
    /// - `val: u16`: The 16-bit unsigned integer timeout value.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration of the FSM long counter timeout value.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function configures the FSM long counter timeout value by writing to the `FSM_LC_TIMEOUT_L` register.
    pub fn long_cnt_int_value_set(&mut self, val: u16) -> Result<(), Error<B::Error>> {
        FsmLcTimeout::from_bits(val).write(self)
    }

    /// Retrieves the current FSM long counter timeout value.
    ///
    /// # Returns
    ///
    /// - `Result<u16, Error<B::Error>>`:
    ///   - `u16`: The current FSM long counter timeout value.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `FSM_LC_TIMEOUT_L` register to retrieve the current FSM long counter timeout value.
    pub fn long_cnt_int_value_get(&mut self) -> Result<u16, Error<B::Error>> {
        Ok(FsmLcTimeout::read(self)?.fsm_lc_timeout())
    }

    /// Configures the FSM number of programs.
    ///
    /// # Arguments
    ///
    /// - `val: u8`: The value to configure the FSM number of programs.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration of the FSM number of programs.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function configures the FSM number of programs by writing to the `FSM_PROGRAMS` register.
    pub fn fsm_programs_num_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        FsmPrograms::from_bits(val).write(self)
    }

    /// Retrieves the current FSM number of programs.
    ///
    /// # Returns
    ///
    /// - `Result<u8, Error<B::Error>>`:
    ///   - `u8`: The current FSM number of programs.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `FSM_PROGRAMS` register to retrieve the current FSM number of programs.
    pub fn fsm_programs_num_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ok(FsmPrograms::read(self)?.fsm_n_prog())
    }

    /// Configures the FSM start address.
    ///
    /// # Arguments
    ///
    /// - `val: u16`: The 16-bit unsigned integer start address.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration of the FSM start address.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function configures the FSM start address by writing to the `FSM_START_ADD_L` register.
    pub fn fsm_start_address_set(&mut self, val: u16) -> Result<(), Error<B::Error>> {
        FsmStartAdd::from_bits(val).write(self)
    }

    /// Retrieves the current FSM start address.
    ///
    /// # Returns
    ///
    /// - `Result<u16, Error<B::Error>>`:
    ///   - `u16`: The current FSM start address.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `FSM_START_ADD_L` register to retrieve the current FSM start address.
    pub fn fsm_start_address_get(&mut self) -> Result<u16, Error<B::Error>> {
        Ok(FsmStartAdd::read(self)?.fsm_start())
    }

    /// Configures the Machine Learning Core (MLC) mode.
    ///
    /// # Arguments
    ///
    /// - `val: MlcMode`: The desired MLC mode configuration:
    ///   - `MlcMode::Off`: Disables the MLC.
    ///   - `MlcMode::On`: Enables the MLC.
    ///   - `MlcMode::OnBeforeFsm`: Enables the MLC before the FSM.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration of the MLC mode.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function configures the MLC mode by modifying the `mlc_en` field in the `EMB_FUNC_EN_B` register
    /// and the `mlc_before_fsm_en` field in the `EMB_FUNC_EN_A` register.
    pub fn mlc_set(&mut self, val: MlcMode) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_emb(self, |state| {
            let mut emb_en_a = EmbFuncEnA::read(state)?;
            let mut emb_en_b = EmbFuncEnB::read(state)?;

            match val {
                MlcMode::Off => {
                    emb_en_a.set_mlc_before_fsm_en(0);
                    emb_en_b.set_mlc_en(0);
                }
                MlcMode::On => {
                    emb_en_a.set_mlc_before_fsm_en(0);
                    emb_en_b.set_mlc_en(1);
                }
                MlcMode::OnBeforeFsm => {
                    emb_en_a.set_mlc_before_fsm_en(1);
                    emb_en_b.set_mlc_en(0);
                }
            }

            emb_en_a.write(state)?;
            emb_en_b.write(state)
        })
    }

    /// Retrieves the current Machine Learning Core (MLC) mode.
    ///
    /// # Returns
    ///
    /// - `Result<MlcMode, Error<B::Error>>`:
    ///   - `MlcMode`: The current MLC mode configuration:
    ///     - `MlcMode::Off`: MLC is disabled.
    ///     - `MlcMode::On`: MLC is enabled.
    ///     - `MlcMode::OnBeforeFsm`: MLC is enabled before the FSM.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `EMB_FUNC_EN_B` and `EMB_FUNC_EN_A` registers to retrieve the current MLC mode configuration.
    pub fn mlc_get(&mut self) -> Result<MlcMode, Error<B::Error>> {
        MemBank::operate_over_emb(self, |state| {
            let emb_en_a = EmbFuncEnA::read(state)?;
            let emb_en_b = EmbFuncEnB::read(state)?;

            let val = if emb_en_a.mlc_before_fsm_en() == 0 && emb_en_b.mlc_en() == 0 {
                MlcMode::Off
            } else if emb_en_a.mlc_before_fsm_en() == 0 && emb_en_b.mlc_en() == 1 {
                MlcMode::On
            } else {
                MlcMode::OnBeforeFsm
            };

            Ok(val)
        })
    }

    /// Retrieves the Machine Learning Core (MLC) status.
    ///
    /// # Returns
    ///
    /// - `Result<MlcStatusMainpage, Error<B::Error>>`:
    ///   - `MlcStatusMainpage`: The MLC status from the `MLC_STATUS_MAINPAGE` register.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `MLC_STATUS_MAINPAGE` register to retrieve the MLC status.
    pub fn mlc_status_get(&mut self) -> Result<MlcStatusMainpage, Error<B::Error>> {
        MlcStatusMainpage::read(self)
    }

    /// Retrieves the output values of all MLC decision trees.
    ///
    /// # Returns
    ///
    /// - `Result<[u8; 4], Error<B::Error>>`:
    ///   - `[u8; 4]`: The output values of all MLC decision trees.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `MLC1_SRC` register to retrieve the output values of all MLC decision trees.
    pub fn mlc_out_get(&mut self) -> Result<[u8; 4], Error<B::Error>> {
        MemBank::operate_over_emb(self, |state| {
            let mut buff: [u8; 4] = [0; 4];
            MlcSrc::read_more(state, &mut buff)?;
            Ok(buff)
        })
    }

    /// Configures the Machine Learning Core (MLC) data rate.
    ///
    /// # Arguments
    ///
    /// - `val: MlcOdrVal`: The desired MLC data rate:
    ///   - `MlcOdrVal::_12_5hz`: 12.5 Hz.
    ///   - `MlcOdrVal::_25hz`: 25 Hz.
    ///   - `MlcOdrVal::_50hz`: 50 Hz.
    ///   - `MlcOdrVal::_100hz`: 100 Hz.
    ///   - `MlcOdrVal::_200hz`: 200 Hz.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration of the MLC data rate.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function configures the MLC data rate by modifying the `mlc_odr` field in the `EMB_FUNC_ODR_CFG_C` register.
    pub fn mlc_data_rate_set(&mut self, val: MlcOdrVal) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_emb(self, |state| {
            let mut reg = MlcOdr::read(state)?;
            reg.set_mlc_odr(val as u8);
            reg.write(state)
        })
    }

    /// Retrieves the current Machine Learning Core (MLC) data rate.
    ///
    /// # Returns
    ///
    /// - `Result<MlcOdrVal, Error<B::Error>>`:
    ///   - `MlcOdrVal`: The current MLC data rate:
    ///     - `MlcOdrVal::_12_5hz`: 12.5 Hz.
    ///     - `MlcOdrVal::_25hz`: 25 Hz.
    ///     - `MlcOdrVal::_50hz`: 50 Hz.
    ///     - `MlcOdrVal::_100hz`: 100 Hz.
    ///     - `MlcOdrVal::_200hz`: 200 Hz.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `EMB_FUNC_ODR_CFG_C` register to retrieve the current MLC data rate.
    pub fn mlc_data_rate_get(&mut self) -> Result<MlcOdrVal, Error<B::Error>> {
        MemBank::operate_over_emb(self, |state| {
            let val = MlcOdr::read(state)?.mlc_odr();
            Ok(MlcOdrVal::try_from(val).unwrap_or_default())
        })
    }

    /// Configures the MLC FIFO enable bit.
    ///
    /// # Arguments
    ///
    /// - `val: u8`: The value to configure the `mlc_fifo_en` field in the `EMB_FUNC_FIFO_EN` register.
    ///
    /// # Returns
    ///
    /// - `Result<(), Error<B::Error>>`:
    ///   - `Ok`: Indicates successful configuration of the MLC FIFO enable bit.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function configures the MLC FIFO enable bit by modifying the `mlc_fifo_en` field in the `EMB_FUNC_FIFO_EN` register.
    pub fn mlc_fifo_en_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_emb(self, |state| {
            let mut fifo_reg = EmbFuncFifoEn::read(state)?;
            fifo_reg.set_mlc_fifo_en(val);
            fifo_reg.write(state)
        })
    }

    /// Retrieves the current MLC FIFO enable bit configuration.
    ///
    /// # Returns
    ///
    /// - `Result<u8, Error<B::Error>>`:
    ///   - `u8`: The value of the `mlc_fifo_en` field in the `EMB_FUNC_FIFO_EN` register.
    ///   - `Err`: Returns an error if the operation fails. Possible error variants include:
    ///     - `Error::Bus`: Indicates an error at the bus level.
    ///
    /// # Description
    ///
    /// This function reads the `EMB_FUNC_FIFO_EN` register to retrieve the current MLC FIFO enable bit configuration.
    pub fn mlc_fifo_en_get(&mut self) -> Result<u8, Error<B::Error>> {
        MemBank::operate_over_emb(self, |state| Ok(EmbFuncFifoEn::read(state)?.mlc_fifo_en()))
    }
}

/// Converts raw accelerometer data from LSB to milli-g for a ±2g full-scale range.
///
/// # Arguments
///
/// - `lsb: i16`: The raw accelerometer data in LSB.
///
/// # Returns
///
/// - `f32`: The converted acceleration value in milli-g.
///
/// # Description
///
/// This function converts raw accelerometer data from LSB to milli-g for a ±2g full-scale range.
/// The conversion factor used is 0.061.
pub fn from_fs2g_to_mg(lsb: i16) -> f32 {
    lsb as f32 * 0.061
}

/// Converts raw accelerometer data from LSB to milli-g for a ±4g full-scale range.
///
/// # Arguments
///
/// - `lsb: i16`: The raw accelerometer data in LSB.
///
/// # Returns
///
/// - `f32`: The converted acceleration value in milli-g.
///
/// # Description
///
/// This function converts raw accelerometer data from LSB to milli-g for a ±4g full-scale range.
/// The conversion factor used is 0.122.
pub fn from_fs4g_to_mg(lsb: i16) -> f32 {
    lsb as f32 * 0.122
}

/// Converts raw accelerometer data from LSB to milli-g for a ±8g full-scale range.
///
/// # Arguments
///
/// - `lsb: i16`: The raw accelerometer data in LSB.
///
/// # Returns
///
/// - `f32`: The converted acceleration value in milli-g.
///
/// # Description
///
/// This function converts raw accelerometer data from LSB to milli-g for a ±8g full-scale range.
/// The conversion factor used is 0.244.
pub fn from_fs8g_to_mg(lsb: i16) -> f32 {
    lsb as f32 * 0.244
}

/// Converts raw accelerometer data from LSB to milli-g for a ±16g full-scale range.
///
/// # Arguments
///
/// - `lsb: i16`: The raw accelerometer data in LSB.
///
/// # Returns
///
/// - `f32`: The converted acceleration value in milli-g.
///
/// # Description
///
/// This function converts raw accelerometer data from LSB to milli-g for a ±16g full-scale range.
/// The conversion factor used is 0.488.
pub fn from_fs16g_to_mg(lsb: i16) -> f32 {
    lsb as f32 * 0.488
}

/// Converts raw temperature data from LSB to degrees Celsius.
///
/// # Arguments
///
/// - `lsb: i16`: The raw temperature data in LSB.
///
/// # Returns
///
/// - `f32`: The converted temperature value in degrees Celsius.
///
/// # Description
///
/// This function converts raw temperature data from LSB to degrees Celsius. The conversion formula
/// used is `(lsb / 355.5) + 25.0`.
pub fn from_lsb_to_celsius(lsb: i16) -> f32 {
    (lsb as f32 / 355.5) + 25.0
}

/// Converts raw AH_QVAR data from LSB to millivolts.
///
/// # Arguments
///
/// - `lsb: i16`: The raw AH_QVAR data in LSB.
///
/// # Returns
///
/// - `f32`: The converted voltage value in millivolts.
///
/// # Description
///
/// This function converts raw AH_QVAR data from LSB to millivolts. The conversion factor used is 74.4.
pub fn from_lsb_to_mv(lsb: i16) -> f32 {
    (lsb as f32) / 74.4
}

/// Represents the I2C addresses for the sensor.
///
/// This enum is used to specify the possible I2C addresses that the sensor can use for communication.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum I2CAddress {
    /// Low I2C address.
    ///
    /// This address is used when the SA0 pin is set to 0.
    I2cAddL = 0x18,

    /// High I2C address.
    ///
    /// This address is used when the SA0 pin is set to 1.
    I2cAddH = 0x19,
}

///Device Who am I
pub const ID: u8 = 0x47;

const BOOT_TIME_DELAY_MS: u8 = 25;
const SW_RESET_DELAY_US: u8 = 50;

const BOOT_SWRESET_MAX_ATTEMPTS: u8 = 5;

pub const PROPERTY_ENABLE: u8 = 1;
pub const PROPERTY_DISABLE: u8 = 0;
