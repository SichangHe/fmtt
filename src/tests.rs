use insta::assert_snapshot;
use tracing::Level;
use tracing_subscriber::EnvFilter;

use super::*;

fn default_format(text: &str) -> String {
    format(text, 80, false, &Default::default()).join("")
}

#[test]
fn long_word() {
    init_tracing();
    let input = r#"
But, it chokes at very long splits such as `this_function_does_absolutely_nothing_i_am_afraid_but_it_needs_to_be_here_or_the_program_breaks`.
"#.trim_start();
    let formatted = default_format(input);
    assert_snapshot!(&formatted);
}

#[test]
fn backtick() {
    init_tracing();
    let input = r#"
First, I updated the `DATASET_URL` environment variable for the ML container in `k8s-tasks.yml` to point to the second dataset rather than the first.
"#.trim_start();
    let formatted = default_format(input);
    assert_snapshot!(&formatted);
}

#[test]
fn parentheses() {
    init_tracing();
    let input = r#"
As a matter of fact (or, rather as factually as I know or to the extent of my knowledge), some people (maybe quite a lot of people, or just a few people, depending on who you ask) really love over-using parentheses (I might be one of those people, oh no what have I done…).
"#.trim_start();
    let formatted = default_format(input);
    assert_snapshot!(&formatted);
}

#[test]
fn extra_line_breaks() {
    init_tracing();
    let input = r#"

Blah.

Blah blah blah.

"#;
    let formatted = default_format(input);
    assert_snapshot!(&formatted);
}

#[test]
fn gpt1() {
    init_tracing();
    let input = r#"
The department heads will convene at 3 P.M. to engage in a comprehensive discussion regarding the second-quarter budget, and it is imperative that you bring your identification card for seamless access; subsequently, the Information Technology team is scheduled to conduct a software demonstration at 4 P.M., with the esteemed presence of Mr. Chief Executive Officer's beautiful personal assistant.

In preparation for the impending meeting, a thorough review of the key performance indicators and return on investment is requested before the designated time of the meeting (which is set for 5 P.M.), and your prompt RSVP by 12 P.M. is kindly anticipated to ensure optimal (or at least as optimal as possible) coordination for the arrival of the very important persons at 2 P.M.! Furthermore, it is completely acceptable if you find yourself out of the office during this period.
    Simultaneously, the Quality Assurance team is seeking your valuable input in relation to the user interface and user experience; your collaboration is essential before the end of the day. A brief touch-base is suggested before the close of business hours to synchronize efforts and align objectives for maximum efficiency?

Meanwhile, the Research and Development team is deeply immersed in the execution of a project shrouded in secrecy, with the exact estimated time of arrival for the project launch yet to be determined! Additionally, the Human Resources department requires your date of birth for the forthcoming birthday celebration, an event exclusively reserved for individuals of paramount importance, and your immediate response for attendance is highly encouraged. Enjoy the festivities!
"#.trim_start();
    let formatted = default_format(input);
    assert_snapshot!(&formatted);
}

#[test]
fn abbr() {
    init_tracing();
    let input = r#"
I asked (emailed Prof. He Who Must Not Be Named and CCed Prof. YouKnowWho). The former has a budget of 7500, and the latter 5000.
"#.trim_start();
    let formatted = default_format(input);
    assert_snapshot!(&formatted);
}

#[test]
fn bracket() {
    init_tracing();
    let input = r#"
\begin{figure}
    \centering
    \includegraphics[width=0.9\linewidth]{figs/fig-name.pdf}
    \caption{Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.\texdtbf{lorem ipsum dolor sit amet}}
    \label{fig:lorem}
\end{figure}
"#.trim_start();
    let formatted = default_format(input);
    assert_snapshot!(&formatted);
}

#[test]
fn indented() {
    init_tracing();
    let input = r#"
Lorem ipsum dolor sit amet, consectetur adipiscing elit.
Sed do eiusmod tempor incididunt ut labore
et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat.
    Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu
    fugiat nulla pariatur.


Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
"#
    .trim_start();
    let formatted = default_format(input);
    assert_snapshot!(&formatted);
}

