use anyhow::{bail, Result};

/// SysExフォーマット: F0 7D <JSON bytes> F7
/// 0x7D は教育・開発用途の非商用ID。JSONはUTF-8。
const SYSEX_START: u8 = 0xF0;
const SYSEX_END: u8 = 0xF7;
const MANUFACTURER_ID: u8 = 0x7D;

pub fn encode(json: &serde_json::Value) -> Result<Vec<u8>> {
    let bytes = serde_json::to_vec(json)?;
    // SysExデータバイトは0x80未満でなければならない。JSON(UTF-8のASCII範囲)は通常OKだが、
    // 日本語等のマルチバイトが0x80以上を含むためチェックする。
    for b in &bytes {
        if *b >= 0x80 {
            bail!("JSON contains non-7bit bytes; base64 or escaping is required for SysEx");
        }
    }
    let mut out = Vec::with_capacity(bytes.len() + 3);
    out.push(SYSEX_START);
    out.push(MANUFACTURER_ID);
    out.extend_from_slice(&bytes);
    out.push(SYSEX_END);
    Ok(out)
}

pub fn decode(data: &[u8]) -> Result<serde_json::Value> {
    if data.len() < 3 {
        bail!("SysEx too short");
    }
    if data[0] != SYSEX_START || *data.last().unwrap() != SYSEX_END {
        bail!("Invalid SysEx framing");
    }
    if data[1] != MANUFACTURER_ID {
        bail!("Unexpected manufacturer ID");
    }
    let json: serde_json::Value = serde_json::from_slice(&data[2..data.len() - 1])?;
    Ok(json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn roundtrip_ascii_json() {
        let v = json!({"jsonrpc":"2.0","method":"command.exec","params":{"id":"Transport_Play"},"id":3});
        let enc = encode(&v).unwrap();
        assert_eq!(enc[0], 0xF0);
        assert_eq!(*enc.last().unwrap(), 0xF7);
        let dec = decode(&enc).unwrap();
        assert_eq!(dec, v);
    }

    #[test]
    fn rejects_non_7bit() {
        // 日本語を含むとUTF-8で0x80以上が出るためエラーになる仕様
        let v = json!({"method":"mixer.set","params":{"track":"ボーカル"}});
        assert!(encode(&v).is_err());
    }
}
