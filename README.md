# Stupid RISC Assembler
This Rusty assembler can assemble my Stupid RISC Assembly language, which is as stupid as the name suggests.

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
        (cd ~/path-to-destination && ./Stupid-RISC-assembler "$@")
}
```
