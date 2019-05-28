# WebApp Manager

For an easy dev with elm

## Foreword

This small app have two purposes:

* To help me learn rust
* To ease the use of elm with a visual assistant

## Some commands

If you're lost offline, there is an 'help' argument ;-)

* Without any argument, the app launch a web-based window with a directory selector
* With the 'DIRECTORY' argument, you can skip the directory selection with the window
* with the 'compile' subcommand you just have to specify the 'FILE', and if you want, a 'OUTPUT', and if you can, an '--optimized', to generate your final index.html, without using the GUI
* with the 'serve' subcommand you can go no-GUI, but with the same options

### IMPORTANT: Priorities

Compile > Serve > Gui

Because Gui is coming without a subcommand, as the default behavior, and 'compile' is the most specific mode.

## TODO

* args of the 'compile' and the 'serve' subcommands
* check of the args FILE, OUTPUT and --optimized for the subcommand 'compile'
* launch of the elm reactor
* filesystem observer
* all the elm interface
