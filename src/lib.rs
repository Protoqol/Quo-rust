mod info;
mod request;
mod tests;
mod types;

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
/// quo_create_payload(big_number, "big_number", line!(), file!());
///
#[cfg(debug_assertions)]
#[doc(hidden)]
fn quo_create_payload<T: Debug>(
    value: T,
    name: &str,
    line: u32,
    file: &str,
    is_mutable: bool,
    package_name: &str,
) -> QuoPayload {
    let var_type = std::any::type_name_of_val(&value).to_string();
    let value = format!("{:?}", value);

    let (time_epoch_ms, uid) = get_time();

    QuoPayload {
        language: QuoPayloadLanguage::Rust,
        meta: QuoPayloadMeta {
            id: 0,
            uid,
            origin: package_name.to_string(),
            sender_origin: format!("{}:{}", file, line),
            time_epoch_ms,
            variable: QuoPayloadVariable {
                var_type: var_type.clone(),
                name: name.to_string(),
                value: value.clone(),
                mutable: is_mutable,
                is_constant: name == name.to_uppercase(),
            },
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
/// quo(big_number, "big_number", line!(), file!());
///
#[cfg(debug_assertions)]
#[doc(hidden)]
fn quo<T: Debug>(
    value: T,
    name: &str,
    line: u32,
    file: &str,
    is_mutable: bool,
    package_name: &str,
) {
    #[cfg(debug_assertions)]
    {
        let env_host = option_env!("QUO_HOST").unwrap_or("http://127.0.0.1");
        let env_port = option_env!("QUO_PORT").unwrap_or("7312");

        let send_fn = move || {
            let body = quo_create_payload(value, name, line, file, is_mutable, package_name);
            let quo_server_address = format!("{}:{}/payload", env_host, env_port);

            make_request(&quo_server_address, body);
        };

        #[cfg(target_family = "wasm")]
        {
            send_fn();
        }

        #[cfg(not(target_family = "wasm"))]
        {
            // @TODO async
            send_fn();
        }
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
    package_name: &str,
) {
    quo(value, name, line, file, is_mutable, package_name)
}

/// This macro sends the provided variable to Quo using the quo() fn.
///
/// # Example
///
/// let mut big_number: i128;
///
/// big_number = 170141183460469231731687303715884105727;
///
/// quo!(big_number);
///
#[macro_export]
macro_rules! quo {
    ($( mut $var:ident ),*) => {{
        #[cfg(debug_assertions)]
        $(
            {
                let __quo_package_name = option_env!("CARGO_PKG_NAME").unwrap_or("Rust project"); // Make sure we don't just get `quo-rust`.

                #[cfg(target_family = "wasm")]
                {
                    let __quo_file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/", file!());
                    $crate::__private_quo(&$var, stringify!($var), line!(), &__quo_file_path, true, __quo_package_name);
                }

                #[cfg(not(target_family = "wasm"))]
                {
                    let __quo_path_buf = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(file!());
                    let __quo_file_path = __quo_path_buf.to_str().unwrap_or(file!());
                    $crate::__private_quo(&$var, stringify!($var), line!(), __quo_file_path, true, __quo_package_name);
                }
            }
        )*
    }};
    ($( $var:ident ),*) => {{
        #[cfg(debug_assertions)]
        $(
            {
                let __quo_package_name = option_env!("CARGO_PKG_NAME").unwrap_or("Rust project"); // Make sure we don't just get `quo-rust`.

                #[cfg(target_family = "wasm")]
                {
                    let __quo_file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/", file!());
                    $crate::__private_quo(&$var, stringify!($var), line!(), &__quo_file_path.to_owned(), false, __quo_package_name);
                }

                #[cfg(not(target_family = "wasm"))]
                {
                    let __quo_path_buf = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(file!());
                    let __quo_file_path = __quo_path_buf.to_str().unwrap_or(file!());
                    $crate::__private_quo(&$var, stringify!($var), line!(), __quo_file_path, false, __quo_package_name);
                }
            }
        )*
    }};
}
