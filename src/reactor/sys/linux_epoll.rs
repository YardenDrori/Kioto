use std::io::Error;

use libc::__errno_location;

use crate::{error::KiotoError, reactor::error::ReactorError};

pub struct EpollEventQueue {
    epoll_obj_fd: i32,
}

impl EpollEventQueue {
    pub fn new() -> Result<Self, ReactorError> {
        unsafe {
            //EPOLL_CLOEXEC - if the process is overwritten eg fork exec free the epoll buffer
            let epoll_fd = libc::epoll_create1(libc::EPOLL_CLOEXEC);
            if epoll_fd == -1 {
                return Err(ReactorError::IoError(Error::last_os_error()));
            }
            Ok(EpollEventQueue { epoll_obj_fd: epoll_fd })
        }
    }
}
