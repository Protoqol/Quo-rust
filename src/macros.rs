/// This macro sends the provided variable(s) or expression(s) to Quo using the `__private_quo` function.
/// It is only active when `debug_assertions` are enabled (typically in debug builds).
///
/// # Examples
///
/// ### Sending a single variable
/// ```rust
/// # extern crate quo_rust as quo;
/// use quo::quo;
///
/// let variable = 12;
/// quo!(variable);
/// ```
///
/// ### Sending a mutable variable
/// To report mutability for a variable binding, use the `mut` keyword:
/// ```rust
/// # extern crate quo_rust as quo;
/// use quo::quo;
///
/// let mut big_number = 170141183460469231731687303715884105727i128;
/// quo!(mut big_number);
/// ```
///
/// Alternatively, passing a mutable reference also works automatically:
/// ```rust
/// # extern crate quo_rust as quo;
/// use quo::quo;
///
/// let mut big_number = 170141183460469231731687303715884105727i128;
/// quo!(&mut big_number);
/// ```
///
/// ### Sending an expression
/// ```rust
/// # extern crate quo_rust as quo;
/// use quo::quo;
///
/// quo!(1 + 1);
/// ```
///
/// ### Sending multiple arguments
/// ```rust
/// # extern crate quo_rust as quo;
/// use quo::quo;
///
/// let variable = 43;
/// quo!(variable, "string", 1 + 1);
/// ```
#[macro_export]
macro_rules! quo {
    ($( mut $var:ident ), + $(,)?) => {{
        #[cfg(debug_assertions)]
        $(
            let quo_package_name = option_env!("CARGO_PKG_NAME").unwrap_or("Rust project"); // Make sure we don't just get `quo-rust`.

            #[cfg(target_family = "wasm")]
            {
                let quo_file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/", file!());
                $crate::__private_quo(&$var, stringify!($var), line!(), &quo_file_path, true, false, quo_package_name);
            }

            #[cfg(not(target_family = "wasm"))]
            {
                let quo_path_buf = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(file!());
                let quo_file_path = quo_path_buf.to_str().unwrap_or(file!());
                $crate::__private_quo(&$var, stringify!($var), line!(), quo_file_path, true, false, quo_package_name);
            }
        )*
    }};

    ($( $var:ident ), + $(,)?) => {
        #[cfg(debug_assertions)]
        $(
            {
                let __quo_package_name = option_env!("CARGO_PKG_NAME").unwrap_or("Rust project"); // Make sure we don't just get `quo-rust`.

                #[cfg(target_family = "wasm")]
                {
                    let quo_file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/", file!());
                    $crate::__private_quo(&$var, stringify!($var), line!(), &quo_file_path.to_owned(), false, false, __quo_package_name);
                }

                #[cfg(not(target_family = "wasm"))]
                {
                    let quo_path_buf = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(file!());
                    let quo_file_path = quo_path_buf.to_str().unwrap_or(file!());
                    $crate::__private_quo(&$var, stringify!($var), line!(), quo_file_path, false, false, __quo_package_name);
                }
            }
        )*
    };

    ($( $var:expr ), + $(,)?) => {
        #[cfg(debug_assertions)]
        $(
            {
                let __quo_package_name = option_env!("CARGO_PKG_NAME").unwrap_or("Rust project"); // Make sure we don't just get `quo-rust`.

                #[cfg(target_family = "wasm")]
                {
                    let quo_file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/", file!());
                    $crate::__private_quo(&$var, stringify!($var), line!(), &quo_file_path.to_owned(), false, true, __quo_package_name);
                }

                #[cfg(not(target_family = "wasm"))]
                {
                    let quo_path_buf = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(file!());
                    let quo_file_path = quo_path_buf.to_str().unwrap_or(file!());
                    $crate::__private_quo(&$var, stringify!($var), line!(), quo_file_path, false, true, __quo_package_name);
                }
            }
        )*
    };
}
