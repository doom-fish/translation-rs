use std::ffi::c_void;
use std::sync::mpsc;
use std::thread;

use translation::{
    TranslationError, TranslationRequest, TranslationSession, TranslationSessionConfiguration,
};

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    static kCFRunLoopDefaultMode: *const c_void;
    fn CFRunLoopRunInMode(mode: *const c_void, seconds: f64, return_after_source: u8) -> i32;
}

const SOURCES: [&str; 2] = ["hello world", "good morning"];

fn stream_after_a_timeout(started: &mpsc::Sender<()>) -> Result<Vec<String>, TranslationError> {
    let session = TranslationSession::new(TranslationSessionConfiguration::new("en", "es"))?;
    let requests = SOURCES.map(TranslationRequest::new);
    let mut stream = session.translate_batch_streaming(&requests)?;
    let first = stream.try_next();
    let _ = started.send(());
    let mut sources = Vec::new();
    match first {
        Err(TranslationError::TimedOut(message)) => assert!(message.contains("main thread")),
        Ok(Some(response)) => sources.push(response.source_text().to_owned()),
        other => return other.map(|_| sources),
    }
    while let Some(response) = stream.try_next()? {
        sources.push(response.source_text().to_owned());
    }
    Ok(sources)
}

fn main() {
    let (started_tx, started_rx) = mpsc::channel();
    let worker = thread::spawn(move || stream_after_a_timeout(&started_tx));
    let _ = started_rx.recv();
    while !worker.is_finished() {
        unsafe { CFRunLoopRunInMode(kCFRunLoopDefaultMode, 0.01, 0) };
    }
    match worker.join().expect("streaming worker panicked") {
        Ok(sources) => assert_eq!(sources, SOURCES),
        Err(
            TranslationError::UnavailableOnThisMacOS(_)
            | TranslationError::NotInstalled(_)
            | TranslationError::UnsupportedLanguagePairing(_)
            | TranslationError::UnsupportedSourceLanguage(_)
            | TranslationError::UnsupportedTargetLanguage(_),
        ) => println!("[skip] en -> es translation is not available"),
        Err(error) => panic!("streaming failed after the timeout: {error:?}"),
    }
}
