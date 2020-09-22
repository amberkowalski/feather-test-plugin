#[cfg(feature = "host")]
pub use host::*;

#[cfg(feature = "host")]
use module::*;

#[cfg(not(feature = "host"))]
pub use module::*;

pub mod module {
    use std::ops::Deref;

    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct Owned<T: Clone + Copy>(pub T);

    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct Pass<T: Clone + Copy>(pub T);

    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct Ref<T: Clone + Copy>(pub T);

    impl<T: Clone + Copy> Deref for Owned<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }

    impl<T: Clone + Copy> Deref for Pass<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }

    impl<T: Clone + Copy> Deref for Ref<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }

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
                len: string.len(),
            }
        }
    }

    impl From<String> for Pass<FFIString> {
        fn from(string: String) -> Self {
            let as_str_boxed = Box::new(string.as_str().as_bytes());

            Pass(FFIString {
                len: as_str_boxed.len(),
                ptr: Box::into_raw(as_str_boxed) as *const u8,
            })
        }
    }

    impl From<&str> for Pass<FFIString> {
        fn from(str: &str) -> Self {
            let as_str_boxed: Box<[u8]> = Box::from(str.as_bytes());

            Pass(FFIString {
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
        elements: *const [T],
    }

    impl<T> From<&[T]> for Pass<FFISlice<T>>
    where
        T: Clone + Copy,
    {
        fn from(from: &[T]) -> Pass<FFISlice<T>> {
            let as_box: Box<[T]> = from.into();

            Pass(FFISlice {
                len: as_box.len(),
                elements: Box::into_raw(as_box),
            })
        }
    }

    /// A type that allows system definitions to be sent over FFI.
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct FFISystem {
        pub stage: crate::SystemStage,
        pub name: Pass<FFIString>,
    }

    /// A type that defines a plugin's properties.
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct FFIPluginRegister {
        pub name: Pass<FFIString>,
        pub version: Pass<FFIString>,
        pub systems: Pass<FFISlice<FFISystem>>,
    }
}

#[cfg(feature = "host")]
pub mod host {
    use std::marker::PhantomData;
    use wasmer::ValueType;
    use wasmer::{WasmPtr, NativeFunc, Memory};
    use std::alloc::{dealloc, Layout};

    use std::ops::Deref;

    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct Owned<T: ValueType>(pub T);

    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct Pass<T: ValueType>(pub T);

    #[repr(transparent)]
    #[derive(Copy, Clone, Debug)]
    pub struct Ref<T: ValueType>(pub T);

    impl<T: ValueType> Deref for Owned<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }

    impl<T: ValueType> Deref for Pass<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }

    impl<T: ValueType> Deref for Ref<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }

    pub trait WasmFree: ValueType {
        fn free(self, free_func: &NativeFunc<(WasmPtr<u8>, u32, u32)>);
    }

    unsafe impl ValueType for crate::SystemStage {}

    /// A type that allows Strings to be sent over FFI.
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct FFIString {
        pub ptr: u32,
        pub len: u32,
    }

    impl WasmFree for Owned<FFIString> {
        fn free(self, free_func: &NativeFunc<(WasmPtr<u8>, u32, u32)>) {
            let type_size = std::mem::size_of::<u8>() as u32;
            let type_align = std::mem::align_of::<FFIString>() as u32;

            // Adjust the layout to free the entire slice
            let free_size = type_size * self.len;

            free_func.call(WasmPtr::new(self.ptr), free_size as u32, type_align as u32).unwrap();

        }
    }

    unsafe impl ValueType for FFIString {}

    /// A type that allows slices to be sent over FFI.
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct FFISlice<T: ValueType> {
        pub len: u32,
        pub elements: u32, // *const [T]
        _marker: PhantomData<T>,
    }

    // Known Bug: things that are `WasmFree` wont get freed inside the FFISS

    impl<T> WasmFree for Owned<FFISlice<T>> where T: ValueType {
        fn free(self, free_func: &NativeFunc<(WasmPtr<u8>, u32, u32)>) {
            let type_size = std::mem::size_of::<T>() as u32;
            let type_align = std::mem::align_of::<T>() as u32;

            // Adjust the layout to free the entire slice
            let free_size = type_size * self.len;

            free_func.call(WasmPtr::new(self.elements), free_size as u32, type_align as u32).unwrap();
        }
    }

    unsafe impl<T> ValueType for FFISlice<T> where T: ValueType {}

    /// A type that allows system definitions to be sent over FFI.
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct FFISystem {
        pub stage: crate::SystemStage,
        pub name: Owned<FFIString>,
    }

    impl WasmFree for Owned<FFISystem> {
        fn free(self, free_func: &NativeFunc<(WasmPtr<u8>, u32, u32)>) {
            self.name.free(free_func)
        }
    }

    unsafe impl ValueType for FFISystem {}

    /// A type that defines a plugin's properties.
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct FFIPluginRegister {
        pub name: Owned<FFIString>,
        pub version: Owned<FFIString>,
        pub systems: Owned<FFISlice<FFISystem>>,
    }

    impl WasmFree for Owned<FFIPluginRegister> {
        fn free(self, free_func: &NativeFunc<(WasmPtr<u8>, u32, u32)>) {
            self.name.free(free_func);
            self.version.free(free_func);
            self.systems.free(free_func);
        }
    }

    impl Owned<FFIPluginRegister> {
        pub fn free_ptr_to(ptr: WasmPtr<Owned<FFIPluginRegister>>, free_func: &NativeFunc<(WasmPtr<u8>, u32, u32)>) {
            let type_size = std::mem::size_of::<Self>() as u32;
            let type_align = std::mem::align_of::<Self>() as u32;

            free_func.call(WasmPtr::new(ptr.offset()), type_size, type_align).unwrap()
        }
    }

    unsafe impl ValueType for FFIPluginRegister {}

    unsafe impl<T> ValueType for Owned<T> where T: ValueType {}
    unsafe impl<T> ValueType for Pass<T> where T: ValueType {}
    unsafe impl<T> ValueType for Ref<T> where T: ValueType {}
}

/// C-Compatible representation of a system stage
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum SystemStage {
    Pre,
    Tick,
    SendPackets,
    CleanUp,
}
