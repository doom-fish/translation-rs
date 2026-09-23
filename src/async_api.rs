//! Async API for `Translation`
//!
//! Provides executor-agnostic Futures for the key Translation.framework async
//! operations. Requires the `async` Cargo feature.
//!
//! ## Available Types
//!
//! | Type | Swift API | macOS |
//! |------|-----------|-------|
//! | [`AsyncTranslationSession`] | `TranslationSession.translate(_:)` | 26+ |
//! | [`AsyncTranslationSession`] | `TranslationSession.translations(from:)` | 26+ |
//! | [`AsyncTranslationSession`] | `TranslationSession.prepareTranslation()` | 26+ |
//! | [`AsyncLanguageAvailability`] | `LanguageAvailability.status(from:to:)` | 15+ |
//! | [`AsyncLanguageAvailability`] | `LanguageAvailability.supportedLanguages` | 15+ |

use core::ffi::{c_char, c_void};
use std::ffi::CStr;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use doom_fish_utils::completion::{AsyncCompletion, AsyncCompletionFuture};
use doom_fish_utils::panic_safe::catch_user_panic;
use serde::de::DeserializeOwned;

use crate::ffi;
use crate::language::Language;
use crate::language_availability::{LanguageAvailability, LanguageAvailabilityStatus};
use crate::private::error_from_payload;
use crate::translation_error::TranslationError;
use crate::translation_response::TranslationResponse;
use crate::translation_session::{TranslationRequest, TranslationSession};

fn decode_json<T: DeserializeOwned>(json: &str, context: &str) -> Result<T, TranslationError> {
    serde_json::from_str(json)
        .map_err(|error| TranslationError::Unknown(format!("failed to decode {context}: {error}")))
}

type Outcome<T> = Result<T, TranslationError>;

fn flatten<T>(result: Result<Outcome<T>, String>) -> Outcome<T> {
    result.unwrap_or_else(|message| Err(TranslationError::Unknown(message)))
}

unsafe fn async_error(status: i32, error: *const c_char) -> TranslationError {
    let payload = if error.is_null() {
        format!("Swift bridge call failed with status code {status}")
    } else {
        unsafe { CStr::from_ptr(error) }
            .to_string_lossy()
            .into_owned()
    };
    error_from_payload(status, payload)
}

fn complete_json_callback(
    result: *const c_void,
    status: i32,
    error: *const c_char,
    ctx: *mut c_void,
    context: &str,
) {
    let outcome: Outcome<String> = if status != ffi::status::OK {
        Err(unsafe { async_error(status, error) })
    } else if result.is_null() {
        Err(TranslationError::Unknown(format!(
            "null result pointer for {context}"
        )))
    } else {
        Ok(unsafe { CStr::from_ptr(result.cast::<c_char>()) }
            .to_string_lossy()
            .into_owned())
    };
    unsafe { AsyncCompletion::complete_ok(ctx, outcome) };
}

extern "C" fn translate_cb(
    result: *const c_void,
    status: i32,
    error: *const c_char,
    ctx: *mut c_void,
) {
    catch_user_panic("translate_cb", || {
        complete_json_callback(result, status, error, ctx, "translation response");
    });
}

extern "C" fn translations_batch_cb(
    result: *const c_void,
    status: i32,
    error: *const c_char,
    ctx: *mut c_void,
) {
    catch_user_panic("translations_batch_cb", || {
        complete_json_callback(result, status, error, ctx, "translation batch responses");
    });
}

extern "C" fn prepare_translation_cb(
    _result: *const c_void,
    status: i32,
    error: *const c_char,
    ctx: *mut c_void,
) {
    catch_user_panic("prepare_translation_cb", || {
        let outcome: Outcome<()> = if status == ffi::status::OK {
            Ok(())
        } else {
            Err(unsafe { async_error(status, error) })
        };
        unsafe { AsyncCompletion::complete_ok(ctx, outcome) };
    });
}

