use std::convert::TryFrom;
use std::path::{Path, PathBuf};

use crate::convert::TryAsRef;
use crate::unix::{UnixPath, UnixPathBuf};
use crate::windows::{WindowsPath, WindowsPathBuf};

/// Represents a path with a known type that can be one of:
///
/// * [`UnixPath`]
/// * [`WindowsPath`]
pub enum TypedPath<'a> {
    Unix(&'a UnixPath),
    Windows(&'a WindowsPath),
}

impl<'a> TypedPath<'a> {
    /// Creates a new typed path from a byte slice by determining if the path represents a Windows
    /// or Unix path. This is accomplished by first trying to parse as a Windows path. If the
    /// resulting path contains a prefix such as `C:` or begins with a `\`, it is assumed to be a
    /// [`WindowsPath`]; otherwise, the slice will be represented as a [`UnixPath`].
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_path::TypedPath;
    ///
    /// assert!(TypedPath::new(br#"C:\some\path\to\file.txt"#).is_windows());
    /// assert!(TypedPath::new(br#"\some\path\to\file.txt"#).is_windows());
    /// assert!(TypedPath::new(br#"/some/path/to/file.txt"#).is_unix());
    ///
    /// // NOTE: If we don't start with a backslash, it's too difficult to
    /// //       determine and we therefore just assume a Unix/POSIX path.
    /// assert!(TypedPath::new(br#"some\path\to\file.txt"#).is_unix());
    /// assert!(TypedPath::new(b"file.txt").is_unix());
    /// assert!(TypedPath::new(b"").is_unix());
    /// ```
    pub fn new(s: &'a [u8]) -> Self {
        let winpath = WindowsPath::new(s);
        if winpath.components().has_prefix() || s.first() == Some(&b'\\') {
            Self::Windows(winpath)
        } else {
            Self::Unix(UnixPath::new(s))
        }
    }

    /// Returns true if this path represents a Unix path.
    #[inline]
    pub fn is_unix(&self) -> bool {
        matches!(self, Self::Unix(_))
    }

    /// Returns true if this path represents a Windows path.
    #[inline]
    pub fn is_windows(&self) -> bool {
        matches!(self, Self::Windows(_))
    }

    /// Converts into a [`TypedPathBuf`].
    pub fn to_path_buf(&self) -> TypedPathBuf {
        match self {
            Self::Unix(path) => TypedPathBuf::Unix(path.to_path_buf()),
            Self::Windows(path) => TypedPathBuf::Windows(path.to_path_buf()),
        }
    }
}

impl<'a> From<&'a [u8]> for TypedPath<'a> {
    #[inline]
    fn from(s: &'a [u8]) -> Self {
        TypedPath::new(s)
    }
}

impl<'a> From<&'a str> for TypedPath<'a> {
    #[inline]
    fn from(s: &'a str) -> Self {
        TypedPath::new(s.as_bytes())
    }
}

impl TryAsRef<UnixPath> for TypedPath<'_> {
    fn try_as_ref(&self) -> Option<&UnixPath> {
        match self {
            Self::Unix(path) => Some(path),
            _ => None,
        }
    }
}

impl TryAsRef<WindowsPath> for TypedPath<'_> {
    fn try_as_ref(&self) -> Option<&WindowsPath> {
        match self {
            Self::Windows(path) => Some(path),
            _ => None,
        }
    }
}

/// Represents a pathbuf with a known type that can be one of:
///
/// * [`UnixPathBuf`]
/// * [`WindowsPathBuf`]
pub enum TypedPathBuf {
    Unix(UnixPathBuf),
    Windows(WindowsPathBuf),
}

impl TypedPathBuf {
    /// Returns true if this path represents a Unix path.
    #[inline]
    pub fn is_unix(&self) -> bool {
        matches!(self, Self::Unix(_))
    }

    /// Returns true if this path represents a Windows path.
    #[inline]
    pub fn is_windows(&self) -> bool {
        matches!(self, Self::Windows(_))
    }

    /// Converts into a [`TypedPath`].
    pub fn as_path(&self) -> TypedPath<'_> {
        match self {
            Self::Unix(path) => TypedPath::Unix(path.as_path()),
            Self::Windows(path) => TypedPath::Windows(path.as_path()),
        }
    }
}

