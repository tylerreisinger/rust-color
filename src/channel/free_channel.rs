use std::fmt;
use approx;
use channel::{FreeChannelScalar, ColorChannel, ChannelCast, ChannelFormatCast};
use color::Lerp;

pub struct FreeChannelTag;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct FreeChannel<T>(pub T);

impl<T> ColorChannel for FreeChannel<T>
    where T: FreeChannelScalar
{
    type Format = T;
    type Scalar = T;
    type Tag = FreeChannelTag;

    fn min_bound() -> T {
        T::min_value()
    }

    fn max_bound() -> T {
        T::min_value()
    }

    fn value(&self) -> T {
        self.0.clone()
    }

    impl_channel_clamp!(FreeChannel, T);

    fn scalar(&self) -> T {
        self.0.clone()
    }

    fn from_scalar(value: T) -> Self {
        FreeChannel(value)
    }

    fn new(value: T) -> Self {
        FreeChannel(value)
    }
}

impl<T> Default for FreeChannel<T>
    where T: FreeChannelScalar + Default
{
    fn default() -> Self {
        FreeChannel(T::default())
    }
}

impl<T> Lerp for FreeChannel<T>
    where T: FreeChannelScalar + Lerp
{
    type Position = <T as Lerp>::Position;
    fn lerp(&self, right: &FreeChannel<T>, pos: Self::Position) -> Self {
        FreeChannel::new(self.0.lerp(&right.0, pos))
    }
}

impl<T> ChannelCast for FreeChannel<T>
    where T: FreeChannelScalar
{
    fn channel_cast<To>(self) -> To
        where Self::Format: ChannelFormatCast<To::Format>,
              To: ColorChannel
    {
        To::new(self.0.cast())
    }
    fn scalar_cast<To>(self) -> To
        where Self::Format: ChannelFormatCast<To>
    {
        self.0.cast()
    }
}

impl<T> fmt::Display for FreeChannel<T>
    where T: FreeChannelScalar + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> approx::ApproxEq for FreeChannel<T>
    where T: FreeChannelScalar + approx::ApproxEq
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }

    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    fn relative_eq(&self,
                   other: &Self,
                   epsilon: Self::Epsilon,
                   max_relative: Self::Epsilon)
                   -> bool {
        self.0.relative_eq(&other.0, epsilon, max_relative)
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.0.ulps_eq(&other.0, epsilon, max_ulps)
    }
}