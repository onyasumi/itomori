use crate::{cpu, println};
use core::panic::PanicInfo;


/// Prevent panic(&PanicInfo) from being called a second time
/// 
/// Returns normally when called for the first time, loops continuously for all future calls
fn panic_prevent_reenter() {
    use core::sync::atomic::{AtomicBool, Ordering};

    #[cfg(not(target_arch = "aarch64"))]
    compile_error!("Add the target_arch to above's check if the following code is safe to use");

    static PANIC_IN_PROGRESS: AtomicBool = AtomicBool::new(false);

    if !PANIC_IN_PROGRESS.load(Ordering::Relaxed) {
        PANIC_IN_PROGRESS.store(true, Ordering::Relaxed);

        return;
    }

    cpu::wait_forever()
}


/// Unwinds stack and loops continuously
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {

    panic_prevent_reenter();

    let (location, line, column) = match info.location() {
        Some(loc) => (loc.file(), loc.line(), loc.column()),
        _ => ("???", 0, 0),
    };

    println!(
        "Kernel panic!\n\n\
        Panic location:\n      File '{}', line {}, column {}\n\n\
        {}",
        location,
        line,
        column,
        info.message().unwrap_or(&format_args!("")),
    );


    cpu::wait_forever()
}