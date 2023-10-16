// Ready
pub use serde_json::{
    json, to_string, to_string_pretty, to_vec, to_vec_pretty, to_writer, to_writer_pretty, Map,
};
pub use value::{to_value, Number, Value};

pub use error::{Error, Result};

// Maybe
pub use de::{Deserializer, StreamDeserializer};
pub use serde_json::Serializer;

pub use de::{from_reader, from_slice, from_str};
pub mod de {
    // TODO
    pub use serde_json::de::{Deserializer, IoRead, Read, SliceRead, StrRead, StreamDeserializer};

    use crate::Result;

    use std::io;

    use serde::de::{Deserialize, DeserializeOwned};

    /// Deserialize an instance of type `T` from a string of JSON text.
    ///
    /// Equivalent to [serde_json::from_str] but with errors extended with [serde_path_to_error].
    ///
    /// See [serde_json::from_str] for more documentation.
    pub fn from_str<'a, T>(s: &'a str) -> Result<T>
    where
        T: Deserialize<'a>,
    {
        let jd = &mut serde_json::Deserializer::from_str(s);

        serde_path_to_error::deserialize(jd)
    }

    /// Deserialize an instance of type `T` from an I/O stream of JSON.
    ///
    /// Equivalent to [serde_json::from_reader] but with errors extended with [serde_path_to_error].
    ///
    /// See [serde_json::from_reader] for more documentation.
    pub fn from_reader<R, T>(rdr: R) -> Result<T>
    where
        R: io::Read,
        T: DeserializeOwned,
    {
        let jd = &mut serde_json::Deserializer::from_reader(rdr);

        serde_path_to_error::deserialize(jd)
    }

    /// Deserialize an instance of type `T` from bytes of JSON text.
    ///
    /// Equivalent to [serde_json::from_slice] but with errors extended with [serde_path_to_error].
    ///
    /// See [serde_json::from_slice] for more documentation.
    pub fn from_slice<'a, T>(v: &'a [u8]) -> Result<T>
    where
        T: Deserialize<'a>,
    {
        let jd = &mut serde_json::Deserializer::from_slice(v);

        serde_path_to_error::deserialize(jd)
    }
}

/// When serializing or deserializing JSON goes wrong.
pub mod error {
    pub use serde_json::error::Category;
    /// This type represents all possible errors that can occur when serializing or
    /// deserializing JSON data.
    pub type Error = serde_path_to_error::Error<serde_json::Error>;
    /// Alias for a `Result` with the error type `serde_json_path_to_error::Error`.
    pub type Result<T> = std::result::Result<T, Error>;
}
pub mod ser {
    use crate::Result;
    use serde::Serialize;

    // TODO
    pub use serde_json::ser::*;

    /// Serialize the given data structure as a String of JSON.
    ///
    /// Equivalent to [serde_json::to_string] but with errors extended with [serde_path_to_error].
    ///
    /// See [serde_json::to_string] for more documentation.
    pub fn to_string<T>(value: &T) -> Result<String>
    where
        T: ?Sized + Serialize,
    {
        let mut out = Vec::new();
        let jser = &mut serde_json::Serializer::new(&mut out);

        serde_path_to_error::serialize(&value, jser)
    }
}
pub mod map {

    // TODO
    pub use serde_json::map::*;
}

pub use value::from_value;
pub mod value {
    use serde::de::DeserializeOwned;

    // TODO
    pub use serde_json::value::{to_value, Index, Number, Serializer, Value};

    use crate::Result;

    /// Interpret a `serde_json::Value` as an instance of type `T`.
    ///
    /// Equivalent to [serde_json::from_value] but with errors extended with [serde_path_to_error].
    ///
    /// See [serde_json::from_value] for more documentation.
    pub fn from_value<T>(value: Value) -> Result<T>
    where
        T: DeserializeOwned,
    {
        serde_path_to_error::deserialize(value)
    }
}
