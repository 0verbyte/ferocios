use crate::qemu::{exit_qemu, QemuExitCode};

pub fn test_runner(tests: &[&dyn Testable]) {
    let amount = tests.len();
    serial_println!("Running {} tests", amount);
    if amount > 0 {
        let max_name_len = tests
            .iter()
            .max_by_key(|x| x.name_len())
            .unwrap()
            .name_len();
        for test in tests {
            test.run(max_name_len);
        }
    }
    exit_qemu(QemuExitCode::Success)
}

pub trait Testable {
    fn name(&self) -> &'static str;
    fn name_len(&self) -> usize;
    fn run(&self, max_name_len: usize);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn name(&self) -> &'static str {
        core::any::type_name::<T>()
    }

    fn name_len(&self) -> usize {
        self.name().len()
    }

    fn run(&self, max_name_len: usize) {
        let name = self.name();
        serial_print!(
            "{}..{:>width$}",
            name,
            "",
            width = max_name_len - name.len() + 1
        );
        self();
        serial_println!("[ok]")
    }
}
