use alloc::rc::Rc;
use core::marker::PhantomData;
use drone_cortexm::reg::prelude::*;
use drone_stm32_map::periph::gpio::{
    head::GpioHeadMap,
    pin::{GpioPinMap, GpioPinPeriph},
};

/// Pin configuration.
pub struct GpioPin<Pin: GpioPinMap, Mode, Type, Pull> {
    pub(crate) pin: Rc<GpioPinPeriph<Pin>>,
    mode: PhantomData<Mode>,
    type_: PhantomData<Type>,
    pull: PhantomData<Pull>,
}

impl<Pin: GpioPinMap, Mode, Type, Pull>
    From<Rc<GpioPinPeriph<Pin>>> for GpioPin<Pin, Mode, Type, Pull>
{
    fn from(pin: Rc<GpioPinPeriph<Pin>>) -> Self {
        Self {
            pin,
            mode: PhantomData,
            type_: PhantomData,
            pull: PhantomData,
        }
    }
}

/// Generic dont-care mode for undefined state.
pub struct DontCare;

/// General purpose input mode (MODER=0b00).
pub struct InputMode;

/// General purpose output mode  (MODER=0b01).
pub struct OutputMode;

/// Alternate function mode  (MODER=0b10).
pub struct AlternateMode<Af: PinAf> {
    af: PhantomData<Af>,
}

// TODO: Analog mode

/// Push/pull type (OTYPER=0).
/// This is only applicabale for OutputMode and AlternateMode.
pub struct PushPullType;

/// Output open-drain type (OTYPER=1).
/// This is only applicabale for OutputMode and AlternateMode.
pub struct OpenDrainType;

/// No pull-up nor pull-down. For inputs this means floating.
pub struct NoPull;

/// Pull up.
pub struct PullUp;

/// Pull down.
pub struct PullDown;

pub struct PinAf0;
pub struct PinAf1;
pub struct PinAf2;
pub struct PinAf3;
pub struct PinAf4;
pub struct PinAf5;
pub struct PinAf6;
pub struct PinAf7;
pub struct PinAf8;
pub struct PinAf9;
pub struct PinAf10;
pub struct PinAf11;
pub struct PinAf12;
pub struct PinAf13;
pub struct PinAf14;
pub struct PinAf15;

pub trait PinAf: Send {
    const NUM: u32;
}

macro_rules! af_token {
    ($af:ident, $num:expr) => {
        impl PinAf for $af {
            const NUM: u32 = $num;
        }
    };
}

af_token!(PinAf0, 0);
af_token!(PinAf1, 1);
af_token!(PinAf2, 2);
af_token!(PinAf3, 3);
af_token!(PinAf4, 4);
af_token!(PinAf5, 5);
af_token!(PinAf6, 6);
af_token!(PinAf7, 7);
af_token!(PinAf8, 8);
af_token!(PinAf9, 9);
af_token!(PinAf10, 10);
af_token!(PinAf11, 11);
af_token!(PinAf12, 12);
af_token!(PinAf13, 13);
af_token!(PinAf14, 14);
af_token!(PinAf15, 15);
af_token!(DontCare, 0);

/// Gpio pin speed.
pub enum GpioPinSpeed {
    LowSpeed,
    MediumSpeed,
    HighSpeed,
    VeryHighSpeed,
}

impl<Pin: GpioPinMap> GpioPin<Pin, DontCare, DontCare, DontCare> {
    /// Set pin into general purpose input mode.
    pub fn into_input(self) -> GpioPin<Pin, InputMode, DontCare, DontCare> {
        self.pin.gpio_moder_moder.write_bits(0b00);
        self.pin.into()
    }

    /// Set pin into general purpose output mode.
    pub fn into_output(self) -> GpioPin<Pin, OutputMode, DontCare, DontCare> {
        self.pin.gpio_moder_moder.write_bits(0b01);
        self.pin.into()
    }

    /// Set the pin into alternate function mode.
    pub fn into_alternate<Af: PinAf>(self) -> GpioPin<Pin, AlternateMode<Af>, DontCare, DontCare> {
        self.pin.gpio_afr_afr.write_bits(Af::NUM);
        self.pin.gpio_moder_moder.write_bits(0b10);
        self.pin.into()
    }
}

pub trait TypeModes {}
impl TypeModes for InputMode {}
impl TypeModes for OutputMode {}
impl<Af: PinAf> TypeModes for AlternateMode<Af> {}

impl<Pin: GpioPinMap, Mode: TypeModes> GpioPin<Pin, Mode, DontCare, DontCare> {
    /// Let pin type be push/pull.
    pub fn into_pushpull(self) -> GpioPin<Pin, Mode, PushPullType, DontCare> {
        self.pin.gpio_otyper_ot.clear_bit();
        self.pin.gpio_pupdr_pupdr.write_bits(0b00); // No pull-up nor pull-down.
        self.pin.into()
    }

