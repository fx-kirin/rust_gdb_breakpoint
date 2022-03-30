extern crate nix;

use nix::sys::signal::{kill, Signal};
use nix::unistd::getpid;
use std::process::Command;

pub fn breakpoint() {
    let gdb = format!("sudo ugdb --gdb=rust-gdb --pid {}", getpid());
    let argv = vec!["neww", &gdb];
    let argv_c = argv.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    Command::new("tmux")
        .args(&argv_c[..])
        .spawn()
        .unwrap();
    kill(getpid(), Signal::SIGSTOP).unwrap();
}

#[cfg(test)]
mod tests {
    use crate::breakpoint;
    #[test]
    fn it_works() {
        let x = 3 + 4;
        breakpoint();
        assert!(true);
    }
}
