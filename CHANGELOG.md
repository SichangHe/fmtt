# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
