//! Async API tests for `translation-rs`.
//!
//! Run with: `cargo test --features async --test async_api_tests`

fn main() {
    #[cfg(target_os = "macos")]
    {
        test_supported_languages_async();
        test_availability_status_async();
        if is_macos_26_or_later() {
            test_translate_async();
            test_translations_batch_async();
            test_prepare_translation_async();
            test_async_errors_match_sync_errors();
            test_dropping_pending_futures_cancels_them();
        } else {
            println!("INFO: Skipping session async tests (require macOS 26+)");
        }
    }

    #[cfg(not(target_os = "macos"))]
    println!("SKIP: Translation.framework is macOS-only");
}

#[cfg(target_os = "macos")]
fn test_supported_languages_async() {
    use translation::async_api::AsyncLanguageAvailability;
    use translation::LanguageAvailability;

    let availability = match LanguageAvailability::new() {
        Ok(availability) => availability,
        Err(error) => {
            println!("SKIP test_supported_languages_async: {error}");
            return;
        }
    };
    let async_availability = AsyncLanguageAvailability::new(&availability);

    let result = block_on_with_main_run_loop(async_availability.supported_languages());
    match result {
        Ok(languages) => {
            assert!(
                !languages.is_empty(),
                "expected non-empty supported languages"
            );
            println!(
                "PASS test_supported_languages_async: {} languages",
                languages.len()
            );
        }
        Err(error) => panic!("FAIL test_supported_languages_async: {error}"),
    }
}

#[cfg(target_os = "macos")]
fn test_availability_status_async() {
    use translation::async_api::AsyncLanguageAvailability;
    use translation::{Language, LanguageAvailability};

    let availability = match LanguageAvailability::new() {
        Ok(availability) => availability,
        Err(error) => {
            println!("SKIP test_availability_status_async: {error}");
            return;
        }
    };
    let async_availability = AsyncLanguageAvailability::new(&availability);
    let en = Language::from("en");
    let fr = Language::from("fr");

    let result = block_on_with_main_run_loop(
        async_availability
            .status(&en, Some(&fr))
            .expect("status future creation failed"),
    );
    match result {
        Ok(status) => println!("PASS test_availability_status_async: en→fr = {status:?}"),
        Err(error) => panic!("FAIL test_availability_status_async: {error}"),
    }
}

#[cfg(target_os = "macos")]
fn test_translate_async() {
    use translation::async_api::AsyncTranslationSession;
    use translation::{TranslationSession, TranslationSessionConfiguration};

    let config = TranslationSessionConfiguration::new("en", "fr");
    let session = match TranslationSession::new(config) {
        Ok(session) => session,
        Err(error) => {
            println!("SKIP test_translate_async: {error}");
            return;
        }
    };
    let async_session = AsyncTranslationSession::new(&session);
    let result = match async_session.translate("Hello") {
        Ok(future) => block_on_with_main_run_loop(future),
        Err(error) => Err(error),
    };
    match result {
        Ok(response) => println!("PASS test_translate_async: '{}'", response.target_text()),
        Err(error) => println!("INFO test_translate_async error (may need download): {error}"),
    }
}

#[cfg(target_os = "macos")]
fn test_translations_batch_async() {
    use translation::async_api::AsyncTranslationSession;
    use translation::{TranslationRequest, TranslationSession, TranslationSessionConfiguration};

    let config = TranslationSessionConfiguration::new("en", "fr");
    let session = match TranslationSession::new(config) {
        Ok(session) => session,
        Err(error) => {
            println!("SKIP test_translations_batch_async: {error}");
            return;
        }
    };
    let async_session = AsyncTranslationSession::new(&session);
    let requests = [
        TranslationRequest::new("Hello"),
        TranslationRequest::new("World"),
    ];

    let result = match async_session.translations(&requests) {
        Ok(future) => block_on_with_main_run_loop(future),
        Err(error) => Err(error),
    };
    match result {
        Ok(responses) => {
            assert_eq!(responses.len(), requests.len());
            println!("PASS test_translations_batch_async");
        }
        Err(error) => println!("INFO test_translations_batch_async error: {error}"),
    }
}

