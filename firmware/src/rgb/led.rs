// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use embassy_rp::peripherals::PIO0;
use embassy_rp::pio_programs::ws2812::PioWs2812;
use smart_leds::RGB8;

pub struct RgbLed {
    led: PioWs2812<'static, PIO0, 0, 1>,
    buf: [RGB8; 1],
}

impl RgbLed {
    pub fn new(led: PioWs2812<'static, PIO0, 0, 1>) -> Self {
        Self {
            led,
            buf: [RGB8 { r: 0, g: 0, b: 0 }],
        }
    }

    pub async fn set_rgb(&mut self, r: u8, g: u8, b: u8) {
        self.buf[0] = RGB8 { r, g, b };
        self.led.write(&self.buf).await;
    }

    pub async fn off(&mut self) {
        self.set_rgb(0, 0, 0).await;
    }
}
