//! .areal file header — 256 bytes.

use crate::types::error::{RealityError, RealityResult};
use crate::types::{AREAL_MAGIC, FORMAT_VERSION, HEADER_SIZE};
use serde::{Deserialize, Serialize};

/// File header flags.
pub mod flags {
    pub const COMPRESSED: u64 = 1 << 0;
    pub const SENSITIVE: u64 = 1 << 1;
    pub const ENCRYPTED: u64 = 1 << 2;
    pub const MIGRATED: u64 = 1 << 3;
    pub const SNAPSHOT: u64 = 1 << 4;
    pub const FULL_TOPOLOGY: u64 = 1 << 5;
    pub const HAS_INCARNATION_MEMORY: u64 = 1 << 6;
}

/// Compression method.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum Compression {
    None = 0,
    Lz4 = 1,
    Zstd = 2,
}

/// The 256-byte header of a .areal file.
#[derive(Debug, Clone)]
pub struct ArealHeader {
    pub magic: [u8; 4],
    pub version: u32,
    pub flags: u64,
    pub incarnation_id: [u8; 16],
    pub created: i64,
    pub modified: i64,
    pub section_count: u32,
    pub section_table_offset: u64,
    pub compression: Compression,
    pub checksum: [u8; 8],
}

impl ArealHeader {
    pub fn new(incarnation_id: [u8; 16]) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            magic: AREAL_MAGIC,
            version: FORMAT_VERSION,
            flags: 0,
            incarnation_id,
            created: now,
            modified: now,
            section_count: 0,
            section_table_offset: HEADER_SIZE as u64,
            compression: Compression::None,
            checksum: [0u8; 8],
        }
    }

    pub fn write_to(&self, buf: &mut Vec<u8>) -> RealityResult<()> {
        buf.extend_from_slice(&self.magic);
        buf.extend_from_slice(&self.version.to_le_bytes());
        buf.extend_from_slice(&self.flags.to_le_bytes());
        buf.extend_from_slice(&self.incarnation_id);
        buf.extend_from_slice(&self.created.to_le_bytes());
        buf.extend_from_slice(&self.modified.to_le_bytes());
        buf.extend_from_slice(&self.section_count.to_le_bytes());
        buf.extend_from_slice(&self.section_table_offset.to_le_bytes());
        buf.push(self.compression as u8);
        // Reserved: pad to 248 bytes (256 - 8 checksum)
        let current = buf.len();
        let target = HEADER_SIZE - 8;
        if current < target {
            buf.resize(target, 0);
        }
        buf.extend_from_slice(&self.checksum);
        Ok(())
    }

    pub fn read_from(data: &[u8]) -> RealityResult<Self> {
        if data.len() < HEADER_SIZE {
            return Err(RealityError::InvalidFormat("header too short".into()));
        }
        let magic: [u8; 4] = data[0..4].try_into().map_err(|_| RealityError::InvalidMagic)?;
        if magic != AREAL_MAGIC {
            return Err(RealityError::InvalidMagic);
        }
        let version = u32::from_le_bytes(data[4..8].try_into().map_err(|_| RealityError::InvalidFormat("version".into()))?);
        if version != FORMAT_VERSION {
            return Err(RealityError::VersionMismatch { expected: FORMAT_VERSION, got: version });
        }
        let flags = u64::from_le_bytes(data[8..16].try_into().map_err(|_| RealityError::InvalidFormat("flags".into()))?);
        let incarnation_id: [u8; 16] = data[16..32].try_into().map_err(|_| RealityError::InvalidFormat("incarnation_id".into()))?;
        let created = i64::from_le_bytes(data[32..40].try_into().map_err(|_| RealityError::InvalidFormat("created".into()))?);
        let modified = i64::from_le_bytes(data[40..48].try_into().map_err(|_| RealityError::InvalidFormat("modified".into()))?);
        let section_count = u32::from_le_bytes(data[48..52].try_into().map_err(|_| RealityError::InvalidFormat("section_count".into()))?);
        let section_table_offset = u64::from_le_bytes(data[52..60].try_into().map_err(|_| RealityError::InvalidFormat("section_table_offset".into()))?);
        let compression = match data[60] {
            0 => Compression::None,
            1 => Compression::Lz4,
            2 => Compression::Zstd,
            v => return Err(RealityError::InvalidFormat(format!("unknown compression: {}", v))),
        };
        let checksum: [u8; 8] = data[HEADER_SIZE-8..HEADER_SIZE].try_into()
            .map_err(|_| RealityError::InvalidFormat("checksum".into()))?;

        Ok(Self { magic, version, flags, incarnation_id, created, modified, section_count, section_table_offset, compression, checksum })
    }
}
