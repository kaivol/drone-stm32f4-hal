//! The threads.

pub use drone_cortexm::thr::{init, init_extended};
pub use drone_stm32_map::thr::*;

use drone_cortexm::thr;

thr! {
    /// The thread data.
    thread => pub Thr {};

    /// The thread-local storage.
    local => pub ThrLocal {};

    /// The vector table type.
    vtable => pub Vtable;

    /// A set of thread tokens.
    index => pub Thrs;

    /// Threads initialization token.
    init => pub ThrsInit;

    threads => {
        exceptions => {
            /// All classes of faults.
            pub hard_fault;
        };
        interrupts => {
            5: pub rcc;
            14: pub dma1_ch3; // USART3_TX: DMA1, stream 3, channel 4.
            // TODO: USART3_RX: DMA1, stream 1, channel 4.
            39: pub usart3;
        }
    };
}
