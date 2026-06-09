use crate::b64::encode_bytes;
use std::io::Write;

pub(crate) struct Base64Writer<T: Write> {
    inner: T,
    buffer: [u8; 3],
    offset: usize,
}

impl<T: Write> Base64Writer<T> {
    pub(crate) fn new(inner: T) -> Self {
        Self {
            inner,
            buffer: [0; 3],
            offset: 0,
        }
    }

    pub(crate) fn finish(&mut self) -> std::io::Result<()> {
        self.flush_buffer()
    }

    fn flush_buffer(&mut self) -> std::io::Result<()> {
        if self.offset == 0 {
            return Ok(());
        }

        let encoded = encode_bytes(&self.buffer[0..self.offset]);
        let result = self.inner.write_all(encoded.as_slice());
        self.offset = 0;
        self.buffer = [0; 3];
        result
    }
}

impl<T: Write> Write for Base64Writer<T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        for byte in buf {
            self.buffer[self.offset] = *byte;
            self.offset += 1;

            if self.offset == 3 {
                self.flush_buffer()?;
            }
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}
