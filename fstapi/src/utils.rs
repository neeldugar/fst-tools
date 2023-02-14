use crate::{Error, Result};
use std::ffi::{CStr, CString};
use std::path::Path;

/// Trait for converting [`Path`] into string.
pub(crate) trait PathToStr<'a> {
  /// Converts to <code>&[str]</code>.
  fn to_str(&'a self) -> Result<&'a str>;
}

impl<'a, P> PathToStr<'a> for P
where
  P: AsRef<Path>,
{
  fn to_str(&'a self) -> Result<&'a str> {
    self.as_ref().to_str().ok_or(Error::InvalidUtf8Str(None))
  }
}

/// Trait for converting raw C string into string.
pub(crate) trait RawToStr {
  /// Converts to <code>&[str]</code>.
  unsafe fn to_str<'a>(self) -> Result<&'a str>;
}

impl RawToStr for *const i8 {
  unsafe fn to_str<'a>(self) -> Result<&'a str> {
    CStr::from_ptr(self)
      .to_str()
      .map_err(|e| Error::InvalidUtf8Str(Some(e)))
  }
}

/// Trait for converting bytes into [`CString`].
pub(crate) trait IntoCString {
  /// Converts into [`CString`].
  fn into_cstring(self) -> Result<CString>;
}

impl<T> IntoCString for T
where
  T: Into<Vec<u8>>,
{
  fn into_cstring(self) -> Result<CString> {
    CString::new(self).map_err(Error::CStringConv)
  }
}

/// Trait for converting `Option<Handle>` into handle for C API.
pub(crate) trait IntoCHandle {
  /// Converts into C handle.
  fn into_handle(self) -> u32;
}
