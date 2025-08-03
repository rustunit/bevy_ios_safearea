use crate::IosSafeAreaResource;

/// Get the safe area insets for Android.
#[cfg(target_os = "android")]
pub(crate) fn try_get_safe_area() -> Option<IosSafeAreaResource> {
    use jni::{
        objects::JObject,
        sys::{_jobject, JNIInvokeInterface_},
    };

    let android_app = bevy::window::ANDROID_APP.get()?;
    let vm = unsafe {
        jni::JavaVM::from_raw(android_app.vm_as_ptr() as *mut *const JNIInvokeInterface_).ok()?
    };
    let activity = unsafe { JObject::from_raw(android_app.activity_as_ptr() as *mut _jobject) };
    let mut env = vm.attach_current_thread().ok()?;

    // Get the Window from the Activity
    let window = env
        .call_method(&activity, "getWindow", "()Landroid/view/Window;", &[])
        .ok()?;

    let window_obj = window.l().ok()?;

    // Get the DecorView from the Window
    let decor_view = env
        .call_method(&window_obj, "getDecorView", "()Landroid/view/View;", &[])
        .ok()?;

    let decor_view_obj = decor_view.l().ok()?;

    // Get the root window insets
    let root_insets = env
        .call_method(
            &decor_view_obj,
            "getRootWindowInsets",
            "()Landroid/view/WindowInsets;",
            &[],
        )
        .ok()?;

    let insets_obj = root_insets.l().ok()?;

    // Get system window insets (for API level 20+)
    let top = env
        .call_method(&insets_obj, "getSystemWindowInsetTop", "()I", &[])
        .ok()?
        .i()
        .ok()?;
    let bottom = env
        .call_method(&insets_obj, "getSystemWindowInsetBottom", "()I", &[])
        .ok()?
        .i()
        .ok()?;
    let left = env
        .call_method(&insets_obj, "getSystemWindowInsetLeft", "()I", &[])
        .ok()?
        .i()
        .ok()?;
    let right = env
        .call_method(&insets_obj, "getSystemWindowInsetRight", "()I", &[])
        .ok()?
        .i()
        .ok()?;

    Some(IosSafeAreaResource {
        top: top as f32,
        bottom: bottom as f32,
        left: left as f32,
        right: right as f32,
    })
}
