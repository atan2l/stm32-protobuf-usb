use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use heapless::Vec;

pub type ProtoMessage = Vec<u8, 512>;

pub static USB_RX: Channel<CriticalSectionRawMutex, ProtoMessage, 4> = Channel::new();
pub static USB_TX: Channel<CriticalSectionRawMutex, ProtoMessage, 4> = Channel::new();

pub struct FramedBuffer {
    buf: ProtoMessage,
    state: FrameState,
    expected_length: usize,
}

pub enum FrameState {
    WaitingForHeader,
    WaitingForLengthLo,
    WaitingForLengthHi(u8),
    Accumulating(usize),
}

impl FramedBuffer {
    pub fn new() -> Self {
        Self {
            buf: ProtoMessage::new(),
            state: FrameState::WaitingForHeader,
            expected_length: 0,
        }
    }

    pub fn push(&mut self, data: &[u8]) -> Option<ProtoMessage> {
        for &byte in data {
            match self.state {
                FrameState::WaitingForHeader => {
                    if byte == 0xAA {
                        self.buf.clear();
                        self.state = FrameState::WaitingForLengthLo;
                    }
                }
                FrameState::WaitingForLengthLo => {
                    self.state = FrameState::WaitingForLengthHi(byte);
                }
                FrameState::WaitingForLengthHi(length_lo) => {
                    let len = (byte as usize) << 8 | length_lo as usize;
                    self.expected_length = len;
                    self.state = FrameState::Accumulating(len);
                }
                FrameState::Accumulating(expected_length) => {
                    self.buf.push(byte).ok();
                    if self.buf.len() == expected_length {
                        self.state = FrameState::WaitingForHeader;
                        return Some(self.buf.clone());
                    }
                }
            }
        }

        None
    }
}
