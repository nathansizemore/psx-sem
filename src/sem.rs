// Copyright 2023 Nathan Sizemore <nathanrsizemore@gmail.com>
//
// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL was not
// distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.

use std::{ffi::CString, io};

use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct OpenOptions: libc::c_int {
        /// Create if not exists.
        const CREATE = libc::O_CREAT;
        /// Open for read.
        const READ = libc::O_RDONLY;
        /// Open for write.
        const WRITE = libc::O_WRONLY;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct OpenMode: libc::mode_t {
        /// User read.
        const R_USR = libc::S_IRUSR;
        /// User write.
        const W_USR = libc::S_IWUSR;
        /// Group read.
        const R_GRP = libc::S_IRGRP;
        /// Group write.
        const W_GRP = libc::S_IWGRP;
        /// Other read.
        const R_OTH = libc::S_IROTH;
        /// Other write.
        const W_OTH = libc::S_IWOTH;
    }
}

#[derive(Debug)]
pub struct Sem {
    ptr: *mut libc::sem_t,
}

impl Sem {
    /// Opens a semaphore at `name`.
    pub fn open(
        name: &str,
        oflags: OpenOptions,
        mode: OpenMode,
        initial: usize,
    ) -> io::Result<Self> {
        let cstr = CString::new(name).unwrap();
        let r = unsafe {
            libc::sem_open(
                cstr.as_ptr(),
                oflags.bits(),
                mode.bits() as libc::c_uint,
                initial,
            )
        };
        if r == libc::SEM_FAILED {
            return Err(io::Error::last_os_error());
        }

        Ok(Self {
            ptr: r as *mut libc::sem_t,
        })
    }

    /// Increments the semaphore.
    pub fn post(&self) -> io::Result<()> {
        let r = unsafe { libc::sem_post(self.ptr) };
        if r != 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    /// Decrements the semaphore.
    pub fn wait(&self) -> io::Result<()> {
        let r = unsafe { libc::sem_wait(self.ptr) };
        if r != 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }
}

impl Drop for Sem {
    fn drop(&mut self) {
        unsafe { libc::sem_close(self.ptr) };
    }
}

unsafe impl Send for Sem {}
unsafe impl Sync for Sem {}
