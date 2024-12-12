#![no_std]
#![feature(split_array)]
#![feature(allocator_api)]
#![feature(ascii_char)]
#![feature(format_args_nl)]
#![feature(slice_ptr_get)]
#![allow(dead_code)]
#![deny(clippy::no_mangle_with_rust_abi)]
#![deny(improper_ctypes)]
#![deny(improper_ctypes_definitions)]

extern crate alloc;
use crate::menus::main_menu::MainMenu;

mod game;
mod menus;
mod rando;
mod system;
mod utils;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::utils::printf::debug_print(format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::utils::printf::debug_print(format_args_nl!($($arg)*));
    }};
}

// A Common Place where Custom code can be injected to run once per frame
// Returns whether or not to stop (0 == continue)
// Its current by changing r31 we can stop the game :D
#[no_mangle]
fn custom_main_additions(in_r31: u32) -> u32 {
    let mut ret_val = in_r31;

    // Example menu
    if in_r31 == 0 && MainMenu::display() {
        ret_val = 1;
    }

    // Example Text
    // write_text_on_screen();

    return ret_val;
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
