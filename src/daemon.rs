use std::env::set_current_dir;

use libc::{close, fork, setsid, umask, STDERR_FILENO, STDIN_FILENO, STDOUT_FILENO};

unsafe fn _daemonize() -> Result<(), &'static str> {
    match fork() {
        -1 => return Err("Fork failed."),
        // The child process becomes the daemon, and can now continue running as normal.
        0 => {}
        // The parent process can exit,
        // as the daemon will continue running in the child process.
        _ => libc::_exit(0),
    };

    // Create a new session for the daemon,
    // if the value returned is -1, the session could not be created.
    if setsid() == -1 {
        return Err("Could not set sid.");
    }

    // Ensure that we are not the session leader.
    match fork() {
        -1 => return Err("Fork failed."),
        // The child process becomes the daemon, and can now continue running as normal.
        0 => {}
        // The parent process can exit,
        // as the daemon will continue running in the child process.
        _ => libc::_exit(0),
    };

    // We want to use the system-wide default file-mode for the daemon.
    umask(0);

    // The current directory was inherited from the parent process,
    // to reset it to 'default', we switch to the root directory.
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
