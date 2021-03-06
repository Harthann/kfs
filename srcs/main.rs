#![feature(const_mut_refs)]
#![feature(nonnull_slice_from_raw_parts)]
#![feature(rustc_attrs)]
#![feature(box_syntax)]
#![feature(ptr_internals)]
#![feature(fundamental)]
#![feature(lang_items)]
#![no_std]
#![allow(dead_code)]
#![no_main]


/*  Custom test framwork    */
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(test)]
#[macro_export]
macro_rules! function {
	() => {{
		fn f() {}
		fn type_name_of<T>(_: T) -> &'static str {
			core::any::type_name::<T>()
		}
		let mut name = type_name_of(f);
		name = &name[..name.len() - 3];
		let split = name.split("::");
		split.last().unwrap()
	}}
}

#[cfg(test)]
#[macro_export]
macro_rules! print_fn {
	() => {
		crate::kprint!("{:40}{}", function!(), "");
	}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
	kprintln!("Running {} tests", tests.len());
	for test in tests {
		test.run();
	}
	io::outb(0xf4, 0x10);
}

#[cfg(test)]
pub trait Testable {
	fn run(&self) -> ();
}

#[cfg(test)]
impl<T> Testable for T
where T: Fn(),
{
	fn run(&self) {
		self();
		change_color!(Color::Green, Color::Black);
		kprintln!("[ok]");
		change_color!(Color::White, Color::Black);
	}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}


const GLOBAL_ALIGN: usize = 8;

/*  Modules import  */
mod cli;
mod gdt;
mod keyboard;
mod memory;
mod multiboot;
mod vec;
mod interrupts;
mod io;
mod vga_buffer;

/*  Modules used function and variable  */
use memory::paging::{init_paging, page_directory};
use memory::allocator::linked_list::LinkedListAllocator;
use vga_buffer::color::Color;
use cli::Command;
use memory::allocator::{Box};

#[global_allocator]
static mut ALLOCATOR: LinkedListAllocator = LinkedListAllocator::new();

static mut KALLOCATOR: LinkedListAllocator = LinkedListAllocator::new();

/*  Code from boot section  */
#[allow(dead_code)]
extern "C" {
	static gdt_desc: u16;
	fn stack_bottom();
	fn stack_top();
	fn heap();
}

use crate::memory::{init_heap, init_stack, VirtAddr};
use crate::memory::paging::{PAGE_WRITABLE, PAGE_USER};

/*  Kernel initialisation   */
#[no_mangle]
pub extern "C" fn kinit() {
	multiboot::read_tags();
	init_paging();
	unsafe {init_heap(heap as u32, 100 * 4096, PAGE_WRITABLE, true, &mut ALLOCATOR)};
//	unsafe {init_heap(heap as u32 + 100 * 4096, 5 * 4096, PAGE_WRITABLE, true , &mut KALLOCATOR)};
	let kstack_addr: VirtAddr = 0xffbfffff; /* stack kernel */
	init_stack(kstack_addr, 8192, PAGE_WRITABLE, false);
	let stack_addr: VirtAddr = 0xbfffffff; /* stack user */
	init_stack(stack_addr, 8192, PAGE_WRITABLE | PAGE_USER, false);
	/* Reserve some spaces to push things before main */
	unsafe{core::arch::asm!("mov esp, eax", in("eax") kstack_addr - 256)};

	#[cfg(test)]
	test_main();

	#[cfg(not(test))]
	kmain();
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {

	kprintln!("Hello World of {}!", 42);

	change_color!(Color::Red, Color::White);
	kprintln!("Press Ctrl-{} to navigate to the second workspace", '2');
	change_color!(Color::White, Color::Black);

	#[cfg(not(test))]
	test();

	let x = Box::new(5 as u64);
	kprintln!("New value: {}", x);
	kprint!("$> ");
	loop {
		if keyboard::keyboard_event() {
			let charcode = keyboard::handle_event();
			clihandle!(charcode);
		}
	}
}


/*  Function to put all tests and keep main clean */
#[cfg(not(test))]
fn test() {
}

