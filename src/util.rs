pub struct ByteString(Vec<u16>);

impl ByteString {
    pub fn from(string: String) -> ByteString {
        ByteString(string.encode_utf16().collect())
    }

    pub fn get_bytes(&self) -> &[u16] {
        self.0.as_slice()
    }
}
