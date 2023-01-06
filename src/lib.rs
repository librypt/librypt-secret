/// Holds a secret that can't be copied/cloned. Automcatically zeroes the memory when dropped.
///
/// NOTE: In the future, this may implement securing the memory containing the secret (via the OS).
pub struct Secret<const LENGTH: usize> {
    bytes: Box<[u8; LENGTH]>,
}

impl<const LENGTH: usize> Secret<LENGTH> {
    /// Initialize a secret from the provided buffer, zeroing the provided buffer once the secret has been copied.
    pub fn new(secret: &mut [u8; LENGTH]) -> Self {
        let mut bytes = Box::new([0u8; LENGTH]);

        std::mem::swap(bytes.as_mut(), secret);

        Self { bytes }
    }
}

impl<const LENGTH: usize> AsRef<[u8; LENGTH]> for Secret<LENGTH> {
    /// NOTE: Do not copy/store any bytes received this.
    fn as_ref(&self) -> &[u8; LENGTH] {
        &self.bytes
    }
}

impl<const LENGTH: usize> Drop for Secret<LENGTH> {
    /// NOTE: This may not be sufficient for security. More research will be done.
    fn drop(&mut self) {
        for i in self.bytes.as_mut() {
            *i = 0;
        }
    }
}
