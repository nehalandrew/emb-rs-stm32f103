#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

// use alloc::format;
// use panic_halt as _;
use panic_semihosting as _;

use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let p = pac::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = p.AFIO.constrain();
    // let gpioa = p.GPIOA.split();
    let mut gpiob = p.GPIOB.split();
    // let (_pa15, _pb3, pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

    // TIM4
    let pb6 = gpiob.pb6.into_alternate_push_pull(&mut gpiob.crl);
    let pb7 = gpiob.pb7.into_alternate_push_pull(&mut gpiob.crl);

    let pwm = Timer::new(p.TIM4, &clocks).pwm_hz((pb6, pb7), &mut afio.mapr, 50.Hz());

    let maxd = pwm.get_max_duty(); // 53333
    let min = 2272 as u16; // pwm.get_max_duty() / 31; // 1720
    let max = 50250 as u16; // pwm.get_max_duty() / 9; // 5925

    // hprintln!("max duty: {}", maxd);
    // hprintln!("max duty: {}", min);
    // hprintln!("max duty: {}", max);

    let mut pwm_channels = pwm.split();

    // Enable the individual channels

    // full
    // dim
    // pwm_channels.1.set_duty(max / 4);

    // asm::bkpt();

    // zero
    pwm_channels.0.set_duty(min);
    pwm_channels.1.set_duty(min);

    pwm_channels.0.enable();
    pwm_channels.1.enable();
    // asm::bkpt();
    let mut delay = cp.SYST.delay(&clocks);
    delay.delay_ms(100u16);
    loop {
        // for i in min..=max {
        pwm_channels.0.set_duty(max);
        pwm_channels.1.set_duty(max);
        delay.delay_ms(10000u16);
        // }
        pwm_channels.0.set_duty(min);
        pwm_channels.1.set_duty(min);
        delay.delay_ms(10000u16);
        // delay.delay_ms(1000u16);
        // pwm_channels.0.set_duty(0);
        // pwm_channels.1.set_duty(0);
        // delay.delay_ms(1000u16);
    }
}
