use esp_println::println;
use fugit::ExtU32;

use crate::app::{task1, task1b, task2, task3, task4, timer0_task, timer1_task};
use esp_hal::prelude::*;
use rtic::mutex_prelude::*;

pub(crate) fn timer0_task(cx: timer0_task::Context) {
    cx.local.timer0.clear_interrupt();
    println!("Timer task");
    task1::spawn().unwrap();
    task1b::spawn().unwrap();
    task4::spawn().unwrap();
    task2::spawn().unwrap();
    task3::spawn().unwrap();
    cx.local.timer0.start(2u32.secs());
}

pub(crate) fn timer1_task(mut cx: timer1_task::Context) {
    cx.shared.timer1.lock(|t| t.clear_interrupt());
    println!("Inside timer 1 task");
    cx.shared.timer1.lock(|t| t.start(3u32.secs()));
}
