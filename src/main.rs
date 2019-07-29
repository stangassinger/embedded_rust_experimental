#![no_main]
#![no_std]
#[cfg(not(test))]

extern crate stm32f4;
extern crate panic_halt;
extern crate cortex_m_semihosting;

use rtfm::app;
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::{delay, gpio, stm32, timer};


use cortex_m_semihosting::{debug, hprintln};


#[app(device = stm32f4::stm32f429 )]
const APP: () = {

  
 



    #[init]
    fn init(){
        static mut X: u32 = 0;

        hprintln!("Hello, world!").unwrap();
        // Safe access to local `static mut` variable
        let _x: &'static mut u32 = X;
        
        
        

        
    }
    
   // Interrupt handlers used to dispatch software tasks
   extern "C" {
       fn EXTI2();
   }   
    
    
};
