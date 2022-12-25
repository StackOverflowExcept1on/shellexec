//! Library for allocating RWX memory on Unix and Windows
//!
//! ```
//! use virtual_memory::*;
//!
//! let buf = &[
//!     //mov eax, 1337
//!     0xb8, 0x39, 0x05, 0x00, 0x00,
//!     //ret
//!     0xc3,
//! ];
//!
//! let mut memory = VirtualMemory::new(buf.len()).expect("failed to allocate rwx memory");
//! memory.copy_from_slice(buf);
//!
//! let f: extern "C" fn() -> u32 = unsafe { std::mem::transmute(memory.as_ptr()) };
//! assert_eq!(f(), 1337);
//! ```

#![no_std]

use core::ops::{Deref, DerefMut};

/// Custom `Result` type that is used by this crate
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Describes possible errors, currently it's only memory allocation error
#[derive(Debug)]
pub enum Error {
    Allocation,
}

/// Wraps OS-specific functionality related to allocation RWX memory
pub struct VirtualMemory {
    ptr: *mut core::ffi::c_void,
    len: usize,
}

impl VirtualMemory {
    /// Trying to allocate RWX memory
    pub fn new(len: usize) -> Result<Self> {
        #[cfg(unix)]
        {
            let ptr = unsafe {
                libc::mmap(
                    core::ptr::null_mut(),
                    len,
                    libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
                    libc::MAP_PRIVATE | libc::MAP_ANON,
                    -1,
                    0,
                )
            };

            if ptr == libc::MAP_FAILED {
                return Err(Error::Allocation);
            }

            Ok(Self { ptr, len })
        }

        #[cfg(windows)]
        {
            use windows_sys::Win32::System::Memory::{
                VirtualAlloc, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE,
            };

            let ptr = unsafe {
                VirtualAlloc(
                    core::ptr::null(),
                    len,
                    MEM_COMMIT | MEM_RESERVE,
                    PAGE_EXECUTE_READWRITE,
                )
            };

            if ptr.is_null() {
                return Err(Error::Allocation);
            }

            Ok(Self { ptr, len })
        }
    }

    /// Returns pointer to this memory
    #[inline]
    pub fn ptr(&self) -> *const u8 {
        self.ptr as *const u8
    }

    /// Returns mutable pointer to this memory
    #[inline]
    pub fn mut_ptr(&mut self) -> *mut u8 {
        self.ptr as *mut u8
    }

    /// Returns length of allocated memory
    #[inline]
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.len
    }
}

/// Allows to pass `VirtualMemory` as `&[u8]`
impl Deref for VirtualMemory {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { core::slice::from_raw_parts(self.ptr as _, self.len) }
    }
}

/// Allows to pass `VirtualMemory` as `&mut [u8]`
impl DerefMut for VirtualMemory {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::slice::from_raw_parts_mut(self.ptr as _, self.len) }
    }
}

/// Acts as destructor (OS-specific)
impl Drop for VirtualMemory {
    fn drop(&mut self) {
        #[cfg(unix)]
        unsafe {
            libc::munmap(self.ptr, self.len);
        }

        #[cfg(windows)]
        unsafe {
            use windows_sys::Win32::System::Memory::{VirtualFree, MEM_RELEASE};

            VirtualFree(self.ptr, 0, MEM_RELEASE);
        }
    }
}
