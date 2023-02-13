use thiserror::Error;

/// A enum of possible error types during modifier operations.
#[derive(Error, Debug)]
pub enum ModifierError {
    #[error("{0:?} is not an ANSI modifier.")]
    ValueIsNotAModifier(String),

    #[error("{1:?} is not the correct ANSI code for {0}; {2} expected.")]
    MismatchedANSICode(String, u8, u8),

    #[error("{1:?} is not a recognised pattern for {0}.")]
    ValueNotRecognised(String, String),

    #[error("{1:?} is not the correct end character for {0}.")]
    UnexpectedEndCharacter(String, String),

    #[error("codes {1:?} do not represent an existing {0} variant.")]
    VariantNotFound(String, Vec<u8>),

    #[error("bad Captures group passed to {0}.")]
    BadCapturesGroup(String),
}
