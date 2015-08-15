use std::default::Default;
use std::ffi::CString;

use nix::sys::signal::{SigNum, SIGKILL};
use libc::{uid_t, gid_t, c_int};

use idmap::{UidMapSetter, GidMapSetter};
use chroot::Pivot;


pub struct Config {
    pub death_sig: Option<SigNum>,
    pub work_dir: Option<CString>,
    pub chroot_dir: Option<CString>,
    pub pivot_root: Option<Pivot>,  // TODO(tailhook) related to chroot_dir
    pub uid: Option<uid_t>,
    pub gid: Option<gid_t>,
    pub supplementary_gids: Option<Vec<gid_t>>,
    pub namespaces: Option<c_int>,
    pub uid_map: Option<UidMapSetter>,
    pub gid_map: Option<GidMapSetter>,
    // TODO(tailhook) stdin/stdout/stderr file descriptors
    // TODO(tailhook) sigmasks
    // TODO(tailhook) wakeup/error pipe
    // TODO(tailhook) session leader
}

impl Default for Config {
    fn default() -> Config {
        Config {
            death_sig: Some(SIGKILL),
            work_dir: None,
            chroot_dir: None,
            pivot_root: None,
            uid: None,
            gid: None,
            supplementary_gids: None,
            namespaces: None,
            uid_map: None,
            gid_map: None,
        }
    }
}
