//use anyhow::Result;
use std::convert::TryInto;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum UrletDecodeError {
    #[error("Input not decodable to base64")]
    DecodeError {
        #[from]
        source: base64::DecodeError,
    },
    #[error("Input convertable to 128bits")]
    ConversionError,
}

pub fn encode(id: Uuid) -> String {
    base64::encode_config(id.as_bytes(), base64::URL_SAFE_NO_PAD)
}

pub fn decode(b64: &str) -> Result<Uuid, UrletDecodeError> {
    let decoded = base64::decode_config(b64, base64::URL_SAFE_NO_PAD)?;
    let buffer: [u8; 16] = decoded
        .try_into()
        .map_err(|_| UrletDecodeError::ConversionError)?;
    Ok(Uuid::from_bytes(buffer))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn decodes_from_endocec() {
        let uuid = super::Uuid::new_v4();
        let res = decode(&encode(uuid)).unwrap();
        assert_eq!(uuid, res);
    }
}
