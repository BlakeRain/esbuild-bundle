# Changelog

All notable changes to this library will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/1.0.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.2](https://git.blakerain.com/BlakeRain/esbuild-bundle/compare/v0.1.1..v0.1.2) - 2024-08-26

This is another small revision to address a bug caused since the addition of the environment
expansion.

### Fixed

- The bundle mapping (`.bundles.json` file) now associates the target path with the destination file
  name. Moreover this is what is returned from the `javascript!` macro, rather than the complete
  bundle path. Otherwise the returned paths are absolute when used with environment expansion.

## [0.1.1](https://git.blakerain.com/BlakeRain/esbuild-bundle/compare/v0.1.0..v0.1.1) - 2024-08-26

This is a small revision that adds a little functionality.

### Added

- Uses [shellexpand](https://docs.rs/shellexpand/latest/shellexpand/) to expand environment
  variables in both the bundle path and the path to source files.

