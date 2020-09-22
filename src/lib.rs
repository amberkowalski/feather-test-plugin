use feather_ffi::{
    PluginRegister, PluginSlice, PluginString, PluginSystem, PluginBox, PluginRef, SystemStage,
};

extern "C" {
    fn unsafe_print(string: *const PluginRef<PluginString>);
}

pub fn print(text: &str) {
    // Create an FFIString from the str and get a reference to it
    let ffi_string = unsafe { PluginString::from_borrow(&text) };

    unsafe { unsafe_print(&PluginRef(ffi_string)) };
}

#[no_mangle]
extern "C" fn __quill_free(ptr: *const u8, size: usize, align: usize) {
    use std::alloc::{dealloc, Layout};

    let layout = Layout::from_size_align(size, align).unwrap();

    print(format!("{:?}", layout).as_str());

    unsafe { dealloc(ptr as *mut u8, layout) };
}

#[no_mangle]
pub extern "C" fn __quill_setup() -> *const PluginBox<PluginRegister> {
    PluginRegister {
        name: "Testing Plugin".into(),
        version: "1.0.0".into(),
        systems: (&[PluginSystem {
            stage: SystemStage::Tick,
            name: "test_system".into(),
        }] as &[_])
            .into(),
    }
    .into()
}

#[no_mangle]
pub extern "C" fn test_system() {
    print("Plugin just ticked!")
}
