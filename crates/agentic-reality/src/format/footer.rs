//! .areal file footer — 64 bytes.

use crate::types::error::{RealityError, RealityResult};
use crate::types::{FOOTER_MAGIC, FOOTER_SIZE};

/// The 64-byte footer of a .areal file.
#[derive(Debug, Clone)]
pub struct ArealFooter {
    pub global_checksum: [u8; 32],
    pub sections_verified: u8,
    pub reserved: [u8; 23],
    pub magic: [u8; 8],
}

impl ArealFooter {
    pub fn new(checksum: [u8; 32], sections_verified: u8) -> Self {
        Self {
            global_checksum: checksum,
            sections_verified,
            reserved: [0u8; 23],
            magic: FOOTER_MAGIC,
        }
    }

    pub fn write_to(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.global_checksum);
        buf.push(self.sections_verified);
        buf.extend_from_slice(&self.reserved);
        buf.extend_from_slice(&self.magic);
    }

    pub fn read_from(data: &[u8]) -> RealityResult<Self> {
        if data.len() < FOOTER_SIZE {
            return Err(RealityError::InvalidFormat("footer too short".into()));
        }
        let start = data.len() - FOOTER_SIZE;
        let footer_data = &data[start..];
        let global_checksum: [u8; 32] = footer_data[0..32]
            .try_into()
            .map_err(|_| RealityError::InvalidFormat("global checksum".into()))?;
        let sections_verified = footer_data[32];
        let reserved: [u8; 23] = footer_data[33..56]
            .try_into()
            .map_err(|_| RealityError::InvalidFormat("reserved".into()))?;
        let magic: [u8; 8] = footer_data[56..64]
            .try_into()
            .map_err(|_| RealityError::InvalidFormat("footer magic".into()))?;
        if magic != FOOTER_MAGIC {
            return Err(RealityError::InvalidFormat("invalid footer magic".into()));
        }
        Ok(Self {
            global_checksum,
            sections_verified,
            reserved,
            magic,
        })
    }
}
