#![no_std]
#![no_main]

use core::fmt::Write as _;

use uefi::table::boot::MemoryMap;
use util::{
    buffer::StrBuf,
    graphics::{GlayscalePixelWrite as _, GlayscalePrint as _},
    screen::{FrameBufferInfo, GlayscaleScreen},
};

#[no_mangle]
fn _start(fb_info: &FrameBufferInfo, memmap: &MemoryMap) {
    let mut screen = GlayscaleScreen::new(fb_info.clone());
    // Draw whole screen with gray.
    let size = screen.range();
    for x in 0..size.0 {
        for y in 0..size.1 {
            screen.write((x, y), 0x00);
        }
    }

    // Display memmap
    let mut buf = [0; 256];
    // 1 memory descritpor length in display.
    let item_len = 60;

    let col_num = screen.range().0 / 8;
    let row_num = screen.range().1 / 16;
    let item_per_row = col_num / item_len;
    for (count, desc) in memmap.entries().enumerate().take(row_num * item_per_row) {
        let mut buf = StrBuf::new(&mut buf);
        let _ = write!(
            buf,
            "{:016x}-{:016x}: {:?}",
            desc.phys_start,
            desc.phys_start + desc.page_count * 4096,
            desc.ty,
        );
        // Calculate start position.
        let col = count / row_num;
        let row = count % row_num;
        screen.print(buf.to_str(), (col * item_len * 8, row * 16));
    }

    loop {
        unsafe { core::arch::asm!("hlt") };
    }
}

#[panic_handler]
fn _panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe { core::arch::asm!("hlt") };
    }
}
