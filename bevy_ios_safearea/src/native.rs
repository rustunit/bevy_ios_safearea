use std::ffi;

extern "C" {
    pub(crate) fn swift_safearea_top(view: *mut ffi::c_void) -> ffi::c_float;
    pub(crate) fn swift_safearea_bottom(view: *mut ffi::c_void) -> ffi::c_float;
}
