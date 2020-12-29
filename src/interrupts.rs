use lazy_static::lazy_static;
use pic8259_simple::ChainedPics;
use spinning::Mutex;

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

use crate::{gdt, keyboard};

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

lazy_static! {
    static ref PICS: Mutex<ChainedPics> =
        Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.divide_error.set_handler_fn(divide_by_zero_handler);
        idt.debug.set_handler_fn(debug_handler);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.non_maskable_interrupt
            .set_handler_fn(non_maskable_interrupt_handler);
        idt.overflow.set_handler_fn(overflow_handler);
        idt.bound_range_exceeded
            .set_handler_fn(bound_range_exceeded_handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
        idt.device_not_available
            .set_handler_fn(device_not_available_handler);

        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX)
        };

        idt.invalid_tss.set_handler_fn(invalid_tss_handler);
        idt.segment_not_present
            .set_handler_fn(segment_not_present_handler);
        idt.general_protection_fault
            .set_handler_fn(general_protection_fault_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.x87_floating_point
            .set_handler_fn(x87_floating_point_handler);
        idt.alignment_check.set_handler_fn(alignment_check_handler);
        idt.machine_check.set_handler_fn(machine_check_handler);
        idt.simd_floating_point
            .set_handler_fn(simd_floating_point_handler);
        idt.security_exception
            .set_handler_fn(security_exception_handler);

        // Hardware interrupt codes
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_handler);

        // Register handlers for tests
        #[cfg(test)]
        idt.divide_error.set_handler_fn(divide_by_zero_handler_test);

        idt
    };
}

pub fn init() {
    IDT.load();
    enable_hardware_interrupts()
}

fn enable_hardware_interrupts() {
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable()
}

extern "x86-interrupt" fn divide_by_zero_handler(stack_frame: &mut InterruptStackFrame) {
    print_exception_stack_frame("divide_by_zero_handler", stack_frame, None)
}

extern "x86-interrupt" fn debug_handler(stack_frame: &mut InterruptStackFrame) {
    print_exception_stack_frame("debug_handler", stack_frame, None)
}

extern "x86-interrupt" fn non_maskable_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    print_exception_stack_frame("non_maskable_interrupt_handler", stack_frame, None)
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    print_exception_stack_frame("breakpoint_handler", stack_frame, None)
}

extern "x86-interrupt" fn overflow_handler(stack_frame: &mut InterruptStackFrame) {
    print_exception_stack_frame("overflow_handler", stack_frame, None)
}

extern "x86-interrupt" fn bound_range_exceeded_handler(stack_frame: &mut InterruptStackFrame) {
    print_exception_stack_frame("bound_range_exceeded_handler", stack_frame, None)
}

extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: &mut InterruptStackFrame) {
    print_exception_stack_frame("invalid_opcode_handler", stack_frame, None)
}

extern "x86-interrupt" fn device_not_available_handler(stack_frame: &mut InterruptStackFrame) {
    print_exception_stack_frame("device_not_available_handler", stack_frame, None)
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: u64,
) -> ! {
    print_exception_stack_frame("double_fault_handler", stack_frame, Some(error_code));
    loop {}
}

extern "x86-interrupt" fn invalid_tss_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: u64,
) {
    print_exception_stack_frame("invalid_tss_handler", stack_frame, Some(error_code))
}

extern "x86-interrupt" fn segment_not_present_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: u64,
) {
    print_exception_stack_frame("segment_not_present_handler", stack_frame, Some(error_code))
}

extern "x86-interrupt" fn general_protection_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: u64,
) {
    print_exception_stack_frame(
        "general_protection_fault_handler",
        stack_frame,
        Some(error_code),
    )
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    _error_code: PageFaultErrorCode,
) {
    print_exception_stack_frame("page_fault_handler", stack_frame, None)
}

extern "x86-interrupt" fn x87_floating_point_handler(stack_frame: &mut InterruptStackFrame) {
    print_exception_stack_frame("x87_floating_point_handler", stack_frame, None)
}

extern "x86-interrupt" fn alignment_check_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: u64,
) {
    print_exception_stack_frame("alignment_check_handler", stack_frame, Some(error_code))
}

extern "x86-interrupt" fn machine_check_handler(stack_frame: &mut InterruptStackFrame) -> ! {
    print_exception_stack_frame("machine_check_handler", stack_frame, None);
    loop {}
}

extern "x86-interrupt" fn simd_floating_point_handler(stack_frame: &mut InterruptStackFrame) {
    print_exception_stack_frame("simd_floating_point_handler", stack_frame, None)
}

extern "x86-interrupt" fn security_exception_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: u64,
) {
    print_exception_stack_frame("security_exception_handler", stack_frame, Some(error_code))
}

extern "x86-interrupt" fn timer_handler(_stack_frame: &mut InterruptStackFrame) {
    ack_interrupt(InterruptIndex::Timer)
}

extern "x86-interrupt" fn keyboard_handler(_stack_frame: &mut InterruptStackFrame) {
    keyboard::process_input();
    ack_interrupt(InterruptIndex::Keyboard)
}

// Notify PIC that the interrupt was handled, which allows for new interrupts
// to be received.
fn ack_interrupt(index: InterruptIndex) {
    unsafe { PICS.lock().notify_end_of_interrupt(index.as_u8()) }
}

fn print_exception_stack_frame(
    fn_name: &str,
    stack_frame: &mut InterruptStackFrame,
    error_code: Option<u64>,
) {
    println!("Exception {}:\n{:#?}", fn_name, stack_frame);
    if let Some(code) = error_code {
        println!("Error code: {:?}", code)
    }
    // Spin so we have a chance to read the output stack frame. Otherwise, for
    // exceptions that set the instruction_pointer to the instruction that
    // caused the exception iretq will return to the faulty instruction and
    // endless loop
    #[cfg(not(test))]
    loop {}
}

// Progresses the instruction pointer by N bytes. This is useful in situations
// where an exception occurs and the instruction_pointer is set to faulty a
// fault instruction. By progressing the instruction pointer we can resume
// execution _after_ fixing the error.
#[allow(dead_code)]
fn incr_instruction_pointer(stack_frame: &mut InterruptStackFrame, num_bytes: u64) {
    use x86_64::addr::VirtAddr;

    unsafe {
        let new_ip = stack_frame.instruction_pointer.as_u64() + num_bytes;
        stack_frame.as_mut().instruction_pointer = VirtAddr::new(new_ip)
    }
}

#[test_case]
fn test_debug() {
    unsafe { asm!("int 1") }
}

#[test_case]
fn test_breakpoint_handler() {
    x86_64::instructions::interrupts::int3()
}

#[cfg(test)]
extern "x86-interrupt" fn divide_by_zero_handler_test(mut stack_frame: &mut InterruptStackFrame) {
    // 0x3 is the number of bytes for the instruction that triggered the
    // exception: `divw %dx` (66 f7 f2) is a 3-byte instruction.
    incr_instruction_pointer(&mut stack_frame, 0x3)
}

#[test_case]
fn test_divide_by_zero() {
    // The Rust runtime guards against divide by zero errors by triggering a
    // panic. So writing the assembly to divide by zero to bypass the runtime.
    unsafe {
        asm!("mov dx, 0");
        asm!("div dx")
    }
    // The exception handler will progress the instruction_pointer to this instruction so the test
    // will pass. The value is put into a variable because `assert!(true)` could be optimized out by
    // the compiler.
    let res = true;
    assert!(res)
}
