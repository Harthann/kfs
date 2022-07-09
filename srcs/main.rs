#![feature(const_mut_refs)]
#![feature(lang_items)]
#![no_std]

use core::arch::asm;
mod io;
mod keyboard;
mod vga_buffer;
mod gdt;
mod cli;
mod paging;

//use paging::PAGE_DIRECTORY;
//use paging::PAGE_TABLE;
//use paging::KERNEL_PAGE_TABLE;

#[allow(dead_code)]
extern "C" {
	static gdt_desc: u16;
	fn _start();
	fn stack_bottom();
	fn stack_top();
}

use vga_buffer::color::Color;
use cli::Command;

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
/*
	let ptr = 0xdeadbeaf as *mut u32;
	unsafe { *ptr = 42; }
*/
	println!("Hello World of {}!", 42);
//	unsafe{asm!("hlt")};

	change_color!(Color::Red, Color::White);
	println!("Press Ctrl-{} to navigate to the second workspace", '2');
	change_color!(Color::White, Color::Black);

	//println!("Stack bottom: {:x}\nStack top:{:x}\nStart: {:x}\nRust main {:x}", stack_bottom as u32, stack_top as u32, _start as u32, rust_start as u32);
	hexdump!(0x800 as *mut _, unsafe{gdt_desc as usize});
	print!("$> ");
	loop {
		if keyboard::keyboard_event() {
			let charcode = keyboard::handle_event();
			clihandle!(charcode);
		}
	}
}

#[no_mangle]
pub extern "C" fn rust_start() {
//	unsafe{PAGE_DIRECTORY.entries[0] = (((&PAGE_TABLE as *const _) as usize) | 3) as *mut _};
//	unsafe{PAGE_DIRECTORY.entries[767] = (((&KERNEL_PAGE_TABLE as *const _) as usize) | 3) as *mut _};
//	enable_paging!();
//	reload_gdt!();
//	unsafe{asm!("hlt")};
//	unsafe{asm!("hlt");}
//	unsafe{asm!("mov esp, stack_top")};
	kernel_main();
}
