use lazy_static::lazy_static;
use regex::Regex;
use stdout_toolbox::modifiers2::*;

lazy_static! {
    // Special modified ESCAPE_CODE_START_PATTERN to test cases where the regex did not
    // block invalid strings successfully
    pub static ref LAX_ESCAPE_CODE_START_PATTERN: Regex =
        Regex::new(r#"^\x1b\[(?P<codes>(?:\w*[;:])*\d+)(?P<end_char>[A-Za-z])"#).unwrap();
}

#[cfg(test)]
mod test_try_from_captures {
    use super::*;

    macro_rules! test_factory {
        (
            $name:ident,
            $text:literal,
            $expected:expr
        ) => {
            #[test]
            fn $name() {
                let parsed = LAX_ESCAPE_CODE_START_PATTERN
                    .captures($text)
                    .ok_or(ModifierError::ValueIsNotAModifier($text.to_string()))
                    .and_then(|captures| Colour::try_from(&captures));

                if let Ok(variant) = parsed {
                    assert_eq!(variant, $expected.unwrap());
                } else {
                    if $expected.is_ok() {
                        panic!("{}", parsed.unwrap_err())
                    } else {
                        assert_eq!(
                            $expected.unwrap_err().to_string(),
                            parsed.unwrap_err().to_string(),
                        )
                    }
                }
            }
        };
    }

    test_factory!(
        test_simple_word,
        "\x1b[38:5:125m",
        Ok::<Colour, ModifierError>(Colour::R3G0B1)
    );
    test_factory!(
        test_simple_apply,
        "\x1b[39m",
        Ok::<Colour, ModifierError>(Colour::Reset)
    );
    test_factory!(
        test_mismatched_command,
        "\x1b[40m",
        Err::<Colour, ModifierError>(ModifierError::MismatchedANSICode(
            String::from("Colour"),
            40,
            38,
        ))
    );
    test_factory!(
        test_simple_apply_with_nondigit,
        "\x1b[38:a:125m",
        Err::<Colour, ModifierError>(ModifierError::ValueNotRecognised(
            String::from("Colour"),
            String::from("a"),
        ))
    );
    test_factory!(
        test_simple_apply_with_wrong_end_digit,
        "\x1b[38:5:125n",
        Err::<Colour, ModifierError>(ModifierError::UnexpectedEndCharacter(
            String::from("Colour"),
            String::from("n"),
        ))
    );
    test_factory!(
        test_reset_with_extra_codes,
        "\x1b[39:5:125m",
        Err::<Colour, ModifierError>(ModifierError::ValueNotRecognised(
            String::from("Colour"),
            format!("{:?}", vec![39, 5, 125]),
        ))
    );
    test_factory!(
        test_apply_with_wrong_code,
        "\x1b[38:6:125m",
        Err::<Colour, ModifierError>(ModifierError::ValueNotRecognised(
            String::from("Colour"),
            format!("{:?}", vec![38, 6, 125]),
        ))
    );
    test_factory!(
        test_simple_word_with_trailing_text,
        "\x1b[38:5:125mThis is some extra text added",
        Ok::<Colour, ModifierError>(Colour::R3G0B1)
    );
    test_factory!(
        test_simple_word_with_leading_text,
        "This is some extra text added\x1b[38:5:125m",
        Err::<Colour, ModifierError>(ModifierError::ValueIsNotAModifier(String::from(
            "This is some extra text added\u{1b}[38:5:125m"
        )))
    );
}
