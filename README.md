# spook

[![Crates.io](https://img.shields.io/crates/v/spooky.svg)](https://crates.io/crates/spooky) [![Build Status](https://travis-ci.com/montao/spook.svg?branch=master)](https://travis-ci.com/montao/spook)  


Spook(y) in Rust (todo:) via HTTP. It was originaly the Spook amusement for Emacs. 

* [Installation](#installation)
* [Commands](#commands)

## Installation

You can install `spooks` from either this repository, or from Crates (once it's published):

```shell
# install from Cargo
$ cargo install spooks

# install the latest from GitHub
$ cargo install --git https://github.com/montao/spook.git
```

## Commands

You must use a file named "src/spooks.lines" (this requirement will be relaxed in the near future). 

Running the program generates a new spooking object like the following. 

```shell
$ spooks
Manuel Rodriguez assault Uzi efnet Legion of Doom
```
