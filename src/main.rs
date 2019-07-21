#![no_main]
#![no_std]

#[cfg(not(test))]
extern crate panic_semihosting;


use rtfm::app;
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::{delay, gpio, stm32, timer};



#[app(device = stm32f4xx_hal::stm32)]
const APP: () = {
    static mut TIMER: stm32f4xx_hal::timer::Timer<stm32::TIM3> = ();


    #[init()]
    fn init() -> init::LateResources {
        let mut flash = device.FLASH.constrain();
        let mut rcc = device.RCC.constrain();
        let mut afio = device.AFIO.constrain(&mut rcc.apb2);
        let clocks = rcc
            .cfgr
            .use_hse(20.mhz())
            .sysclk(168.mhz())
            .pclk1(42.mhz())
            .pclk2(84.mhz())           
            .freeze(&mut flash.acr);
        let mut gpiog = device.GPIOG.split(&mut rcc.apb2);
        let led13 = gpiog.pg13.into_alternate_push_pull(&mut gpiog.crl);

        
    }



    #[idle]
    fn idle() -> ! {
        static mut X: u32 = 0;

        // Safe access to local `static mut` variable
        let mut _x: &'static mut u32 = X;

        _x = _x + 1;

        loop {}
    }





    // Interrupt handlers used to dispatch software tasks
    extern "C" {
        fn EXTI2();
    }
};
