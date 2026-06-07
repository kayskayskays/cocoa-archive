use std::io::Write;

const BASE_64: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

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

    fn finish(&mut self) -> std::io::Result<()> {
        self.flush_buffer()
    }

    fn flush_buffer(&mut self) -> std::io::Result<()> {
        let n = ((self.buffer[0] as u32) << 16) | ((self.buffer[1] as u32) << 8) | (self.buffer[2] as u32);

        if self.offset == 0 {
            return Ok(())
        }

        let char_1 = BASE_64[(n >> 18) as usize];
        let char_2 = BASE_64[((n >> 12) & 0b11_1111) as usize];
        let char_3 = BASE_64[((n >> 6) & 0b11_1111) as usize];
        let char_4 = BASE_64[(n & 0b11_1111) as usize];

        let mut buf = [char_1, char_2, char_3, char_4];

        let padding = 3 - self.offset;
        for idx in 0..padding {
            buf[buf.len() - 1 - idx] = b'=';
        }

        let result = self.inner.write_all(buf.as_slice());
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
