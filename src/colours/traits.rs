pub trait HasValue<T> {
    fn value(&self) -> T;
}
pub trait HasResetter {
    fn resetter(&self) -> Self;
}

pub trait ANSIApply: HasValue<u8>+HasResetter
where   Self: Sized {
    fn apply<T>(&self, s: &T) -> String
    where T: ToString {
        
        format!(
            "\x1b[{}m",
            self.value()
        )
        + &s.to_string()
        + &format!(
            "\x1b[{}m",
            self.resetter().value()
        )
    }
}