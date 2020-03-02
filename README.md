# What is it?

xkcd_ascii is a small program that downloads comic from xkcd.com and converts it to ASCII Art.

# Showcase

[![asciicast](https://asciinema.org/a/14.png)](https://asciinema.org/a/287527)

# Installing

In order to use it, you have to install [rust](https://asciinema.org/a/287527).

```
$ git clone https://github.com/flabbet/xkcd_ascii.git
$ cd xkcd_ascii
$ cargo build --release
```

after that go to release directory, executable is stored there.

# Usage

To use it, simply type in terminal
```
$ xkcd_ascii
```
It will generate random comic with automatic, terminal size resolution. `resize` parameter is optional, you can adjust res with it.

```
$ xkcd_ascii -s [width] [height]
```

example

```
$ xkcd_ascii -s 203 62

or

$ xkcd_ascii --resize 203 62
```

## Id parameter

Id parameter is used to generate choosen comic. On xkcd.com there every comic has own unusual id found in URL.

### Usage

```
$ xkcd_ascii -i 402
or
$ xkcd_ascii --id 402
```

# Notice

This repo contains image to ascii generator from [asciify library](https://github.com/edelsonc/asciify)
