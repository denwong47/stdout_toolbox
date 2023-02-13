use super::super::structs::ANSIEscapeCode;

pub trait IntoANSIEscapeCode {
    fn into_ansi_escape_code(&self) -> ANSIEscapeCode;
}
