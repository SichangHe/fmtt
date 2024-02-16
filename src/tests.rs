use tracing::Level;
use tracing_subscriber::EnvFilter;

use super::*;

#[test]
fn abbr() {
    init_tracing();

    let input = r#"
I asked (emailed Prof. He Who Must Not Be Named and CCed Prof. YouKnowWho). The former has a budget of 7500, and the latter 5000.
"#.trim_start();
    let expected = r#"
I asked (emailed Prof. He Who Must Not Be Named and CCed Prof. YouKnowWho).
The former has a budget of 7500, and the latter 5000.
"#
    .trim_start();

    let formatted = format(input, 80).join("");
    println!("{formatted}");
    assert_eq!(&formatted, expected);
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
    let expected = r#"
\begin{figure}
    \centering \includegraphics[width=0.9\linewidth]{figs/fig-name.pdf}
    \caption{Lorem ipsum dolor sit amet,
    qui minim labore adipisicing minim sint cillum sint consectetur
    cupidatat.\texdtbf{lorem ipsum dolor sit amet}} \label{fig:lorem}
\end{figure}
"#
    .trim_start();

    let formatted = format(input, 80).join("");
    println!("{formatted}");
    assert_eq!(&formatted, expected);
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
    let expected = r#"
Lorem ipsum dolor sit amet, consectetur adipiscing elit.
Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat.
    Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore
    eu fugiat nulla pariatur.

Excepteur sint occaecat cupidatat non proident,
sunt in culpa qui officia deserunt mollit anim id est laborum.
"#
    .trim_start();

    let formatted = format(input, 80).join("");
    println!("{formatted}");
    assert_eq!(&formatted, expected);
}

#[test]
fn lorem() {
    init_tracing();

    let input = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.\n";
    let expected = r#"
Lorem ipsum dolor sit amet, consectetur adipiscing elit.
Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat.
Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu
fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident,
sunt in culpa qui officia deserunt mollit anim id est laborum.
"#
    .trim_start();

    let formatted = format(input, 80).join("");
    println!("{formatted}");
    assert_eq!(&formatted, expected);
}

#[test]
fn split_point_words() {
    assert!(is_split_point_word("and,"));
    assert!(is_split_point_word("Or,"));
    assert!(is_split_point_word("A..Z."));
    assert!(is_split_point_word("Black)."));
    assert!(is_split_point_word("I18n."));
    assert!(is_split_point_word("A.n."));
    assert!(!is_split_point_word("Mr."));
    assert!(!is_split_point_word("Ph.D."));
    assert!(!is_split_point_word("A.K.A."));
    assert!(!is_split_point_word("U.S."));
}

#[test]
fn correct_indentation() {
    assert_eq!(get_indentation("blah"), 0);
    assert_eq!(get_indentation("blah\n"), 0);
    assert_eq!(get_indentation("blah blah\n"), 0);
    assert_eq!(get_indentation("blah blah \n"), 0);

    assert_eq!(get_indentation("    \n"), 4);
    assert_eq!(get_indentation("    a\n"), 4);
    assert_eq!(get_indentation("    "), 0);

    assert_eq!(get_indentation("   \n"), 3);
    assert_eq!(get_indentation("   a\n"), 3);
    assert_eq!(get_indentation("   "), 0);
}

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(Level::INFO.into())
                .from_env_lossy(),
        )
        .init();
}
