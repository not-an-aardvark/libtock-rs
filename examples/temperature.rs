#![feature(alloc)]
#![no_std]
extern crate alloc;
extern crate tock;

use alloc::string::String;
use tock::console::Console;
use tock::temperature::TemperatureDriver;

#[allow(unreachable_code)]
fn main() {
    let mut console = Console::new();
    #[allow(unused_variables)]
    let temperature = TemperatureDriver::start_measurement(|result: isize| {
        console.write(String::from("Temperature:"));
        console.write(tock::fmt::i32_as_decimal(result as i32));
        console.write(String::from("\n"));
    });

    loop {
        tock::syscalls::yieldk();
    }
    // FIXME: Find another solution to prevent the compiler from calling drop too early.
    temperature.unwrap();
}
