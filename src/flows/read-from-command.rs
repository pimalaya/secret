//! Module dedicated to the [`ReadFromCommand`] secret I/O-free flow.

use process_flows::{flows::SpawnThenWaitWithOutput, Command, Io, State};
use secrecy::SecretString;

/// I/O-free flow for reading a secret from a shell command.
#[derive(Debug)]
pub struct ReadFromCommand {
    spawn: SpawnThenWaitWithOutput,
}

impl ReadFromCommand {
    pub fn new(cmd: Command) -> Self {
        let spawn = SpawnThenWaitWithOutput::new(cmd);
        Self { spawn }
    }

    pub fn next(&mut self) -> Result<SecretString, Io> {
        let output = self.spawn.next()?;

        let first_line = match memchr::memchr(b'\n', &output.stdout) {
            Some(n) => &output.stdout[..n],
            None => &output.stdout,
        };

        let secret = String::from_utf8_lossy(first_line).to_string();
        Ok(SecretString::from(secret))
    }
}

impl AsMut<State> for ReadFromCommand {
    fn as_mut(&mut self) -> &mut State {
        self.spawn.as_mut()
    }
}
