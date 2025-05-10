#![no_std]
#![no_main]

mod command_handler;
mod comms;
mod context;
mod dispatcher;
mod usb_task;

use crate::context::Context;
use crate::dispatcher::command_dispatcher;
use crate::usb_task::usb_task;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::time::Hertz;
use embassy_stm32::usb::Driver;
use embassy_stm32::{bind_interrupts, peripherals, usb, Config};
use embassy_sync::mutex::Mutex;
use embassy_usb::class::cdc_acm::{CdcAcmClass, State};
use embassy_usb::Builder;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

static EP_OUT_BUFFER: StaticCell<[u8; 256]> = StaticCell::new();
static CONFIG_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
static BOS_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
static CONTROL_BUF: StaticCell<[u8; 64]> = StaticCell::new();
static STATE: StaticCell<State> = StaticCell::new();
static CTX: StaticCell<Context> = StaticCell::new();

bind_interrupts!(struct Irqs {
    OTG_FS => usb::InterruptHandler<peripherals::USB_OTG_FS>;
});

mod command {
    include!(concat!(env!("OUT_DIR"), "/command.rs"));
}

mod response {
    include!(concat!(env!("OUT_DIR"), "/response.rs"));
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.hse = Some(Hse {
            freq: Hertz(8_000_000),
            mode: HseMode::Bypass,
        });
        config.rcc.pll_src = PllSource::HSE;
        config.rcc.pll = Some(Pll {
            prediv: PllPreDiv::DIV4,
            mul: PllMul::MUL168,
            divp: Some(PllPDiv::DIV2), // 8mhz / 4 * 168 / 2 = 168Mhz.
            divq: Some(PllQDiv::DIV7), // 8mhz / 4 * 168 / 7 = 48Mhz.
            divr: None,
        });
        config.rcc.ahb_pre = AHBPrescaler::DIV1;
        config.rcc.apb1_pre = APBPrescaler::DIV4;
        config.rcc.apb2_pre = APBPrescaler::DIV2;
        config.rcc.sys = Sysclk::PLL1_P;
        config.rcc.mux.clk48sel = mux::Clk48sel::PLL1_Q;
    }

    let peripherals = embassy_stm32::init(config);

    let ctx = CTX.init(Context {
        led: Mutex::new(Output::new(peripherals.PA5, Level::Low, Speed::Low)),
        power: Mutex::new(Output::new(peripherals.PA6, Level::Low, Speed::Low)),
    });

    // Create the driver, from the HAL
    let ep_out_buffer = EP_OUT_BUFFER.init([0; 256]);
    let mut config = usb::Config::default();

    /*
     * Do not enable vbus_detection. This is a safe default that works on all boards.
     * However, if your USB device is self-powered (can stay powered on if USB is unplugged),
     * you need to enable vbus_detection to comply with the USB spec. If you enable it, the board
     * has to support it or USB won't work at all. See docs on `vbus_detection` for details.
     */
    config.vbus_detection = false;

    let driver = Driver::new_fs(
        peripherals.USB_OTG_FS,
        Irqs,
        peripherals.PA12,
        peripherals.PA11,
        ep_out_buffer,
        config,
    );

    let mut usb_device_config = embassy_usb::Config::new(0xc0de, 0xcafe);
    usb_device_config.manufacturer = Some("Embassy");
    usb_device_config.product = Some("USB-Serial-Prototype");
    usb_device_config.serial_number = Some("1234567890");

    /*
     * Create the embassy-usb DeviceBuilder using the driver and config.
     * It needs some buffers for building the descriptors.
     */
    let config_descriptor = CONFIG_DESCRIPTOR.init([0; 256]);
    let bos_descriptor = BOS_DESCRIPTOR.init([0; 256]);
    let control_buf = CONTROL_BUF.init([0; 64]);

    let state = STATE.init(State::new());
    let mut builder = Builder::new(
        driver,
        usb_device_config,
        config_descriptor,
        bos_descriptor,
        &mut [],
        control_buf,
    );

    let class = CdcAcmClass::new(&mut builder, state, 64);
    let mut usb = builder.build();

    spawner.spawn(usb_task(class)).unwrap();
    spawner.spawn(command_dispatcher(ctx)).unwrap();

    usb.run().await;
}
