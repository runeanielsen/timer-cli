use std::env::set_current_dir;

use libc::{close, fork, setsid, umask, STDERR_FILENO, STDIN_FILENO, STDOUT_FILENO};

unsafe fn _daemonize() -> Result<(), &'static str> {
    match fork() {
        -1 => return Err("Fork failed."),
        // The child process becomes the daemon, and can now continue running as normal.
        0 => {}
        // The parent process can now exit, as the daemon will continue running in the child process.
        _ => libc::_exit(0),
    };

    // We want to use the system-wide default file-mode for the daemon.
    umask(0);

    // Create a new session for the daemon,
    // if the value returned is less than 0, the session could not be created.
    if setsid() < 0 {
        return Err("Could not set sid.");
    }

    // The daemon does not require a specific.
    if set_current_dir("/").is_err() {
        return Err("Could not set current directory.");
    };

    // The daemon should no longer communicate via 'STD'.
    close(STDIN_FILENO);
    close(STDOUT_FILENO);
    close(STDERR_FILENO);

    Ok(())
}

pub fn daemonize() -> Result<(), &'static str> {
    unsafe { _daemonize() }
}
