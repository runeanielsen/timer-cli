# Timer CLI

Simple timer command-line interface, that creates a timer that is detached from the shell so it can run in the background and if the shell is closed.

## Platform

This is not cross-platform, only works on Linux since it depends on libc fork to daemonize.
