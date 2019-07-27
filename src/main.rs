#![no_main]
#![no_std]

#[cfg(not(test))]
extern crate panic_semihosting;


use rtfm::app;
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::{delay, gpio, stm32, timer};



#[app(device = stm32f4xx_hal::stm32 )]
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
