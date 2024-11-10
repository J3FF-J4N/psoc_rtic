#![no_std]
#![no_main]

use cortex_m_semihosting::hprint;
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
//use cortex_m_rt::entry;

//extern crate panic_semihosting;
extern crate psoc6_hal;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use psoc6_hal::delay::Delay;
use psoc6_hal::prelude::*;

#[entry]
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    let p = psoc6_pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let gpio = p.GPIO.split();

    let mut led3 = gpio.p5_3.into_strong_output();
    let mut led4 = gpio.p5_4.into_strong_output();

    let mut delay = Delay::new(cp.SYST);

    loop {
        //hprintln!("red led low").unwrap();
        led3.set_low().unwrap();
        led4.set_high().unwrap();
        delay.delay_ms(1000u32);

        //hprintln!("red led high").unwrap();
        led3.set_high().unwrap();
        led4.set_low().unwrap();
        delay.delay_ms(1000u32);
    }
}
