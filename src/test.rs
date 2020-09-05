use crate::qemu::{exit_qemu, QemuExitCode};
use crate::util::digit_width;

pub fn test_runner(tests: &[&dyn Testable]) {
    let amount = tests.len();
    serial_println!("Running {} tests", amount);
    if amount > 0 {
        let max_name_len = tests
            .iter()
            .max_by_key(|x| x.name_len())
            .unwrap()
            .name_len();

        let mut num = 0;
        for test in tests {
            test.run(num, amount, max_name_len);
            num += 1;
        }
    }
    exit_qemu(QemuExitCode::Success)
}

pub trait Testable {
    fn name(&self) -> &'static str;
    fn name_len(&self) -> usize;
    fn run(&self, test_num: usize, test_amount: usize, max_name_len: usize);
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

    fn run(&self, test_num: usize, test_amount: usize, max_name_len: usize) {
        let name = self.name();
        serial_print!(
            "[{}] {}..{:>width$}",
            format_args!(
                "{:width$}/{:width$}",
                test_num + 1,
                test_amount,
                width = digit_width(test_amount)
            ),
            name,
            "",
            width = max_name_len - name.len() + 1
        );
        self();
        serial_println!("[ok]")
    }
}
