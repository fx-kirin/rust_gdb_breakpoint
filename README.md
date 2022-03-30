# GDB breakpoint like python's breakpoint

Launch `ugdb` and attach to current process with new tmux window.


``` rust
use gdb_breakpoint::breakpoint;

pub fn main() {
    let x = 3 + 4;
    breakpoint();
}
```
