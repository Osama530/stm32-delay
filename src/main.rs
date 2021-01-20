#![no_std]
#![no_main]

#[allow(unused_imports)]
 extern crate panic_semihosting ;
// extern crate cortex_m_rt;
// extern crate cortex_m_semihosting;
// extern crate stm32f3;

use cortex_m_rt::entry;
use cortex_m_semihosting:: {hprintln, hprint};
use stm32f3::stm32f303;


fn delay(ms: u16, tim6: &Peripherals::TIM6) {
    // let peripherals =stm32f3::stm32f303::Peripherals::take().unwrap();
    // let tim6 = peripherals.TIM6;
    // Set the timer to go off in `ms` ticks
    // 1 tick = 1 ms
    tim6.arr.write(|w| unsafe{
        w
            .arr().bits(ms) }   );

    // CEN: Enable the counter
    tim6.cr1.modify(|_, w| w.cen().set_bit());

    // Wait until the alarm goes off (until the update event occurs)
    while !tim6.sr.read().uif().bit_is_set() {}

    // Clear the update event flag
    tim6.sr.modify(|_, w| w.uif().clear_bit());
}

#[entry]
fn main() ->! {
    //initializing peripherals
    let peripherals =stm32f3::stm32f303::Peripherals::take().unwrap();
    let rcc = peripherals.RCC;
    // calling tim6 from library
    let tim6 = peripherals.TIM6;
    //enabling tim6 timer
    //rcc.apb1enr.modify(|_, w| w.tim6en().set_bit());
    rcc.apb1enr.write(|w| w.tim6en().set_bit());
    //configur the timmer to oprate in one puls mode
    //and disableing counter 
    tim6.cr1.write(|w| w.opm().set_bit().cen().clear_bit());
    //configuring the prescaler so the counter opratea at 1K Hz (1000ms)
    tim6.psc.write(|w| w.psc().bits(7999));
    
    
    
    
    let port_E = peripherals.GPIOE;
    rcc.ahbenr.write(|w| w.iopeen().bit(true));
    port_E.moder.write(|w| unsafe{
        w
            .moder8().bits(0b01)
    });
    port_E.pupdr.write(|w| unsafe{ 
        w
            .pupdr8().bits(0b00)
    });
    //port_E.bsrr.write(|w| w.bs8().set_bit());

    let ms = 100;
    hprintln!("hello world !").unwrap();
        loop {
            port_E.bsrr.write(|w| w.bs8().set_bit());
            delay(1000,tim6);
            //delay(100);
            port_E.bsrr.write(|w| w.br8().set_bit());
            delay(1000,tim6);
            //delay(100);

            
        }
}


//*************** Delay without using function ********/
// #![no_std]
// #![no_main]

// #[allow(unused_imports)]
//  extern crate panic_semihosting ;
// // extern crate cortex_m_rt;
// // extern crate cortex_m_semihosting;
// // extern crate stm32f3;

// use cortex_m_rt::entry;
// use cortex_m_semihosting:: {hprintln, hprint};
// use stm32f3::stm32f303;



// // use `main` as the entry point of this application
// // `main` is not allowed to return

// #[entry]
// fn main() ->! {
//     //initializing peripherals
//     let peripherals =stm32f3::stm32f303::Peripherals::take().unwrap();
//     let rcc = peripherals.RCC;
//     // calling tim6 from library
//     let tim6 = peripherals.TIM6;
//     //enabling tim6 timer
//     //rcc.apb1enr.modify(|_, w| w.tim6en().set_bit());
//     rcc.apb1enr.write(|w| w.tim6en().set_bit());
//     //configur the timmer to oprate in one puls mode
//     //and disableing counter 
//     tim6.cr1.write(|w| w.opm().set_bit().cen().clear_bit());
//     //configuring the prescaler so the counter opratea at 1K Hz (1000ms)
//     tim6.psc.write(|w| w.psc().bits(7999));
    
    
    
    
//     let port_E = peripherals.GPIOE;
//     rcc.ahbenr.write(|w| w.iopeen().bit(true));
//     port_E.moder.write(|w| unsafe{
//         w
//             .moder8().bits(0b01)
//     });
//     port_E.pupdr.write(|w| unsafe{ 
//         w
//             .pupdr8().bits(0b00)
//     });
//     //port_E.bsrr.write(|w| w.bs8().set_bit());

//     let ms = 100;
//     hprintln!("hello world !").unwrap();
//         loop {
//             port_E.bsrr.write(|w| w.bs8().set_bit());
//             tim6.arr.write(|w| unsafe{
//                 w
//                     .arr().bits(ms) }   );
        
//             // CEN: Enable the counter
//             tim6.cr1.modify(|_, w| w.cen().set_bit());
        
//             // Wait until the alarm goes off (until the update event occurs)
//             while !tim6.sr.read().uif().bit_is_set() {}
        
//             // Clear the update event flag
//             tim6.sr.modify(|_, w| w.uif().clear_bit());
//             //delay(100);
//             port_E.bsrr.write(|w| w.br8().set_bit());
//             //delay(100);
//             tim6.arr.write(|w| unsafe{
//                 w
//                     .arr().bits(ms) }   );
        
//             // CEN: Enable the counter
//             tim6.cr1.modify(|_, w| w.cen().set_bit());
        
//             // Wait until the alarm goes off (until the update event occurs)
//             while !tim6.sr.read().uif().bit_is_set() {}
        
//             // Clear the update event flag
//             tim6.sr.modify(|_, w| w.uif().clear_bit());
//         }
// }