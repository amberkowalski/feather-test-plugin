use std::ops::Deref;

#[cfg(feature = "host")]
pub use host::*;

#[cfg(feature = "host")]
use module::*;

#[cfg(not(feature = "host"))]
pub use module::*;

mod module {
    /// A type that allows Strings to be sent over FFI.
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct FFIString {
        ptr: *const u8,
        len: usize,
    }

    impl FFIString {
        pub unsafe fn from_borrow(string: &str) -> Self {
            Self {
                ptr: string.as_ptr(),
                len: string.len()
            }
        }
    }

    impl From<String> for crate::Pass<FFIString> {
        fn from(string: String) -> Self {
            let as_str_boxed = Box::new(string.as_str().as_bytes());
    
            crate::Pass(FFIString {
                len: as_str_boxed.len(),
                ptr: Box::into_raw(as_str_boxed) as *const u8,
            })
        }
    }
    
    impl From<&str> for crate::Pass<FFIString> {
        fn from(str: &str) -> Self {
            let as_str_boxed: Box<[u8]> = Box::from(str.as_bytes());
    
            crate::Pass(FFIString {
                len: as_str_boxed.len(),
                ptr: Box::into_raw(as_str_boxed) as *const u8,
            })
        }
    }

    /// A type that allows slices to be sent over FFI.
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct FFISlice<T: Copy + Clone> {
        len: usize,
        elements: *const [T]
    }

    impl<T> From<&[T]> for crate::Pass<FFISlice<T>> where T: Clone + Copy {
        fn from(from: &[T]) -> crate::Pass<FFISlice<T>> {
            let as_box: Box<[T]> = from.into();

            crate::Pass (
                FFISlice {
                    len: as_box.len(),
                    elements: Box::into_raw(as_box)
                }
            )
        }
    }

    /// A type that allows system definitions to be sent over FFI.
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct FFISystem {
        pub stage: crate::SystemStage,
        pub name: crate::Pass<FFIString>,
    }

    /// A type that defines a plugin's properties.
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct FFIPluginRegister {
        pub name: crate::Pass<FFIString>,
        pub version: crate::Pass<FFIString>,
        pub systems: crate::Pass<FFISlice<FFISystem>>,
    }
}

#[cfg(feature = "host")]
mod host {
    use wasmer::ValueType;
    use std::marker::PhantomData;

    pub trait WasmFree {
        fn free(self, memory: (), free_func: ());
    }

    unsafe impl ValueType for crate::SystemStage {}

    /// A type that allows Strings to be sent over FFI.
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct FFIString {
        ptr: u32,
        len: u32,
    }

    impl WasmFree for crate::Owned<FFIString> {
        fn free(self, memory: (), free_func: ()) {
            // Logic
            todo!();
        }
    }

    unsafe impl ValueType for FFIString {}

    /// A type that allows slices to be sent over FFI.
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct FFISlice<T: ValueType> {
        len: u32,
        elements: u32, // *const [T]
        _marker: PhantomData<T>
    }

    impl<T> WasmFree for crate::Owned<FFISlice<T>> where T: ValueType {
        fn free(self, memory: (), free_func: ()) {
            // Logic
            todo!();
        }
    }

    unsafe impl<T> ValueType for FFISlice<T> where T: ValueType {}

    /// A type that allows system definitions to be sent over FFI.
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct FFISystem {
        pub stage: crate::SystemStage,
        pub name: crate::Owned<FFIString>,
    }

    impl WasmFree for crate::Owned<FFISystem> {
        fn free(self, memory: (), free_func: ()) {
            // Logic
            todo!();
        }
    }

    unsafe impl ValueType for FFISystem {}

    /// A type that defines a plugin's properties.
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct FFIPluginRegister {
        pub name: crate::Owned<FFIString>,
        pub version: crate::Owned<FFIString>,
        pub systems: crate::Owned<FFISlice<FFISystem>>,
    }

    impl WasmFree for FFIPluginRegister {
        fn free(self, memory: (), free_func: ()) {
            self.name.free((), ());
            self.version.free((), ());
            self.systems.free((), ());

            // Iterate through the slice and free
            todo!();
        }
    }

    unsafe impl ValueType for FFIPluginRegister {}

    unsafe impl<T> ValueType for crate::Owned<T> where T: ValueType {}
    unsafe impl<T> ValueType for crate::Pass<T> where T: ValueType {}
    unsafe impl<T> ValueType for crate::Ref<T> where T: ValueType {}
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct Owned<T>(pub T);

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct Pass<T>(pub T);

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct Ref<T>(pub T);

/// C-Compatible representation of a system stage
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum SystemStage {
    Pre,
    Tick,
    SendPackets,
    CleanUp,
}


impl<T> Deref for Owned<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> Deref for Pass<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> Deref for Ref<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}