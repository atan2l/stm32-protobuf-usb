use defmt::info;
use crate::comms::{Packet, USB_RX, USB_TX};
use embassy_stm32::peripherals::USB_OTG_FS;
use embassy_stm32::usb::Driver;
use embassy_usb::class::cdc_acm::CdcAcmClass;
use embassy_usb::UsbDevice;

#[embassy_executor::task]
pub async fn usb_task(mut cdc: CdcAcmClass<'static, Driver<'static, USB_OTG_FS>>) {
    let tx = USB_TX.receiver();
    let rx = USB_RX.sender();
    let mut buf = [0u8; 64];

    loop {
        cdc.wait_connection().await;
        if let Ok(n) = cdc.read_packet(&mut buf).await {
            let mut pkt = Packet::new();
            pkt.extend_from_slice(&buf[..n]).ok();
            let _ = rx.send(pkt).await;
        }
        
        if let Ok(reply) = tx.try_receive() {
            let _ = cdc.write_packet(&reply).await;
        }
    }
}

#[embassy_executor::task]
pub async fn usb_run(mut usb: UsbDevice<'static, Driver<'static, USB_OTG_FS>>) {
    info!("Running USB controller");
    usb.run().await;
}
