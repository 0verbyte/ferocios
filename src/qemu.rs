#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

#[allow(dead_code)]
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    const QEMU_IOBASE: u16 = 0xf4;

    unsafe {
        let mut port = Port::new(QEMU_IOBASE);
        port.write(exit_code as u32)
    }
}