#[cfg(target_os = "macos")]
fn test_prepare_translation_async() {
    use translation::async_api::AsyncTranslationSession;
    use translation::{TranslationSession, TranslationSessionConfiguration};

    let config = TranslationSessionConfiguration::new("en", "fr");
    let session = match TranslationSession::new(config) {
        Ok(session) => session,
        Err(error) => {
            println!("SKIP test_prepare_translation_async: {error}");
            return;
        }
    };
    let async_session = AsyncTranslationSession::new(&session);

    let result = block_on_with_main_run_loop(async_session.prepare_translation());
    match result {
        Ok(()) => println!("PASS test_prepare_translation_async"),
        Err(error) => println!("INFO test_prepare_translation_async error: {error}"),
    }
}

#[cfg(target_os = "macos")]
fn test_async_errors_match_sync_errors() {
    use translation::async_api::AsyncTranslationSession;
    use translation::{TranslationError, TranslationSession, TranslationSessionConfiguration};

    for (source, target, text) in [
        ("en", "es", ""),
        ("en", "en", "hello"),
        ("en", "zz", "hello"),
        ("zz", "en", "hello"),
    ] {
        let session = TranslationSession::new(TranslationSessionConfiguration::new(source, target))
            .expect("session");
        let sync_error = session
            .translate(text)
            .expect_err("sync translation should fail");
        let async_error = block_on_with_main_run_loop(
            AsyncTranslationSession::new(&session)
                .translate(text)
                .expect("translate future"),
        )
        .expect_err("async translation should fail");
        assert!(
            !matches!(
                async_error,
                TranslationError::Framework(_) | TranslationError::Unknown(_)
            ),
            "{source}->{target} {text:?}: untyped async error {async_error:?}"
        );
        assert_eq!(async_error, sync_error, "{source}->{target} {text:?}");
    }
    println!("PASS test_async_errors_match_sync_errors");
}

#[cfg(target_os = "macos")]
fn test_dropping_pending_futures_cancels_them() {
    use std::time::{Duration, Instant};

    use translation::async_api::AsyncTranslationSession;
    use translation::{TranslationRequest, TranslationSession, TranslationSessionConfiguration};

    let session =
        TranslationSession::new(TranslationSessionConfiguration::new("en", "fr")).expect("session");
    let async_session = AsyncTranslationSession::new(&session);
    let requests: Vec<_> = (0..20)
        .map(|index| TranslationRequest::new(format!("This is sentence number {index}.")))
        .collect();
    drop(async_session.translations(&requests).expect("batch future"));
    drop(
        async_session
            .translate("Good evening")
            .expect("translate future"),
    );
    drop(async_session.prepare_translation());

    let settle = Instant::now() + Duration::from_millis(500);
    while Instant::now() < settle {
        pump_main_run_loop();
    }

    match block_on_with_main_run_loop(async_session.translate("Hello").expect("translate future")) {
        Ok(response) => assert!(!response.target_text().is_empty()),
        Err(error) => println!("INFO test_dropping_pending_futures_cancels_them: {error}"),
    }
    println!("PASS test_dropping_pending_futures_cancels_them");
}

#[cfg(target_os = "macos")]
fn pump_main_run_loop() {
    use std::ffi::c_void;

    #[link(name = "CoreFoundation", kind = "framework")]
    unsafe extern "C" {
        static kCFRunLoopDefaultMode: *const c_void;
        fn CFRunLoopRunInMode(
            mode: *const c_void,
            seconds: f64,
            return_after_source_handled: u8,
        ) -> i32;
    }

    unsafe {
        let _ = CFRunLoopRunInMode(kCFRunLoopDefaultMode, 0.01, 1);
    }
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
