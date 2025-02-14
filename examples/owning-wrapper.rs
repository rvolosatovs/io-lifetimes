//! A simple example implementing the main traits for a type.

#[cfg(not(windows))]
#[cfg(any(feature = "close", not(io_lifetimes_use_std)))]
use io_lifetimes::FromFd;
#[cfg(windows)]
#[cfg(any(feature = "close", not(io_lifetimes_use_std)))]
use io_lifetimes::FromHandle;
#[cfg(not(windows))]
#[cfg(not(io_lifetimes_use_std))]
use io_lifetimes::IntoFd;
#[cfg(windows)]
#[cfg(not(io_lifetimes_use_std))]
use io_lifetimes::IntoHandle;
use io_lifetimes::OwnedFilelike;
#[cfg(not(windows))]
use io_lifetimes::{AsFd, BorrowedFd, OwnedFd};
#[cfg(windows)]
use io_lifetimes::{AsHandle, BorrowedHandle, OwnedHandle};

/// A wrapper around a file descriptor.
///
/// Implementing `AsFd`, `IntoFd`, and `FromFd` for a type that wraps an
/// `Owned*` is straightforward. `Owned*` types also automatically close the
/// handle in its `Drop`.
///
/// Should owning wrappers implement `AsRawFd`, `IntoRawFd`, and `FromRawFd`
/// too? They can, and there's no need to remove them from a type that already
/// implements them. But for new code, they can be omitted. Users that really
/// need the raw value can always do `as_fd().as_raw_fd()`,
/// `.into_fd().into_raw_fd()`, or `T::from_fd(OwnedFd::from_raw_fd(raw_fd))`.
/// But if possible, users should use just `as_fd`, `into_fd`, and `from_fd`
/// and avoid working with raw values altogether.
struct Thing {
    filelike: OwnedFilelike,
}

#[cfg(not(windows))]
impl AsFd for Thing {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.filelike.as_fd()
    }
}

#[cfg(not(io_lifetimes_use_std))]
#[cfg(not(windows))]
impl IntoFd for Thing {
    #[inline]
    fn into_fd(self) -> OwnedFd {
        self.filelike
    }
}

#[cfg(not(windows))]
impl From<Thing> for OwnedFd {
    #[inline]
    fn from(owned: Thing) -> Self {
        owned.filelike
    }
}

#[cfg(not(io_lifetimes_use_std))]
#[cfg(not(windows))]
impl FromFd for Thing {
    #[inline]
    fn from_fd(filelike: OwnedFd) -> Self {
        Self { filelike }
    }
}

#[cfg(not(windows))]
impl From<OwnedFd> for Thing {
    #[inline]
    fn from(filelike: OwnedFd) -> Self {
        Self { filelike }
    }
}

#[cfg(windows)]
impl AsHandle for Thing {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        self.filelike.as_handle()
    }
}

#[cfg(not(io_lifetimes_use_std))]
#[cfg(windows)]
impl IntoHandle for Thing {
    #[inline]
    fn into_handle(self) -> OwnedHandle {
        self.filelike
    }
}

#[cfg(windows)]
impl From<Thing> for OwnedHandle {
    #[inline]
    fn from(owned: Thing) -> Self {
        owned.filelike
    }
}

#[cfg(not(io_lifetimes_use_std))]
#[cfg(windows)]
impl FromHandle for Thing {
    #[inline]
    fn from_handle(filelike: OwnedHandle) -> Self {
        Self { filelike }
    }
}

#[cfg(windows)]
impl From<OwnedHandle> for Thing {
    #[inline]
    fn from(filelike: OwnedHandle) -> Self {
        Self { filelike }
    }
}

#[cfg(feature = "close")]
fn main() {
    use io_lifetimes::{AsFilelike, FromFilelike, IntoFilelike};

    // Minimally exercise `Thing`'s Posix-ish API.
    #[cfg(not(windows))]
    {
        let file = std::fs::File::open("Cargo.toml").unwrap();
        let thing = Thing::from_into_fd(file);
        let _ = thing.as_fd();
        let _: OwnedFd = thing.into();
    }

    // Minimally exercise `Thing`'s Windows API.
    #[cfg(windows)]
    {
        let file = std::fs::File::open("Cargo.toml").unwrap();
        let thing = Thing::from_into_handle(file);
        let _ = thing.as_handle();
        let _: OwnedHandle = thing.into();
    }

    // Implementing the above traits makes the blanket impls for the portable
    // `Filelike` traits available too.
    {
        let file = std::fs::File::open("Cargo.toml").unwrap();
        let thing = Thing::from_into_filelike(file);
        let _ = thing.as_filelike();
        let _ = thing.into_filelike();
    }
}

#[cfg(not(feature = "close"))]
fn main() {
    println!("This example requires the \"close\" feature.");
}
