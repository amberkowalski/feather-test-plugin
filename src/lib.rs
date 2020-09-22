use quill_internals::raw::{
    LogLevel, PluginBox, PluginRegister, PluginSystem,
    SystemStage,
};

use quill_internals::module_externs::log;

#[no_mangle]
extern "C" fn __quill_free(ptr: *const u8, size: usize, align: usize) {
    use std::alloc::{dealloc, Layout};

    let layout = Layout::from_size_align(size, align).unwrap();

    log(LogLevel::Info, &format!("{:?}", layout).as_str());

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
    log(LogLevel::Debug, &"Plugin just ticked!")
}
