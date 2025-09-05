#!/usr/bin/bash

# ./lazy - a lazy tool to manage containers.
#        inspired by cargo!
VERSION="0.1.3"
VERSION_TAG="lazy - v$VERSION"

# Source all config files
for conf in *.conf; do 
  source "$conf"
done
source lazy_cli.conf lazy.conf lazy_help.conf lazy_install.conf 
#####################
# HANDLE ARGS/FLAGS #
#####################
# Loop through all args and allow combinations 
# of multiple args when passed.
for arg in "$@"; do
  case "$arg" in
    --complete|complete) lazy_shell_complete ;;
    --complete=bash) _lazy_bash_complete ;;
    --complete=fish) _lazy_fish_complete ;;
    i|install|--install) lazy_install ;;
    u|update|--update) lazy_update ;;
    n|-n|new|--new) lazy_new;;
    h|-h|--help|help) lazy_help | less ;;
    v|-v|version|--version) lazy_help "$VERSION" ;;
    b|-b|build|--build) lazy_build;;
    r|-r|--run) lazy_run ;;
    rm|remove|--rm|--remove) lazy_rm ;;
    w|-w|--watch|watch) lazy_watch ;;
    *) 
      echo "Unknown Option: $arg..."
      echo "for usage details see: lazy --help" ;;
  esac
done

# If no args passed. show help menu. 
if [[ $# -eq 0 ]]; then 
  # show brief version of --help.
  # removing color from grep with tee.
  lazy_help | grep -A 21 Synopsis | tee /dev/null 
fi
