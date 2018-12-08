#![no_std]

#[panic_handler]
fn panic_handler( _: &core::panic::PanicInfo ) -> ! {
    loop {}
}

#[no_mangle]
pub fn main() {
    dependency::trigger();
}
