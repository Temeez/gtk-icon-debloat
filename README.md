# gtk-icon-debloat


CLI tool that copies the GTK icons you need somewhere you want.
The usage does not have to be limited to this though.

**WARNING:** Any matching existing files in the `out` path will be overridden.


```
USAGE:
    gtk-icon-debloat.exe [OPTIONS] --source <PATH>

OPTIONS:
    -h, --help             Print help information
    -i, --icons <FILE>     Path to a file containing GTK icon names. Looks for `iconlist.txt` by
                           default.
    -o, --out <PATH>       Directory path to which the icon files are copied to. Missing directories
                           will be created. Uses `out` by default.
    -s, --source <PATH>    Path to GTK icons directory.
    -V, --version          Print version information
```

## Installation
[Archives of precompiled binaries for gtk-icon-debloat are available for Windows and Linux.](https://github.com/Temeez/gtk-icon-debloat/releases)

### Building from source
![Minimum Rust: Nightly](https://img.shields.io/badge/Minimum%20Rust%20Version-nightly%201.45.0-orange.svg)

Building from source requires rust nightly.
Alternatively you can use stable but the `trim` needs to be disabled in the Cargo.toml file.

## Example
iconlist.txt
```
pan-down-symbolic
pan-up-symbolic
```

This is what you do:

`gtk-icon-debloat.exe -s C:\msys64\mingw64\share\icons`


This is what you get:
```
out
|-- Adwaita
    |-- 16x16
    |   |-- ui
    |       |-- pan-down-symbolic.symbolic.png
    |       |-- pan-up-symbolic.symbolic.png
    |-- 24x24
    |   |-- ui
    |       |-- pan-down-symbolic.symbolic.png
    |       |-- pan-up-symbolic.symbolic.png
    |-- 32x32
    |   |-- ui
    |       |-- pan-down-symbolic.symbolic.png
    |       |-- pan-up-symbolic.symbolic.png
    |-- 48x48
    |   |-- ui
    |       |-- pan-down-symbolic.symbolic.png
    |       |-- pan-up-symbolic.symbolic.png
    |-- 64x64
    |   |-- ui
    |       |-- pan-down-symbolic.symbolic.png
    |       |-- pan-up-symbolic.symbolic.png
    |-- 96x96
    |   |-- ui
    |       |-- pan-down-symbolic.symbolic.png
    |       |-- pan-up-symbolic.symbolic.png
    |-- scalable
        |-- ui
            |-- pan-down-symbolic.svg
            |-- pan-up-symbolic.svg
```

## Lisence
MIT