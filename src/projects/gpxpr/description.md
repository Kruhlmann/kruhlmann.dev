GPX file parser and renderer.

## Installation

### From PIP

    $ pip install gpxpr

### Manual

    $ git clone https://github.com/Kruhlmann/gpxpr
    $ cd gpxpr
    $ make install

## Usage

    usage: gpxpr [-h] -r RENDERER -w WALKING -v RUNNING -a {time,distance} [-o DESTINATION] target

    positional arguments:
      target                File to process

    optional arguments:
      -h, --help            show this help message and exit
      -r RENDERER, --renderer RENDERER
                            Renderer to use [stdout, matplot]
      -w WALKING, --min-walking-speed WALKING
                            Slowest min/km where a runner is considered 'walking'
      -v RUNNING, --min-running-speed RUNNING
                            Slowest min/km when a runner is considered 'running'
      -a {time,distance}, --aggregation-value {time,distance}
                            Value to base the aggregation on
      -o DESTINATION, --output DESTINATION
                            Output location. Default is /dev/stdout

## Developing

### Windows

The recommended way to develop on Windows is the make use of the [Windows Subsystem for Linux](https://docs.microsoft.com/en-us/windows/wsl/install) and then referring to the UNIX-like section.

### UNIX-like

Auto-running tests on code change is achievable with [entr](https://archlinux.org/packages/community/x86_64/entr/)

    find src tests -name "*.py" | entr make test

Similarly for linting on file change:

    find src tests -name "*.py" | entr make lint
