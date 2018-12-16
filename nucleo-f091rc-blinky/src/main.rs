#![no_main]
#![no_std]

extern crate stm32f0;
extern crate panic_halt;

use stm32f0::stm32f0x1;
use cortex_m::asm;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {

    // Get references to STM32F091RC rcc and gpioa peripherals
    let per = stm32f0x1::Peripherals::take().unwrap();
    let rcc = &per.RCC;
    let gpioa = &per.GPIOA;

    // RCC GPIOA Clock Enable
    rcc.ahbenr.write(|w| w.iopaen().set_bit());

    // Set PA5 as digital output
    gpioa.moder.write(|w| w.moder5().bits(1));

    // LED toggle bit
    let mut led_toggle = false;

    loop {

        // Flip toggle bit and write to output data bit
        led_toggle = !led_toggle;
        gpioa.odr.write(|w| w.odr5().bit(led_toggle));

        // Wait for 5000 nops
        for _i in 0..5000 {
            asm::nop()
        }
    }
}