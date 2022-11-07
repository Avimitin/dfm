# DOTINFO SPECIFICATION

DOTINFO file should follow the [`hjson`](https://hjson.github.io/) spec.

## name

The current configuration project name.

## depends

Dependencies that need to be download via system package manager.

### os

Current available os: `arch_linux`, `alpine`, `ubuntu`

## sources

Required resources for preparing the dotfile.
It should at least contains the dotfiles.

### path

A string value indicate the local path.

### url

A remote url for resource fetching

### git

A git fetchable url

### alias

A new name for the downloaded resources.

## steps

List of step should be done one by one.

### name

Name of the current step.

### actions

List of actions. Current available actions: `mkdir`, `copy`, `execute`, `symlink`, `remove`.

### env

Defined runtime expanded value.
You can reuse the value by syntax: `"${key}"`.

---

## Runtime Available Key

- `${srcdir}`: indicate where the dotfile source code located
