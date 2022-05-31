#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};

use core::fmt::Write; // for pretty formatting of the serial output

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();

    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.use_hse(25.MHz()).freeze();

    // define RX/TX pins
    let tx_pin = gpioa.pa9.into_alternate();

    // configure serial
    let mut tx = dp.USART1.tx(tx_pin, 115200.bps(), &clocks).unwrap();

    writeln!(tx, "Hello Dojo").unwrap();
    
    loop {
    }
}