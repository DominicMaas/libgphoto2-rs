use libc::c_void;

/// The gphoto context structure.
///
/// This structure allows callback handling, passing error contexts back,
/// progress handling and download cancellation and similar things.
/// It is usually passed around the functions.
#[repr(C)]
pub struct GPContext {
    __private: c_void,
}

/// Return codes that can be returned by progress handling.
///
/// An application can return special values back to the libgphoto2
/// progress callback handling functions. If "Cancel" is selected,
/// libgphoto2 and the camera driver will try to cancel transfer.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(C)]
pub enum GPContextFeedback {
    /// Everything ok... proceed.
    GP_CONTEXT_FEEDBACK_OK = 0,
    /// Please cancel the current transfer if possible.
    GP_CONTEXT_FEEDBACK_CANCEL = 1,
}

extern "C" {
    pub fn gp_context_new() -> *mut GPContext;

    pub fn gp_context_ref(context: *mut GPContext);
    pub fn gp_context_unref(context: *mut GPContext);
}
