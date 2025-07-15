use crate::comms::{FramedBuffer, USB_RX, USB_TX};
use embassy_futures::select::{select, Either};
use embassy_stm32::peripherals::USB_OTG_FS;
use embassy_stm32::usb::Driver;
use embassy_usb::class::cdc_acm::CdcAcmClass;
use embassy_usb::UsbDevice;

#[embassy_executor::task]
pub async fn usb_task(mut cdc: CdcAcmClass<'static, Driver<'static, USB_OTG_FS>>) {
    let tx = USB_TX.receiver();
    let rx = USB_RX.sender();
    let mut chunk = [0u8; 64];
    let mut framed_buffer = FramedBuffer::new();

    loop {
        cdc.wait_connection().await;

        loop {
            match select(cdc.read_packet(&mut chunk), tx.receive()).await {
                Either::First(read_result) => {
                    if let Ok(n) = read_result {
                        if let Some(full_frame) = framed_buffer.push(&chunk[..n]) {
                            let _ = rx.send(full_frame).await;
                        }
                    } else {
                        break;
                    }
                }
                Either::Second(reply) => {
                    // Send the reply 64-byte chunks at a time.
                    let mut last_chunk = Default::default();
                    for chunk in reply.chunks(cdc.max_packet_size() as usize) {
                        let _ = cdc.write_packet(chunk).await;
                        last_chunk = chunk;
                    }

                    if last_chunk.len() == cdc.max_packet_size() as usize {
                        /*
                         * The last packet is full. We need to send an empty packet to mark
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
