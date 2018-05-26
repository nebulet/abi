use super::sys;
use {Result, Error, HandleRights};

/// An opaque handle.
/// Internally just a `u32`.
#[derive(PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Handle(u32);

impl Handle {
    /// Duplicate a handle.
    pub fn duplicate(&self, rights: HandleRights) -> Result<Handle> {
        let out = unsafe {
            sys::handle_duplicate(Handle(self.0), rights.bits())
        };

        Error::demux(out)
            .map(|index| Handle(index))
    }

    /// Close the handle.
    pub fn close(self) {
        unsafe {
            sys::handle_close(self);
        }
    }
}
