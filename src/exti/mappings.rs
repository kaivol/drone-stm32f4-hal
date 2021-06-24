use crate::{line::HeadNum, exti_line};
use drone_stm32_map::periph::{exti::*, gpio::head::*, gpio::pin::*};

macro_rules! port {
    ($head:ident, $num:expr, ($($exti:ident, $pin:ident;)+)) => {
        impl HeadNum for $head {
            const NUM: u32 = $num;
        }
        $(exti_line!($exti, $head, $pin;);)+
    }
}

impl crate::ExtiMap for Exti0 {}
impl crate::ExtiMap for Exti1 {}
impl crate::ExtiMap for Exti2 {}
impl crate::ExtiMap for Exti3 {}
impl crate::ExtiMap for Exti4 {}
impl crate::ExtiMap for Exti5 {}
impl crate::ExtiMap for Exti6 {}
impl crate::ExtiMap for Exti7 {}
impl crate::ExtiMap for Exti8 {}
impl crate::ExtiMap for Exti9 {}
impl crate::ExtiMap for Exti10 {}
impl crate::ExtiMap for Exti11 {}
impl crate::ExtiMap for Exti12 {}
impl crate::ExtiMap for Exti13 {}
impl crate::ExtiMap for Exti14 {}
impl crate::ExtiMap for Exti15 {}

port!(GpioAHead, 0,
    (
        Exti0, GpioA0;
        Exti1, GpioA1;
        Exti2, GpioA2;
        Exti3, GpioA3;
        Exti4, GpioA4;
        Exti5, GpioA5;
        Exti6, GpioA6;
        Exti7, GpioA7;
        Exti8, GpioA8;
        Exti9, GpioA9;
        Exti10, GpioA10;
        Exti11, GpioA11;
        Exti12, GpioA12;
        Exti13, GpioA13;
        Exti14, GpioA14;
        Exti15, GpioA15;
    )
);

port!(GpioBHead, 1,
    (
        Exti0, GpioB0;
        Exti1, GpioB1;
        Exti2, GpioB2;
        Exti3, GpioB3;
        Exti4, GpioB4;
        Exti5, GpioB5;
        Exti6, GpioB6;
        Exti7, GpioB7;
        Exti8, GpioB8;
        Exti9, GpioB9;
        Exti10, GpioB10;
        Exti11, GpioB11;
        Exti12, GpioB12;
        Exti13, GpioB13;
        Exti14, GpioB14;
        Exti15, GpioB15;
    )
);

port!(GpioCHead, 2,
    (
        Exti0, GpioC0;
        Exti1, GpioC1;
        Exti2, GpioC2;
        Exti3, GpioC3;
        Exti4, GpioC4;
        Exti5, GpioC5;
        Exti6, GpioC6;
        Exti7, GpioC7;
        Exti8, GpioC8;
        Exti9, GpioC9;
        Exti10, GpioC10;
        Exti11, GpioC11;
        Exti12, GpioC12;
        Exti13, GpioC13;
        Exti14, GpioC14;
        Exti15, GpioC15;
    )
);

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f411",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
port!(GpioDHead, 3,
    (
        Exti0, GpioD0;
        Exti1, GpioD1;
        Exti2, GpioD2;
        Exti3, GpioD3;
        Exti4, GpioD4;
        Exti5, GpioD5;
        Exti6, GpioD6;
        Exti7, GpioD7;
        Exti8, GpioD8;
        Exti9, GpioD9;
        Exti10, GpioD10;
        Exti11, GpioD11;
        Exti12, GpioD12;
        Exti13, GpioD13;
        Exti14, GpioD14;
        Exti15, GpioD15;
    )
);

