mod info;
pub mod macros;
mod request;
mod tests;
mod types;

use crate::info::hash::get_hash;
use crate::info::runtime::get_runtime;
use crate::info::stack_trace::get_stack_trace;
use crate::info::system_usage::get_system_usage;
use crate::info::thread::get_thread_id;
use crate::info::time::get_time;
use crate::request::make_request;
use crate::types::{QuoPayload, QuoPayloadLanguage, QuoPayloadMeta, QuoPayloadVariable};
use std::fmt::Debug;

/// This fn creates a QuoPayload. You might or might not question why this is a separate function: for testing.
///
/// # Example
///
/// let mut big_number: i128;
///
/// big_number = 170141183460469231731687303715884105727;
///
/// quo_create_payload(&big_number, "big_number", line!(), file!(), false, false, "quo-rust");
///
#[cfg(debug_assertions)]
#[doc(hidden)]
fn quo_create_payload<T: Debug>(
    value: T,
    name: &str,
    line: u32,
    file: &str,
    is_mutable: bool,
    is_expression: bool,
    package_name: &str,
) -> QuoPayload {
    let id = 0; // @TODO Pretty useless currently.
    let var_type_raw = std::any::type_name_of_val(&value).to_string();
    let var_type = var_type_raw
        .strip_prefix('&')
        .unwrap_or(&var_type_raw)
        .to_string();

    let value_str = format!("{:?}", value);
    let (time_epoch_ms, uid) = get_time();
    let memory_address = Some(format!("{:p}", &value as *const T));
    let grouping_hash = get_hash(&var_type_raw, name, package_name);
    let (stack_trace, caller_function) = get_stack_trace();
    let thread_info = get_thread_id();
    let (cpu_usage, memory_usage) = get_system_usage();
    let runtime = get_runtime();

    QuoPayload {
        language: QuoPayloadLanguage::Rust,
        meta: QuoPayloadMeta {
            origin: package_name.to_string(),
            sender_origin: format!("{}:{}", file, line),
            variable: QuoPayloadVariable {
                var_type: var_type.clone(),
                name: name.to_string(),
                value: value_str,
                is_constant: name == name.to_uppercase(),
                is_mutable: is_mutable || var_type_raw.contains("&mut "),
                is_expression,
                memory_address,
                grouping_hash,
            },
            id,
            uid,
            time_epoch_ms,
            stack_trace,
            thread_info,
            runtime,
            cpu_usage,
            memory_usage,
            caller_function,
        },
    }
}

/// This fn sends the provided variable to Quo.
///
/// # Example
///
/// let mut big_number: i128;
///
/// big_number = 170141183460469231731687303715884105727;
///
/// quo(&big_number, "big_number", line!(), file!(), false, false, "quo-rust");
///
#[cfg(debug_assertions)]
#[doc(hidden)]
fn quo<T: Debug>(
    value: T,
    name: &str,
    line: u32,
    file: &str,
    is_mutable: bool,
    is_expression: bool,
    package_name: &str,
) {
    #[cfg(debug_assertions)]
    {
        let env_host = option_env!("QUO_HOST").unwrap_or("http://127.0.0.1");
        let env_port = option_env!("QUO_PORT").unwrap_or("7312");

        let body = quo_create_payload(
            value,
            name,
            line,
            file,
            is_mutable,
            is_expression,
            package_name,
        );
        let quo_server_address = format!("{}:{}/payload", env_host, env_port);

        make_request(&quo_server_address, body);
    }
}

/// Use the `quo!()` macro and not this fn directly.
#[cfg(debug_assertions)]
#[doc(hidden)]
pub fn __private_quo<T: Debug>(
    value: T,
    name: &str,
    line: u32,
    file: &str,
    is_mutable: bool,
    is_expression: bool,
    package_name: &str,
) {
    quo(
        value,
        name,
        line,
        file,
        is_mutable,
        is_expression,
        package_name,
    )
}
