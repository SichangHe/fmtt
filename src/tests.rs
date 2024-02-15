use super::*;

#[test]
fn indented() {
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
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo conseq
    Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore
    eu fugiat nulla pariatur

Excepteur sint occaecat cupidatat non proident,
sunt in culpa qui officia deserunt mollit anim id est laborum.
"#
    .trim_start();

    let formatted = format(input).join("");
    println!("{formatted}");
    assert_eq!(&formatted, expected);
}

#[test]
fn lorem() {
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

    let formatted = format(input).join("");
    println!("{formatted}");
    assert_eq!(&formatted, expected);
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
