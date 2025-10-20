// SPDX-FileCopyrightText: 2025 hexaTune LLC
// SPDX-License-Identifier: MIT

use embassy_rp::gpio::Output;
use embassy_time::{Duration, Timer};

use crate::error::Error;

const CTRL_PWRDOWN: u8 = 1 << 7;
const CTRL_SLEEP: u8 = 1 << 6;
//const CTRL_X6: u8 = 1 << 5; // AD9851 x6 PLL
const CTRL_PHASE0: u8 = 0;
const PULSE_US: u64 = 1;

pub struct Ad985x {
    wclk: Output<'static>,
    fq_ud: Output<'static>,
    data: Output<'static>,
    rst: Output<'static>,
    ref_clk_hz: u32,
    ctrl_base: u8,
}

impl Ad985x {
    pub fn new(
        wclk: Output<'static>,
        fq_ud: Output<'static>,
        data: Output<'static>,
        rst: Output<'static>,
        ref_clk_hz: u32,
        ctrl_base: u8,
    ) -> Self {
        Self {
            wclk,
            fq_ud,
            data,
            rst,
            ref_clk_hz,
            ctrl_base,
        }
    }

    #[inline(always)]
    async fn pulse_high_low(pin: &mut Output<'static>) {
        pin.set_high();
        Timer::after(Duration::from_micros(PULSE_US)).await;
        pin.set_low();
        Timer::after(Duration::from_micros(PULSE_US)).await;
    }

    async fn shift_lsb_first(&mut self, mut value: u64, bits: usize) {
        for _ in 0..bits {
            if (value & 1) != 0 {
                self.data.set_high();
            } else {
                self.data.set_low();
            }
            self.wclk.set_high();
            Timer::after(Duration::from_micros(PULSE_US)).await;
            self.wclk.set_low();
            Timer::after(Duration::from_micros(PULSE_US)).await;
            value >>= 1;
        }
    }

    async fn write_ftw_ctrl(&mut self, ftw: u32, ctrl: u8) {
        self.shift_lsb_first(ftw as u64, 32).await;
        self.shift_lsb_first(ctrl as u64, 8).await;
        Self::pulse_high_low(&mut self.fq_ud).await;
    }

    fn hz_to_ftw(&self, freq_hz: u32) -> u32 {
        let num = (freq_hz as u64) << 32;
        let den = self.ref_clk_hz as u64;
        ((num + den / 2) / den) as u32
    }

    pub async fn reset(&mut self) -> Option<Error> {
        self.rst.set_high();
        Timer::after(Duration::from_micros(5)).await;
        self.rst.set_low();
        Timer::after(Duration::from_micros(5)).await;

        for _ in 0..5 {
            Self::pulse_high_low(&mut self.wclk).await;
        }
        Self::pulse_high_low(&mut self.fq_ud).await;

        self.write_ftw_ctrl(0, self.ctrl_base & !(CTRL_SLEEP | CTRL_PWRDOWN))
            .await;

        None
    }

    /*pub async fn sleep(&mut self) -> Option<Error> {
        self.write_ftw_ctrl(0, self.ctrl_base | CTRL_SLEEP).await;
        None
    }*/

    pub async fn down(&mut self) -> Option<Error> {
        self.write_ftw_ctrl(0, self.ctrl_base | CTRL_PWRDOWN).await;
        None
    }

    pub async fn up(&mut self) -> Option<Error> {
        self.write_ftw_ctrl(0, self.ctrl_base & !(CTRL_SLEEP | CTRL_PWRDOWN))
            .await;
        None
    }

    pub async fn set_freq(&mut self, freq_hz: u32, dwell_ms: u32) -> Option<Error> {
        if let Some(e) = self.down().await {
            return Some(e);
        }

        if let Some(e) = self.up().await {
            return Some(e);
        }

        if let Some(e) = self.reset().await {
            return Some(e);
        }

        if let Some(e) = self.set_freq_immediate(freq_hz).await {
            return Some(e);
        }

        Timer::after(Duration::from_millis(dwell_ms as u64)).await;

        if let Some(e) = self.down().await {
            return Some(e);
        }

        None
    }

    async fn set_freq_immediate(&mut self, freq_hz: u32) -> Option<Error> {
        let ftw = self.hz_to_ftw(freq_hz);
        self.write_ftw_ctrl(ftw, self.ctrl_base | CTRL_PHASE0).await;
        None
    }
}
