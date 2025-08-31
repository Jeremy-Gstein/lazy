#!/usr/bin/bash

# ./lazy - a lazy tool to manage containers.
#        inspired by cargo!
VERSION="0.0.1"
VERSION_TAG="lazy - v$VERSION"

##########
# CONFIG #
##########
# dockerfile template
_lazy_dockerfile() {
  cat <<EOF
FROM $IMAGE AS build

RUN apk add --no-cache \
  python3 \
  py3-pip \

WORKDIR /app

CMD ["echo", "Hello World"]
EOF
}

# .lazy file template
_lazyfile_default() {
  cat <<EOF
NAME="lazy-default"
IMAGE_NAME="alpine"
IMAGE_TAG="latest"
DOCKERFILE="$PWD/Dockerfile"
ENV="$PWD/.env"
EOF
}

# set values in local .lazy file.
# check if lazy file exists and source it.
if [[ -f .lazy ]]; then
  source .lazy
  IMAGE="$IMAGE_NAME:$IMAGE_TAG"
  # load .lazy file from ~/.config/.lazy
elif [[ -f "$HOME/.config/.lazy" ]]; then
  source "$HOME/.config/.lazy"
  IMAGE="$IMAGE_NAME:$IMAGE_TAG"
# or use defaults if we cant find a .lazy file.
else 
  read -p "No .lazy file.. Create a new one? [y/N]: " choice
  case "$choice" in 
    y|Y)
      _lazyfile_default > "$HOME/.config/.lazy"
      clear
      echo "New Default .lazy file -> ~/.config/.lazy"
      ;;
    *)
      echo "Using default config..."
      eval "$(_lazyfile_default)"
      ;;
  esac
fi

####################
# LAZY CLI OPTIONS #
####################
# Build container image from dockerfile. 
# Confirm its created with docker images.
lazy_build() {
  # check if a dockerfile exists in dir.
  if [[ -f "$DOCKERFILE" ]]; then
    docker build -f "$DOCKERFILE" -t "$IMAGE" .
  else
    echo "Generating Default Dockerfile..."
    _lazy_dockerfile | docker build -f - -t "$IMAGE" .
  fi
 }

# Run the container image. 
# TODO check if image exists and build image.
lazy_run() {
  # check if .env exists for --env-file.
  if [[ -f "$ENV_FILE" ]]; then
    echo "Running: $IMAGE with: $ENV_FILE" 
    docker run -itd --env-file $ENV_FILE --name $NAME $IMAGE && \
      clear && \
      docker ps
  else
    echo "Running: $IMAGE"
    docker run -itd --name $NAME $IMAGE && \
      clear && \
      docker ps
  fi
}

# Remove existing container and image.
lazy_rm() {
  docker rm -f $NAME 
  docker image rm -f $IMAGE
  clear
}

# Check for changes to files - delete, rebuild and run new container. 
lazy_watch() {
  last_mod=0
  while true; do
    new_mod=$(stat -c %Y *.py) 
    if [[ $new_mod != $last_mod ]]; then
      lazy_rm 
      lazy_build 
      lazy_run
      last_mod=$new_mod
    fi
    # 'logs --follow' is blocking so we run/clear inbetween 'sleep 1'
    docker logs "$NAME" 2>&1 | tee /tmp/lazy_logs.out
    sleep 1
    clear
  done
}

# Initialize project with .lazy file.
lazy_new() {
  # create local .lazy if missing
  if [[ -f .lazy ]]; then
    echo ".lazy already exists, skipping..."
  else
    cp "$HOME/.config/.lazy" .lazy
    echo "Created .lazy from user config."
  fi

  # create Dockerfile with .lazy file config if missing
  if [[ -f Dockerfile ]]; then
    echo "Dockerfile already exists, skipping..."
  else
    _lazy_dockerfile > Dockerfile
    echo "Created default Dockerfile."
  fi
}

