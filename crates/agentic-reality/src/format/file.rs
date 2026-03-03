//! .areal file reader and writer.

use std::path::Path;

use crate::engine::RealityEngine;
use crate::types::error::{RealityError, RealityResult};
use crate::types::{FOOTER_SIZE, HEADER_SIZE};

use super::footer::ArealFooter;
use super::header::ArealHeader;
use super::sections::{SectionEntry, SectionType, SECTION_ENTRY_SIZE};

/// Writer for .areal files.
pub struct ArealWriter;

impl ArealWriter {
    /// Save a reality engine to a .areal file.
    pub fn save(engine: &RealityEngine, path: &Path) -> RealityResult<()> {
        let incarnation_bytes = engine
            .incarnation_id()
            .map(|id| *id.as_uuid().as_bytes())
            .unwrap_or([0u8; 16]);

        let mut header = ArealHeader::new(incarnation_bytes);
        let mut sections_data: Vec<(SectionType, Vec<u8>)> = vec![];

        // Serialize each domain store
        let deployment = serde_json::to_vec(&engine.deployment_store)
            .map_err(|e| RealityError::Serialization(e.to_string()))?;
        sections_data.push((SectionType::Deployment, deployment));

        let environment = serde_json::to_vec(&engine.environment_store)
            .map_err(|e| RealityError::Serialization(e.to_string()))?;
        sections_data.push((SectionType::Environment, environment));

        let resource = serde_json::to_vec(&engine.resource_store)
            .map_err(|e| RealityError::Serialization(e.to_string()))?;
        sections_data.push((SectionType::Resource, resource));

        let reality = serde_json::to_vec(&engine.reality_store)
            .map_err(|e| RealityError::Serialization(e.to_string()))?;
        sections_data.push((SectionType::Reality, reality));

        let topology = serde_json::to_vec(&engine.topology_store)
            .map_err(|e| RealityError::Serialization(e.to_string()))?;
        sections_data.push((SectionType::Topology, topology));

        let temporal = serde_json::to_vec(&engine.temporal_store)
            .map_err(|e| RealityError::Serialization(e.to_string()))?;
        sections_data.push((SectionType::Temporal, temporal));

        let stakes = serde_json::to_vec(&engine.stakes_store)
            .map_err(|e| RealityError::Serialization(e.to_string()))?;
        sections_data.push((SectionType::Stakes, stakes));

        let coherence = serde_json::to_vec(&engine.coherence_store)
            .map_err(|e| RealityError::Serialization(e.to_string()))?;
        sections_data.push((SectionType::Coherence, coherence));

        header.section_count = sections_data.len() as u32;

        // Build output buffer
        let mut buf = Vec::new();

        // Write header placeholder
        header.write_to(&mut buf)?;

        // Section table offset = after header
        let section_table_start = buf.len();
        header.section_table_offset = section_table_start as u64;

        // Reserve space for section table
        let table_size = sections_data.len() * SECTION_ENTRY_SIZE;
        buf.resize(buf.len() + table_size, 0);

        // Write section data and build entries
        let mut entries = Vec::new();
        for (section_type, data) in &sections_data {
            let offset = buf.len() as u64;
            let len = data.len() as u64;
            let checksum_hash = blake3::hash(data);
            let mut checksum = [0u8; 8];
            checksum.copy_from_slice(&checksum_hash.as_bytes()[..8]);

            entries.push(SectionEntry {
                section_type: *section_type,
                flags: 0,
                offset,
                length: len,
                uncompressed_length: len,
                checksum,
            });

            buf.extend_from_slice(data);
        }

        // Write section table entries
        let mut table_buf = Vec::new();
        for entry in &entries {
            entry.write_to(&mut table_buf);
        }
        buf[section_table_start..section_table_start + table_size].copy_from_slice(&table_buf);

        // Write footer
        let global_hash = blake3::hash(&buf);
        let footer = ArealFooter::new(*global_hash.as_bytes(), entries.len() as u8);
        footer.write_to(&mut buf);

        // Update header with final checksum
        let header_hash = blake3::hash(&buf[..HEADER_SIZE - 8]);
        let mut header_checksum = [0u8; 8];
        header_checksum.copy_from_slice(&header_hash.as_bytes()[..8]);

        // Write atomically using temp file
        let tmp_path = path.with_extension("areal.tmp");
        std::fs::write(&tmp_path, &buf)?;
        std::fs::rename(&tmp_path, path)?;

        Ok(())
    }
}

/// Reader for .areal files.
pub struct ArealReader;

impl ArealReader {
    /// Load a reality engine from a .areal file.
    pub fn load(path: &Path) -> RealityResult<RealityEngine> {
        let data = std::fs::read(path)?;

        if data.len() < HEADER_SIZE + FOOTER_SIZE {
            return Err(RealityError::InvalidFormat("file too small".into()));
        }

        // Validate header
        let header = ArealHeader::read_from(&data)?;

        // Validate footer
        let _footer = ArealFooter::read_from(&data)?;

        // Read section table
        let table_start = header.section_table_offset as usize;
        let section_count = header.section_count as usize;

        let mut engine = RealityEngine::new();

        for i in 0..section_count {
            let entry_start = table_start + i * SECTION_ENTRY_SIZE;
            if entry_start + SECTION_ENTRY_SIZE > data.len() {
                return Err(RealityError::InvalidFormat("section table overflow".into()));
            }
            let entry = SectionEntry::read_from(&data[entry_start..])?;
            let section_data = &data[entry.offset as usize..(entry.offset + entry.length) as usize];

            match entry.section_type {
                SectionType::Deployment => {
                    engine.deployment_store = serde_json::from_slice(section_data)
                        .map_err(|e| RealityError::Serialization(e.to_string()))?;
                }
                SectionType::Environment => {
                    engine.environment_store = serde_json::from_slice(section_data)
                        .map_err(|e| RealityError::Serialization(e.to_string()))?;
                }
                SectionType::Resource => {
                    engine.resource_store = serde_json::from_slice(section_data)
                        .map_err(|e| RealityError::Serialization(e.to_string()))?;
                }
                SectionType::Reality => {
                    engine.reality_store = serde_json::from_slice(section_data)
                        .map_err(|e| RealityError::Serialization(e.to_string()))?;
                }
                SectionType::Topology => {
                    engine.topology_store = serde_json::from_slice(section_data)
                        .map_err(|e| RealityError::Serialization(e.to_string()))?;
                }
                SectionType::Temporal => {
                    engine.temporal_store = serde_json::from_slice(section_data)
                        .map_err(|e| RealityError::Serialization(e.to_string()))?;
                }
                SectionType::Stakes => {
                    engine.stakes_store = serde_json::from_slice(section_data)
                        .map_err(|e| RealityError::Serialization(e.to_string()))?;
                }
                SectionType::Coherence => {
                    engine.coherence_store = serde_json::from_slice(section_data)
                        .map_err(|e| RealityError::Serialization(e.to_string()))?;
                }
                SectionType::Indexes => {
                    engine.indexes = serde_json::from_slice(section_data)
                        .map_err(|e| RealityError::Serialization(e.to_string()))?;
                }
            }
        }

        Ok(engine)
    }
}
