use std::{ptr::null_mut, ffi::c_void};

#[repr(C)]
pub struct _RenderFactory {
    entry: *const c_void,
}

#[link(name = "acurex")]
extern "stdcall" {
    fn CreateRenderFactory() -> *mut _RenderFactory;
}

pub struct RenderFactory {
    inner: *mut _RenderFactory
}

impl RenderFactory {
    pub fn create() -> Self {
        let factory = unsafe { CreateRenderFactory() };
        Self {
            inner: factory
        }
    }
}