use feather_ffi::{FFIPluginRegister, FFISlice, FFIString, FFISystem, Pass, Ref, SystemStage};

extern "C" {
    fn unsafe_print(string: Ref<FFIString>);
}

pub fn print(text: &str) {
    // Create an FFIString from the str and get a reference to it
    let ffi_string = unsafe { FFIString::from_borrow(&text) };

    unsafe { unsafe_print(Ref(ffi_string)) };
}

#[no_mangle]
extern "C" fn __quill_free(ptr: *mut u8) {
    unsafe { drop(Box::from_raw(ptr)) };
}

static PLUGIN_NAME: &'static str = "Testing Plugin";
static PLUGIN_VERSION: &'static str = "1.0.0";

#[no_mangle]
pub extern "C" fn __quill_setup() -> Pass<*const FFIPluginRegister> {
    print("Yay!");

    let test_system_name = "Poggers";

    Pass(Box::into_raw(Box::new(FFIPluginRegister {
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
