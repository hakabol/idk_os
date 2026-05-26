#![no_std] //disables std
#![no_main] //makes it so that main is no longer needed

use core::panic::PanicInfo;

#[panic_handler]  //since there isnt any stds the panic isnt handled
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[unsafe(no_mangle)] //dont mangle(hash the name) of this func 
pub extern "C" fn _start() -> ! {  //uses extern "C" for C`s naming convention`and the -> ! is to say that this fn wont end
    
    //Vga requires 2 bytes per letter one for the letter and one for the color     
    //which is a 4bit bg + a 4bit fg
    
    let vga_buffer = 0xb8000 as *mut u8;

    let bg = 0x1;
    let fg = 0x4;

    let color = (bg << 4) | fg; //shifts bg by 4 bits to the left(<<) and does or on bg and fg (|)

    for i in 0..80*25{
        unsafe {
            *vga_buffer.offset(i as isize * 2) = b' '; //offset is used tp change the value at n bits after the current loc
            *vga_buffer.offset(i as isize * 2 + 1) = color;

        }
    }

    for (i, &byte) in HELLO.iter().enumerate(){
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = color;
        }
    }

    loop {}
}
