# Timer CLI

Creates a timer that is detached from the shell so it can run in the background. The daemonization of the process makes it easy to integrate it into external programs such as status-bars. If configured, it can execute a program/script when the timer completes.

## Platform

This is not cross-platform, only works on Linux since it depends on the `fork` function from `libc` to `daemonize` the timer.

## Installation

```sh
cargo install --path .
```

## Usage

### Timer default duration

Start timer, defaults to 25 minutes if no duration flag is supplied.

```sh
timer start
```

### Timer duration specified

Start timer for 10 minutes using the `-d` flag.

```sh
timer start -d 10
```

### Timer duration specified with finished flag

Start timer that executes a program or script when the timer completes.

```sh
timer start -d 10 -f /home/my_user/finished.sh
```

Example of a bash script that shows a notification and plays a sound when the timer completes.

--- `finished.sh`
```bash
#!/bin/env bash

set -o errexit
set -o pipefail

dunstify "Timer finished."
ffplay -volume 50 -nodisp -autoexit ~/.config/timer-cli/break_sound.mp3
```

## Timer status

Timer status writes the remaining time before the timer completes, an example is `09:52`.

```sh
timer status 
```

## Timer cancel

Cancels the timer, if it is running.

```sh
timer cancel
```
