//! Reads `-[UIView safeAreaInsets]` straight from UIKit via `objc2-ui-kit`'s
//! generated `UIView` bindings. Earlier versions of this crate called into a
//! companion Swift Package for this (`@_cdecl` symbols the consumer's Xcode
//! project had to add via SPM); `objc2-ui-kit` exposes the same UIKit method
//! as a plain Rust binding, so that whole tandem is gone.
//!
//! UIKit is main-thread-only: `-safeAreaInsets` (like nearly everything on
//! `UIView`) is undefined behaviour to call off the main thread. This module
//! does not re-check that itself — its one caller, `plugin::init`, is a Bevy
//! system with a `NonSendMarker` parameter, and Bevy only ever schedules
//! `NonSend` systems on the main thread, so the invariant is already upheld
//! by the time execution reaches here.

use objc2_ui_kit::UIView;

/// The four safe-area insets, in points, as returned by
/// `-[UIView safeAreaInsets]`.
pub(crate) struct Insets {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

/// Reads the safe-area insets of the `UIView` at `ui_view`.
///
/// # Safety
///
/// `ui_view` must point at a live, valid `UIView` — as handed back by
/// winit's `RawWindowHandle::UiKit(handle).ui_view` — and the calling thread
/// must be the main thread (see the module docs above).
pub(crate) unsafe fn safe_area_insets(ui_view: *mut core::ffi::c_void) -> Insets {
    // SAFETY: caller upholds both preconditions above. `UIView*` is a valid
    // `objc2_ui_kit::UIView` pointer by construction (that's what winit's
    // UiKit window handle contains), and this borrow does not outlive the call.
    let view: &UIView = unsafe { &*ui_view.cast::<UIView>() };
    let raw = view.safeAreaInsets();
    Insets {
        top: raw.top as f32,
        bottom: raw.bottom as f32,
        left: raw.left as f32,
        right: raw.right as f32,
    }
}
