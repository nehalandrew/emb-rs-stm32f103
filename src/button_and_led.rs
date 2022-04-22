//! Blinks an LED
//!
//! This assumes that a LED is connected to pc13 as is the case on the blue pill board.
//!
//! Note: Without additional hardware, PC13 should not be used to drive an LED, see page 5.1.2 of
//! the reference manual for an explanation. This is not an issue on the blue pill.

#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f1xx_hal::{gpio::PinState, pac, prelude::*};

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding HAL structs
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in `clocks`
    let clock = rcc.cfgr.freeze(&mut flash.acr);
    // Acquire the GPIOC peripheral
    let mut gpioa = dp.GPIOA.split();
    // let mut gpiob = dp.GPIOB.split();
    let mut gpioc = dp.GPIOC.split();

    // Configure gpio C pin 13 as a push-pull openocd -f interface/stlink-v2-1.cfg -f target/stm32f1x.cfgoutput. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let mut led = gpioc
        .pc13
        .into_push_pull_output_with_state(&mut gpioc.crh, PinState::High);
    // COnfigure button
    let button = gpioa.pa0.into_pull_up_input(&mut gpioa.crl);

    // Configure the states
    // let mut led_state = false;into_push_pull_output(&mut gpioc.crh);
    // let mut button_up = false;

    // Configure the syst timer to trigger an update every second
    // Wait for the timer to trigger an update and change the state of the LED
    let mut delay = cp.SYST.delay(&clock);

    loop {
        let button_state = button.is_low();
        if button_state == true {
            // button_up = false;
            delay.delay_ms(10u8);
            led.toggle();
        }
    }
}
