use serde::Deserializer;

/// A trait to allow for structs to have a default implementation upon deserialization.
/// Typically used with structs that have no meaningful serialization, but requires
/// initiation upon deserialization.
pub trait DeserializeWith {
    fn deserialize_with<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        Self: Sized,
        D: Deserializer<'de>;
}

mod implementations {
    use sysinfo::{self, SystemExt};
    use systemstat::{self, Platform};

    use super::*;

    impl DeserializeWith for sysinfo::System {
        fn deserialize_with<'de, D>(_: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(sysinfo::System::new_all())
        }
    }

    impl DeserializeWith for systemstat::System {
        fn deserialize_with<'de, D>(_: D) -> Result<Self, D::Error>
        where
            Self: Sized,
            D: Deserializer<'de>,
        {
            Ok(systemstat::System::new())
        }
    }
}
