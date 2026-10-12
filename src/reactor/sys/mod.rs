#[cfg(target_os = "linux")]
pub(super) mod linux_epoll;
#[cfg(target_os = "linux")]
pub(super) type EventQueue = linux_epoll::EpollEventQueue;

#[cfg(target_os = "macos")]
pub(super) mod macos_kqueue;
#[cfg(target_os = "macos")]
pub(super) type EventQueue = macos_kqueue::KqueueEventQueue;

#[cfg(target_os = "windows")]
pub(super) mod winbloat_iocp;
#[cfg(target_os = "windows")]
pub(super) type EventQueue = winbloat_iocp::IocpEventQueue;
