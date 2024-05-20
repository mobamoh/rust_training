use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

pub const DATA_COLLECTOR_ADDRESS: &str = "";
const MAGIC_NUMBER: u16 = 1234;
const VERSION_NUMBER: u16 = 1;

fn unix_now() -> u32 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backward");
    since_the_epoch.as_secs() as u32
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CollectorCommandV1 {
    SubmitData {
        collector_id: u128,
        total_memory: u64,
        used_memory: u64,
        average_cpu_usage: f32,
    },
}

pub fn encode_v1(command: CollectorCommandV1) -> Vec<u8> {
    let json = serde_json::to_string(&command).unwrap();
    let json_byte = json.as_bytes();
    let crc = crc32fast::hash(json_byte);
    let payload_size = json_byte.len() as u32;
    let timestamp = unix_now();

    let mut result = Vec::with_capacity(140);
    result.extend_from_slice(&MAGIC_NUMBER.to_be_bytes());
    result.extend_from_slice(&VERSION_NUMBER.to_be_bytes());
    result.extend_from_slice(&timestamp.to_be_bytes());
    result.extend_from_slice(&payload_size.to_be_bytes());
    result.extend_from_slice(json_byte);
    result.extend_from_slice(&crc.to_be_bytes());
    result
}

pub fn decode_v1(bytes: &[u8]) -> (u32, CollectorCommandV1) {
    let magic_number = u16::from_be_bytes([bytes[0], bytes[1]]);
    let version_number = u16::from_be_bytes([bytes[2], bytes[3]]);
    let timestamp = u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    let payload_size = u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
    let payload = &bytes[12..12 + payload_size as usize];
    let crc = u32::from_be_bytes([
        bytes[12 + payload_size as usize],
        bytes[13 + payload_size as usize],
        bytes[14 + payload_size as usize],
        bytes[15 + payload_size as usize],
    ]);

    assert_eq!(magic_number, MAGIC_NUMBER);
    assert_eq!(version_number, VERSION_NUMBER);
    let computed_crc = crc32fast::hash(payload);
    assert_eq!(crc, computed_crc);

    (timestamp, serde_json::from_slice(payload).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let command = CollectorCommandV1::SubmitData {
            collector_id: 1234,
            total_memory: 100,
            used_memory: 50,
            average_cpu_usage: 0.5,
        };

        let encoded = encode_v1(command.clone());
        let (timestamp, decoded) = decode_v1(&encoded);
        assert_eq!(decoded, command);
        assert!(timestamp > 0);
    }
}
