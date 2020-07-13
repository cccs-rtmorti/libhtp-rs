use crate::{list::List, Status};

extern "C" {
    #[no_mangle]
    fn calloc(_: libc::size_t, _: libc::size_t) -> *mut core::ffi::c_void;
    #[no_mangle]
    fn free(__ptr: *mut core::ffi::c_void);
}

#[derive(Clone)]
pub struct htp_hook_t {
    pub callbacks: List<*mut core::ffi::c_void>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_callback_t {
    pub fn_0: htp_callback_fn_t,
}
pub type htp_callback_fn_t = Option<unsafe extern "C" fn(_: *mut core::ffi::c_void) -> Status>;

/// Creates a new hook.
///
/// Returns New htp_hook_t structure on success, NULL on failure.
pub unsafe fn htp_hook_create() -> *mut htp_hook_t {
    let mut hook: *mut htp_hook_t =
        calloc(1, ::std::mem::size_of::<htp_hook_t>()) as *mut htp_hook_t;
    if hook.is_null() {
        return 0 as *mut htp_hook_t;
    }
    (*hook).callbacks = List::with_capacity(4);
    hook
}

/// Destroys an existing hook. It is all right to send a NULL
/// to this method because it will simply return straight away.
pub unsafe fn htp_hook_destroy(hook: *mut htp_hook_t) {
    if hook.is_null() {
        return;
    }
    for each in &(*hook).callbacks {
        free(*each);
    }
    drop(&(*hook).callbacks);
    free(hook as *mut core::ffi::c_void);
}

/// Registers a new callback with the hook.
///
/// Returns HTP_OK on success, HTP_ERROR on memory allocation error.
pub unsafe fn htp_hook_register(
    hook: *mut *mut htp_hook_t,
    callback_fn: htp_callback_fn_t,
) -> Status {
    if hook.is_null() {
        return Status::ERROR;
    }
    let mut callback: *mut htp_callback_t =
        calloc(1, ::std::mem::size_of::<htp_callback_t>()) as *mut htp_callback_t;
    if callback.is_null() {
        return Status::ERROR;
    }
    (*callback).fn_0 = callback_fn;
    // Create a new hook if one does not exist
    if (*hook).is_null() {
        *hook = htp_hook_create();
        if (*hook).is_null() {
            free(callback as *mut core::ffi::c_void);
            return Status::ERROR;
        }
    }
    // Add callback
    (**hook).callbacks.push(callback as *mut core::ffi::c_void);
    Status::OK
}

/// Runs all the callbacks associated with a given hook. Only stops if
/// one of the callbacks returns an error (HTP_ERROR) or stop (HTP_STOP).
///
/// Returns HTP_OK if at least one hook ran successfully, HTP_STOP if there was
///         no error but processing should stop, and HTP_ERROR or any other value
///         less than zero on error.
pub unsafe fn htp_hook_run_all(
    hook: *const htp_hook_t,
    user_data: *mut core::ffi::c_void,
) -> Status {
    if hook.is_null() {
        return Status::OK;
    }
    // Loop through the registered callbacks, giving each a chance to run.
    for each in &(*hook).callbacks {
        let callback = (*each) as *mut htp_callback_t;
        let rc: Status = (*callback).fn_0.expect("non-null function pointer")(user_data);
        // A hook can return HTP_OK to say that it did some work,
        // or HTP_DECLINED to say that it did no work. Anything else
        // is treated as an error.
        if rc != Status::OK && rc != Status::DECLINED {
            return rc;
        }
    }
    Status::OK
}
