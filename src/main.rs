#![no_std]
#![no_main]
#![feature(asm)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    exit_qemu(QemuExitCode::Failed);
    loop {} // just spin after a panic
}

mod io;
mod vga;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Booted.");
    #[cfg(test)]
    {
        test_main();
        println!("Finished Tests.");
        exit_qemu(QemuExitCode::Failed);
        loop{}
    }
    println!("Passing control to kmain().");
    kmain();
    println!("\nkmain() returned control. Halting.");
    exit_qemu(QemuExitCode::Success);
    loop {} //so as not to run garbage after this in mem.
}

#[no_mangle]
fn kmain() {
    vga::clear();
    pi(4);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

fn pi(num_digits : usize) {
    print!("pi = 3.");
    let mut digits: usize = 0;
    let groups = 200;
    loop {
        digits += 1;
        let err = ex(-(digits as i32) - 1);
        let mut i = 1;
        let mut sum: f64 = 0.0;
        loop {
            if abs(term(i)) <= err {
                print!("{}", nthd(digits, sum));
                break;
            }
            for _ in 0..groups {
                sum += term(i);
                i += 1;
            }
        }
        if digits >= num_digits {
            break;
        }
    }
}
fn term(i: usize) -> f64 {
    if is_even(i) {
        -4.0 / (2.0 * (i as f64) - 1.0)
    } else {
        4.0 / (2.0 * (i as f64) - 1.0)
    }
}

fn abs(n: f64) -> f64 {
    if n < 0.0 {
        -n
    } else {
        n
    }
}

fn is_even(i: usize) -> bool {
    (i % 2) == 0
}

fn ex(i: i32) -> f64 {
    if i < 0 {
        1.0 / ex(-i)
    } else {
        let mut x: f64 = 1.0;
        for _ in 0..i {
            x *= 10.0;
        }
        x
    }
}

fn nthd(n: usize, num: f64) -> u8 {
    (((abs(num) * ex(n as i32)) as u64) % 10) as u8
}


#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

    #[test_case]
    fn simple() {
        assert_eq!(2+2,4);
    }

