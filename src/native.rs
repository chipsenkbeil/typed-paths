pub use self::non_utf8::*;
pub use self::utf8::*;

mod non_utf8 {
    /// [`Encoding`](crate::Encoding) that is native to the platform during compilation
    #[cfg(unix)]
    pub type NativeEncoding = crate::unix::UnixEncoding;

    /// [`Path`](crate::Path) that is native to the platform during compilation
    #[cfg(unix)]
    pub type NativePath = crate::unix::UnixPath;

    /// [`PathBuf`](crate::PathBuf) that is native to the platform during compilation
    #[cfg(unix)]
    pub type NativePathBuf = crate::unix::UnixPathBuf;

    /// [`Component`](crate::Component) that is native to the platform during compilation
    #[cfg(unix)]
    pub type NativeComponent<'a> = crate::unix::UnixComponent<'a>;

    /// [`Encoding`](crate::Encoding) that is native to the platform during compilation
    #[cfg(windows)]
    pub type NativeEncoding = crate::windows::WindowsEncoding;

    /// [`Path`](crate::Path) that is native to the platform during compilation
    #[cfg(windows)]
    pub type NativePath = crate::windows::WindowsPath;

    /// [`PathBuf`](crate::PathBuf) that is native to the platform during compilation
    #[cfg(windows)]
    pub type NativePathBuf = crate::windows::WindowsPathBuf;

    /// [`Component`](crate::Component) that is native to the platform during compilation
    #[cfg(windows)]
    pub type NativeComponent<'a> = crate::windows::WindowsComponent<'a>;

    #[cfg(unix)]
    mod unix_impl {
        use super::*;
        use crate::typed::{TypedPath, TypedPathBuf};

        impl NativePath {
            pub fn to_typed_path(&self) -> TypedPath {
                TypedPath::Unix(self)
            }

            pub fn to_typed_path_buf(&self) -> TypedPathBuf {
                TypedPathBuf::from_unix(self)
            }
        }
    }

    #[cfg(windows)]
    mod windows_impl {
        use super::*;
        use crate::typed::{TypedPath, TypedPathBuf};

        impl NativePath {
            pub fn to_typed_path(&self) -> TypedPath {
                TypedPath::Windows(self)
            }

            pub fn to_typed_path_buf(&self) -> TypedPathBuf {
                TypedPathBuf::from_windows(self)
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn native_path_buf_should_be_cloneable() {
            let path = NativePathBuf::from("hello.txt");
            assert_eq!(path, path.clone());
        }
    }
}

mod utf8 {
    /// [`Utf8Path`](crate::Utf8Encoding) that is native to the platform during compilation
    #[cfg(unix)]
    pub type Utf8NativeEncoding = crate::unix::Utf8UnixEncoding;

    /// [`Utf8Path`](crate::Utf8Path) that is native to the platform during compilation
    #[cfg(unix)]
    pub type Utf8NativePath = crate::unix::Utf8UnixPath;

    /// [`Utf8PathBuf`](crate::Utf8PathBuf) that is native to the platform during compilation
    #[cfg(unix)]
    pub type Utf8NativePathBuf = crate::unix::Utf8UnixPathBuf;

    /// [`Utf8Component`](crate::Utf8Component) that is native to the platform during compilation
    #[cfg(unix)]
    pub type Utf8NativeComponent<'a> = crate::unix::Utf8UnixComponent<'a>;

    /// [`Utf8Path`](crate::Utf8Encoding) that is native to the platform during compilation
    #[cfg(windows)]
    pub type Utf8NativeEncoding = crate::windows::Utf8WindowsEncoding;

    /// [`Utf8Path`](crate::Utf8Path) that is native to the platform during compilation
    #[cfg(windows)]
    pub type Utf8NativePath = crate::windows::Utf8WindowsPath;

    /// [`Utf8PathBuf`](crate::Utf8PathBuf) that is native to the platform during compilation
    #[cfg(windows)]
    pub type Utf8NativePathBuf = crate::windows::Utf8WindowsPathBuf;

    /// [`Utf8Component`](crate::Utf8Component) that is native to the platform during compilation
    #[cfg(windows)]
    pub type Utf8NativeComponent<'a> = crate::windows::Utf8WindowsComponent<'a>;

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn utf8_native_path_buf_should_be_cloneable() {
            let path = Utf8NativePathBuf::from("hello.txt");
            assert_eq!(path, path.clone());
        }
    }
}
