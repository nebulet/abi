use {Result, Error};

bitflags! {
    pub struct HandleRights: u32 {
        /// The right to duplicate a handle;
        const DUPLICATE = 1 << 0;
        /// The right to transfer a handle
        /// to another process.
        const TRANSFER  = 1 << 1;
        /// The right to read the object
        /// refered to by a handle.
        const READ      = 1 << 2;
        /// The right to read the object
        /// refered to by a handle.
        const WRITE     = 1 << 3;
    }
}

impl HandleRights {
    pub fn has(&self, rights: HandleRights) -> Result<()> {
        if self.contains(rights) {
            Ok(())
        } else {
            Err(Error::ACCESS_DENIED)
        }
    }
}
