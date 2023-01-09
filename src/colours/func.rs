use super::{
    ANSI256Colours,
    ANSIIntensity,

    ANSIApply,
};

/// Produce a colouring factory of the particular style.
pub fn colouriser<T>(
    fg: Option<ANSI256Colours>,
    bg: Option<ANSI256Colours>,
    intensity: Option<ANSIIntensity>,
) -> impl Fn(&T) -> String
where   T: ToString
{
    move | s: &T | {
        let mut result = s.to_string();

        if let Some(colour) = &fg {
            result = colour.foreground(&result)
        }

        if let Some(colour) = &bg {
            result = colour.background(&result)
        }

        if let Some(modifier) = &intensity {
            result = modifier.apply(&result)
        }

        result
    }
}