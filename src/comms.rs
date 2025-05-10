use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use heapless::Vec;

pub type Packet = Vec<u8, 64>;

pub static USB_RX: Channel<CriticalSectionRawMutex, Packet, 4> = Channel::new();
pub static USB_TX: Channel<CriticalSectionRawMutex, Packet, 4> = Channel::new();
