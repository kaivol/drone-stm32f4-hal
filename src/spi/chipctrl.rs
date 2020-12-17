use crate::master::SpiMasterDrv;
use drone_cortexm::thr::prelude::*;
use drone_stm32_map::periph::{
    dma::ch::DmaChMap,
    gpio::pin::GpioPinMap,
    spi::{SpiCr1, SpiMap},
};
use drone_stm32f4_gpio_drv::{GpioPin, OutputMode, PinPullToken, PinTypeToken};

pub struct SpiChip<Pin: GpioPinMap, PinType: PinTypeToken, PinPull: PinPullToken> {
    cs: GpioPin<Pin, OutputMode, PinType, PinPull>,
}

impl<Pin: GpioPinMap, PinType: PinTypeToken, PinPull: PinPullToken> SpiChip<Pin, PinType, PinPull> {
    /// Select the chip by setting the CS pin low.
    fn select(&self) {
        self.cs.clear();
    }

    /// Deselect the chip by setting the CS pin high.
    fn deselect(&self) {
        self.cs.set();
    }
}

impl<Pin: GpioPinMap, PinType: PinTypeToken, PinPull: PinPullToken> SpiChip<Pin, PinType, PinPull> {
    /// Initialize a new `SpiChip` as deselected.
    pub fn init(cs: GpioPin<Pin, OutputMode, PinType, PinPull>) -> Self {
        let chip = Self { cs };
        chip.deselect();
        chip
    }
}

pub struct SelectGuard<'a, Pin: GpioPinMap, PinType: PinTypeToken, PinPull: PinPullToken> {
    chip: &'a SpiChip<Pin, PinType, PinPull>,
}

impl<Pin: GpioPinMap, PinType: PinTypeToken, PinPull: PinPullToken> Drop
    for SelectGuard<'_, Pin, PinType, PinPull>
{
    fn drop(&mut self) {
        self.chip.deselect();
    }
}

pub trait ChipCtrl {
    /// Select a specific chip and return a guard that deselects the chip when dropped.
    fn select<'guard, Pin: GpioPinMap, PinType: PinTypeToken, PinPull: PinPullToken>(
        &mut self,
        chip: &'guard SpiChip<Pin, PinType, PinPull>,
    ) -> SelectGuard<'guard, Pin, PinType, PinPull> {
        chip.select();
        SelectGuard { chip }
    }
}

impl<
        'drv,
        Spi: SpiMap + SpiCr1,
        DmaRx: DmaChMap,
        DmaRxInt: IntToken,
        DmaTx: DmaChMap,
        DmaTxInt: IntToken,
    > ChipCtrl for SpiMasterDrv<'drv, Spi, DmaRx, DmaRxInt, DmaTx, DmaTxInt>
{
}
