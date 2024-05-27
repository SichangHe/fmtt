use insta::assert_snapshot;
use tracing::Level;
use tracing_subscriber::EnvFilter;

use super::*;

mod format;

fn markdown_paragraph_starts() -> ParagraphStarts {
    ParagraphStarts::preset(true, false).expect("Preset regex is incorrect.")
}

#[test]
fn markdown_regex() {
    let ParagraphStarts {
        single_line: Some(single_line),
        multi_line: Some(multi_line),
        ignore_line: _,
    } = markdown_paragraph_starts()
    else {
        panic!("Should have regex.")
    };

    assert!(single_line.is_match("# Header\nblah"));
    assert!(single_line.is_match("###### Header\nblah"));
    assert!(!single_line.is_match("####### body\nblah"));
    assert!(single_line.is_match("---\nblah"));
    assert!(single_line.is_match("===\nblah"));
    assert!(single_line.is_match("----\nblah"));

    assert!(multi_line.is_match("- lorem\nblah"));
    assert!(multi_line.is_match("* lorem\nblah"));
    assert!(multi_line.is_match("1. lorem\nblah"));
}

fn latex_paragraph_starts() -> ParagraphStarts {
    ParagraphStarts::preset(false, true).expect("Preset regex is incorrect.")
}

#[test]
fn latex_regex() {
    let ParagraphStarts {
        single_line: _,
        multi_line: Some(multi_line),
        ignore_line: Some(ignore_line),
    } = latex_paragraph_starts()
    else {
        panic!("Should have regex.")
    };

    assert!(multi_line.is_match("\\Rightarrow x^2\n\\"));
    assert!(multi_line.is_match("\\input{intro}"));

    assert!(ignore_line.is_match("%blah\nblah"));
    assert!(ignore_line.is_match("% blah"));
}

#[test]
fn split_point_words() {
    use SentencePosition::*;
    assert_eq!(SubStart, word_sentence_position("(though"));
    assert_eq!(SubStart, word_sentence_position("[1"));
    assert_eq!(SubEnd, word_sentence_position("and,"));
    assert_eq!(SubEnd, word_sentence_position("Or,"));
    assert_eq!(SubEnd, word_sentence_position("[1]"));
    assert_eq!(SubEnd, word_sentence_position("so)"));
    assert_eq!(End, word_sentence_position("A..Z."));
    assert_eq!(End, word_sentence_position("Black)."));
    assert_eq!(End, word_sentence_position("I18n."));
    assert_eq!(End, word_sentence_position("A.n."));
    assert_eq!(End, word_sentence_position("Program."));
    assert_eq!(End, word_sentence_position("HMM."));
    assert_eq!(End, word_sentence_position("(i.e."));
    assert_eq!(Other, word_sentence_position("Mr."));
    assert_eq!(Other, word_sentence_position("Ph.D."));
    assert_eq!(Other, word_sentence_position("A.K.A."));
    assert_eq!(Other, word_sentence_position("U.S."));
    assert_eq!(Other, word_sentence_position("Assoc."));
    assert_eq!(Other, word_sentence_position("Prof."));
}

#[test]
fn correct_indentation() {
    assert_eq!(first_line_indentation("blah"), 0);
    assert_eq!(first_line_indentation("blah\n"), 0);
    assert_eq!(first_line_indentation("blah blah\n"), 0);
    assert_eq!(first_line_indentation("blah blah \n"), 0);

    assert_eq!(first_line_indentation("    \n"), 0);
    assert_eq!(first_line_indentation("    a\n"), 4);
    assert_eq!(first_line_indentation("    "), 0);

    assert_eq!(first_line_indentation("   \n"), 0);
    assert_eq!(first_line_indentation("   a\n"), 3);
    assert_eq!(first_line_indentation("   "), 0);
}

fn init_tracing() {
    _ = tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(Level::INFO.into())
                .from_env_lossy(),
        )
        .try_init();
}
