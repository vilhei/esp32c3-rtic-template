#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
use esp_backtrace as _;
// use esp_hal::peripherals::Interrupt::FROM_CPU_INTR0;
use rtic::app; // global logger + panicking-behavior + memory layout
               // use esp_hal as _;
               // use esp32c3::Interrupt::FROM_CPU_INTR0;

#[app(device=esp32c3,dispatchers=[FROM_CPU_INTR0])]
mod app {

    use esp_println::println;

    // Shared resources go here
    #[shared]
    struct Shared {}

    // Local resources go here
    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        // defmt::info!("init");

        // let sysclk = { /* clock setup + returning sysclk as an u32 */ };
        // let token = rtic_monotonics::create_systick_token!();
        // rtic_monotonics::systick::Systick::new(cx.core.SYST, sysclk, token);
        // esp32c3::Interrupt::FROM_CPU_INTR0
        task1::spawn().ok();

        (
            Shared {
                // Initialization of shared resources go here
            },
            Local {
                // Initialization of local resources go here
            },
        )
    }

    // Optional idle, can be removed if not needed.
    #[idle]
    fn idle(_: idle::Context) -> ! {
        // defmt::info!("idle");
        println!("Idling");

        loop {
            continue;
        }
    }

    // // TODO: Add tasks
    #[task(priority = 1)]
    async fn task1(_cx: task1::Context) {
        println!("Hello from task1");
    }
}
