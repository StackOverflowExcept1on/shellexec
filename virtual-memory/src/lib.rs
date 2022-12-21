#![no_std]

use core::ops::{Deref, DerefMut};

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    Allocation,
}

pub struct VirtualMemory {
    ptr: *mut core::ffi::c_void,
    len: usize,
}

impl VirtualMemory {
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

    #[inline]
    pub fn ptr(&self) -> *const u8 {
        self.ptr as *const u8
    }

    #[inline]
    pub fn mut_ptr(&mut self) -> *mut u8 {
        self.ptr as *mut u8
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }
}

impl Deref for VirtualMemory {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { core::slice::from_raw_parts(self.ptr(), self.len()) }
    }
}

impl DerefMut for VirtualMemory {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::slice::from_raw_parts_mut(self.mut_ptr(), self.len()) }
    }
}

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