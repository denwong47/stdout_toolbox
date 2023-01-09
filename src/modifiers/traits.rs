use super::ANSIModifiers;

pub trait HasValue<T> {
    fn value(&self) -> T;
}
impl<U> HasValue<String> for U
where U: HasValue<u8> {
    fn value(&self) -> String {
        format!(
            "\x1b[{}m",
            self.value()
        )
    }
}


pub trait HasResetter {
    fn resetter(&self) -> Self;
}

pub trait Modifier {
    fn wraps<T>(&self, s: &T) -> String
    where T: ToString;
}
pub trait JointModifier {
    type Output;

    fn join<T>(self, rhs: T) -> Self::Output
    where T: HasValue<String>+HasResetter;
}

impl<U> Modifier for U
where U: HasValue<String>+HasResetter+Sized {
    fn wraps<T>(&self, s: &T) -> String
    where T: ToString {
        
        self.value()
        + &s.to_string()
        + &self.resetter().value()
    }
}

impl<U> JointModifier for U
where U: HasValue<String>+HasResetter {
    type Output = ANSIModifiers;

    fn join<T>(self, rhs: T) -> Self::Output
    where T: HasValue<String>+HasResetter {
        ANSIModifiers::from(self).join(rhs)
    }
}