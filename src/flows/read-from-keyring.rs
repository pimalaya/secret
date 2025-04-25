//! Module dedicated to the [`ReadFromKeyring`] secret I/O-free flow.

use keyring_flows::{flows::Read, Entry, Io, State};
use secrecy::SecretString;

/// I/O-free flow for reading a secret from a keyring entry.
#[derive(Debug)]
pub struct ReadFromKeyring {
    read: Read,
}

impl ReadFromKeyring {
    pub fn new(entry: Entry) -> Self {
        let read = Read::new(entry);
        Self { read }
    }

    pub fn next(&mut self) -> Result<SecretString, Io> {
        self.read.next()
    }
}

impl AsMut<State> for ReadFromKeyring {
    fn as_mut(&mut self) -> &mut State {
        self.read.as_mut()
    }
}
