# DOTBUILD SPECIFICATION

A `.build` file is a bash compatible script file. It takes the idea from
PKGBUILD. This make it easy to integrate with some daily using tools like curl
or wget.

When `.dfm` is run, it will recursively search all folders that contains `.build` or
`DOTBUILD` file. Then `.dfm` will read information from this file and get the
job done.

This document discuss each fields for the `.build` file. You can check the
example at [`DOTBUILD.example`](./DOTBUILD.example).
