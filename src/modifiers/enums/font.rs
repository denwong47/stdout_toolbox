use std::fmt::Display;
use strum_macros::EnumIter;

use super::super::{HasResetter, HasValue};
use duplicate::duplicate_item;

#[allow(dead_code)]
#[derive(EnumIter, Debug, PartialEq)]
pub enum Intensity {
    Normal,
    Bold,
    Faint,
}
#[allow(dead_code)]
impl HasValue<u8> for Intensity {
    fn value(&self) -> u8 {
        match self {
            Self::Normal => 22,
            Self::Bold => 1,
            Self::Faint => 2,
        }
    }
}
#[allow(dead_code)]
impl HasResetter for Intensity {
    fn resetter(&self) -> Self {
        Self::Normal
    }
}
impl Display for Intensity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", HasValue::<String>::value(self))
    }
}

// =====================================================================================

#[duplicate_item(
    __enum_name__;
    [ ForegroundColours ];
    [ BackgroundColours ];
)]
#[derive(EnumIter, Debug, PartialEq)]
#[allow(dead_code)]
pub enum __enum_name__ {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    R0G0B0,
    R0G0B1,
    R0G0B2,
    R0G0B3,
    R0G0B4,
    R0G0B5,
    R0G1B0,
    R0G1B1,
    R0G1B2,
    R0G1B3,
    R0G1B4,
    R0G1B5,
    R0G2B0,
    R0G2B1,
    R0G2B2,
    R0G2B3,
    R0G2B4,
    R0G2B5,
    R0G3B0,
    R0G3B1,
    R0G3B2,
    R0G3B3,
    R0G3B4,
    R0G3B5,
    R0G4B0,
    R0G4B1,
    R0G4B2,
    R0G4B3,
    R0G4B4,
    R0G4B5,
    R0G5B0,
    R0G5B1,
    R0G5B2,
    R0G5B3,
    R0G5B4,
    R0G5B5,
    R1G0B0,
    R1G0B1,
    R1G0B2,
    R1G0B3,
    R1G0B4,
    R1G0B5,
    R1G1B0,
    R1G1B1,
    R1G1B2,
    R1G1B3,
    R1G1B4,
    R1G1B5,
    R1G2B0,
    R1G2B1,
    R1G2B2,
    R1G2B3,
    R1G2B4,
    R1G2B5,
    R1G3B0,
    R1G3B1,
    R1G3B2,
    R1G3B3,
    R1G3B4,
    R1G3B5,
    R1G4B0,
    R1G4B1,
    R1G4B2,
    R1G4B3,
    R1G4B4,
    R1G4B5,
    R1G5B0,
    R1G5B1,
    R1G5B2,
    R1G5B3,
    R1G5B4,
    R1G5B5,
    R2G0B0,
    R2G0B1,
    R2G0B2,
    R2G0B3,
    R2G0B4,
    R2G0B5,
    R2G1B0,
    R2G1B1,
    R2G1B2,
    R2G1B3,
    R2G1B4,
    R2G1B5,
    R2G2B0,
    R2G2B1,
    R2G2B2,
    R2G2B3,
    R2G2B4,
    R2G2B5,
    R2G3B0,
    R2G3B1,
    R2G3B2,
    R2G3B3,
    R2G3B4,
    R2G3B5,
    R2G4B0,
    R2G4B1,
    R2G4B2,
    R2G4B3,
    R2G4B4,
    R2G4B5,
    R2G5B0,
    R2G5B1,
    R2G5B2,
    R2G5B3,
    R2G5B4,
    R2G5B5,
    R3G0B0,
    R3G0B1,
    R3G0B2,
    R3G0B3,
    R3G0B4,
    R3G0B5,
    R3G1B0,
    R3G1B1,
    R3G1B2,
    R3G1B3,
    R3G1B4,
    R3G1B5,
    R3G2B0,
    R3G2B1,
    R3G2B2,
    R3G2B3,
    R3G2B4,
    R3G2B5,
    R3G3B0,
    R3G3B1,
    R3G3B2,
    R3G3B3,
    R3G3B4,
    R3G3B5,
    R3G4B0,
    R3G4B1,
    R3G4B2,
    R3G4B3,
    R3G4B4,
    R3G4B5,
    R3G5B0,
    R3G5B1,
    R3G5B2,
    R3G5B3,
    R3G5B4,
    R3G5B5,
    R4G0B0,
    R4G0B1,
    R4G0B2,
    R4G0B3,
    R4G0B4,
    R4G0B5,
    R4G1B0,
    R4G1B1,
    R4G1B2,
    R4G1B3,
    R4G1B4,
    R4G1B5,
    R4G2B0,
    R4G2B1,
    R4G2B2,
    R4G2B3,
    R4G2B4,
    R4G2B5,
    R4G3B0,
    R4G3B1,
    R4G3B2,
    R4G3B3,
    R4G3B4,
    R4G3B5,
    R4G4B0,
    R4G4B1,
    R4G4B2,
    R4G4B3,
    R4G4B4,
    R4G4B5,
    R4G5B0,
    R4G5B1,
    R4G5B2,
    R4G5B3,
    R4G5B4,
    R4G5B5,
    R5G0B0,
    R5G0B1,
    R5G0B2,
    R5G0B3,
    R5G0B4,
    R5G0B5,
    R5G1B0,
    R5G1B1,
    R5G1B2,
    R5G1B3,
    R5G1B4,
    R5G1B5,
    R5G2B0,
    R5G2B1,
    R5G2B2,
    R5G2B3,
    R5G2B4,
    R5G2B5,
    R5G3B0,
    R5G3B1,
    R5G3B2,
    R5G3B3,
    R5G3B4,
    R5G3B5,
    R5G4B0,
    R5G4B1,
    R5G4B2,
    R5G4B3,
    R5G4B4,
    R5G4B5,
    R5G5B0,
    R5G5B1,
    R5G5B2,
    R5G5B3,
    R5G5B4,
    R5G5B5,
    Grayscale00,
    Grayscale01,
    Grayscale02,
    Grayscale03,
    Grayscale04,
    Grayscale05,
    Grayscale06,
    Grayscale07,
    Grayscale08,
    Grayscale09,
    Grayscale10,
    Grayscale11,
    Grayscale12,
    Grayscale13,
    Grayscale14,
    Grayscale15,
    Grayscale16,
    Grayscale17,
    Grayscale18,
    Grayscale19,
    Grayscale20,
    Grayscale21,
    Grayscale22,
    Grayscale23,

