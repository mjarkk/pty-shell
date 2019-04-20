extern crate pty_shell;

use pty_shell::*;

struct Shell {
    input: Vec<u8>,
}
impl PtyHandler for Shell {
    fn input(&mut self, input: &[u8]) {
        if input.len() == 1 && input[0] == 4 {
            println!("{}", String::from_utf8(self.input.clone()).unwrap());
        }
    }

    fn output(&mut self, output: &[u8]) {
        self.input.extend_from_slice(output);
    }

    fn resize(&mut self, winsize: &winsize::Winsize) {
        // do something with winsize
    }

    fn shutdown(&mut self) {
    }
}

fn main() {
    let child = tty::Fork::from_ptmx().unwrap();

    child.exec("bash");
    child.proxy(Shell { input: Vec::new() });
    child.wait();
}