    /// Let pin type be open-drain.
    pub fn into_opendrain(self) -> GpioPin<Pin, Mode, OpenDrainType, DontCare> {
        self.pin.gpio_otyper_ot.set_bit();
        self.pin.into()
    }
}

pub trait PullModes {}
impl PullModes for InputMode {}
impl PullModes for OutputMode {}
impl<Af: PinAf> PullModes for AlternateMode<Af> {}

impl<Pin: GpioPinMap, Mode: PullModes>
    GpioPin<Pin, Mode, PushPullType, DontCare>
{
    /// No pull-up nor pull-down.
    pub fn into_nopull(self) -> GpioPin<Pin, Mode, PushPullType, NoPull> {
        self.pin.gpio_pupdr_pupdr.write_bits(0b00);
        self.pin.into()
    }

    /// Let pin be pulled-up.
    pub fn into_pullup(self) -> GpioPin<Pin, Mode, PushPullType, PullUp> {
        self.pin.gpio_pupdr_pupdr.write_bits(0b01);
        self.pin.into()
    }

    /// Let pin be pulled-down.
    pub fn into_pulldown(self) -> GpioPin<Pin, Mode, PushPullType, PullDown> {
        self.pin.gpio_pupdr_pupdr.write_bits(0b10);
        self.pin.into()
    }
}

pub trait WithSpeedModes {}
impl WithSpeedModes for OutputMode {}
impl<Af: PinAf> WithSpeedModes for AlternateMode<Af> {}

impl<
        Pin: GpioPinMap,
        Mode: WithSpeedModes,
        Type,
        Pull,
    > GpioPin<Pin, Mode, Type, Pull>
{
    /// Set pin speed.
    pub fn with_speed(self, speed: GpioPinSpeed) -> Self {
        self.pin.gpio_ospeedr_ospeedr.write_bits(match speed {
            GpioPinSpeed::LowSpeed => 0,
            GpioPinSpeed::MediumSpeed => 1,
            GpioPinSpeed::HighSpeed => 2,
            GpioPinSpeed::VeryHighSpeed => 3,
        });
        self
    }
}

pub trait GetModes {}
impl GetModes for InputMode {}
impl GetModes for OutputMode {}
impl<Af: PinAf> GetModes for AlternateMode<Af> {}

impl<Pin: GpioPinMap, Mode: GetModes, Type, Pull>
    GpioPin<Pin, Mode, Type, Pull>
{
    /// Get the current pin state.
    pub fn get(&self) -> bool {
        self.pin.gpio_idr_idr.read_bit()
    }
}

impl<Pin: GpioPinMap, Type, Pull> GpioPin<Pin, OutputMode, Type, Pull> {
    /// Set output pin high.
    pub fn set(&self) {
        // Set output pin to high by writing BS (bit set) to the bit set/reset register.
        self.pin.gpio_bsrr_bs.set_bit();
    }

    /// Set output pin low.
    pub fn clear(&self) {
        // Clear output pin to low by writing BR (bit reset) to the bit set/reset register.
        self.pin.gpio_bsrr_br.set_bit();
    }
}

impl<Pin: GpioPinMap, Mode, Type, Pull>
    GpioPin<Pin, Mode, Type, Pull>
{
    /// Clone the pin
    ///
    /// # Safety
    /// The function is unsafe as there are no guarantees that the two configuration can co-exist.
    pub unsafe fn clone(&self) -> Self {
        Self {
            pin: self.pin.clone(),
            mode: self.mode,
            type_: self.type_,
            pull: self.pull,
        }
    }
}

pub trait NewPin<Head: GpioHeadMap, Pin: GpioPinMap> {
    /// Create a new pin configuration from a pin peripheral.
    fn pin(&self, pin: GpioPinPeriph<Pin>) -> GpioPin<Pin, DontCare, DontCare, DontCare>;
}

#[macro_export]
macro_rules! pin_init {
    ($($head:ident, $pin:ident;)+) => {
        $(
            impl
                crate::pin::NewPin<
                    $head,
                    $pin,
                > for crate::head::GpioHead<$head>
            {
                fn pin(
                    &self,
                    pin: ::drone_stm32_map::periph::gpio::pin::GpioPinPeriph<
                        $pin,
                    >,
                ) -> crate::pin::GpioPin<
                    $pin,
                    crate::pin::DontCare,
                    crate::pin::DontCare,
                    crate::pin::DontCare,
                > {
                    crate::pin::GpioPin::from(alloc::rc::Rc::new(pin))
                }
            }
        )+
    };
}
