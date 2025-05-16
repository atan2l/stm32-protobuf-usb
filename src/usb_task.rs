use crate::comms::{Packet, USB_RX, USB_TX};
use embassy_futures::select::{select, Either};
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

        loop {
            match select(cdc.read_packet(&mut buf), tx.receive()).await {
                Either::First(read_result) => {
                    if let Ok(n) = read_result {
                        if let Ok(pkt) = Packet::from_slice(&buf[..n]) {
                            let _ = rx.send(pkt).await;
                        }
                    } else {
                        break;
                    }
                }
                Either::Second(reply) => {
                    let _ = cdc.write_packet(&reply).await;
                    if reply.len() == 64 {
                        /*
                         * The previous packet is full. We need to send an empty packet to mark
                         * the end of data transmission.
                         */
                        let _ = cdc.write_packet(&[]).await;
                    }
                }
            }
        }
    }
}

#[embassy_executor::task]
pub async fn usb_run(mut usb: UsbDevice<'static, Driver<'static, USB_OTG_FS>>) {
    usb.run().await;
}
