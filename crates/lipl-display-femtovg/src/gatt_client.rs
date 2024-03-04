use std::process::{Child, ChildStdout, Command, Stdio};

pub struct ProcessChild {
    child: Child,
    out: ChildStdout,
}

impl ProcessChild {
    pub fn new(name: &str, args: Vec<&str>) -> Result<Self, std::io::Error> {
        let mut child = Command::new(name).args(args).stdout(Stdio::piped()).spawn()?;
        let out = child.stdout.take().ok_or(std::io::Error::new(std::io::ErrorKind::Other, "no stdout"))?;
        Ok(
            Self {
                child,
                out,
        })
    }

    pub fn out(&mut self) -> &mut ChildStdout {
        &mut self.out
    }
}

impl Drop for ProcessChild {
    fn drop(&mut self) {
        if let Err(error) = self.child.kill() {
            eprintln!("Failed to kill child process: {}", error);
        }
    }
}

#[cfg(test)]
mod test {
    use std::io::{BufRead, BufReader};
    use super::ProcessChild;

    #[test]
    fn list_ls() {
        let mut ps = ProcessChild::new("find", vec!["/", "-name", "Cargo.toml"]).unwrap();
        let out: BufReader<_> = BufReader::new(ps.out());

        out.lines().for_each(|s| {
            if let Ok(line) = s {
                println!("{}", line);
            }
        });
    }
}