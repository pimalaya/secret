//! Module dedicated to [`serde`] de/serialization of [`Secret`].

use secrecy::{ExposeSecret, SecretString};
use serde::{de::value::Error, Deserialize, Serialize, Serializer};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Secret {
    #[serde(serialize_with = "serialize_secret_string")]
    Raw(SecretString),
    #[cfg(feature = "command")]
    #[serde(alias = "cmd")]
    Command(process_flows::Command),
    #[cfg(not(feature = "command"))]
    #[serde(alias = "cmd", with = "missing_command")]
    Command,
    #[cfg(feature = "keyring")]
    Keyring(keyring_flows::Entry),
    #[cfg(not(feature = "keyring"))]
    #[serde(with = "missing_keyring")]
    Keyring,
}

impl TryFrom<Secret> for crate::Secret {
    type Error = Error;

    fn try_from(secret: Secret) -> Result<Self, Self::Error> {
        match secret {
            Secret::Raw(secret) => Ok(Self::Raw(secret)),
            #[cfg(feature = "command")]
            Secret::Command(cmd) => Ok(Self::Command(cmd)),
            #[cfg(not(feature = "command"))]
            Secret::Command => Err(serde::de::Error::custom("missing `command` cargo feature")),
            #[cfg(feature = "keyring")]
            Secret::Keyring(entry) => Ok(Self::Keyring(entry)),
            #[cfg(not(feature = "keyring"))]
            Secret::Keyring => Err(serde::de::Error::custom("missing `keyring` cargo feature")),
        }
    }
}

impl Into<Secret> for crate::Secret {
    fn into(self) -> Secret {
        match self {
            Self::Raw(secret) => Secret::Raw(secret),
            #[cfg(feature = "command")]
            Self::Command(cmd) => Secret::Command(cmd),
            #[cfg(feature = "keyring")]
            Self::Keyring(cmd) => Secret::Keyring(cmd),
        }
    }
}

fn serialize_secret_string<S: Serializer>(secret: &SecretString, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(secret.expose_secret())
}

#[cfg(not(feature = "command"))]
mod missing_command {
    pub(crate) fn serialize<S: serde::Serializer>(_: S) -> Result<S::Ok, S::Error> {
        Err(serde::ser::Error::custom("missing `command` cargo feature"))
    }
    pub(crate) fn deserialize<'de, D: serde::Deserializer<'de>>(_: D) -> Result<(), D::Error> {
        Err(serde::de::Error::custom("missing `command` cargo feature"))
    }
}

#[cfg(not(feature = "keyring"))]
mod missing_keyring {
    pub(crate) fn serialize<S: serde::Serializer>(_: S) -> Result<S::Ok, S::Error> {
        Err(serde::ser::Error::custom("missing `keyring` cargo feature"))
    }
    pub(crate) fn deserialize<'de, D: serde::Deserializer<'de>>(_: D) -> Result<(), D::Error> {
        Err(serde::de::Error::custom("missing `keyring` cargo feature"))
    }
}

#[cfg(test)]
mod tests {
    use secrecy::{ExposeSecret, SecretString};

    use crate::Secret;

    #[test]
    fn serialize_raw() {
        let secret: Secret = toml::from_str("raw = \"password\"").unwrap();
        let Secret::Raw(secret) = secret else {
            panic!("should serialize Secret::Raw variant");
        };

        assert_eq!(secret.expose_secret(), "password");
    }

    #[test]
    fn deserialize_raw() {
        let secret = Secret::Raw(SecretString::from("password"));
        let toml = toml::to_string(&secret).unwrap();

        assert_eq!(toml.trim(), "raw = \"password\"");
    }

    #[cfg(feature = "command")]
    #[test]
    fn serialize_command_str() {
        let secret: Secret = toml::from_str("command = \"echo password\"").unwrap();
        let Secret::Command(cmd) = secret else {
            panic!("should serialize Secret::Command variant");
        };

        assert_eq!(cmd.program, "echo");

        let args = cmd.args.unwrap();
        assert_eq!(1, args.len());

        let arg = args.into_iter().next().unwrap();
        assert_eq!(arg, "password");
    }

    #[cfg(feature = "command")]
    #[test]
    fn serialize_command_seq() {
        let secret: Secret = toml::from_str("command = [\"echo\", \"password\"]").unwrap();
        let Secret::Command(cmd) = secret else {
            panic!("should serialize Secret::Command variant");
        };

        assert_eq!(cmd.program, "echo");

        let args = cmd.args.unwrap();
        assert_eq!(1, args.len());

        let arg = args.into_iter().next().unwrap();
        assert_eq!(arg, "password");
    }

    #[cfg(not(feature = "command"))]
    #[test]
    fn serialize_command() {
        let err = toml::from_str::<Secret>("command = \"echo password\"").unwrap_err();
        assert_eq!(err.message(), "missing `command` cargo feature");
    }

    #[cfg(feature = "command")]
    #[test]
    fn deserialize_command() {
        use process_flows::Command;

        let mut cmd = Command::new("echo");
        cmd.arg("password");

        let secret = Secret::Command(cmd);
        let toml = toml::to_string(&secret).unwrap();

        assert_eq!(toml.trim(), "command = [\"echo\", \"password\"]");
    }

    #[cfg(feature = "keyring")]
    #[test]
    fn serialize_keyring() {
        let secret: Secret = toml::from_str("keyring = \"entry\"").unwrap();
        let Secret::Keyring(entry) = secret else {
            panic!("should serialize Secret::Keyring variant");
        };

        assert_eq!(entry.name, "entry");
    }

    #[cfg(not(feature = "keyring"))]
    #[test]
    fn serialize_keyring() {
        let err = toml::from_str::<Secret>("keyring = \"echo password\"").unwrap_err();
        assert_eq!(err.message(), "missing `keyring` cargo feature");
    }

    #[cfg(feature = "keyring")]
    #[test]
    fn deserialize_keyring() {
        use keyring_flows::Entry;

        let entry = Entry::new("entry");
        let secret = Secret::Keyring(entry);
        let toml = toml::to_string(&secret).unwrap();

        assert_eq!(toml.trim(), "keyring = \"entry\"");
    }
}
