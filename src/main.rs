#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
// use panic_rtt_target as _;
use esp_backtrace as _;
// use esp_hal::peripherals::Interrupt::FROM_CPU_INTR0;
use esp_hal as _;

use rtic::app; // global logger + panicking-behavior + memory layout
#[app(device=esp32c3 ,dispatchers=[FROM_CPU_INTR0])]
mod app {

    use esp_hal::{clock::ClockControl, delay::Delay, peripherals::Peripherals, prelude::*};
    use esp_println::println;
    // use rtt_target::{rprintln, rtt_init_print};

    // Shared resources go here
    #[shared]
    struct Shared {}

    // Local resources go here
    #[local]
    struct Local {
        delay: Delay,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let peripherals = Peripherals::take();
        let system = peripherals.SYSTEM.split();

        let clocks = ClockControl::max(system.clock_control).freeze();
        let delay = Delay::new(&clocks);
        println!("Init");
        println!("spwaned");

        (
            Shared {
                // Initialization of shared resources go here
            },
            Local {
                delay, // Initialization of local resources go here
            },
        )
    }

    // Optional idle, can be removed if not needed.
    #[idle(local = [delay])]
    fn idle(cx: idle::Context) -> ! {
        loop {
            println!("Idling");
            cx.local.delay.delay_millis(5000);
        }
    }

    // TODO: Add tasks
    #[task(priority = 1)]
    async fn task1(_cx: task1::Context) {
        println!("Hello from task1");
    }
}