###############################################
# HELP / INSTALL / VERSION / SHELL COMPLETION #
###############################################
# Help menu and version tag
lazy_help() {
  if [[ "$1" == "$VERSION" ]]; then
    echo "$VERSION_TAG"
    exit 0
  fi
  cat <<'EOF'
Name:
  lazy - manage your project containers. (the lazy way)

Description:  
  lazy is a tool inspired by cargo and other similar tools.
  build, run, remove, watch, initialize projects with dockerfiles.
  any specific changes needed for a given projet can be configured 
  with the .lazy file by setting variables. See Eamples. 

Synopsis: 
  lazy [OPTIONS]

Options:
[CORE]
  -b,  --build            Build the container from .lazy.
  -r,  --run              Run the container from .lazy.
  -rm, --remove           Remove existing container and its image. 
  -w,  --watch            Rebuild on changes to files in current dir.
  -n,  --new              Initialize/Setup project with default .lazy file.
[CONFIG/HELP]
  -h,  --help             Show this help menu.
  -v,  --version          Show current version of lazy.
  --complete              Add shell completetion for all lazy commands.
  --install               Install lazy to ~/.local/share, ~/.config/.lazy

Examples:
LAZY:
  lazy --help 
  lazy -rm
  lazy b r 

.LAZY FILE: 
  NAME="FOO
  IMAGE="BAR"
  DOCKERFILE="/path/to/dockerfile"

Author:
  Jeremy Grosenstein - jeremy51b5@pm.me
EOF
echo "$VERSION_TAG"
}

# lazy fish shell completion
_lazy_fish_complete() {
  cat <<'EOF'
# Fish completion and description for lazy
complete -c lazy -a build -d "Build the container"
complete -c lazy -a run -d "Run the container"
complete -c lazy -a rm -d "Remove container and image"
complete -c lazy -a watch -d "Watch for changes and rebuild"
complete -c lazy -a new -d "Add default lazy files in current directory."
complete -c lazy -a version -d "Print lazy version."
complete -c lazy -a help -d "Show help"
# just shell completion for lazy no description
complete -c lazy -l help 
complete -c lazy -l build
complete -c lazy -l run 
complete -c lazy -l rm
complete -c lazy -l watch
complete -c lazy -l help 
complete -c lazy -l complete 
complete -c lazy -l version 
complete -c lazy -l new
EOF
}

# lazy bash shell completion
_lazy_bash_complete() {
  cat <<'EOF'
_lazy_completion() {
  local cur opts
  COMPREPLY=()
  cur="${COMP_WORDS[COMP_CWORD]}"
  opts="build run remove rm watch w help -h --help --completion"

  COMPREPLY=( $(compgen -W "$opts" -- "$cur") )
  return 0
}
complete -F _lazy_completion lazy.sh lazy ./lazy.sh
EOF
}

# print examples of shell complete for lazy.
lazy_shell_complete() {
  echo "Bash Completion:"
  _lazy_bash_complete
  echo
  echo "source <(./lazy.sh --complete=bash)"
  echo "Fish Completion:"
  _lazy_fish_complete
  echo
  echo "save to ~/.config/fish/completions/lazy.fish"
  echo "./lazy.sh --complete=fish > ~/.config/fish/completions/lazy.fish"
}

# install lazy to ~/.local/bin 
# create ~/.config/.lazy file 
lazy_install() {
  LINK_TARGET="$HOME/.local/bin/lazy"
  INSTALL_TARGET="$HOME/.local/share/lazy"
  CONFIG_PATH="$HOME/.config/.lazy"
  cp ./lazy.sh "$INSTALL_TARGET"
  touch "$CONFIG_PATH"
  # check if lazy exists and ask to delete it before linking.
  if [[ -e "$LINK_TARGET" ]]; then
    read -p "Found lazy at $LINK_TARGET. Delete and replace it? [y/N]: " choice
    case "$choice" in 
      y|Y)
        rm -f "$LINK_TARGET"
        ln -s $INSTALL_TARGET $LINK_TARGET
        clear
        echo "Installed new lazy -> ~/.local/bin/lazy"
        ;;
      *)
        echo "Canceled install NOT overwriting existing lazy ($LINK_TARGET)"
        exit 0
        ;;
    esac
  else
    ln -s "$INSTALL_TARGET" "$LINK_TARGET"
    echo "Installed lazy -> ~/.local/bin/lazy"
  fi 
  # PATH/alias examples:
  echo "if $INSTALL_TARGET not in PATH:"
  echo "install for system (requires root/sudo)"
  echo "  - sudo ln -sf $INSTALL_TARGET $LINK_TARGET"
  echo "add as alias to .bashrc"
  echo "  - alias --save $LINK_TARGET"

}

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
    install|--install) lazy_install ;;
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
  lazy_help | grep -A 20 Synopsis | tee /dev/null 
fi
