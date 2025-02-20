# Stupid RISC Assembler
This Rusty assembler can assemble my Stupid RISC Assembly language, which is as stupid as the name suggests.

## Table of contents
- [Usage](README.md#Usage)
- [Installation](/README.md#Installation)

## Usage
To visually select an input file, run
```shell
sriscasm -v
```
*You might need to replace sriscasm with your own chosen name or the path to the executable depending on how you installed it.*

To parse the input path as an argument, run
```shell
sriscasm -f path/to/file
```



## Installation
### This programm can't actually be installed, but it can be aliased.
To perform this, please build it first.

### Building
Please ensure you have cargo and rustc installed properly.
If you have it installed, enter the following code in your terminal in this projects subfolder:
```sh
cargo build
```
*This will build the project for your computer.*

Now, you can locate the binary, which should usually be in ```target/debug/Stupid-RISC-assembler```.
You can now put this into your program folder (Application folder in macOS, /etc in Linux, any in Windows).

### Alias
You can make an alias in your bash/zsh/fish profile.
For example (**change for your own system!**)
```shell
sriscasm(){
    args=()
    for arg in "$@"; do
        if [[ -d "$arg" || -f "$arg" ]]; then
            abs_path="$(realpath "$arg")"
            echo "Converted $arg -> $abs_path"  # Debug print
            args+=("$abs_path")
        else
            args+=("$arg")
        fi
    done
    echo "Final args: ${args[@]}"  # Debug print
    (cd ~/Applications/ && ./Stupid-RISC-assembler "${args[@]}")
}
```
