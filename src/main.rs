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

  
 


    #[init(spawn = [foo])]
    fn init() {
        spawn.foo().unwrap();
    }

    #[task(spawn = [bar, baz])]
    fn foo() {
        hprintln!("foo").unwrap();

        // spawns `bar` onto the task scheduler
        // `foo` and `bar` have the same priority so `bar` will not run until
        // after `foo` terminates
        spawn.bar().unwrap();

        // spawns `baz` onto the task scheduler
        // `baz` has higher priority than `foo` so it immediately preempts `foo`
        spawn.baz().unwrap();
    }

    #[task]
    fn bar() {
        hprintln!("bar").unwrap();

     
    }

    #[task(priority = 2)]
    fn baz() {
        hprintln!("baz").unwrap();
    }
        
        

        
    
    
   // Interrupt handlers used to dispatch software tasks
   extern "C" {
       fn EXTI2();
       fn USART1();
   }   
    
    
};
