
pub unsafe fn outb(data: u8, port: u16) {
    asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(data) :: "volatile");
}

#[allow(unused)]
pub unsafe fn inb(port: u16) -> u8 {
    let result: u8;
    asm!("inb %dx, %al" : "={al}"(result) : "{dx}"(port) :: "volatile");
    result
}


