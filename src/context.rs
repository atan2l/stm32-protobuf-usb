use embassy_stm32::gpio::Output;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;

pub struct Context {
    pub led: Mutex<NoopRawMutex, Output<'static>>,
    pub power: Mutex<NoopRawMutex, Output<'static>>,
}
