// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use defmt::*;
use embassy_futures::select::{Either, select};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

use crate::AT_CH;
use crate::RGB_CH;
use crate::channel::*;
use crate::rgb::RgbLed;

#[embassy_executor::task]
pub async fn rgb_task(mut rgb_led: RgbLed) {
    info!("Starting RGB task");

    // Boot animation
    rgb_led.set_rgb(0, 0, 255).await;
    info!("RGB set to blue");
    Timer::after_secs(1).await;
    rgb_led.set_rgb(0, 255, 0).await;
    info!("RGB set to green");
    Timer::after_secs(1).await;
    rgb_led.set_rgb(255, 0, 0).await;
    info!("RGB set to red");
    Timer::after_secs(1).await;
    rgb_led.set_rgb(0, 0, 0).await;
    info!("RGB set to off");
    Timer::after_secs(1).await;

    // State machine blink loop
    let mut state = RgbState::Idle;
    let mut idle_color: (u8, u8, u8) = (0, 255, 0);

    info!("Entering RGB state machine loop");

    loop {
        let (color, on_ms, off_ms) = match state {
            RgbState::Idle => (idle_color, 1000, 4000),
            RgbState::Prepare => ((0, 0, 255), 100, 100),
            RgbState::Generating => ((0, 0, 255), 200, 800),
        };

        // LED ON phase
        rgb_led.set_rgb(color.0, color.1, color.2).await;
        match select(Timer::after_millis(on_ms), RGB_CH.receive()).await {
            Either::First(_) => {}
            Either::Second(msg) => {
                handle_rgb_msg(&mut state, &mut idle_color, msg).await;
                continue;
            }
        }

        // LED OFF phase
        rgb_led.set_rgb(0, 0, 0).await;
        match select(Timer::after_millis(off_ms), RGB_CH.receive()).await {
            Either::First(_) => {}
            Either::Second(msg) => {
                handle_rgb_msg(&mut state, &mut idle_color, msg).await;
                continue;
            }
        }
    }
}

async fn handle_rgb_msg(state: &mut RgbState, idle_color: &mut (u8, u8, u8), msg: Msg) {
    match msg {
        Msg::RgbSet { id, r, g, b } => {
            info!("Updating idle color to ({}, {}, {})", r, g, b);
            *idle_color = (r, g, b);
            AT_CH.send(Msg::Done(id)).await;
        }
        Msg::RgbMode(new_state) => {
            info!("RGB state changed");
            *state = new_state;
        }
        _ => {}
    }
}
