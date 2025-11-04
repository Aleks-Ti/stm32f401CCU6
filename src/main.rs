#![no_std]
#![no_main]
use cortex_m_rt::entry;
use stm32f4xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    // Получаем периферию
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // Тактирование: 84 МГц
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(84.MHz()).freeze();

    // Порты
    let gpioc = dp.GPIOC.split();
    let mut led = gpioc.pc13.into_push_pull_output();

    // Используем SysTick как задержку
    let mut delay = cortex_m::delay::Delay::new(cp.SYST, clocks.sysclk().to_Hz());

    loop {
        led.set_low(); // Включить (горит — инверсия!)
        delay.delay_ms(300);
        led.set_high(); // Выключить
        delay.delay_ms(300);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
