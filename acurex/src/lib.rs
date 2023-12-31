#[cfg(target_os = "windows")]
pub mod win32;

use ash::Entry;

#[repr(C)]
pub enum AXResult {

}

#[repr(transparent)]
pub struct _Acurex {
    id: u32,
}

pub enum Color {
    ARGB(u8, u8, u8, u8),
}

#[repr(C)]
pub enum Command {
    Clear(Color),
}

#[repr(C)]
pub struct _RenderFactory {
    entry: *const Entry,
}

struct _Entry {
    e: Entry
}

#[no_mangle]
pub extern "C" fn CreateRenderFactory() -> *mut _RenderFactory {
    let entry = Entry::linked();

    let factory = _RenderFactory {
        entry: &entry
    };

    Box::into_raw(Box::new(factory))
}

#[no_mangle]
pub extern "C" fn FillRectangle(acurex: *mut Acurex) {

}
