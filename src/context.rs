use embassy_stm32::gpio::Output;
use embassy_stm32::i2c::I2c;
use embassy_stm32::mode::Async;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;
use ssd1306::mode::BufferedGraphicsModeAsync;
use ssd1306::prelude::{DisplaySize128x64, I2CInterface};
use ssd1306::Ssd1306Async;

pub struct Context {
    pub led: Mutex<NoopRawMutex, Output<'static>>,
    pub power: Mutex<NoopRawMutex, Output<'static>>,
    pub display: Mutex<
        NoopRawMutex,
        Ssd1306Async<
            I2CInterface<I2c<'static, Async>>,
            DisplaySize128x64,
            BufferedGraphicsModeAsync<DisplaySize128x64>,
        >,
    >,
}
