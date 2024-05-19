# ForMaT Text

A diff-friendly text formatter that breaks lines on sensible punctuations and words to fit a line width.

This is more useful to use with Git than `fmt` because the formatting is more consistent, resulting in smaller diffs.

- Respect line width limit.
- Prioritize splitting on
    1. sentence ends like `.`, then
    1. sub-sentence ends like `,`, then
    1. sub-sentence starts like `(`, and finally
    1. sentence-connection words like `and`.
- Limited support for abbreviations using heuristics.

## Installation

```sh
cargo install fmtt
```

## Usage

```sh
$ fmtt --help
ForMaT Text diff-friendly,
breaking lines on sensible punctuations and words to fit a line width.

Like fmt, FMTT is a text formatter;
it formats its input to have lines shorter than the line width limit
(if possible).
It reads an input file or StdIn and prints the formatted text to StdOut.
Like LaTeX,
FMTT does not distinguish different whitespaces or their amount except for
double line breaks; it only preserves leading spaces, not tabs.

This help message is formatted using FMTT itself as an example.


Usage: fmtt [OPTIONS]

Options:
  -w, --line-width <LINE_WIDTH>
          Maximum line width limit.
          
          [default: 80]

  -f, --filename <FILENAME>
          Name of input file; if omitted, read from StdIn.

  -c, --change-in-place
          If input file is provided, write output to it.

  -p, --allow-indented-paragraphs
          Allow indented paragraphs.
          If not set, any change indentation changes start a new paragraph.

  -m, --markdown-friendly
          Treat `# `/`## `/…/`###### `/`---`/`===`-started lines as single paragraphs;
          treat `- `/`* `/regex`\d+\. `-started lines as paragraph starts.
          Useful for Markdown, especially with `-p`.

  -l, --latex-friendly
          Ignore `%`-started lines;
          treat `\` started lines as paragraph starts.
          Useful for LaTeX.

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Examples

Plain text:

```sh
$ echo "
The department heads will convene at 3 P.M. to engage in a comprehensive discussion regarding the second-quarter budget, and it is imperative that you bring your identification card for seamless access; subsequently, the Information Technology team is scheduled to conduct a software demonstration at 4 P.M., with the esteemed presence of Mr. Chief Executive Officer's beautiful personal assistant.

In preparation for the impending meeting, a thorough review of the key performance indicators and return on investment is requested before the designated time of the meeting (which is set for 5 P.M.), and your prompt RSVP by 12 P.M. is kindly anticipated to ensure optimal (or at least as optimal as possible) coordination for the arrival of the very important persons at 2 P.M.! Furthermore, it is completely acceptable if you find yourself out of the office during this period.
    Simultaneously, the Quality Assurance team is seeking your valuable input in relation to the user interface and user experience; your collaboration is essential before the end of the day. A brief touch-base is suggested before the close of business hours to synchronize efforts and align objectives for maximum efficiency?

Meanwhile, the Research and Development team is deeply immersed in the execution of a project shrouded in secrecy, with the exact estimated time of arrival for the project launch yet to be determined! Additionally, the Human Resources department requires your date of birth for the forthcoming birthday celebration, an event exclusively reserved for individuals of paramount importance, and your immediate response for attendance is highly encouraged. Enjoy the festivities!
" | fmtt

The department heads will convene at 3 P.M. to engage in
a comprehensive discussion regarding the second-quarter budget,
and it is imperative that you bring your identification card for
seamless access; subsequently,
the Information Technology team is scheduled to
conduct a software demonstration at 4 P.M.,
with the esteemed presence of
Mr. Chief Executive Officer's beautiful personal assistant.

In preparation for the impending meeting,
a thorough review of the key performance indicators and return on
investment is requested before the designated time of the meeting
(which is set for 5 P.M.),
and your prompt RSVP by 12 P.M. is kindly anticipated to ensure optimal
(or at least as optimal as possible)
coordination for the arrival of the very important persons at 2 P.M.!
Furthermore,
it is completely acceptable if you find yourself out of
the office during this period.
    Simultaneously,
    the Quality Assurance team is seeking your valuable input in relation to
    the user interface and user experience;
    your collaboration is essential before the end of the day.
    A brief touch-base is suggested before the close of business hours to
    synchronize efforts and align objectives for maximum efficiency?

Meanwhile,
the Research and Development team is deeply immersed in the execution of
a project shrouded in secrecy,
with the exact estimated time of arrival for the project launch yet to
be determined!
Additionally,
the Human Resources department requires your date of birth for
the forthcoming birthday celebration,
an event exclusively reserved for individuals of paramount importance,
and your immediate response for attendance is highly encouraged.
Enjoy the festivities!

```

Markdown-friendly mode:

```sh
$ echo "
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
" | fmtt -pm

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
####### This is just ordinary text, not a header.

```

LaTeX-friendly mode:

```sh
$ echo "
\begin{figure}
    \centering
    \includegraphics[width=0.9\linewidth]{figs/fig-name.pdf}
    \caption{Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.\texdtbf{lorem ipsum dolor sit amet}}
    \label{fig:lorem}
\end{figure}
" | fmtt

\begin{figure}
    \centering
    \includegraphics[width=0.9\linewidth]{figs/fig-name.pdf}
    \caption{Lorem ipsum dolor sit amet,
    qui minim labore adipisicing minim sint cillum sint consectetur
    cupidatat.\texdtbf{lorem ipsum dolor sit amet}}
    \label{fig:lorem}
\end{figure}

```
