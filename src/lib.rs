extern crate nix;

use nix::sys::signal::{kill, Signal};
use nix::unistd::getpid;
use std::process::Command;

use std::sync::Once;

static INIT: Once = Once::new();

pub fn breakpoint() {
    let mut init = false;
    INIT.call_once(|| {
        let gdb = format!("sudo ugdb --gdb=rust-gdb --pid {}", getpid());
        let argv = vec!["neww", &gdb];
        let argv_c = argv.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        Command::new("tmux").args(&argv_c[..]).spawn().unwrap();
        init = true;
    });
    if init{
        kill(getpid(), Signal::SIGSTOP).unwrap();
    } else {
        kill(getpid(), Signal::SIGINT).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::breakpoint;
    #[test]
    fn it_works() {
        let x = 3 + 4;
        breakpoint();
        assert!(true);
        breakpoint();
        assert!(true);
    }
}
