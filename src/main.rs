#![no_std] //disables std
#![no_main] //makes it so that main is no longer needed

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::{fmt::Write, panic::PanicInfo};

mod vga_buffer;
mod serial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode{
    Success = 0x10,
    Fail = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode){
    use x86_64::instructions::port::Port;

    match exit_code{
        QemuExitCode::Success => serial_println!("test results: success\n"),
        QemuExitCode::Fail => serial_print!("test results: failed\n\n\n _____________ \n\n")
    }

    unsafe{
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t\t\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}


#[cfg(not(test))]
#[panic_handler]  //since there isnt any stds the panic isnt handled
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Fail);

    loop {}
}

#[unsafe(no_mangle)] //dont mangle(hash the name) of this func 
pub extern "C" fn _start() -> ! {  //uses extern "C" for C`s naming convention`and the -> ! is to say that this fn wont end
    //Vga requires 2 bytes per letter one for the letter and one for the color     
    //which is a 4bit bg + a 4bit fg

    vga_buffer::WRITER.lock().paint_screen();

    println!("hello world\n");
    println!("!");

    #[cfg(test)]
    test_main();

    loop {}
}

#[test_case]
fn trivial_assertion() {
    //serial_println!("trivial assertion... ");
    assert_eq!(1, 1);
    //serial_println!("[ok]");
}


#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    serial_println!("Tests are done");
    exit_qemu(QemuExitCode::Success)
}
