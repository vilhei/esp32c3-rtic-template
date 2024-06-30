#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use rtic::app;

// remove dispatchers if you do not need software tasks. You need to have as many dispatchers as you have different priority levels for software tasks
#[app(device=esp32c3, dispatchers=[FROM_CPU_INTR0])]
mod app {
    use esp_backtrace as _; // Panic behaviour

    use esp_hal::{
        self as _,
        clock::ClockControl,
        delay::Delay,
        gpio::{Gpio9, Input, PullUp},
        peripherals::{Peripherals, TIMG0},
        prelude::*,
        timer::{Timer, Timer0, TimerGroup},
        IO,
    };
    use esp_println::println;

    // Shared resources go here
    #[shared]
    struct Shared {}

    // Local resources go here
    #[local]
    struct Local {
        delay: Delay,
        timer0: Timer<Timer0<TIMG0>>,
        button: Gpio9<Input<PullUp>>,
    }

    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        println!("Init");

        let peripherals = Peripherals::take();
        let system = peripherals.SYSTEM.split();
        let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

        let clocks = ClockControl::max(system.clock_control).freeze();
        let delay = Delay::new(&clocks);

        let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
        let mut timer0 = timer_group0.timer0;

        let mut button = io.pins.gpio9.into_pull_up_input();
        button.listen(esp_hal::gpio::Event::FallingEdge);

        timer0.listen();
        timer0.start(2u64.secs());

        task1::spawn().expect("Failed to spawn task1");

        (
            Shared {},
            Local {
                delay,
                timer0,
                button,
            },
        )
    }

    // Optional idle, can be removed if not needed. Has priority of 0
    #[idle(local = [delay])]
    fn idle(_: idle::Context) -> ! {
        loop {
            println!("Idling");
            esp_hal::esp_riscv_rt::riscv::asm::wfi(); // use this to sleep and wake on interruptions
                                                      // cx.local.delay.delay_ms(1000u32); //
        }
    }

    /// Tasks can be moved to different scope by using extern methods
    #[task(binds= TG0_T0_LEVEL, local=[timer0])]
    fn timer0_task(cx: timer0_task::Context) {
        cx.local.timer0.clear_interrupt();
        println!("Hello from timer0 task");
        cx.local.timer0.start(3u64.secs());
    }

    /// Binds to boot button 
    #[task(binds=GPIO, local=[button])]
    fn button(cx: button::Context) {
        cx.local.button.clear_interrupt();
        println!("Button task");
    }

    // Software task 1
    #[task(priority = 1)]
    async fn task1(_cx: task1::Context) {
        println!("Hello from software task1");
    }
}
