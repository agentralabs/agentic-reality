//! Section types and section table for .areal format.

use crate::types::error::{RealityError, RealityResult};

/// Section type identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum SectionType {
    Deployment = 0x0001,
    Environment = 0x0002,
    Resource = 0x0003,
    Reality = 0x0004,
    Topology = 0x0005,
    Temporal = 0x0006,
    Stakes = 0x0007,
    Coherence = 0x0008,
    Indexes = 0x0009,
}

impl SectionType {
    pub fn from_u16(v: u16) -> RealityResult<Self> {
        match v {
            0x0001 => Ok(Self::Deployment),
            0x0002 => Ok(Self::Environment),
            0x0003 => Ok(Self::Resource),
            0x0004 => Ok(Self::Reality),
            0x0005 => Ok(Self::Topology),
            0x0006 => Ok(Self::Temporal),
            0x0007 => Ok(Self::Stakes),
            0x0008 => Ok(Self::Coherence),
            0x0009 => Ok(Self::Indexes),
            _ => Err(RealityError::InvalidFormat(format!("unknown section type: 0x{:04X}", v))),
        }
    }
}

/// Entry in the section table.
#[derive(Debug, Clone)]
pub struct SectionEntry {
    pub section_type: SectionType,
    pub flags: u16,
    pub offset: u64,
    pub length: u64,
    pub uncompressed_length: u64,
    pub checksum: [u8; 8],
}

/// Size of a section table entry in bytes.
pub const SECTION_ENTRY_SIZE: usize = 2 + 2 + 8 + 8 + 8 + 8; // 36 bytes

impl SectionEntry {
    pub fn write_to(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&(self.section_type as u16).to_le_bytes());
        buf.extend_from_slice(&self.flags.to_le_bytes());
        buf.extend_from_slice(&self.offset.to_le_bytes());
        buf.extend_from_slice(&self.length.to_le_bytes());
        buf.extend_from_slice(&self.uncompressed_length.to_le_bytes());
        buf.extend_from_slice(&self.checksum);
    }

    pub fn read_from(data: &[u8]) -> RealityResult<Self> {
        if data.len() < SECTION_ENTRY_SIZE {
            return Err(RealityError::InvalidFormat("section entry too short".into()));
        }
        let section_type = SectionType::from_u16(u16::from_le_bytes(data[0..2].try_into().map_err(|_| RealityError::InvalidFormat("section_type".into()))?))?;
        let flags = u16::from_le_bytes(data[2..4].try_into().map_err(|_| RealityError::InvalidFormat("flags".into()))?);
        let offset = u64::from_le_bytes(data[4..12].try_into().map_err(|_| RealityError::InvalidFormat("offset".into()))?);
        let length = u64::from_le_bytes(data[12..20].try_into().map_err(|_| RealityError::InvalidFormat("length".into()))?);
        let uncompressed_length = u64::from_le_bytes(data[20..28].try_into().map_err(|_| RealityError::InvalidFormat("uncompressed_length".into()))?);
        let checksum: [u8; 8] = data[28..36].try_into().map_err(|_| RealityError::InvalidFormat("checksum".into()))?;

        Ok(Self { section_type, flags, offset, length, uncompressed_length, checksum })
    }
}