extern "C" fn availability_status_cb(
    result: *const c_void,
    status: i32,
    error: *const c_char,
    ctx: *mut c_void,
) {
    catch_user_panic("availability_status_cb", || {
        let outcome: Outcome<i32> = if status == ffi::status::OK {
            (result as usize)
                .checked_sub(1)
                .and_then(|value| i32::try_from(value).ok())
                .ok_or_else(|| {
                    TranslationError::Unknown(
                        "invalid language availability status result".to_owned(),
                    )
                })
        } else {
            Err(unsafe { async_error(status, error) })
        };
        unsafe { AsyncCompletion::complete_ok(ctx, outcome) };
    });
}

extern "C" fn supported_languages_cb(
    result: *const c_void,
    status: i32,
    error: *const c_char,
    ctx: *mut c_void,
) {
    catch_user_panic("supported_languages_cb", || {
        complete_json_callback(result, status, error, ctx, "supported languages");
    });
}

/// Future returned by [`AsyncTranslationSession::translate`].
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct TranslateResponseFuture {
    inner: AsyncCompletionFuture<Outcome<String>>,
}

impl fmt::Debug for TranslateResponseFuture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TranslateResponseFuture")
            .finish_non_exhaustive()
    }
}

impl Future for TranslateResponseFuture {
    type Output = Result<TranslationResponse, TranslationError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner).poll(cx).map(|result| {
            flatten(result).and_then(|json| decode_json(&json, "translation response"))
        })
    }
}

/// Future returned by [`AsyncTranslationSession::translations`].
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct TranslationsBatchFuture {
    inner: AsyncCompletionFuture<Outcome<String>>,
}

impl fmt::Debug for TranslationsBatchFuture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TranslationsBatchFuture")
            .finish_non_exhaustive()
    }
}

impl Future for TranslationsBatchFuture {
    type Output = Result<Vec<TranslationResponse>, TranslationError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner).poll(cx).map(|result| {
            flatten(result).and_then(|json| decode_json(&json, "translation batch responses"))
        })
    }
}

/// Future returned by [`AsyncTranslationSession::prepare_translation`].
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct PrepareTranslationFuture {
    inner: AsyncCompletionFuture<Outcome<()>>,
}

impl fmt::Debug for PrepareTranslationFuture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PrepareTranslationFuture")
            .finish_non_exhaustive()
    }
}

impl Future for PrepareTranslationFuture {
    type Output = Result<(), TranslationError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner).poll(cx).map(flatten)
    }
}

/// Future returned by [`AsyncLanguageAvailability::status`].
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct AvailabilityStatusFuture {
    inner: AsyncCompletionFuture<Outcome<i32>>,
}

impl fmt::Debug for AvailabilityStatusFuture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AvailabilityStatusFuture")
            .finish_non_exhaustive()
    }
}

impl Future for AvailabilityStatusFuture {
    type Output = Result<LanguageAvailabilityStatus, TranslationError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|result| flatten(result).map(LanguageAvailabilityStatus::from_raw))
    }
}

/// Future returned by [`AsyncLanguageAvailability::supported_languages`].
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct SupportedLanguagesFuture {
    inner: AsyncCompletionFuture<Outcome<String>>,
}

impl fmt::Debug for SupportedLanguagesFuture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SupportedLanguagesFuture")
            .finish_non_exhaustive()
    }
}

impl Future for SupportedLanguagesFuture {
    type Output = Result<Vec<Language>, TranslationError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner).poll(cx).map(|result| {
            flatten(result).and_then(|json| decode_json(&json, "supported languages"))
        })
    }
}

/// Executor-agnostic async wrapper for [`TranslationSession`].
#[must_use]
pub struct AsyncTranslationSession<'a> {
    session: &'a TranslationSession,
}

impl<'a> AsyncTranslationSession<'a> {
    /// Wraps a `TranslationSession` for async Translation.framework calls.
    pub const fn new(session: &'a TranslationSession) -> Self {
        Self { session }
    }

