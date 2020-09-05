#[cfg(test)]
use crate::qemu::{exit_qemu, QemuExitCode};

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success)
}
