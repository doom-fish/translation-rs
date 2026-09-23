#![allow(missing_docs, non_camel_case_types)]

use core::ffi::{c_char, c_void};

pub type TrlAsyncCallback = extern "C" fn(*const c_void, i32, *const c_char, *mut c_void);

extern "C" {
    pub fn trl_string_free(s: *mut c_char);

    pub fn trl_language_canonicalize(
        identifier: *const c_char,
        out_language: *mut *mut c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn trl_language_pair_canonicalize_json(
        source_language: *const c_char,
        target_language: *const c_char,
        out_pair_json: *mut *mut c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;

    pub fn trl_language_availability_new() -> *mut c_void;
    pub fn trl_language_availability_new_with_preferred_strategy(
        preferred_strategy: i32,
        out_token: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn trl_language_availability_release(token: *mut c_void);
    pub fn trl_language_availability_preferred_strategy(
        token: *mut c_void,
        out_strategy: *mut i32,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn trl_language_availability_supported_languages_json(
        token: *mut c_void,
        out_languages_json: *mut *mut c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn trl_language_availability_status_from_to(
        token: *mut c_void,
        source_language: *const c_char,
        target_language: *const c_char,
        out_status: *mut i32,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn trl_language_availability_status_for_text(
        token: *mut c_void,
        text: *const c_char,
        target_language: *const c_char,
        out_status: *mut i32,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn trl_language_availability_status_async(
        token: *mut c_void,
        source_language: *const c_char,
        target_language: *const c_char,
        ctx: *mut c_void,
        cb: TrlAsyncCallback,
    );
    pub fn trl_language_availability_supported_languages_async(
        token: *mut c_void,
        cb: TrlAsyncCallback,
        ctx: *mut c_void,
    );

    pub fn trl_detect_language(
        text: *const c_char,
        out_language: *mut *mut c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;

    pub fn trl_session_new(
        configuration_json: *const c_char,
        out_token: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn trl_session_release(token: *mut c_void);
    pub fn trl_session_can_request_downloads(
        token: *mut c_void,
        out_value: *mut i32,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn trl_session_is_ready(
        token: *mut c_void,
        out_value: *mut i32,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn trl_session_preferred_strategy(
        token: *mut c_void,
        out_strategy: *mut i32,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn trl_session_cancel(token: *mut c_void, out_error_message: *mut *mut c_char) -> i32;
    pub fn trl_session_prepare_translation(
        token: *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn trl_session_translate_async(
        token: *mut c_void,
        text: *const c_char,
        cb: TrlAsyncCallback,
        ctx: *mut c_void,
    );
    pub fn trl_session_translations_async(
        token: *mut c_void,
        requests_json: *const c_char,
        cb: TrlAsyncCallback,
        ctx: *mut c_void,
    );
    pub fn trl_session_prepare_translation_async(
        token: *mut c_void,
        cb: TrlAsyncCallback,
        ctx: *mut c_void,
    );
    pub fn trl_session_translate_text_json(
        token: *mut c_void,
        text: *const c_char,
        out_response_json: *mut *mut c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn trl_session_translate_attributed_json(
        token: *mut c_void,
        attributed_text_json: *const c_char,
        out_response_json: *mut *mut c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn trl_session_translate_batch_json(
        token: *mut c_void,
        requests_json: *const c_char,
        out_responses_json: *mut *mut c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn trl_session_translate_batch_stream_json(
        token: *mut c_void,
        requests_json: *const c_char,
        out_batch_token: *mut *mut c_void,
        out_error_message: *mut *mut c_char,
    ) -> i32;
    pub fn trl_batch_response_release(token: *mut c_void);
    pub fn trl_batch_response_next_json(
        token: *mut c_void,
        out_response_json: *mut *mut c_char,
        out_error_message: *mut *mut c_char,
    ) -> i32;
}

pub mod status {
    pub const OK: i32 = 0;
    pub const INVALID_ARGUMENT: i32 = -1;
    pub const UNAVAILABLE_ON_THIS_MACOS: i32 = -2;
    pub const TIMED_OUT: i32 = -3;
    pub const UNSUPPORTED_SOURCE_LANGUAGE: i32 = -10;
    pub const UNSUPPORTED_TARGET_LANGUAGE: i32 = -11;
    pub const UNSUPPORTED_LANGUAGE_PAIRING: i32 = -12;
    pub const UNABLE_TO_IDENTIFY_LANGUAGE: i32 = -13;
    pub const NOTHING_TO_TRANSLATE: i32 = -14;
    pub const ALREADY_CANCELLED: i32 = -15;
    pub const NOT_INSTALLED: i32 = -16;
    pub const FRAMEWORK_ERROR: i32 = -20;
    pub const UNKNOWN: i32 = -99;
}
