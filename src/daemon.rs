unsafe fn _daemonize() -> Result<(), &'static str> {
    match libc::fork() {
        -1 => return Err("Fork failed."),
        0 => {}
        _ => libc::_exit(0),
    };

    libc::umask(0);

    if libc::setsid() < 0 {
        return Err("Could not set sid.");
    }

    if std::env::set_current_dir("/").is_err() {
        return Err("Could not set current directory.");
    };

    libc::close(libc::STDIN_FILENO);
    libc::close(libc::STDOUT_FILENO);
    libc::close(libc::STDERR_FILENO);

    Ok(())
}

pub fn daemonize() -> Result<(), &'static str> {
    unsafe { _daemonize() }
}
