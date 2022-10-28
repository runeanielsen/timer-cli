# Timer CLI

Timer creates a timer that is detached from the shell so it can run in the background and if the shell is closed.

## Platform

This is not cross-platform, only works on Linux since it depends on `libc` fork to `daemonize` the timer so it can run in the background.

## Installation

```sh
cargo build --release && cargo install --path .
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

Here is an example of a bash script that writes a notifications and plays a file when the timer completes.

--- `finished.sh`
```bash
#!/bin/env bash

set -o errexit
set -o pipefail

dunstify "Timer finished."
ffplay -volume 50 -nodisp -autoexit ~/.config/timer-cli/break_sound.mp3
```

## Timer status

Timer status outputs duration left before the timer completes, example `09:52`.

```sh
timer status 
```

## Timer cancel

```sh
timer cancel
```