#[test]
fn indented_and_allow_indented_paragraphs() {
    init_tracing();
    let input = r#"
Lorem ipsum dolor sit amet, consectetur adipiscing elit.
Sed do eiusmod tempor incididunt ut labore
et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat.
    Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu
    fugiat nulla pariatur.


Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
"#
    .trim_start();
    let formatted = format(input, 80, true, &Default::default()).join("");
    assert_snapshot!(&formatted);
}

#[test]
fn lorem() {
    init_tracing();
    let input = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.\n";
    let formatted = default_format(input);
    assert_snapshot!(&formatted);
}

fn markdown_format(text: &str) -> String {
    format(text, 80, true, &markdown_paragraph_starts()).join("")
}

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

#[test]
fn markdown_ordered_list() {
    init_tracing();
    let input = r#"
1. Lorem ipsum dolor sit amet
2. consectetur adipiscing elit
3. Sed do eiusmod tempor incididunt ut labore et
    dolore magna aliqua
"#
    .trim_start();
    let formatted = markdown_format(input);
    assert_snapshot!(&formatted);
}

#[test]
fn markdown_unordered_list() {
    init_tracing();
    let input = r#"
- Lorem ipsum dolor sit amet
- consectetur adipiscing elit
* Sed do eiusmod tempor incididunt ut labore et
    dolore magna aliqua
"#
    .trim_start();
    let formatted = markdown_format(input);
    assert_snapshot!(&formatted);
}

#[test]
fn markdown_headers_separators() {
    init_tracing();
    let input = r#"
# Header 1
body
---
## Header 2
content
===
###### Header 6
####### This is just ordinary text,
    not a header.
"#
    .trim_start();
    let formatted = markdown_format(input);
    assert_snapshot!(&formatted);
}

#[test]
fn markdown_mix() {
    init_tracing();
    let input = r#"
# Header 1
body
---
more body
## Header 2
content
- Lists are respected.
- These two are not merged into one line.

1. Ordered lists are also preserved.
2. This line is separate from the previous one.
===
###### Header 6
####### This is just ordinary text,
    not a header.
"#
    .trim_start();
    let formatted = markdown_format(input);
    assert_snapshot!(&formatted);
}

fn latex_format(text: &str) -> String {
    format(text, 80, false, &latex_paragraph_starts()).join("")
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
fn latex_figure() {
    init_tracing();
    let input = r#"
\begin{figure}
    \centering
    \includegraphics[width=0.9\linewidth]{figs/fig-name.pdf}
    \caption{Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.\texdtbf{lorem ipsum dolor sit amet}}
    \label{fig:lorem}
\end{figure}
"#.trim_start();
    let formatted = latex_format(input);
    assert_snapshot!(&formatted);
}

#[test]
fn latex_comments() {
    init_tracing();
    let input = r#"
%Future work: Replace with real text. "Lorem ipsum dolor sit amet, consectetur adipiscing elit." is meaningless for most people.
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. % Comment in the middle are not dealt with.
% But, comment starting a line should be ignored, regardless of its length (it will be in one line).
Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
"#.trim_start();
    let formatted = latex_format(input);
    assert_snapshot!(&formatted);
}

#[test]
fn split_point_words() {
    use SentencePosition::*;
    assert_eq!(SubStart, word_sentence_position("(i.e."));
    assert_eq!(SubStart, word_sentence_position("[1]"));
    assert_eq!(SubEnd, word_sentence_position("and,"));
    assert_eq!(SubEnd, word_sentence_position("Or,"));
    assert_eq!(SubEnd, word_sentence_position("so)"));
    assert_eq!(End, word_sentence_position("A..Z."));
    assert_eq!(End, word_sentence_position("Black)."));
    assert_eq!(End, word_sentence_position("I18n."));
    assert_eq!(End, word_sentence_position("A.n."));
    assert_eq!(End, word_sentence_position("Program."));
    assert_eq!(End, word_sentence_position("HMM."));
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
