#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m::asm;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

use cortex_m_rt::entry;

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
    let pb6 = gpiob.pb6.into_alternate_open_drain(&mut gpiob.crl);
    let pb7 = gpiob.pb7.into_alternate_open_drain(&mut gpiob.crl);

    let pwm = Timer::new(p.TIM4, &clocks).pwm_hz((pb6, pb7), &mut afio.mapr, 50.Hz());

    let max = pwm.get_max_duty();

    let mut pwm_channels = pwm.split();

    // Enable the individual channels
    pwm_channels.0.enable();
    pwm_channels.1.enable();

    // full
    // dim
    // pwm_channels.1.set_duty(max / 4);

    // asm::bkpt();

    // zero
    // pwm_channels.0.set_duty(0);
    // pwm_channels.1.set_duty(0);

    // asm::bkpt();
    let mut delay = cp.SYST.delay(&clocks);
    loop {
        pwm_channels.0.set_duty(max / 2);
        pwm_channels.1.set_duty(max / 2);
        delay.delay_ms(1000u16);
        pwm_channels.0.set_duty(0);
        pwm_channels.1.set_duty(0);
        delay.delay_ms(1000u16);
    }
}
