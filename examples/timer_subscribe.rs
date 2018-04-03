#![feature(alloc)]
#![no_std]

extern crate alloc;
extern crate tock;

use alloc::String;
use tock::console::Console;
use tock::syscalls;
use tock::timer;
use tock::timer::Duration;
use tock::timer::TimerCallback;

fn main() {
    let mut console = Console::new();

    let mut callback = TimerCallback::new(|_, _| {
        console.write(String::from(
            "This line is printed 2 seconds after the start of the program.",
        ))
    });

    let mut timer = timer::with_callback(&mut callback).unwrap();

    timer.set_alarm(Duration::from_ms(2000)).unwrap();
    loop {
        syscalls::yieldk();
    }
}
