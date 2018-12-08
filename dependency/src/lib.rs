#![no_std]

#[cfg(feature = "broken")]
pub fn trigger() {
    submodule::call();
}

#[cfg(feature = "broken")]
mod submodule {
    pub(super) fn call() {
        #[link_section = "some-custom-section"]
        static SNIPPET: [u8; 3] = [b'X', b'Y', b'Z'];

        extern "C" {
            fn require_XYZ();
        }

        unsafe {
            require_XYZ();
        }
    }
}

#[cfg(feature = "working")]
pub fn trigger() {
    call();
}

#[cfg(feature = "working")]
fn call() {
    #[link_section = "some-custom-section"]
    static SNIPPET: [u8; 3] = [b'X', b'Y', b'Z'];

    extern "C" {
        fn require_XYZ();
    }

    unsafe {
        require_XYZ();
    }
}
