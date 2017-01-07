#RAUR [![Build Status](https://travis-ci.org/deciduously/raur.svg?branch=master)](https://travis-ci.org/deciduously/raur)

Rhymes with "cower".

This is a very basic AUR helper, designed to seach, download, and install packages.

Uses a TOML config file, which by default lives at `~/config/raur/raur.toml` but can be specified with the -c or --config flag
"root" refers to the destination for downloaded packages.

Example:
```toml
[paths]
root = "/home/user/raur"
```
Will implement a way to auto-generate this on first use.