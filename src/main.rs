#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use rtic::app;

#[app(device=esp32c3, dispatchers=[FROM_CPU_INTR0])]
mod app {

    use esp_backtrace as _;

    use esp_hal::{
        self as _,
        clock::ClockControl,
        delay::Delay,
        gpio::{GpioPin, Input, Io, Pull},
        peripherals::{Peripherals, TIMG0},
        prelude::*,
        system::SystemControl,
        timer::timg::{Timer, TimerGroup, TimerX},
        Blocking,
    };
    use esp_println::println;

    // Shared resources go here
    #[shared]
    struct Shared {}

    // Local resources go here
    #[local]
    struct Local {
        delay: Delay,
        timer0: Timer<TimerX<TIMG0>, Blocking>,
        button: Input<'static, GpioPin<9>>,
    }

    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        println!("Init");

        let peripherals = Peripherals::take();
        let system = SystemControl::new(peripherals.SYSTEM);
        let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

        let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

        let delay = Delay::new(&clocks);

        let timg0 = TimerGroup::new(peripherals.TIMG0, &clocks, None);
        let timer0 = timg0.timer0;

        let mut button: Input<'static, GpioPin<9>> = Input::new(io.pins.gpio9, Pull::Up);
        button.listen(esp_hal::gpio::Event::FallingEdge);

        timer0.load_value(1.secs()).unwrap();
        timer0.start();
        timer0.listen();

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
    fn idle(cx: idle::Context) -> ! {
        loop {
            println!("Idling");
            // esp_hal::esp_riscv_rt::riscv::asm::wfi(); // use this to sleep and wake on interruptions
            cx.local.delay.delay_millis(500);
        }
    }

    #[task(binds= TG0_T0_LEVEL, local=[timer0])]
    fn timer0_task(cx: timer0_task::Context) {
        cx.local.timer0.clear_interrupt();
        println!("Hello from timer0 task");
        cx.local.timer0.start();
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
