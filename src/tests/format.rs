use super::*;

fn default_format(text: &str) -> String {
    format(text, 80, Default::default(), &Default::default()).join("")
}

fn markdown_format(text: &str) -> String {
    format(text, 80, Hanging::Hang, &markdown_paragraph_starts()).join("")
}

fn latex_format(text: &str) -> String {
    format(text, 80, Default::default(), &latex_paragraph_starts()).join("")
}

macro_rules! t {
    ($name:ident, $input:literal) => {
        #[test]
        fn $name() {
            init_tracing();
            let input = $input.trim_start();
            let default_formatted = default_format(input);
            assert_snapshot!(&default_formatted);
            let markdown_formatted = markdown_format(input);
            assert_snapshot!(&markdown_formatted);
            let latex_formatted = latex_format(input);
            assert_snapshot!(&latex_formatted);
        }
    };
}

t!(
    long_word,
    r#"
But, it chokes at very long splits such as `this_function_does_absolutely_nothing_i_am_afraid_but_it_needs_to_be_here_or_the_program_breaks`.
"#
);

t!(
    backtick,
    r#"
First, I updated the `DATASET_URL` environment variable for the ML container in `k8s-tasks.yml` to point to the second dataset rather than the first.
"#
);

t!(
    parenthesis,
    r#"
As a matter of fact (or, rather as factually as I know or to the extent of my knowledge), some people (maybe quite a lot of people, or just a few people, depending on who you ask) really love over-using parentheses (I might be one of those people, oh no what have I done…).
"#
);

t!(
    extra_line_breaks,
    r#"

Blah.

Blah blah blah.

"#
);

t!(
    long_link,
    "[![YouTube icon](https://www.gstatic.com/youtube/img/branding/youtubelogo/svg/youtubelogo.svg) Channel](https://www.youtube.com/@sichanghe)"
);

t!(
    long_link_long_description,
    "![Math for hue-grayscale to RGB conversion on a whiteboard](https://github.com/SichangHe/internet_route_verification/assets/84777573/11f8ad38-403c-4e5d-99da-66176795223f)"
);

t!(
    many_commas,
    r"For a color with linear red, green, blue values $r,g,b\in[0,1]$, hue $h\in[0,360)$, saturation $s\in[0,1]$, lightness $l\in[0,1]$, and grayscale $p\in[0,1]$:"
);

t!(
    gpt1,
    r#"
The department heads will convene at 3 P.M. to engage in a comprehensive discussion regarding the second-quarter budget, and it is imperative that you bring your identification card for seamless access; subsequently, the Information Technology team is scheduled to conduct a software demonstration at 4 P.M., with the esteemed presence of Mr. Chief Executive Officer's beautiful personal assistant.

In preparation for the impending meeting, a thorough review of the key performance indicators and return on investment is requested before the designated time of the meeting (which is set for 5 P.M.), and your prompt RSVP by 12 P.M. is kindly anticipated to ensure optimal (or at least as optimal as possible) coordination for the arrival of the very important persons at 2 P.M.! Furthermore, it is completely acceptable if you find yourself out of the office during this period.
    Simultaneously, the Quality Assurance team is seeking your valuable input in relation to the user interface and user experience; your collaboration is essential before the end of the day. A brief touch-base is suggested before the close of business hours to synchronize efforts and align objectives for maximum efficiency?

Meanwhile, the Research and Development team is deeply immersed in the execution of a project shrouded in secrecy, with the exact estimated time of arrival for the project launch yet to be determined! Additionally, the Human Resources department requires your date of birth for the forthcoming birthday celebration, an event exclusively reserved for individuals of paramount importance, and your immediate response for attendance is highly encouraged. Enjoy the festivities!
"#
);

t!(
    abbr,
    r#"
I asked (emailed Prof. He Who Must Not Be Named and CCed Prof. YouKnowWho). The former has a budget of 7500, and the latter 5000.
"#
);

t!(
    bracket,
    r#"
\begin{figure}
    \centering
    \includegraphics[width=0.9\linewidth]{figs/fig-name.pdf}
    \caption{Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.\texdtbf{lorem ipsum dolor sit amet}}
    \label{fig:lorem}
\end{figure}
"#
);

t!(
    indented,
    r#"
Lorem ipsum dolor sit amet, consectetur adipiscing elit.
Sed do eiusmod tempor incididunt ut labore
et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat.
    Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu
    fugiat nulla pariatur.


Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
"#
);

t!(
    indented_and_allow_indented_paragraphs,
    r#"
Lorem ipsum dolor sit amet, consectetur adipiscing elit.
Sed do eiusmod tempor incididunt ut labore
et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat.
    Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu
    fugiat nulla pariatur.


Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
"#
);

t!(
    lorem,
"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.\n"
    );

t!(
    markdown_ordered_list,
    r#"
1. Lorem ipsum dolor sit amet
2. consectetur adipiscing elit
3. Sed do eiusmod tempor incididunt ut labore et
    dolore magna aliqua
"#
);

t!(
    markdown_unordered_list,
    r#"
- Lorem ipsum dolor sit amet
- consectetur adipiscing elit
* Sed do eiusmod tempor incididunt ut labore et
    dolore magna aliqua
"#
);

t!(
    markdown_headers_separators,
    r#"
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
);

t!(
    markdown_mix,
    r#"
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
);

t!(
    latex_figure,
    r#"
\begin{figure}
    \centering
    \includegraphics[width=0.9\linewidth]{figs/fig-name.pdf}
    \caption{Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.\texdtbf{lorem ipsum dolor sit amet}}
    \label{fig:lorem}
\end{figure}
"#
);

t!(
    latex_comments,
    r#"
%Future work: Replace with real text. "Lorem ipsum dolor sit amet, consectetur adipiscing elit." is meaningless for most people.
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. % Comment in the middle are not dealt with.
% But, comment starting a line should be ignored, regardless of its length (it will be in one line).
Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
"#
);
