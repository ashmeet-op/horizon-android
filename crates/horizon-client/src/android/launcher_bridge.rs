/// `HorizonInput.nativeMirrorEdit` (legacy package)
#[unsafe(no_mangle)]
extern "system" fn Java_com_ashmeet_hyperlauncher_utils_horizon_HorizonInput_nativeMirrorEdit(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    text: jni::objects::JString,
    cursor: i32,
    anchor: i32,
    generation: i32,
) {
    let Ok(text) = env.get_string(&text) else {
        return;
    };
    crate::android::mirror_edit(String::from(text), cursor, anchor, generation);
}

/// `HorizonInput.nativeMirrorAction` (legacy package)
#[unsafe(no_mangle)]
extern "system" fn Java_com_ashmeet_hyperlauncher_utils_horizon_HorizonInput_nativeMirrorAction(
    _env: *mut std::ffi::c_void,
    _class: *mut std::ffi::c_void,
) {
    crate::android::mirror_action();
}

/// `HorizonInput.nativeMirrorKey` (legacy package)
#[unsafe(no_mangle)]
extern "system" fn Java_com_ashmeet_hyperlauncher_utils_horizon_HorizonInput_nativeMirrorKey(
    _env: *mut std::ffi::c_void,
    _class: *mut std::ffi::c_void,
    code: i32,
) {
    crate::android::mirror_key(code);
}

/// `HorizonInput.nativeGetFps` (legacy package)
#[unsafe(no_mangle)]
extern "system" fn Java_com_ashmeet_hyperlauncher_utils_horizon_HorizonInput_nativeGetFps(
    _env: *mut std::ffi::c_void,
    _class: *mut std::ffi::c_void,
) -> i32 {
    crate::android::fps()
}

/// `HorizonInput.nativeSendKey`
#[unsafe(no_mangle)]
extern "system" fn Java_dev_horizon_android_HorizonInput_nativeSendKey(
    _env: *mut std::ffi::c_void,
    _class: *mut std::ffi::c_void,
    _key: i32,
    _is_down: jni::sys::jboolean,
    _mods: i32,
) {}

/// `HorizonInput.nativeSendMouseButton`
#[unsafe(no_mangle)]
extern "system" fn Java_dev_horizon_android_HorizonInput_nativeSendMouseButton(
    _env: *mut std::ffi::c_void,
    _class: *mut std::ffi::c_void,
    button: i32,
    is_down: jni::sys::jboolean,
) {
    let mb = match button {
        0 => winit::event::MouseButton::Left,
        1 => winit::event::MouseButton::Right,
        2 => winit::event::MouseButton::Middle,
        _ => winit::event::MouseButton::Left,
    };
    crate::android::push_mouse_event(crate::android::MouseEvent::Button(mb, is_down == 1));
}

/// `HorizonInput.nativeSendMouseMotion`
#[unsafe(no_mangle)]
extern "system" fn Java_dev_horizon_android_HorizonInput_nativeSendMouseMotion(
    _env: *mut std::ffi::c_void,
    _class: *mut std::ffi::c_void,
    dx: f32,
    dy: f32,
) {
    crate::android::push_mouse_event(crate::android::MouseEvent::Motion(f64::from(dx), f64::from(dy)));
}

/// `HorizonInput.nativeSendCursorPosition`
#[unsafe(no_mangle)]
extern "system" fn Java_dev_horizon_android_HorizonInput_nativeSendCursorPosition(
    _env: *mut std::ffi::c_void,
    _class: *mut std::ffi::c_void,
    x: f32,
    y: f32,
) {
    crate::android::push_mouse_event(crate::android::MouseEvent::Moved(f64::from(x), f64::from(y)));
}

/// `HorizonInput.nativeSendScroll`
#[unsafe(no_mangle)]
extern "system" fn Java_dev_horizon_android_HorizonInput_nativeSendScroll(
    _env: *mut std::ffi::c_void,
    _class: *mut std::ffi::c_void,
    dx: f32,
    dy: f32,
) {
    crate::android::push_mouse_event(crate::android::MouseEvent::Wheel(dx, dy));
}

/// `HorizonInput.nativeSendTouch`
#[unsafe(no_mangle)]
extern "system" fn Java_dev_horizon_android_HorizonInput_nativeSendTouch(
    _env: *mut std::ffi::c_void,
    _class: *mut std::ffi::c_void,
    pointer_id: i32,
    action: i32,
    x: f32,
    y: f32,
) {
    crate::android::push_touch(pointer_id, action, x, y);
}

/// `HorizonInput.nativeMirrorEdit`
#[unsafe(no_mangle)]
extern "system" fn Java_dev_horizon_android_HorizonInput_nativeMirrorEdit(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    text: jni::objects::JString,
    cursor: i32,
    anchor: i32,
    generation: i32,
) {
    let Ok(text) = env.get_string(&text) else {
        return;
    };
    crate::android::mirror_edit(String::from(text), cursor, anchor, generation);
}

/// `HorizonInput.nativeMirrorAction`
#[unsafe(no_mangle)]
extern "system" fn Java_dev_horizon_android_HorizonInput_nativeMirrorAction(
    _env: *mut std::ffi::c_void,
    _class: *mut std::ffi::c_void,
) {
    crate::android::mirror_action();
}

/// `HorizonInput.nativeMirrorKey`
#[unsafe(no_mangle)]
extern "system" fn Java_dev_horizon_android_HorizonInput_nativeMirrorKey(
    _env: *mut std::ffi::c_void,
    _class: *mut std::ffi::c_void,
    code: i32,
) {
    crate::android::mirror_key(code);
}

/// `HorizonInput.nativeGetFps`
#[unsafe(no_mangle)]
extern "system" fn Java_dev_horizon_android_HorizonInput_nativeGetFps(
    _env: *mut std::ffi::c_void,
    _class: *mut std::ffi::c_void,
) -> i32 {
    crate::android::fps()
}

/// `HorizonInput.nativeSetTouchUiEnabled`
#[unsafe(no_mangle)]
extern "system" fn Java_dev_horizon_android_HorizonInput_nativeSetTouchUiEnabled(
    _env: *mut std::ffi::c_void,
    _class: *mut std::ffi::c_void,
    enabled: jni::sys::jboolean,
) {
    crate::android::set_touch_ui_enabled(enabled != 0);
}
