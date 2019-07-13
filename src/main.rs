#![no_std]
#![no_main]
#![feature(asm)]

use core::panic::PanicInfo;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {} // just spin after a panic
}

mod io;
mod vga;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Booted. Passing control to kmain().\n");
    kmain();
    println!("\nkmain() returned control. Halting.");
    loop {} //so as not to run garbage after this in mem.
}

#[no_mangle]
fn kmain() {
    vga::clear();
    pi()
}
fn pi() {
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
        if digits >= 10 {
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
