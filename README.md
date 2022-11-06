# DFM

Pure bash (minimal dependencies requried) script to help me manage my dotfiles.

## Usage

Create a new directory, separate configuration to different directory and group
them by who using them. Like `.zshrc`, `.zshenv` should all go inside `zsh`
directory, `.bashrc` should place into `bash`, `~/.config/nvim` just placed as `nvim`.

In each folder, create a JSON file and named it `DOTINFO`.
Run `dfm init` to get all configuration done.

> See [DOTINFO](./tests/example_info.json) for the example
