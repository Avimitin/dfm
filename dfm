#!/bin/bash

colorize() {
  # prefer terminal safe colored and bold text when tput is supported
  if tput setaf 0 &>/dev/null; then
  	ALL_OFF="$(tput sgr0)"
  	BOLD="$(tput bold)"
  	BLUE="${BOLD}$(tput setaf 4)"
  	GREEN="${BOLD}$(tput setaf 2)"
  	RED="${BOLD}$(tput setaf 1)"
  	YELLOW="${BOLD}$(tput setaf 3)"
  else
  	ALL_OFF="\e[0m"
  	BOLD="\e[1m"
  	BLUE="${BOLD}\e[34m"
  	GREEN="${BOLD}\e[32m"
  	RED="${BOLD}\e[31m"
  	YELLOW="${BOLD}\e[33m"
  fi
  readonly ALL_OFF BOLD BLUE GREEN RED YELLOW
}

msg() {
  (( QUIET )) && return
  local mesg=$1; shift
  printf "${GREEN}==>${ALL_OFF}${BOLD} ${mesg}${ALL_OFF}\n" "$@"
}

msg2() {
  (( QUIET )) && return
  local mesg=$1; shift
  printf "${BLUE}  ->${ALL_OFF}${BOLD} ${mesg}${ALL_OFF}\n" "$@"
}

colorize

# Options
RUN_INIT=0
DIRS_TO_INIT=()

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
  while getopts 'hi' opt; do
    case $opt in
      h)
        show_help
        exit 0
        ;;
      i)
        RUN_INIT=1
        ;;
      *)
        show_help
        exit 1
        ;;
    esac
  done
  shift $((OPTIND-1))

  while [[ $1 ]]; do
    DIRS_TO_INIT+=($1)
    shift
  done
}

if [[ -z "$@" ]]; then
  show_help
  exit 0
fi

parse_opts "$@"

PENDING_AMOUNT=${#DIRS_TO_INIT[@]}
if (( $PENDING_AMOUNT == 0 )); then
  msg "Searching local directories..."
  DIRS_TO_INIT=($(find -type d -exec test -f '{}'/.build -o -f '{}'/DOTBUILD \; -print))
  PENDING_AMOUNT=${#DIRS_TO_INIT[@]}
fi

msg "$PENDING_AMOUNT config is pending to be initialize: ${DIRS_TO_INIT[*]}"
