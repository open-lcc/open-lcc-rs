#![no_std]

use embassy_rp::gpio::AnyPin;
use embassy_rp::{i2c, spi, uart};
use embassy_rp::peripherals::{FLASH, USB, WATCHDOG};
use embassy_rp::pwm::{Channel, Pwm};
use embassy_rp::uart::Blocking;

pub struct
OpenLCCBoardFeatures<'a, LedPwmChT, EspUartT, IoxUartT, Qwiic1I2cT, Qwiic2I2cT, SettingsFlashSpiT, SdCardSpiT>
    where
        LedPwmChT: Channel,
        EspUartT: uart::Instance,
        IoxUartT: uart::Instance,
        Qwiic1I2cT: i2c::Instance,
        Qwiic2I2cT: i2c::Instance,
        SettingsFlashSpiT: spi::Instance,
        SdCardSpiT: spi::Instance,
{
    pub esp32_uart_rx: uart::UartRx<'a, EspUartT, uart::Async>,
    pub esp32_uart_tx: uart::UartTx<'a, EspUartT, uart::Async>,
    pub iox_uart_rx: Option<uart::UartRx<'a, IoxUartT, uart::Async>>,
    pub iox_uart_tx: Option<uart::UartTx<'a, IoxUartT, uart::Async>>,
    pub qwiic1_i2c: i2c::I2c<'a, Qwiic1I2cT, i2c::Async>,
    pub qwiic2_i2c: i2c::I2c<'a, Qwiic2I2cT, i2c::Async>,
    pub settings_flash_spi: Option<spi::Spi<'a, SettingsFlashSpiT, spi::Async>>,
    pub sd_card_spi: Option<spi::Spi<'a, SdCardSpiT, spi::Async>>,

    pub serial_boot_pin: AnyPin,

    pub settings_flash_cs_pin: AnyPin,
    pub settings_flash_sclk_pin: Option<AnyPin>,
    pub settings_flash_miso_pin: Option<AnyPin>,
    pub settings_flash_mosi_pin: Option<AnyPin>,
    pub settings_flash_wp_d2_pin: Option<AnyPin>,
    pub settings_flash_res_d3_pin: Option<AnyPin>,

    pub sd_det_a_pin: AnyPin,
    pub sd_det_b_pin: Option<AnyPin>,
    pub sd_cs_dat3_pin: Option<AnyPin>,
    pub sd_sclk_pin: Option<AnyPin>,
    pub sd_miso_pin: Option<AnyPin>,
    pub sd_mosi_pin: Option<AnyPin>,
    pub sd_dat1_pin: Option<AnyPin>,
    pub sd_dat2_pin: Option<AnyPin>,

    pub led_pin: Option<AnyPin>,
    pub led_pwm: Option<Pwm<'a, LedPwmChT>>,

    pub usb: USB,
}

pub struct BootloaderFeatures<'a, UartT> where UartT: uart::Instance{
    pub bootloader_uart: uart::Uart<'a, UartT, Blocking>,
    pub bootloader_trigger_pin: AnyPin,
    pub watchdog: WATCHDOG,
    pub flash: FLASH,
    pub flash_size: usize
}