//! Async language availability example.
//!
//! Demonstrates `AsyncLanguageAvailability` using `pollster::block_on`.
//! Requires macOS 15+. Skips gracefully on older systems.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "macos")]
    {
        use translation::async_api::AsyncLanguageAvailability;
        use translation::{Language, LanguageAvailability};

        let availability = match LanguageAvailability::new() {
            Ok(availability) => availability,
            Err(error) => {
                println!("SKIP: could not create LanguageAvailability: {error}");
                return Ok(());
            }
        };
        let async_availability = AsyncLanguageAvailability::new(&availability);

        let languages = block_on_with_main_run_loop(async_availability.supported_languages())?;
        println!("Supported languages count: {}", languages.len());
        if !languages.is_empty() {
            let preview_len = languages.len().min(5);
            println!("First few: {:?}", &languages[..preview_len]);
        }

        let en = Language::from("en");
        let fr = Language::from("fr");
        let status_future = async_availability.status(&en, Some(&fr))?;
        let status = block_on_with_main_run_loop(status_future)?;
        println!("en→fr status: {status:?}");
    }

    #[cfg(not(target_os = "macos"))]
    println!("SKIP: Translation.framework is macOS-only");

    Ok(())
}

#[cfg(target_os = "macos")]
fn block_on_with_main_run_loop<F, T>(future: F) -> T
where
    F: std::future::Future<Output = T> + Send,
    T: Send,
{
    use std::ffi::c_void;
    use std::sync::mpsc::sync_channel;

    #[link(name = "CoreFoundation", kind = "framework")]
    unsafe extern "C" {
        static kCFRunLoopDefaultMode: *const c_void;
        fn CFRunLoopRunInMode(
            mode: *const c_void,
            seconds: f64,
            return_after_source_handled: u8,
        ) -> i32;
    }

    let (tx, rx) = sync_channel(1);
    std::thread::scope(|scope| {
        scope.spawn(move || if tx.send(pollster::block_on(future)).is_err() {});

        loop {
            if let Ok(result) = rx.try_recv() {
                break result;
            }
            unsafe {
                let _ = CFRunLoopRunInMode(kCFRunLoopDefaultMode, 0.01, 1);
            }
        }
    })
}
