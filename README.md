# ForMaT Text

A text formatter that tries to break lines on sensible punctuations.

This is more useful to use with Git than `fmt` because the formatting is more consistent, resulting in smaller diffs.

- Limited support for abbreviations using heuristics.

## Installation

```sh
cargo install fmtt
```

## Usage

```sh
$ fmtt --help
A stupid text formatter that tries to break lines on sensible punctuations.

Usage: fmtt [OPTIONS]

Options:
  -l, --line-width <LINE_WIDTH>
          [default: 80]

  -f, --filename <FILENAME>


  -c, --change-in-place


  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
