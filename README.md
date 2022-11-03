# DFM

Pure bash (minimal dependencies requried) script to help me manage my dotfiles.

## Usage

Create a new directory, separate configuration to different directory and group
them by who using them. Like `.zshrc`, `.zshenv` should all go inside `zsh`
directory, `.bashrc` should place into `bash`, `~/.config/nvim` just placed as `nvim`.

In each folder, create a script file and called it `.build`, `DOTBUILD` is
also recognized. And finally, put the `.dfm` executable script at the root of
the dotfile directory, and run `.dfm init` to do all the jobs.

> See [DOTBUILD SPECIFICATION](./DOTBUILD.md) for the grammar
