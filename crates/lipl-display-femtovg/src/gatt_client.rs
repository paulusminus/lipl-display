use std::process::{Child, ChildStdout, Command, Stdio};

pub struct ProcessChild {
    child: Child,
    out: ChildStdout,
}

impl ProcessChild {
    pub fn new(name: &str, args: Vec<&str>) -> Result<Self, std::io::Error> {
        let mut child = Command::new(name)
            .args(args)
            .stdout(Stdio::piped())
            .spawn()?;
        let out = child
            .stdout
            .take()
            .ok_or(std::io::Error::other("no stdout"))?;
        Ok(Self { child, out })
    }

    pub fn out(&mut self) -> &mut ChildStdout {
        &mut self.out
    }
}

impl Drop for ProcessChild {
    fn drop(&mut self) {
        if let Err(error) = self.child.kill() {
            eprintln!("Failed to kill child process: {error}");
        }
    }
}
