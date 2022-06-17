use libc::c_char;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(C)]
pub enum GPVersionVerbosity {
    GP_VERSION_SHORT = 0,
    GP_VERSION_VERBOSE = 1,
}

extern "C" {
    pub fn gp_library_version() -> *const *const c_char;
}
