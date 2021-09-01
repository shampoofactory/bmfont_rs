#[inline(always)]
pub fn utf8_string(line: Option<usize>, bytes: &[u8]) -> crate::Result<String> {
    String::from_utf8(bytes.into())
        .map_err(|e| crate::Error::Parse { line, err: format!("UTF8: {}", e) })
}

#[allow(dead_code)]
#[inline(always)]
pub fn utf8_str(line: Option<usize>, bytes: &[u8]) -> crate::Result<&str> {
    std::str::from_utf8(bytes)
        .map_err(|e| crate::Error::Parse { line, err: format!("UTF8: {}", e) })
}
