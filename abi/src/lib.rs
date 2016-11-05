#![feature(lang_items)]
#![feature(asm)]
#![no_std]

pub trait SetDefault {
    fn set_default(&mut self);
}

#[derive(Debug)]
pub struct CapSystemCall<'a> {
    pub target: &'a [u8],
    pub message: CapSendMessage
}

#[derive(Debug, Clone, Copy)]
pub enum CapSendMessage {
    TCBYield
}

#[derive(Debug, Clone)]
pub enum SystemCall {
    CPoolListDebug,
    Print {
        request: ([u8; 32], usize)
    },
    Inportb {
        request: u16,
        response: Option<u8>
    },
    Outportb {
        request: (u16, u8),
    },
    RetypeCPool {
        request: (usize, usize),
    },
}

#[derive(Debug)]
pub struct TaskBuffer {
    pub call: Option<SystemCall>
}

impl SetDefault for TaskBuffer {
    fn set_default(&mut self) {
        self.call = None;
    }
}
