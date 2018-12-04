//! A collection of traits implemented by the various color types

use num_traits;

/// The base color trait, representing any color
///
/// The main use of `Color` is to manipulate and act on colors in a generic context. It is used
/// extensively internally.
pub trait Color: Clone + PartialEq {
    /// The unique tag unit struct identifying the color type
    type Tag;
    /// A tuple of types for each channel in the color
    type ChannelsTuple;

    /// Return how many channels the color has
    fn num_channels() -> u32;
    /// Convert a color into a tuple of channels
    fn to_tuple(self) -> Self::ChannelsTuple;
}

/// A trait for colors that can be constructed from a tuple of channels
pub trait FromTuple: Color {
    /// Construct `Self` from a tuple of channel values
    fn from_tuple(values: Self::ChannelsTuple) -> Self;
}

/// A color that has a angular hue channel
pub trait PolarColor: Color {
    /// The angular channel's scalar type
    type Angular;
    /// The remaining channels' scalar types
    type Cartesian;
}

// TODO: This should inherit HomogeneousColor and remove ScalarFormat
/// A color that can be represented as a slice without any conversion
pub trait Flatten: Color {
    /// The scalar type of each channel
    type ScalarFormat;
    /// Return `Self` constructed from `values`
    fn from_slice(values: &[Self::ScalarFormat]) -> Self;
    /// Return a slice representation of `Self`
    fn as_slice(&self) -> &[Self::ScalarFormat];
}

// TODO: Move broadcast to its own trait, and implement this for the various wrappers that take extra args
/// A color only having one type of channel
pub trait HomogeneousColor: Color {
    /// The scalar type of each channel
    type ChannelFormat;

    /// Construct `Self` with each channel set to `value`
    fn broadcast(value: Self::ChannelFormat) -> Self;
    /// Clamp the value of each channel between `min` and `max`
    fn clamp(self, min: Self::ChannelFormat, max: Self::ChannelFormat) -> Self;
}

/// A color with three channels
pub trait Color3: Color {}
/// A color with four channels
pub trait Color4: Color {}

/// A value that can be linearly interpolated between two values
pub trait Lerp {
    /// The type of the `pos` argument
    type Position: num_traits::Float;
    /// Interpolate between `self` and `right`
    ///
    /// `pos` specifies how far between the two values to interpolate on a line, between zero and one.
    ///
    /// $`pos = 0`$ would return `self` while $`pos = 1`$ would return right.
    fn lerp(&self, right: &Self, pos: Self::Position) -> Self;
}

/// A value that can be inverted
pub trait Invert {
    /// Invert `Self`
    fn invert(self) -> Self;
}

/// A value that is constrained between a minimum and maximum in its "normal" form
pub trait Bounded {
    /// Return a value clipped inside the normalized range
    fn normalize(self) -> Self;
    /// Return true if the value is normalized
    fn is_normalized(&self) -> bool;
}

/// A color which must have a space specified in order to uniquely represent a color
pub trait DeviceDependentColor: Color {}