    Reset,
}

#[duplicate_item(
    __enum_name__           __apply__           __reset__;
    [ ForegroundColours ]   [ "\x1b[38:5:{}m" ] [ "\x1b[39m" ];
    [ BackgroundColours ]   [ "\x1b[48:5:{}m" ] [ "\x1b[49m" ];
)]
#[allow(dead_code)]
impl __enum_name__ {
    pub fn iter_grayscale() -> impl Iterator<Item = Self> {
        vec![
            Self::Grayscale00,
            Self::Grayscale01,
            Self::Grayscale02,
            Self::Grayscale03,
            Self::Grayscale04,
            Self::Grayscale05,
            Self::Grayscale06,
            Self::Grayscale07,
            Self::Grayscale08,
            Self::Grayscale09,
            Self::Grayscale10,
            Self::Grayscale11,
            Self::Grayscale12,
            Self::Grayscale13,
            Self::Grayscale14,
            Self::Grayscale15,
            Self::Grayscale16,
            Self::Grayscale17,
            Self::Grayscale18,
            Self::Grayscale19,
            Self::Grayscale20,
            Self::Grayscale21,
            Self::Grayscale22,
            Self::Grayscale23,
        ]
        .into_iter()
    }
}
#[duplicate_item(
    __enum_name__           __apply__           __reset__;
    [ ForegroundColours ]   [ "\x1b[38:5:{}m" ] [ "\x1b[39m" ];
    [ BackgroundColours ]   [ "\x1b[48:5:{}m" ] [ "\x1b[49m" ];
)]
#[allow(dead_code)]
impl HasValue<String> for __enum_name__ {
    fn value(&self) -> String {
        let index = match self {
            Self::Black => 0,
            Self::Red => 1,
            Self::Green => 2,
            Self::Yellow => 3,
            Self::Blue => 4,
            Self::Magenta => 5,
            Self::Cyan => 6,
            Self::White => 7,
            Self::BrightBlack => 8,
            Self::BrightRed => 9,
            Self::BrightGreen => 10,
            Self::BrightYellow => 11,
            Self::BrightBlue => 12,
            Self::BrightMagenta => 13,
            Self::BrightCyan => 14,
            Self::BrightWhite => 15,
            Self::R0G0B0 => 16,
            Self::R0G0B1 => 17,
            Self::R0G0B2 => 18,
            Self::R0G0B3 => 19,
            Self::R0G0B4 => 20,
            Self::R0G0B5 => 21,
            Self::R0G1B0 => 22,
            Self::R0G1B1 => 23,
            Self::R0G1B2 => 24,
            Self::R0G1B3 => 25,
            Self::R0G1B4 => 26,
            Self::R0G1B5 => 27,
            Self::R0G2B0 => 28,
            Self::R0G2B1 => 29,
            Self::R0G2B2 => 30,
            Self::R0G2B3 => 31,
            Self::R0G2B4 => 32,
            Self::R0G2B5 => 33,
            Self::R0G3B0 => 34,
            Self::R0G3B1 => 35,
            Self::R0G3B2 => 36,
            Self::R0G3B3 => 37,
            Self::R0G3B4 => 38,
            Self::R0G3B5 => 39,
            Self::R0G4B0 => 40,
            Self::R0G4B1 => 41,
            Self::R0G4B2 => 42,
            Self::R0G4B3 => 43,
            Self::R0G4B4 => 44,
            Self::R0G4B5 => 45,
            Self::R0G5B0 => 46,
            Self::R0G5B1 => 47,
            Self::R0G5B2 => 48,
            Self::R0G5B3 => 49,
            Self::R0G5B4 => 50,
            Self::R0G5B5 => 51,
            Self::R1G0B0 => 52,
            Self::R1G0B1 => 53,
            Self::R1G0B2 => 54,
            Self::R1G0B3 => 55,
            Self::R1G0B4 => 56,
            Self::R1G0B5 => 57,
            Self::R1G1B0 => 58,
            Self::R1G1B1 => 59,
            Self::R1G1B2 => 60,
            Self::R1G1B3 => 61,
            Self::R1G1B4 => 62,
            Self::R1G1B5 => 63,
            Self::R1G2B0 => 64,
            Self::R1G2B1 => 65,
            Self::R1G2B2 => 66,
            Self::R1G2B3 => 67,
            Self::R1G2B4 => 68,
            Self::R1G2B5 => 69,
            Self::R1G3B0 => 70,
            Self::R1G3B1 => 71,
            Self::R1G3B2 => 72,
            Self::R1G3B3 => 73,
            Self::R1G3B4 => 74,
            Self::R1G3B5 => 75,
            Self::R1G4B0 => 76,
            Self::R1G4B1 => 77,
            Self::R1G4B2 => 78,
            Self::R1G4B3 => 79,
            Self::R1G4B4 => 80,
            Self::R1G4B5 => 81,
            Self::R1G5B0 => 82,
            Self::R1G5B1 => 83,
            Self::R1G5B2 => 84,
            Self::R1G5B3 => 85,
            Self::R1G5B4 => 86,
            Self::R1G5B5 => 87,
            Self::R2G0B0 => 88,
            Self::R2G0B1 => 89,
            Self::R2G0B2 => 90,
            Self::R2G0B3 => 91,
            Self::R2G0B4 => 92,
            Self::R2G0B5 => 93,
            Self::R2G1B0 => 94,
            Self::R2G1B1 => 95,
            Self::R2G1B2 => 96,
            Self::R2G1B3 => 97,
            Self::R2G1B4 => 98,
            Self::R2G1B5 => 99,
            Self::R2G2B0 => 100,
            Self::R2G2B1 => 101,
            Self::R2G2B2 => 102,
            Self::R2G2B3 => 103,
            Self::R2G2B4 => 104,
            Self::R2G2B5 => 105,
            Self::R2G3B0 => 106,
            Self::R2G3B1 => 107,
            Self::R2G3B2 => 108,
            Self::R2G3B3 => 109,
            Self::R2G3B4 => 110,
            Self::R2G3B5 => 111,
            Self::R2G4B0 => 112,
            Self::R2G4B1 => 113,
            Self::R2G4B2 => 114,
            Self::R2G4B3 => 115,
            Self::R2G4B4 => 116,
            Self::R2G4B5 => 117,
            Self::R2G5B0 => 118,
            Self::R2G5B1 => 119,
            Self::R2G5B2 => 120,
            Self::R2G5B3 => 121,
            Self::R2G5B4 => 122,
            Self::R2G5B5 => 123,
            Self::R3G0B0 => 124,
            Self::R3G0B1 => 125,
            Self::R3G0B2 => 126,
            Self::R3G0B3 => 127,
            Self::R3G0B4 => 128,
            Self::R3G0B5 => 129,
            Self::R3G1B0 => 130,
            Self::R3G1B1 => 131,
            Self::R3G1B2 => 132,
            Self::R3G1B3 => 133,
            Self::R3G1B4 => 134,
            Self::R3G1B5 => 135,
            Self::R3G2B0 => 136,
            Self::R3G2B1 => 137,
            Self::R3G2B2 => 138,
            Self::R3G2B3 => 139,
            Self::R3G2B4 => 140,
            Self::R3G2B5 => 141,
            Self::R3G3B0 => 142,
            Self::R3G3B1 => 143,
            Self::R3G3B2 => 144,
            Self::R3G3B3 => 145,
            Self::R3G3B4 => 146,
            Self::R3G3B5 => 147,
            Self::R3G4B0 => 148,
            Self::R3G4B1 => 149,
            Self::R3G4B2 => 150,
            Self::R3G4B3 => 151,
            Self::R3G4B4 => 152,
            Self::R3G4B5 => 153,
            Self::R3G5B0 => 154,
            Self::R3G5B1 => 155,
            Self::R3G5B2 => 156,
            Self::R3G5B3 => 157,
            Self::R3G5B4 => 158,
            Self::R3G5B5 => 159,
            Self::R4G0B0 => 160,
            Self::R4G0B1 => 161,
            Self::R4G0B2 => 162,
            Self::R4G0B3 => 163,
            Self::R4G0B4 => 164,
            Self::R4G0B5 => 165,
            Self::R4G1B0 => 166,
            Self::R4G1B1 => 167,
            Self::R4G1B2 => 168,
            Self::R4G1B3 => 169,
            Self::R4G1B4 => 170,
            Self::R4G1B5 => 171,
            Self::R4G2B0 => 172,
            Self::R4G2B1 => 173,
            Self::R4G2B2 => 174,
            Self::R4G2B3 => 175,
            Self::R4G2B4 => 176,
            Self::R4G2B5 => 177,
            Self::R4G3B0 => 178,
            Self::R4G3B1 => 179,
            Self::R4G3B2 => 180,
            Self::R4G3B3 => 181,
            Self::R4G3B4 => 182,
            Self::R4G3B5 => 183,
            Self::R4G4B0 => 184,
            Self::R4G4B1 => 185,
            Self::R4G4B2 => 186,
            Self::R4G4B3 => 187,
            Self::R4G4B4 => 188,
            Self::R4G4B5 => 189,
            Self::R4G5B0 => 190,
            Self::R4G5B1 => 191,
            Self::R4G5B2 => 192,
            Self::R4G5B3 => 193,
            Self::R4G5B4 => 194,
            Self::R4G5B5 => 195,
            Self::R5G0B0 => 196,
            Self::R5G0B1 => 197,
            Self::R5G0B2 => 198,
            Self::R5G0B3 => 199,
            Self::R5G0B4 => 200,
            Self::R5G0B5 => 201,
            Self::R5G1B0 => 202,
            Self::R5G1B1 => 203,
            Self::R5G1B2 => 204,
            Self::R5G1B3 => 205,
            Self::R5G1B4 => 206,
            Self::R5G1B5 => 207,
            Self::R5G2B0 => 208,
            Self::R5G2B1 => 209,
            Self::R5G2B2 => 210,
            Self::R5G2B3 => 211,
            Self::R5G2B4 => 212,
            Self::R5G2B5 => 213,
            Self::R5G3B0 => 214,
            Self::R5G3B1 => 215,
            Self::R5G3B2 => 216,
            Self::R5G3B3 => 217,
            Self::R5G3B4 => 218,
            Self::R5G3B5 => 219,
            Self::R5G4B0 => 220,
            Self::R5G4B1 => 221,
            Self::R5G4B2 => 222,
            Self::R5G4B3 => 223,
            Self::R5G4B4 => 224,
            Self::R5G4B5 => 225,
            Self::R5G5B0 => 226,
            Self::R5G5B1 => 227,
            Self::R5G5B2 => 228,
            Self::R5G5B3 => 229,
            Self::R5G5B4 => 230,
            Self::R5G5B5 => 231,
            Self::Grayscale00 => 232,
            Self::Grayscale01 => 233,
            Self::Grayscale02 => 234,
            Self::Grayscale03 => 235,
            Self::Grayscale04 => 236,
            Self::Grayscale05 => 237,
            Self::Grayscale06 => 238,
            Self::Grayscale07 => 239,
            Self::Grayscale08 => 240,
            Self::Grayscale09 => 241,
            Self::Grayscale10 => 242,
            Self::Grayscale11 => 243,
            Self::Grayscale12 => 244,
            Self::Grayscale13 => 245,
            Self::Grayscale14 => 246,
            Self::Grayscale15 => 247,
            Self::Grayscale16 => 248,
            Self::Grayscale17 => 249,
            Self::Grayscale18 => 250,
            Self::Grayscale19 => 251,
            Self::Grayscale20 => 252,
            Self::Grayscale21 => 253,
            Self::Grayscale22 => 254,
            Self::Grayscale23 => 255,

            Self::Reset => -1,
        };

        match index {
            -1 => String::from(__reset__),
            index => format!(__apply__, index),
        }
    }
}
#[duplicate_item(
    __enum_name__;
    [ ForegroundColours ];
    [ BackgroundColours ];
)]
impl HasResetter for __enum_name__ {
    fn resetter(&self) -> Self {
        Self::Reset
    }
}
#[duplicate_item(
    __enum_name__;
    [ ForegroundColours ];
    [ BackgroundColours ];
)]
impl Display for __enum_name__ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}
