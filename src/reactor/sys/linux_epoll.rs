use crate::error::KiotoError;

pub struct EpollEventQueue {
    epoll_obj_fd: i32,
}

fn convert_error_codes(code: i32) -> KiotoError {
    todo!()
}

impl EpollEventQueue {
    pub fn new() -> Self {
        unsafe {
            //EPOLL_CLOEXEC - if the process is overwritten eg fork exec free the epoll buffer
            let epoll_fd = libc::epoll_create1(libc::EPOLL_CLOEXEC);
            EpollEventQueue { epoll_obj_fd: epoll_fd }
        }
    }
}
