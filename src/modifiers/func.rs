use super::{
    ForegroundColours,
    BackgroundColours,
    Intensity,

    Modifier,
};

/// Produce a colouring factory of the particular style.
#[allow(dead_code)]
pub fn colouriser<T>(
    fg: Option<ForegroundColours>,
    bg: Option<BackgroundColours>,
    intensity: Option<Intensity>,
) -> impl Fn(&T) -> String
where   T: ToString
{
    move | s: &T | {
        let mut result = s.to_string();

        if let Some(colour) = &fg {
            result = colour.wraps(&result)
        }

        if let Some(colour) = &bg {
            result = colour.wraps(&result)
        }

        if let Some(modifier) = &intensity {
            result = modifier.wraps(&result)
        }

        result
    }
}