    /// Starts `TranslationSession.translate(_:)` as a future.
    pub fn translate(&self, text: &str) -> Result<TranslateResponseFuture, TranslationError> {
        let text_c = crate::private::to_cstring(text)?;
        let (future, ctx) = AsyncCompletion::create();
        // SAFETY: FFI function is called with valid session token, C string pointer,
        // valid callback pointer, and valid context pointer from AsyncCompletion::create().
        // The callback is panic-safe and will properly complete the future.
        unsafe {
            ffi::trl_session_translate_async(
                self.session.raw_token(),
                text_c.as_ptr(),
                translate_cb,
                ctx,
            );
        }
        Ok(TranslateResponseFuture { inner: future })
    }

    /// Starts `TranslationSession.translations(from:)` as a future.
    pub fn translations(
        &self,
        requests: &[TranslationRequest],
    ) -> Result<TranslationsBatchFuture, TranslationError> {
        let requests_json = crate::private::json_cstring(requests)?;
        let (future, ctx) = AsyncCompletion::create();
        // SAFETY: FFI function is called with valid session token, C string pointer,
        // valid callback pointer, and valid context pointer from AsyncCompletion::create().
        // The callback is panic-safe and will properly complete the future.
        unsafe {
            ffi::trl_session_translations_async(
                self.session.raw_token(),
                requests_json.as_ptr(),
                translations_batch_cb,
                ctx,
            );
        }
        Ok(TranslationsBatchFuture { inner: future })
    }

    /// Starts `TranslationSession.prepareTranslation()` as a future.
    pub fn prepare_translation(&self) -> PrepareTranslationFuture {
        let (future, ctx) = AsyncCompletion::create();
        // SAFETY: FFI function is called with valid session token, valid callback pointer,
        // and valid context pointer from AsyncCompletion::create().
        // The callback is panic-safe and will properly complete the future.
        unsafe {
            ffi::trl_session_prepare_translation_async(
                self.session.raw_token(),
                prepare_translation_cb,
                ctx,
            );
        }
        PrepareTranslationFuture { inner: future }
    }
}

/// Executor-agnostic async wrapper for [`LanguageAvailability`].
#[must_use]
pub struct AsyncLanguageAvailability<'a> {
    availability: &'a LanguageAvailability,
}

impl<'a> AsyncLanguageAvailability<'a> {
    /// Wraps a `LanguageAvailability` for async Translation.framework calls.
    pub const fn new(availability: &'a LanguageAvailability) -> Self {
        Self { availability }
    }

    /// Starts `LanguageAvailability.status(from:to:)` as a future.
    pub fn status(
        &self,
        source: &Language,
        target: Option<&Language>,
    ) -> Result<AvailabilityStatusFuture, TranslationError> {
        let source_c = crate::private::to_cstring(source.identifier())?;
        let target_c = target
            .map(|language| crate::private::to_cstring(language.identifier()))
            .transpose()?;
        let (future, ctx) = AsyncCompletion::create();
        // SAFETY: FFI function is called with valid availability token, C string pointers
        // (source_c and target_c are owned CStrings, target_c is optionally null),
        // valid context pointer from AsyncCompletion::create(), and valid callback pointer.
        // The callback is panic-safe and will properly complete the future.
        unsafe {
            ffi::trl_language_availability_status_async(
                self.availability.raw_token(),
                source_c.as_ptr(),
                target_c
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                ctx,
                availability_status_cb,
            );
        }
        Ok(AvailabilityStatusFuture { inner: future })
    }

    /// Starts `LanguageAvailability.supportedLanguages` as a future.
    pub fn supported_languages(&self) -> SupportedLanguagesFuture {
        let (future, ctx) = AsyncCompletion::create();
        // SAFETY: FFI function is called with valid availability token, valid callback pointer,
        // and valid context pointer from AsyncCompletion::create().
        // The callback is panic-safe and will properly complete the future.
        unsafe {
            ffi::trl_language_availability_supported_languages_async(
                self.availability.raw_token(),
                supported_languages_cb,
                ctx,
            );
        }
        SupportedLanguagesFuture { inner: future }
    }
}
