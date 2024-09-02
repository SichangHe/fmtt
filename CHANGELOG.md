# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.7.0](https://github.com/SichangHe/fmtt/compare/v0.6.2...v0.7.0) - 2024-09-02

### Fixed
- only split on ASCII whitespace

### Other
- bump pyo3 to 0.22
- quotes as sub-sentence starts&ends
- mention fmtm

## [0.6.2](https://github.com/SichangHe/fmtt/compare/v0.6.1...v0.6.2) - 2024-05-30

### Added
- more break words

### Other
- updated CI

## [0.6.1](https://github.com/SichangHe/fmtt/compare/v0.6.0...v0.6.1) - 2024-05-27

### Other
- make `paragraph_inner_format` side-effect-based for pausing

## [0.6.0](https://github.com/SichangHe/fmtt/compare/v0.5.2...v0.6.0) - 2024-05-27

### Other
- help message align w/ new hanging impl;fix latex config
- markdown-friendly style preserves hanging
- formatting consider hanging;macro for snapshot test
- detect paragraph hanging&3 handling options

## [0.5.2](https://github.com/SichangHe/fmtt/compare/v0.5.1...v0.5.2) - 2024-05-27

### Other
- expose internal paragraph formatting function
- finish recursion refactor
- recursion refactor 2nd stage;one less loop
- format to recursion 1st stage

## [0.5.1](https://github.com/SichangHe/fmtt/compare/v0.5.0...v0.5.1) - 2024-05-25

### Fixed
- fix multiple new line bug

### Other
- prioritize sub-sentence ends over sub-sentence starts
- split before words too long
- shut up warning from tailcall
- free tail call
- ignore python build files
- basic python binding
