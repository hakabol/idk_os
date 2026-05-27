#![no_std] //disables std
#![no_main] //makes it so that main is no longer needed

use core::{fmt::Write, panic::PanicInfo};

mod vga_buffer;

#[panic_handler]  //since there isnt any stds the panic isnt handled
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)] //dont mangle(hash the name) of this func 
pub extern "C" fn _start() -> ! {  //uses extern "C" for C`s naming convention`and the -> ! is to say that this fn wont end
    //Vga requires 2 bytes per letter one for the letter and one for the color     
    //which is a 4bit bg + a 4bit fg
    
    //for i in 0..80*25{
    //    unsafe {
    //        *vga_buffer_pointer.offset(i as isize * 2) = b' '; //offset is used tp change the value at n bits after the current loc
    //        *vga_buffer_pointer.offset(i as isize * 2 + 1) = color;

    //    }
    //}

    //vga_buffer::write_smthin();
    
    let color = vga_buffer::ColorCode::new(vga_buffer::Color::Red, vga_buffer::Color::Blue);
    let mut writer = vga_buffer::Writer::new(color);

    writer.paint_screen();

    writer.row_position = 12;
    writer.column_position = 37;
    writer.write_string("Halloy\n");
    writer.column_position = 36;
    writer.write_string("Woruldy!\n");

    write!(writer, "idk 2^2 = {}", 2_i32.pow(2)).unwrap();
    loop {}
}
