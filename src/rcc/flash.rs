use crate::{traits::*, clktree::Freq, clktree::HClk, periph::FlashPeriph};
use drone_cortexm::reg::prelude::*;

pub struct Flash {
    flash: FlashPeriph,
}

impl Flash {
    pub fn new(flash: FlashPeriph) -> Self {
        Self { flash }
    }

    pub fn set_latency(&self, wait_states: u32) {
        self.flash
            .flash_acr
            .modify(|r| r.write_latency(wait_states));
    }
}

// STM32F405xx/07xx and STM32F415xx/17xx
#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f415",
    stm32_mcu = "stm32f417",
))]
impl HClkExt for HClk {
    fn get_wait_states(&self, voltage: VoltageRange) -> u32 {
        // Table 10 in PM0090.
        let upper = match voltage {
            VoltageRange::HighVoltage => [30, 60, 90, 120, 150, 168].as_ref(),
            VoltageRange::MediumVoltage => [24, 48, 72, 96, 120, 144, 168].as_ref(),
            VoltageRange::LowVoltage => [22, 44, 66, 88, 110, 132, 154, 168].as_ref(),
            VoltageRange::UltraLowVoltage => [20, 40, 60, 80, 100, 120, 140, 160].as_ref(),
        };
        get_wait_states(self, upper)
    }
}

// STM32F411
#[cfg(any(stm32_mcu = "stm32f411"))]
impl HClkExt for HClk {
    fn get_wait_states(&self, voltage: VoltageRange) -> u32 {
        // Table 5 in RM0383.
        let upper = match voltage {
            VoltageRange::HighVoltage => [30, 64, 90, 100].as_ref(),
            VoltageRange::MediumVoltage => [24, 48, 72, 96, 100].as_ref(),
            VoltageRange::LowVoltage => [18, 36, 54, 72, 90, 100].as_ref(),
            VoltageRange::UltraLowVoltage => [16, 32, 48, 64, 80, 96, 100].as_ref(),
        };
        get_wait_states(self, upper)
    }
}

// STM32F42xxx and STM32F43xxx
#[cfg(any(
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f437",
))]
impl HClkExt for HClk {
    fn get_wait_states(&self, voltage: VoltageRange) -> u32 {
        // Table 11 in PM0090.
        let upper = match voltage {
            VoltageRange::HighVoltage => [30, 60, 90, 120, 150, 180].as_ref(),
            VoltageRange::MediumVoltage => [24, 48, 72, 96, 120, 144, 168, 180].as_ref(),
            VoltageRange::LowVoltage => [22, 44, 66, 88, 110, 132, 154, 176, 180].as_ref(),
            VoltageRange::UltraLowVoltage => [20, 40, 60, 80, 100, 120, 140, 160, 180].as_ref(),
        };
        get_wait_states(self, upper)
    }
}

fn get_wait_states(hclk: &HClk, upper: &[u32]) -> u32 {
    let hclk = hclk.freq() / 1_000_000;
    upper
        .iter()
        .position(|max| hclk <= *max)
        .expect("Unable to determine number of wait states. Invalid HCLK frequency?") as u32
}
