//! Async translation session example.
//!
//! Demonstrates `AsyncTranslationSession` using `pollster::block_on`.
//! The actual translation requires macOS 26+. On earlier systems the
//! example prints a graceful skip message and exits 0.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "macos")]
    {
        use translation::async_api::AsyncTranslationSession;
        use translation::{TranslationSession, TranslationSessionConfiguration};

        if !is_macos_26_or_later() {
            println!("SKIP: AsyncTranslationSession requires macOS 26+");
            return Ok(());
        }

        let config = TranslationSessionConfiguration::new("en", "fr");
        let session = match TranslationSession::new(config) {
            Ok(session) => session,
            Err(error) => {
                println!("SKIP: could not create session: {error}");
                return Ok(());
            }
        };
        let async_session = AsyncTranslationSession::new(&session);
        let future = async_session.translate("Hello, world!")?;
        match block_on_with_main_run_loop(future) {
            Ok(response) => println!("Translated: {}", response.target_text()),
            Err(error) => println!("INFO: translation unavailable: {error}"),
        }
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

fn is_macos_26_or_later() -> bool {
    std::process::Command::new("sw_vers")
        .arg("-productVersion")
        .output()
        .ok()
        .and_then(|output| {
            let version = String::from_utf8_lossy(&output.stdout);
            version.trim().split('.').next()?.parse::<u32>().ok()
        })
        .is_some_and(|major| major >= 26)
}
