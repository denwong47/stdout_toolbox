use duplicate::duplicate_item;
use stdout_toolbox::*;

#[test]
fn test_ansi_modifierless_length() {
    let test_str = "The quick brown fox jumps over the lazy dog.";

    assert_eq!(test_str.len(), test_str.len_without_modifiers());
    assert_eq!(test_str.len(), 
        ForegroundColours::BrightCyan
        .wraps(&test_str)
        .len_without_modifiers()
    );
    assert_eq!(test_str.len(), 
        ForegroundColours::BrightCyan
        .join(BackgroundColours::BrightGreen)
        .wraps(&test_str)
        .len_without_modifiers()
    );
    assert_eq!(test_str.len(), 
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
            ("One_Word.", None),
        ] ]
    ]
    [
        __name__    [ test_split_sentence ]
        __text__    [ "The quick brown fox jumps over the lazy dog." ]
        __words__   [ [
            ("The", Some(' ')),
            ("quick", Some(' ')),
            ("brown", Some(' ')),
            ("fox", Some(' ')),
            ("jumps", Some(' ')),
            ("over", Some(' ')),
            ("the", Some(' ')),
            ("lazy", Some(' ')),
            ("dog.", None),
        ] ]
    ]
    [
        __name__    [ test_split_multi_spaces ]
        __text__    [ "  Start  sentence with 2spaces,\nbut don't break this\u{00A0}one.\n" ]
        __words__   [ [
            ("", Some(' ')),
            ("", Some(' ')),
            ("Start", Some(' ')),
            ("", Some(' ')),
            ("sentence", Some(' ')),
            ("with", Some(' ')),
            ("2spaces,", Some('\n')),
            ("but", Some(' ')),
            ("don't", Some(' ')),
            ("break", Some(' ')),
            ("this\u{00A0}one.", Some('\n')),
            ("", None),
        ] ]
    ]
)]
#[test]
fn __name__() {
    let text = __text__;
    let words = Vec::from(__words__);

    text
    .iter_words()
    .zip(words)
    .for_each(
        | ((word, sep), answer) | {
            assert_eq!((word.as_str(), sep), answer)
        }
    );
}