#![no_std]

pub mod framebuf;
pub mod hhdm;
pub mod memmap;

pub struct BootInfo {
    pub memmap: memmap::Memmap,
    pub frame_buffer: framebuf::BootFrameBuf,
    pub hhdm: hhdm::BootHhdm,
}
