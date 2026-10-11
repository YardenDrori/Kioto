pub mod babys_first_reactor;

#[cfg(target_os = "linux")]
pub mod linux_syscall;

#[cfg(target_os = "macos")]
pub mod macos_syscall;

#[cfg(target_os = "windows")]
pub mod winbloat_syscall;
