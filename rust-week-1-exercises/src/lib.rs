// Implement extract_tx_version function below

pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
   
    if raw_tx_hex.len() < 8 {
        return Err("Transaction data too short to contain version".to_string());
    }


    let version_hex = &raw_tx_hex[0..8];


    let bytes = hex::decode(version_hex)
        .map_err(|e| format!("Hex decode error: {}", e))?;

    if bytes.len() != 4 {
        return Err("Decoded version must be exactly 4 bytes".to_string());
    }

   
    let version = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);

    Ok(version)
}
