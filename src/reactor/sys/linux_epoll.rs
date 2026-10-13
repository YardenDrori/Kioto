use std::{fs::File, io::Error, os::fd::FromRawFd};

use crate::reactor::error::ReactorError;

pub struct EpollEventQueue {
    epoll_fd: File,
}

impl EpollEventQueue {
    pub fn new() -> Result<Self, ReactorError> {
        unsafe {
            //EPOLL_CLOEXEC - if the process is overwritten eg fork exec free the epoll buffer
            let epoll_fd = libc::epoll_create1(libc::EPOLL_CLOEXEC);
            if epoll_fd == -1 {
                return Err(ReactorError::IoError(Error::last_os_error()));
            }

            //we convert the raw fd to std's wrapper to use its Drop trait to free the memory when the
            //epoll goes out of scope
            let file = File::from_raw_fd(epoll_fd);
            Ok(EpollEventQueue { epoll_fd: file })
        }
    }

    pub fn register_event() -> Result<(), ReactorError> {
        todo!()
    }

    pub fn wait_until_event_ready() {}
}
