use feather_ffi::{FFIPluginRegister, Owned, FFISlice, FFIString, FFISystem, Pass, Ref, SystemStage};

extern "C" {
    fn unsafe_print(string: *const Ref<FFIString>);
}

pub fn print(text: &str) {
    // Create an FFIString from the str and get a reference to it
    let ffi_string = unsafe { FFIString::from_borrow(&text) };

    unsafe { unsafe_print(&Ref(ffi_string)) };
}

#[no_mangle]
extern "C" fn __quill_free(ptr: *const u8, size: usize, align: usize) {
    use std::alloc::{dealloc, Layout};

    let layout = Layout::from_size_align(size, align).unwrap();

    unsafe { dealloc(ptr as *mut u8, layout) };
}

static PLUGIN_NAME: &'static str = "Testing Plugin";
static PLUGIN_VERSION: &'static str = "1.0.0";

#[no_mangle]
pub extern "C" fn __quill_setup() -> *const Owned<FFIPluginRegister> {
    print("Yay!");

    let test_system_name = "Poggers";

    Box::into_raw(Box::new(Owned(FFIPluginRegister {
        name: PLUGIN_NAME.into(),
        version: PLUGIN_VERSION.into(),
        systems: ((&[FFISystem {
            stage: SystemStage::Tick,
            name: test_system_name.into(),
        }]) as &[FFISystem])
            .into(),
    })))
}

#[no_mangle]
pub extern "C" fn test_system() {
    print("Plugin just ticked!")
}
