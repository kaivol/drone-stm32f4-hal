use crate::diverged::{DmaChDiverged, UartDiverged};
use drone_cortexm::{fib, reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::{
    dma::ch::{traits::*, DmaChMap, DmaChPeriph},
    uart::{traits::*, UartMap, UartPeriph},
};
use futures::prelude::*;

/// Uart setup.
pub struct UartSetup<Uart: UartMap, UartInt: IntToken, DmaTx: DmaChMap, DmaTxInt: IntToken> {
    /// Uart peripheral.
    pub uart: UartPeriph<Uart>,
    /// Uart global interrupt.
    pub uart_int: UartInt,
    /// Uart baud rate.
    pub uart_baud_rate: u32,
    /// Uart word length in bits.
    ///
    /// Valid values are 8 or 9.
    pub uart_word_length: u8,
    /// Uart stop bits.
    pub uart_stop_bits: UartStop,
    /// Uart parity.
    pub uart_parity: UartParity,
    /// Uart oversampling
    ///
    /// Valid values are 8 or 16.
    pub uart_oversampling: u8,
    /// DMA Tx channel peripheral.
    pub dma_tx: DmaChPeriph<DmaTx>,
    /// DMA Tx channel interrupt.
    pub dma_tx_int: DmaTxInt,
    /// DMA Tx channel number.
    pub dma_tx_ch: u32,
    /// DMA Tx channel priority level.
    pub dma_tx_pl: u32,
}

/// Uart stop bits.
#[derive(Clone, Copy, PartialEq)]
pub enum UartStop {
    Half,
    One,
    OneHalf,
    Two,
}

/// Uart parity.
#[derive(Clone, Copy, PartialEq)]
pub enum UartParity {
    None,
    Even,
    Odd,
}

/// Uart driver.
pub struct UartDrv<Uart: UartMap, UartInt: IntToken, DmaTx: DmaChMap, DmaTxInt: IntToken> {
    uart: UartDiverged<Uart>,
    uart_int: UartInt,
    dma_tx: DmaChDiverged<DmaTx>,
    dma_tx_int: DmaTxInt,
}

impl<Uart: UartMap, UartInt: IntToken, DmaTx: DmaChMap, DmaTxInt: IntToken>
    UartDrv<Uart, UartInt, DmaTx, DmaTxInt>
{
    /// Sets up a new [`UartDrv`] from `setup` values.
    #[must_use]
    pub fn init(setup: UartSetup<Uart, UartInt, DmaTx, DmaTxInt>) -> Self {
        let UartSetup {
            uart,
            uart_int,
            uart_baud_rate,
            uart_word_length,
            uart_stop_bits,
            uart_parity,
            uart_oversampling,
            dma_tx,
            dma_tx_int,
            dma_tx_ch,
            dma_tx_pl,
        } = setup;
        let mut drv = Self {
            uart: uart.into(),
            uart_int,
            dma_tx: dma_tx.into(),
            dma_tx_int,
        };
        drv.init_uart(
            uart_baud_rate,
            uart_word_length,
            uart_stop_bits,
            uart_parity,
            uart_oversampling,
        );
        drv.init_dma_tx(dma_tx_ch, dma_tx_pl);
        // drv.init_dma_rx(dma_rx_ch, dma_rx_pl);
        drv
    }

    pub fn tx(&self) -> TxGuard<Uart, UartInt, DmaTx, DmaTxInt> {
        TxGuard::new(&self.uart, &self.uart_int, &self.dma_tx, &self.dma_tx_int)
    }

    fn init_uart(
        &mut self,
        baud_rate: u32,
        word_length: u8,
        stop_bits: UartStop,
        parity: UartParity,
        oversampling: u8,
    ) {
        // Enable uart clock.
        self.uart.rcc_busenr_uarten.set_bit();

        // Configure uart.
        self.uart.uart_cr1.store_reg(|r, v| {
            // Do not enable uart before it is fully configured.

            // Word length.
            if word_length == 9 {
                r.m().set(v);
            }

            // Parity.
            if parity != UartParity::None {
                // Enable parity.
                r.pce().set(v);
                if parity == UartParity::Odd {
                    // Parity selection: odd.
                    r.ps().set(v);
                }
            }

            // Oversampling.
            if oversampling == 8 {
                r.over8().set(v);
            }
        });
        self.uart.uart_cr2.store_reg(|r, v| {
            // Stop bits.
            r.stop().write(
                v,
                match stop_bits {
                    UartStop::One => 0,
                    UartStop::Half => 1,
                    UartStop::Two => 2,
                    UartStop::OneHalf => 3,
                },
            );
        });
        self.uart.uart_brr.store_reg(|r, v| {
            // Baud rate.
            // TODO: How to obtain correct clock instead of using hardcoded value?
            let (div_man, div_frac) = compute_brr(90_000_000, oversampling == 8, baud_rate);
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

    fn init_dma_tx(&mut self, channel: u32, priority: u32) {
        let address = self.uart.uart_dr.as_mut_ptr(); // 8-bit data register
        self.dma_tx.dma_cpar.store_reg(|r, v| {
            r.pa().write(v, address as u32); // peripheral address
        });
        self.dma_tx.dma_ccr.store_reg(|r, v| {
            r.chsel().write(v, channel); // channel selection
            r.pl().write(v, priority); // priority level
            r.msize().write(v, 0b00); // byte (8-bit)
            r.psize().write(v, 0b00); // byte (8-bit)
            r.minc().set(v); // memory address pointer is incremented after each data transfer
            r.pinc().clear(v); // peripheral address pointer is fixed
            r.dir().write(v, 0b01); // memory-to-peripheral
            r.tcie().set(v); // transfer complete interrupt enable
            r.teie().set(v); // transfer error interrupt enable
        });

        // Attach dma error handler
        let dma_isr_dmeif = self.dma_tx.dma_isr_dmeif;
        let dma_isr_feif = self.dma_tx.dma_isr_feif;
        let dma_isr_teif = self.dma_tx.dma_isr_teif;
        self.dma_tx_int.add_fn(move || {
            // Load _entire_ interrupt status register.
            // The value is not masked to TEIF.
            let val = dma_isr_teif.load_val();
            handle_dma_err::<DmaTx>(&val, dma_isr_dmeif, dma_isr_feif, dma_isr_teif);
            fib::Yielded::<(), !>(())
        });
    }
}

pub struct TxGuard<'a, Uart: UartMap, UartInt: IntToken, DmaTx: DmaChMap, DmaTxInt: IntToken> {
    uart: &'a UartDiverged<Uart>,
    uart_int: &'a UartInt,
    dma_tx: &'a DmaChDiverged<DmaTx>,
    dma_tx_int: &'a DmaTxInt,
}

impl<'a, Uart: UartMap, UartInt: IntToken, DmaTx: DmaChMap, DmaTxInt: IntToken>
    TxGuard<'a, Uart, UartInt, DmaTx, DmaTxInt>
{
    fn new(
        uart: &'a UartDiverged<Uart>,
        uart_int: &'a UartInt,
        dma_tx: &'a DmaChDiverged<DmaTx>,
        dma_tx_int: &'a DmaTxInt,
    ) -> Self {
        // Enable transmitter
        uart.uart_cr1.modify_reg(|r, v| {
            r.te().set(v);
        });

        Self {
            uart,
            uart_int,
            dma_tx,
            dma_tx_int,
        }
    }

    /// Write a buffer using DMA to the uart peripheral.
    ///
    /// The write future completes when the DMA transfer has completed, at which time the peripheral is ready for another invokation of write().
    pub async fn write(&mut self, buf: Box<[u8]>) {
        unsafe {
            self.write_unsafe(&buf).await;
        }
    }

    unsafe fn write_unsafe(&mut self, buf: &[u8]) -> impl Future<Output = ()> {
        // PE (Parity error),
        // FE (Framing error),
        // NE (Noise error),
        // ORE (Overrun error), and
        // IDLE (Idle line detected) flags are cleared by the software sequence:
        // 1. a read operation to USART_SR register, followed by
        // 2. a read operation to USART_DR register.
        // See RM0090 page 972.
        self.uart.uart_sr.load_val();
        self.uart.uart_dr.load_val();

        // Setup DMA transfer parameters.
        self.setup_dma(buf);

        // Start listen for DMA transfer to complete.
        // The transfer completes just after the second last byte is being sent on the wire.
        let dma_isr_tcif = self.dma_tx.dma_isr_tcif;
        let dma_ifcr_ctcif = self.dma_tx.dma_ifcr_ctcif;
        let future = self.dma_tx_int.add_future(fib::new_fn(move || {
            if dma_isr_tcif.read_bit() {
                // Clear transfer completed interrupt flag.
                dma_ifcr_ctcif.set_bit();
                fib::Complete(())
            } else {
                fib::Yielded(())
            }
        }));

        // The uart transmission complete flag (TC) is cleared
        // by the sequence: Read status register (SR) and write data register (DR).
        // We read the status register here, and the dma writes the DR.
        self.uart.uart_sr.load_val();

        // Start transfer on DMA channel.
        self.uart.uart_cr3.modify_reg(|r, v| {
            r.dmat().set(v);
        });

        // Wait for DMA transfer to complete.
        future

        // The peripheral automatically disables the DMA stream on completion without error.
    }

    /// Wait for the uart peripheral to actually complete the transfer.
    pub async fn flush(&mut self) {
        // Wait for
        // 1) transmit buffer to become empty (TXE), and
        // 2) transmission to complete (TC).

        // Setup transmission complete interrupt.
        self.uart.uart_cr1.modify_reg(|r, v| {
            // Enable transmission-complete interrupt.
            r.tcie().set(v);
        });

        let uart_sr = self.uart.uart_sr;
        let future = self.uart_int.add_future(fib::new_fn(move || {
            let sr_val = uart_sr.load_val();
            if uart_sr.txe().read(&sr_val) && uart_sr.tc().read(&sr_val) {
                // TXE is cleared by hardware before next write()
                // TC is also cleared by hardware before next write()
                fib::Complete(())
            } else {
                fib::Yielded(())
            }
        }));

        let sr_val = uart_sr.load_val();
        if uart_sr.txe().read(&sr_val) && uart_sr.tc().read(&sr_val) {
            return;
        }

        future.await;
    }

    unsafe fn setup_dma(&mut self, buf_tx: &[u8]) {
        // Set buffer memory addres
        self.dma_tx.dma_cm0ar.store_reg(|r, v| {
            r.m0a().write(v, buf_tx.as_ptr() as u32);
        });

        // Set number of bytes to transfer
        self.dma_tx.dma_cndtr.store_reg(|r, v| {
            r.ndt().write(v, buf_tx.len() as u32);
        });

        // Clear transfer complete interrupt flag
        self.dma_tx.dma_ifcr_ctcif.set_bit();

        // Enable stream
        self.dma_tx.dma_ccr.modify_reg(|r, v| r.en().set(v));
    }
}

impl<Uart: UartMap, UartInt: IntToken, DmaTx: DmaChMap, DmaTxInt: IntToken> Drop
    for TxGuard<'_, Uart, UartInt, DmaTx, DmaTxInt>
{
    /// Stop the transmitter.
    ///
    /// It is preferred that flush() is called before drop so that this will not actually block until transmission completes.
    fn drop(&mut self) {
        // Wait for 1) transmit buffer to become empty (TXE), and 2) for transmission to complete (TC).
        let uart_sr = self.uart.uart_sr;
        loop {
            let sr_val = uart_sr.load_val();
            if uart_sr.txe().read(&sr_val) && uart_sr.tc().read(&sr_val) {
                break;
            }
        }

        // Disable transmitter
        self.uart.uart_cr1.modify_reg(|r, v| {
            r.te().clear(v);
        });
    }
}

fn compute_brr(clock: u32, over8: bool, baud_rate: u32) -> (u32, u32) {
    // see PM0090 §30.3.4 Fractional baud rate generation page 978
    let over8 = over8 as u32;
    // (25*clock) / 2*(2-over8)*baud_rate) === (100*clock) / (8*(2-over8)*baud_rate).
    // But we take the 25 version to ensure that 25 * clock can fit in a u32.
    let div100 = (25 * clock) / (2 * (2 - over8) * baud_rate);
    let div_man = div100 / 100;
    let div_frac = if over8 == 1 {
        // The frac field has 3 bits, 0..15
        ((div100 - div_man * 100) * 16 + 50) / 100
    } else {
        // The frac field has 4 bits
        ((div100 - div_man * 100) * 32 + 50) / 100
    };

    (div_man, div_frac)
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

fn handle_dma_err<T: DmaChMap>(
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