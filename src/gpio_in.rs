#![deny(unsafe_code)]
#![no_std]
#![no_main]
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f1xx_hal::{gpio::PinState, pac, prelude::*};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clock = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split();
    let mut _gpiob = dp.GPIOB.split();
    let mut gpioc = dp.GPIOC.split();
    // let mut gpiod = dp.GPIOD.split();

    // red_led and green_led
    let mut red_led = gpioa
        .pa8
        .into_push_pull_output_with_state(&mut gpioa.crh, PinState::High);
    let mut green_led = gpiod
        .pd2
        .into_push_pull_output_with_state(&mut gpiod.crl, PinState::High);

    let mut afio = dp.AFIO.constrain();
    let (gpioa_pa15, _gpiob_pb3, _gpiob_pb4) =
        afio.mapr.disable_jtag(gpioa.pa15, _gpiob.pb3, _gpiob.pb4);

    // key_0 and key_1
    let key_0 = gpioc.pc5.into_pull_up_input(&mut gpioc.crl);
    let key_1 = gpioa_pa15.into_pull_up_input(&mut gpioa.crh);

    // The key_up for check buttons if long press.
    // if key_up is true, and buttons were not long press.
    let mut key_up: bool = true;
    let mut delay = cp.SYST.delay(&clock);
    loop {
        let key_result = (key_0.is_low(), key_1.is_low());
        if key_up && (key_result.0 || key_result.1) {
            key_up = false;
            delay.delay_ms(10u8);
            match key_result {
                (x, _) if x == true => red_led.toggle(),
                (_, y) if y == true => green_led.toggle(),
                (_, _) => (),
            }
        } else if !key_result.0 && !key_result.1 {
            key_up = true;
            delay.delay_ms(10u8);
        }
    }
}
