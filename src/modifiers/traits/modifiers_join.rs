use super::super::ANSIModifiers;
use super::{HasResetter, HasValue};

pub trait Modifier: HasValue<String> + HasResetter + Sized {}
impl<U> Modifier for U where U: HasValue<String> + HasResetter + Sized {}

pub trait Wrapper {
    fn wraps<T>(&self, s: &T) -> String
    where
        T: ToString;
}
pub trait JointModifier {
    type Output;

    fn join<T>(self, rhs: T) -> Self::Output
    where
        T: HasValue<String> + HasResetter;
}

impl<U> Wrapper for U
where
    U: Modifier,
{
    fn wraps<T>(&self, s: &T) -> String
    where
        T: ToString,
    {
        self.value() + &s.to_string() + &self.resetter().value()
    }
}

impl<U> JointModifier for U
where
    U: HasValue<String> + HasResetter,
{
    type Output = ANSIModifiers;

    fn join<T>(self, rhs: T) -> Self::Output
    where
        T: HasValue<String> + HasResetter,
    {
        ANSIModifiers::from(self).join(rhs)
    }
}
