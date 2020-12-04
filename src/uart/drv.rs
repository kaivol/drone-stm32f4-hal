use crate::{diverged::UartDiverged, rx::UartRxDrv, tx::UartTxDrv};
use drone_cortexm::{fib, reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::{
    dma::ch::{DmaChMap, DmaChPeriph},
    uart::{traits::*, UartMap, UartPeriph},
};

pub mod config {
    use super::*;

    /// Uart setup.
    pub struct UartSetup<Uart: UartMap, UartInt: IntToken> {
        /// Uart peripheral.
        pub uart: UartPeriph<Uart>,
        /// Uart global interrupt.
        pub uart_int: UartInt,
        /// Baud rate.
        pub baud_rate: BaudRate,
        /// Data bits.
        pub data_bits: DataBits,
        /// Parity.
        pub parity: Parity,
        /// Stop bits.
        pub stop_bits: StopBits,
        /// Oversampling mode.
        pub oversampling: Oversampling,
    }

    impl<Uart: UartMap, UartInt: IntToken> UartSetup<Uart, UartInt> {
        /// Create a new uart setup with sensible defaults.
        pub fn new(
            uart: UartPeriph<Uart>,
            uart_int: UartInt,
            baud_rate: BaudRate,
        ) -> UartSetup<Uart, UartInt> {
            UartSetup {
                uart,
                uart_int,
                baud_rate,
                data_bits: DataBits::Eight,
                parity: Parity::None,
                stop_bits: StopBits::One,
                oversampling: Oversampling::By16,
            }
        }
    }

    /// Uart tx/rx dma channel setup.
    pub struct UartDmaSetup<DmaCh: DmaChMap, DmaInt: IntToken> {
        /// DMA channel peripheral.
        pub dma: DmaChPeriph<DmaCh>,
        /// DMA channel interrupt.
        pub dma_int: DmaInt,
        /// DMA channel number.
        pub dma_ch: u32,
        /// DMA channel priority level.
        pub dma_pl: u32,
    }

    #[derive(Copy, Clone)]
    pub enum BaudRate {
        Nominal { baud_rate: u32, f_pclk: u32 },
        Custom { div_man: u32, div_frac: u32 },
    }

    impl BaudRate {
        pub fn nominal(baud_rate: u32, f_pclk: u32) -> BaudRate {
            BaudRate::Nominal { baud_rate, f_pclk }
        }

        pub(crate) fn brr(&self, oversampling: Oversampling) -> (u32, u32) {
            match self {
                BaudRate::Nominal { baud_rate, f_pclk } => {
                    // Compute the uart divider for use by the baud rate register
                    // according to eqn. 1 in PM0090 §30.3.4 page 978.
                    // The computation of the divisor is as follows:
                    //
                    //                            f_pclk
                    //       USARTDIV = ---------------------------
                    //                  8 * (2 - over8) * baud_rate
                    //                |
                    //                V        25 * f_pclk
                    // 100 * USARTDIV = ---------------------------
                    //                  2 * (2 - over8) * baud_rate
                    //
                    // Note that 25 * f_pclk fits safely in a u32 as max f_pclk = 90_000_000.
                    let over8 = (oversampling == Oversampling::By8) as u32;
                    let div100 = (25 * f_pclk) / (2 * (2 - over8) * baud_rate);
                    let div_man = div100 / 100; // The mantissa part is: (100 * USARTDIV) / 100
                    let rem100 = div100 - div_man * 100; // The reminder after the division: (100 * USARTDIV) % 100
                    let div_frac = if over8 == 1 {
                        // The frac field has 3 bits, 0..15
                        (rem100 * 16 + 50) / 100
                    } else {
                        // The frac field has 4 bits, 0..31
                        (rem100 * 32 + 50) / 100
                    };

                    (div_man, div_frac)
                }
                BaudRate::Custom { div_man, div_frac } => (*div_man, *div_frac),
            }
        }
    }

    /// Uart data bits.
    #[derive(Copy, Clone, PartialEq)]
    pub enum DataBits {
        #[doc = "8 data bits."]
        Eight,
        #[doc = "9 data bits."]
        Nine,
    }

    /// Uart parity.
    #[derive(Copy, Clone, PartialEq)]
    pub enum Parity {
        None,
        Even,
        Odd,
    }

    /// Uart stop bits.
    #[derive(Copy, Clone, PartialEq)]
    pub enum StopBits {
        #[doc = "½ stop bit."]
        Half,
        #[doc = "1 stop bit."]
        One,
        #[doc = "1½ stop bit."]
        OneHalf,
        #[doc = "2 stop bits."]
        Two,
    }

    #[derive(Copy, Clone, PartialEq)]
    pub enum Oversampling {
        By8,
        By16,
    }
}

/// Uart driver.
pub struct UartDrv<Uart: UartMap, UartInt: IntToken> {
    uart: UartDiverged<Uart>,
    uart_int: UartInt,
}

impl<Uart: UartMap, UartInt: IntToken> UartDrv<Uart, UartInt> {
    /// Sets up a new [`UartDrv`] from `setup` values.
    #[must_use]
    pub fn init(setup: config::UartSetup<Uart, UartInt>) -> Self {
        let config::UartSetup {
            uart,
            uart_int,
            baud_rate,
            data_bits,
            stop_bits,
            parity,
            oversampling,
        } = setup;
        let mut drv = Self {
            uart: uart.into(),
            uart_int,
        };
        drv.init_uart(baud_rate, data_bits, parity, stop_bits, oversampling);
        drv
    }

    /// Obtain a configured [`UartTxDrv`] from dma `setup` values.
    pub fn tx<DmaCh: DmaChMap, DmaInt: IntToken>(
        &self,
        setup: config::UartDmaSetup<DmaCh, DmaInt>,
    ) -> UartTxDrv<Uart, UartInt, DmaCh, DmaInt> {
        let config::UartDmaSetup {
            dma,
            dma_int,
            dma_ch,
            dma_pl,
        } = setup;
        let mut tx = UartTxDrv {
            uart: &self.uart,
            uart_int: &self.uart_int,
            dma: dma.into(),
            dma_int,
        };
        tx.init_dma_tx(dma_ch, dma_pl);
        tx
    }

    /// Obtain a configured [`UartRxDrv`] from dma `setup` values.
    pub fn rx<DmaCh: DmaChMap, DmaInt: IntToken>(
        &self,
        setup: config::UartDmaSetup<DmaCh, DmaInt>,
    ) -> UartRxDrv<Uart, UartInt, DmaCh, DmaInt> {
        let config::UartDmaSetup {
            dma,
            dma_int,
            dma_ch,
            dma_pl,
        } = setup;
        let mut rx = UartRxDrv {
            uart: &self.uart,
            uart_int: &self.uart_int,
            dma: dma.into(),
            dma_int,
        };
        rx.init_dma_rx(dma_ch, dma_pl);
        rx
    }

    fn init_uart(
        &mut self,
        baud_rate: config::BaudRate,
        data_bits: config::DataBits,
        parity: config::Parity,
        stop_bits: config::StopBits,
        oversampling: config::Oversampling,
    ) {
        use self::config::*;

        // Enable uart clock.
        self.uart.rcc_busenr_uarten.set_bit();

        // Configure uart.
        self.uart.uart_cr1.store_reg(|r, v| {
            // Do not enable uart before it is fully configured.

            // Word length.
            if data_bits == DataBits::Nine {
                r.m().set(v);
            }

            // Parity.
            if parity != Parity::None {
                // Enable parity.
                r.pce().set(v);
                if parity == Parity::Odd {
                    // Parity selection: odd.
                    r.ps().set(v);
                }
            }

            // Oversampling.
            if oversampling == Oversampling::By8 {
                r.over8().set(v);
            }
        });
        self.uart.uart_cr2.store_reg(|r, v| {
            // Stop bits.
            r.stop().write(
                v,
                match stop_bits {
                    StopBits::One => 0,
                    StopBits::Half => 1,
                    StopBits::Two => 2,
                    StopBits::OneHalf => 3,
                },
            );
        });
        self.uart.uart_brr.store_reg(|r, v| {
            // Baud rate.
            let (div_man, div_frac) = baud_rate.brr(oversampling);
            r.div_mantissa().write(v, div_man);
            r.div_fraction().write(v, div_frac);
        });

        self.uart.uart_cr1.modify_reg(|r, v| {
            // Enable parity error interrupt
            r.peie().set(v);
            // Enable ORE or RXNE interrupt
            r.rxneie().set(v);
            // Enable uart after being fully configured.
            r.ue().set(v);
        });

        // Attach uart error handler
        let sr = self.uart.uart_sr;
        self.uart_int.add_fn(move || {
            let val = sr.load_val();
            handle_uart_err::<Uart>(&val, sr);
            fib::Yielded::<(), !>(())
        });
    }
}

fn handle_uart_err<Uart: UartMap>(val: &Uart::UartSrVal, sr: Uart::CUartSr) {
    if sr.rxne().read(&val) {
        panic!("Read data register not empty");
    }
    if sr.ore().read(&val) {
        panic!("Overrun error");
    }
    if sr.nf().read(&val) {
        panic!("Noice");
    }
    if sr.fe().read(&val) {
        panic!("Framing error");
    }
    if sr.pe().read(&val) {
        panic!("Parity error");
    }
}

pub(crate) fn handle_dma_err<T: DmaChMap>(
    val: &T::DmaIsrVal,
    dma_isr_dmeif: T::CDmaIsrDmeif,
    dma_isr_feif: T::CDmaIsrFeif,
    dma_isr_teif: T::CDmaIsrTeif,
) {
    if dma_isr_teif.read(&val) {
        panic!("Transfer error");
    }
    if dma_isr_dmeif.read(&val) {
        panic!("Direct mode error");
    }
    if dma_isr_feif.read(&val) {
        panic!("FIFO error");
    }
}
