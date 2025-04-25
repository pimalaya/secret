//! Module gathering I/O-free, composable and iterable state machines.
//!
//! Flows emit [`crate::Io`] requests that need to be processed by
//! [`crate::handlers`] in order to continue their progression.

#[cfg(feature = "command")]
#[path = "./read-from-command.rs"]
mod read_from_command;
#[cfg(feature = "keyring")]
#[path = "./read-from-keyring.rs"]
mod read_from_keyring;

#[cfg(feature = "command")]
#[doc(inline)]
pub use self::read_from_command::ReadFromCommand;
#[cfg(feature = "keyring")]
#[doc(inline)]
pub use self::read_from_keyring::ReadFromKeyring;
