use feather_ffi::{HostPluginRegister, HostSystem, HostSystems, SystemStage};

extern "C" {
    fn print(ptr: *const u8, len: u32);
}

#[no_mangle]
extern "C" fn __quill_free(ptr: *mut u8) {
    unsafe { drop(Box::from_raw(ptr)) };
}

static PLUGIN_NAME: &'static str = "Testing Plugin";
static PLUGIN_VERSION: &'static str = "1.0.0";

#[no_mangle]
pub extern "C" fn __quill_setup() -> *const HostPluginRegister {
    let hello = "Hello from a Plugin!";
    unsafe {
        print(hello.as_ptr(), hello.len() as u32);
    }

    Box::into_raw(Box::new(HostPluginRegister {
        name: PLUGIN_NAME.into(),
        version: PLUGIN_VERSION.into(),
        systems: HostSystems {
            len: 1,
            systems: &[HostSystem {
                stage: SystemStage::Tick,
                len: "test_system".len() as u32,
                name: "test_system".as_bytes(),
            }],
        },
    }))
}

#[no_mangle]
pub extern "C" fn test_system() {
    let to_print = "Plugin just ticked, awesome!";

    unsafe { print(to_print.as_ptr(), to_print.len() as u32) }
}