#[cfg(any(
    stm32_mcu = "stm32f401",
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f411",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
port!(GpioEHead, 4,
    (
        Exti0, GpioE0;
        Exti1, GpioE1;
        Exti2, GpioE2;
        Exti3, GpioE3;
        Exti4, GpioE4;
        Exti5, GpioE5;
        Exti6, GpioE6;
        Exti7, GpioE7;
        Exti8, GpioE8;
        Exti9, GpioE9;
        Exti10, GpioE10;
        Exti11, GpioE11;
        Exti12, GpioE12;
        Exti13, GpioE13;
        Exti14, GpioE14;
        Exti15, GpioE15;
    )
);

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
port!(GpioFHead, 5,
    (
        Exti0, GpioF0;
        Exti1, GpioF1;
        Exti2, GpioF2;
        Exti3, GpioF3;
        Exti4, GpioF4;
        Exti5, GpioF5;
        Exti6, GpioF6;
        Exti7, GpioF7;
        Exti8, GpioF8;
        Exti9, GpioF9;
        Exti10, GpioF10;
        Exti11, GpioF11;
        Exti12, GpioF12;
        Exti13, GpioF13;
        Exti14, GpioF14;
        Exti15, GpioF15;
    )
);

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f412",
    stm32_mcu = "stm32f413",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f446",
    stm32_mcu = "stm32f469",
))]
port!(GpioGHead, 6,
    (
        Exti0, GpioG0;
        Exti1, GpioG1;
        Exti2, GpioG2;
        Exti3, GpioG3;
        Exti4, GpioG4;
        Exti5, GpioG5;
        Exti6, GpioG6;
        Exti7, GpioG7;
        Exti8, GpioG8;
        Exti9, GpioG9;
        Exti10, GpioG10;
        Exti11, GpioG11;
        Exti12, GpioG12;
        Exti13, GpioG13;
        Exti14, GpioG14;
        Exti15, GpioG15;
    )
);

port!(GpioHHead, 7,
    (
        Exti0, GpioH0;
        Exti1, GpioH1;
        Exti2, GpioH2;
        Exti3, GpioH3;
        Exti4, GpioH4;
        Exti5, GpioH5;
        Exti6, GpioH6;
        Exti7, GpioH7;
        Exti8, GpioH8;
        Exti9, GpioH9;
        Exti10, GpioH10;
        Exti11, GpioH11;
        Exti12, GpioH12;
        Exti13, GpioH13;
        Exti14, GpioH14;
        Exti15, GpioH15;
    )
);

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
port!(GpioIHead, 8,
    (
        Exti0, GpioI0;
        Exti1, GpioI1;
        Exti2, GpioI2;
        Exti3, GpioI3;
        Exti4, GpioI4;
        Exti5, GpioI5;
        Exti6, GpioI6;
        Exti7, GpioI7;
        Exti8, GpioI8;
        Exti9, GpioI9;
        Exti10, GpioI10;
        Exti11, GpioI11;
        Exti12, GpioI12;
        Exti13, GpioI13;
        Exti14, GpioI14;
        Exti15, GpioI15;
    )
);

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
port!(GpioJHead, 9,
    (
        Exti0, GpioJ0;
        Exti1, GpioJ1;
        Exti2, GpioJ2;
        Exti3, GpioJ3;
        Exti4, GpioJ4;
        Exti5, GpioJ5;
        Exti6, GpioJ6;
        Exti7, GpioJ7;
        Exti8, GpioJ8;
        Exti9, GpioJ9;
        Exti10, GpioJ10;
        Exti11, GpioJ11;
        Exti12, GpioJ12;
        Exti13, GpioJ13;
        Exti14, GpioJ14;
        Exti15, GpioJ15;
    )
);

#[cfg(any(
    stm32_mcu = "stm32f405",
    stm32_mcu = "stm32f407",
    stm32_mcu = "stm32f427",
    stm32_mcu = "stm32f429",
    stm32_mcu = "stm32f469",
))]
port!(GpioKHead, 10,
    (
        Exti0, GpioK0;
        Exti1, GpioK1;
        Exti2, GpioK2;
        Exti3, GpioK3;
        Exti4, GpioK4;
        Exti5, GpioK5;
        Exti6, GpioK6;
        Exti7, GpioK7;
        Exti8, GpioK8;
        Exti9, GpioK9;
        Exti10, GpioK10;
        Exti11, GpioK11;
        Exti12, GpioK12;
        Exti13, GpioK13;
        Exti14, GpioK14;
        Exti15, GpioK15;
    )
);