impl<'a, const N: usize> From<&'a [u8; N]> for TypedPathBuf {
    #[inline]
    fn from(s: &'a [u8; N]) -> Self {
        TypedPathBuf::from(s.as_slice())
    }
}

impl<'a> From<&'a [u8]> for TypedPathBuf {
    /// Creates a new typed pathbuf from a byte slice by determining if the path represents a
    /// Windows or Unix path. This is accomplished by first trying to parse as a Windows path. If
    /// the resulting path contains a prefix such as `C:` or begins with a `\`, it is assumed to be
    /// a [`WindowsPathBuf`]; otherwise, the slice will be represented as a [`UnixPathBuf`].
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_path::TypedPathBuf;
    ///
    /// assert!(TypedPathBuf::from(br#"C:\some\path\to\file.txt"#).is_windows());
    /// assert!(TypedPathBuf::from(br#"\some\path\to\file.txt"#).is_windows());
    /// assert!(TypedPathBuf::from(br#"/some/path/to/file.txt"#).is_unix());
    ///
    /// // NOTE: If we don't start with a backslash, it's too difficult to
    /// //       determine and we therefore just assume a Unix/POSIX path.
    /// assert!(TypedPathBuf::from(br#"some\path\to\file.txt"#).is_unix());
    /// assert!(TypedPathBuf::from(b"file.txt").is_unix());
    /// assert!(TypedPathBuf::from(b"").is_unix());
    /// ```
    #[inline]
    fn from(s: &'a [u8]) -> Self {
        TypedPath::new(s).to_path_buf()
    }
}

impl From<Vec<u8>> for TypedPathBuf {
    #[inline]
    fn from(s: Vec<u8>) -> Self {
        // NOTE: We use the typed path to check the underlying format, and then
        //       create it manually to avoid a clone of the vec itself
        match TypedPath::new(s.as_slice()) {
            TypedPath::Unix(_) => TypedPathBuf::Unix(UnixPathBuf::from(s)),
            TypedPath::Windows(_) => TypedPathBuf::Windows(WindowsPathBuf::from(s)),
        }
    }
}

impl<'a> From<&'a str> for TypedPathBuf {
    #[inline]
    fn from(s: &'a str) -> Self {
        TypedPathBuf::from(s.as_bytes())
    }
}

impl From<String> for TypedPathBuf {
    #[inline]
    fn from(s: String) -> Self {
        // NOTE: We use the typed path to check the underlying format, and then
        //       create it manually to avoid a clone of the string itself
        match TypedPath::new(s.as_bytes()) {
            TypedPath::Unix(_) => TypedPathBuf::Unix(UnixPathBuf::from(s)),
            TypedPath::Windows(_) => TypedPathBuf::Windows(WindowsPathBuf::from(s)),
        }
    }
}

impl TryFrom<TypedPathBuf> for UnixPathBuf {
    type Error = TypedPathBuf;

    fn try_from(path: TypedPathBuf) -> Result<Self, Self::Error> {
        match path {
            TypedPathBuf::Unix(path) => Ok(path),
            path => Err(path),
        }
    }
}

impl TryFrom<TypedPathBuf> for WindowsPathBuf {
    type Error = TypedPathBuf;

    fn try_from(path: TypedPathBuf) -> Result<Self, Self::Error> {
        match path {
            TypedPathBuf::Windows(path) => Ok(path),
            path => Err(path),
        }
    }
}

impl<'a> TryAsRef<Path> for TypedPath<'a> {
    fn try_as_ref(&self) -> Option<&Path> {
        match self {
            Self::Unix(path) => path.try_as_ref(),
            Self::Windows(path) => path.try_as_ref(),
        }
    }
}

impl TryFrom<TypedPathBuf> for PathBuf {
    type Error = TypedPathBuf;

    fn try_from(path: TypedPathBuf) -> Result<Self, Self::Error> {
        match path {
            #[cfg(unix)]
            TypedPathBuf::Unix(path) => PathBuf::try_from(path).map_err(TypedPathBuf::Unix),
            #[cfg(windows)]
            TypedPathBuf::Windows(path) => PathBuf::try_from(path).map_err(TypedPathBuf::Windows),
            path => Err(path),
        }
    }
}
