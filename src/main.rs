#![no_main]
#![no_std]

use {defmt_rtt as _, panic_probe as _};

// Prevents panic message being printed twice when defmt::panic is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf();
}

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");

    loop {
        cortex_m::asm::nop();
    }
}
