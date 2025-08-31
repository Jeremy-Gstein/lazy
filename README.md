# lazy - manage your project containers. (the lazy way)
```
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
```
> [!NOTE]
> a fun weekend project to help with developing projects in docker.
