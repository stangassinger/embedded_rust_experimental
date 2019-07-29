#![no_main]
#![no_std]

#[cfg(not(test))]



use rtfm::app;
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::{delay, gpio, stm32, timer};


extern crate stm32f4;
extern crate panic_halt;

#[app(device = stm32f4::stm32f429 )]
const APP: () = {
    static mut TIMER: stm32f4xx_hal::timer::Timer<stm32::TIM3> = ();



    #[init()]
    fn init() -> init::LateResources {
        let mut flash = device.FLASH;
        
        let mut rcc   = device.RCC.constrain();
       
        let clocks = rcc
            .cfgr
            .use_hse(20.mhz())
            .sysclk(168.mhz())
            .pclk1(42.mhz())
            .pclk2(84.mhz())           
            .freeze();
        let mut gpiog = device.GPIOG.split();
        let led13 = gpiog.pg13.into_push_pull_output();        

        
    }






    // Interrupt handlers used to dispatch software tasks
    extern "C" {
        fn EXTI2();
    }
};
