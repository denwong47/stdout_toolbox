//! Test for ANSI Modifier
use stdout_toolbox::modifiers2::*;

mod test_parsing {
    use super::*;

    macro_rules! test_factory {
        (
            $name:ident,
            $text:literal,
            $expected:expr
        ) => {
            #[test]
            fn $name() {
                let parsed: Result<ANSIEscapeCode, ModifierError> = ANSIEscapeCode::try_from($text);

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
        simple_word,
        "\x1b[1m",
        Ok::<_, ModifierError>(ANSIEscapeCode::new(1, None, 'm'))
    );

    test_factory!(
        zero_code,
        "\x1b[0m",
        Ok::<_, ModifierError>(ANSIEscapeCode::new(0, None, 'm'))
    );

    test_factory!(
        negative_code,
        "\x1b[-1m",
        Err::<ANSIEscapeCode, _>(ModifierError::ValueIsNotAModifier(
            String::from("\x1b[-1m"),
            String::from("Code is not a valid u16 integer.")
        ))
    );

    test_factory!(
        invalid_code,
        "\x1b[am",
        Err::<ANSIEscapeCode, _>(ModifierError::ValueIsNotAModifier(
            String::from("\x1b[am"),
            String::from("Unmatchable pattern.")
        ))
    );

    test_factory!(
        simple_modifier,
        "\x1b[1:1m",
        Ok::<_, ModifierError>(ANSIEscapeCode::new(1, Some(vec![1]), 'm'))
    );

    test_factory!(
        multiple_modifiers,
        "\x1b[1:1:2m",
        Ok::<_, ModifierError>(ANSIEscapeCode::new(1, Some(vec![1, 2]), 'm'))
    );

    test_factory!(
        semicolon_separated,
        "\x1b[1;1;2m",
        Ok::<_, ModifierError>(ANSIEscapeCode::new(1, Some(vec![1, 2]), 'm'))
    );

    test_factory!(
        mixed_separated,
        "\x1b[1:1;2m",
        Ok::<_, ModifierError>(ANSIEscapeCode::new(1, Some(vec![1, 2]), 'm'))
    );

    test_factory!(
        empty_modifier,
        "\x1b[1::1m",
        Err::<ANSIEscapeCode, _>(ModifierError::ValueIsNotAModifier(
            String::from("\x1b[1::1m"),
            String::from("Unmatchable pattern.")
        ))
    );

    test_factory!(
        negative_modifier,
        "\x1b[1:-1m",
        Ok::<_, ModifierError>(ANSIEscapeCode::new(1, Some(vec![-1]), 'm'))
    );

    test_factory!(
        long_modifiers,
        "\x1b[1:2:3:4:5:6:7:8:9:10:11:12:13m",
        Ok::<_, ModifierError>(ANSIEscapeCode::new(
            1,
            Some(vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]),
            'm'
        ))
    );

    test_factory!(
        invalid_modifier,
        "\x1b[1:am",
        Err::<ANSIEscapeCode, _>(ModifierError::ValueIsNotAModifier(
            String::from("\x1b[1:am"),
            String::from("Unmatchable pattern.")
        ))
    );

    test_factory!(
        move_cursor_up,
        "\x1b[1A", // This is an actual code for moving cursor up
        Ok::<_, ModifierError>(ANSIEscapeCode::new(1, None, 'A'))
    );

    test_factory!(
        move_cursor_absolute,
        "\x1b[30;60H",
        Ok::<_, ModifierError>(
            // TODO WRONG - correct this
            ANSIEscapeCode::new(30, Some(vec![60]), 'H')
        )
    );
}
