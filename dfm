#!/bin/bash

# Options
RUN_INIT=0

show_help() {
cat << EOF
Usage: ${0##*/} [-hi] [DIR]
Manage the dotfile.

  DIR          Specfic directory to init, optional

  -h           Display this help
  -i           Run init for all directories
EOF
}

parse_opts() {
  local OPTIND=1
  while getopts 'hi:' opt; do
    case $opt in
      h)
        show_help
        exit 0
        ;;
      i)
        RUN_INIT=1
        ;;
    esac
  done
}
