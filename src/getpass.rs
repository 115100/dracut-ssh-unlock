extern crate nix;

use nix::sys::termios;
use std::fs::File;
use std::io::{stdin, stdout, Write};
use std::os::unix::io::AsRawFd;

pub fn getpass(s: &mut String) -> Result<(), Box<std::error::Error>> {
    // Adopted from the description in:
    // http://man7.org/linux/man-pages/man3/getpass.3.html.
    let tty = File::open("/dev/tty")?;
    let old_term = termios::tcgetattr(tty.as_raw_fd())?;
    let mut new_term = old_term.clone();
    new_term.local_flags &= !(termios::LocalFlags::ECHO | termios::LocalFlags::ISIG);
    new_term.local_flags |= termios::LocalFlags::ECHONL;
    termios::tcsetattr(tty.as_raw_fd(), termios::SetArg::TCSAFLUSH, &new_term)?;

    print!("Enter your passphrase: ");
    stdout().flush()?;
    stdin().read_line(s)?;
    if s.ends_with('\n') {
        s.pop();
    }

    // Restore flags.
    termios::tcsetattr(tty.as_raw_fd(), termios::SetArg::TCSAFLUSH, &old_term)?;

    Ok(())
}