#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
use esp_backtrace as _; // Panic behaviour

use rtic::app;
mod timer_task;

// remove dispatchers if you do not need software tasks. You need to have as many dispatchers as you have different priority levels for software tasks
#[app(device=esp32c3, dispatchers = [FROM_CPU_INTR0,FROM_CPU_INTR1, FROM_CPU_INTR2, FROM_CPU_INTR3] )]
mod app {
    use esp_hal::{
        self as _,
        clock::ClockControl,
        delay::Delay,
        gpio::{Gpio9, Input, PullUp},
        peripherals::{Peripherals, TIMG0, TIMG1},
        prelude::*,
        timer::{Timer, Timer0, TimerGroup},
        IO,
    };
    use esp_println::println;

    // Shared resources go here
    #[shared]
    struct Shared {
        timer1: Timer<Timer0<TIMG1>>,
    }

    // Local resources go here
    #[local]
    struct Local {
        delay: Delay,
        timer0: Timer<Timer0<TIMG0>>,
        button: Gpio9<Input<PullUp>>,
        timer1_is_running: bool,
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

        let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
        let timer1 = timer_group1.timer0;

        let mut button = io.pins.gpio9.into_pull_up_input();
        button.listen(esp_hal::gpio::Event::FallingEdge);

        timer0.listen();
        timer0.start(2u32.secs());

        task1::spawn().expect("Failed to spawn task1");

        (
            Shared { timer1 },
            Local {
                delay,
                timer0,
                button,
                timer1_is_running: false,
            },
        )
    }

    // Optional idle, can be removed if not needed. Has prioty of 0
    #[idle(local = [delay])]
    fn idle(_: idle::Context) -> ! {
        loop {
            println!("Idling");
            esp_hal::esp_riscv_rt::riscv::asm::wfi(); // use this to sleep and wake on interruptions
                                                      // cx.local.delay.delay_ms(1000u32); //
        }
    }

    use crate::timer_task::{timer0_task, timer1_task};

    /// Tasks can be moved to different scope by using extern methods
    extern "Rust" {
        #[task(binds= TG0_T0_LEVEL, local=[timer0])]
        fn timer0_task(cx: timer0_task::Context);

        #[task(binds= TG1_T0_LEVEL, shared=[timer1])]
        fn timer1_task(mut cx: timer1_task::Context);
    }

    /// Sets timer1_task on and off via the boot button on the esp32c3
    #[task(binds=GPIO, local=[button, timer1_is_running], shared=[timer1])]
    fn button(mut cx: button::Context) {
        cx.local.button.clear_interrupt();
        println!("Button task");
        cx.shared.timer1.lock(|t| {
            *cx.local.timer1_is_running = match cx.local.timer1_is_running {
                true => {
                    t.clear_interrupt();
                    t.unlisten();
                    false
                }
                false => {
                    t.clear_interrupt();
                    t.listen();
                    t.start(3u32.secs());
                    true
                }
            }
        })
    }

    // Software task 1
    #[task(priority = 1)]
    async fn task1(_cx: task1::Context) {
        println!("Hello from software task1");
    }

    #[task(priority = 1)]
    async fn task1b(_cx: task1b::Context) {
        println!("Hello from software task1b");
    }

    // Software task 2 with higher priority than task 1
    #[task(priority = 2)]
    async fn task2(_cx: task2::Context) {
        println!("Hello from software task2");
    }

    #[task(priority = 3)]
    async fn task3(_cx: task3::Context) {
        println!("Hello from software task3");
    }

    #[task(priority = 4)]
    async fn task4(_cx: task4::Context) {
        println!("Hello from software task4");
    }
}
