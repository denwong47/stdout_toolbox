use duplicate::duplicate_item;
use stdout_toolbox::*;

#[test]
fn test_ansi_modifierless_length() {
    let test_str = "The quick brown fox jumps over the lazy dog.";

    assert_eq!(test_str.len(), test_str.len_without_modifiers());
    assert_eq!(
        test_str.len(),
        ForegroundColours::BrightCyan
            .wraps(&test_str)
            .len_without_modifiers()
    );
    assert_eq!(
        test_str.len(),
        ForegroundColours::BrightCyan
            .join(BackgroundColours::BrightGreen)
            .wraps(&test_str)
            .len_without_modifiers()
    );
    assert_eq!(
        test_str.len(),
        ForegroundColours::BrightCyan
            .join(MoveCursor::Right(20))
            .join(MoveCursor::Up(20))
            .wraps(&test_str)
            .len_without_modifiers()
    );

    // ForegroundColours::BrightCyan
    // .join(MoveCursor::Right(20))
    // .join(MoveCursor::Up(20))
    // .wraps(&test_str)
    // .iter_modifier_strs()
    // .for_each(|s| println!("{:?}", s))
}

#[duplicate_item(
    [
        __name__    [ test_split_single_word ]
        __text__    [ "One_Word." ]
        __words__   [ [
            ("One_Word.", SpecialUnicodeChar::Nothing),
        ] ]
    ]
    [
        __name__    [ test_split_sentence ]
        __text__    [ "The quick brown fox\n jumps over the lazy dog." ]
        __words__   [ [
            ("The", SpecialUnicodeChar::Space),
            ("quick", SpecialUnicodeChar::Space),
            ("brown", SpecialUnicodeChar::Space),
            ("fox", SpecialUnicodeChar::LineFeed),
            ("", SpecialUnicodeChar::Space),
            ("jumps", SpecialUnicodeChar::Space),
            ("over", SpecialUnicodeChar::Space),
            ("the", SpecialUnicodeChar::Space),
            ("lazy", SpecialUnicodeChar::Space),
            ("dog.", SpecialUnicodeChar::Nothing),
        ] ]
    ]
    [
        __name__    [ test_split_multi_spaces ]
        __text__    [ "  Start  sentence with 2-spaces,\nbut don't break this\u{00A0}one.\n" ]
        __words__   [ [
            ("", SpecialUnicodeChar::Space),
            ("", SpecialUnicodeChar::Space),
            ("Start", SpecialUnicodeChar::Space),
            ("", SpecialUnicodeChar::Space),
            ("sentence", SpecialUnicodeChar::Space),
            ("with", SpecialUnicodeChar::Space),
            ("2", SpecialUnicodeChar::Hyphen),
            ("spaces,", SpecialUnicodeChar::LineFeed),
            ("but", SpecialUnicodeChar::Space),
            ("don't", SpecialUnicodeChar::Space),
            ("break", SpecialUnicodeChar::Space),
            ("this\u{00A0}one.", SpecialUnicodeChar::LineFeed),
            ("", SpecialUnicodeChar::Nothing),
        ] ]
    ]
)]
#[test]
fn __name__() {
    let text = __text__;
    let words = Vec::from(__words__);

    text.iter_words()
        .zip(words)
        .for_each(|((word, sep), answer)| {
            println!("Ouput: {:?} Answer: {:?}", (word.as_str(), &sep), answer);
            assert_eq!((word.as_str(), sep), answer)
        });
}

#[test]
fn test_split_line() {
    let text = "The three sections of output include the unit tests, the integration test, and the doc tests. Note that if any test in a section fails, the following sections will not be run. For example, if a unit test fails, there won’t be any output for integration and doc tests because those tests will only be run if all unit tests are passing.
    The \x1b[38:5:125mfirst \x1b[38:5:127ms\x1b[38:5:128me\x1b[38:5:129mc\x1b[38:5:130mt\x1b[38:5:131mi\x1b[38:5:132mo\x1b[38:5:133mn for the unit tests is the same as we’ve been seeing:\x1b[39m one line for each unit test (one named internal that we added in Listing 11-12) and then a summary line for the unit tests.
    The integration tests section starts with the line Running tests/integration_test.rs. Next, there is a line for each test function in that integration test and a summary line for the results of the integration test just before the Doc-tests adder section starts.

    Each integration test file has its own section, so if we add more files in the tests directory, there will be more integration test sections.
    We can still run a particular integration test function by specifying the test function’s name as an argument to cargo test. To run all the tests in a particular integration test file, use the --test argument of cargo test followed by the name of the file:
    ";

    text.iter_lines(60).for_each(|s| println!("{}", s));
    // .for_each(|s| println!("{}", s));
}
