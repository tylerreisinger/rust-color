use color;
use color::Lerp;
use angle;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AngularChannel<T>(pub T);

impl<T> AngularChannel<T>
    where T: angle::Angle
{
    pub fn new(val: T) -> Self {
        AngularChannel(val)
    }
}

impl<T> color::Invert for AngularChannel<T>
    where T: angle::Angle
{
    fn invert(self) -> Self {
        AngularChannel(self.0.invert())
    }
}

impl<T> Lerp for AngularChannel<T>
    where T: angle::Angle + Lerp
{
    type Position = T::Position;
    fn lerp(&self, right: &Self, pos: Self::Position) -> Self {
        AngularChannel(self.0.lerp(&right.0, pos))
    }
}

impl<T> color::Bounded for AngularChannel<T>
    where T: angle::Angle
{
    fn normalize(self) -> Self {
        AngularChannel(<T as angle::Angle>::normalize(&self.0))
    }
    fn is_normalized(&self) -> bool {
        <T as angle::Angle>::is_normalized(&self.0)
    }
}