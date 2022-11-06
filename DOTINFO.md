# DOTINFO SPECIFICATION

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

### key

Extra non-reserved keys are still store in the runtime. It's useful to define some values for reuse proposal.
You can reuse the value by syntax: `"${extra-key}"`.

---

## Runtime Available Key

- `${srcdir}`: indicate where the dotfile source code located
