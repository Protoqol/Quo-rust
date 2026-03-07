
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Debug, PartialEq)]
pub struct QuoPayloadVariable {
    pub var_type: String,
    pub name: String,
    pub value: String,
    pub mutable: bool,
    pub is_constant: bool,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct QuoPayloadMeta {
    pub id: u32,
    pub uid: String,
    pub origin: String,
    pub sender_origin: String,
    pub time_epoch_ms: i64,
    pub variable: QuoPayloadVariable,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum QuoPayloadLanguage {
    Rust,
    Php,
    Javascript,
    Typescript,
    Python,
    Ruby,
    Go,
    #[default]
    Unknown,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct QuoPayload {
    pub meta: QuoPayloadMeta,
    pub language: QuoPayloadLanguage,
}

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
fn quo_create_payload<T: Debug>(value: T, name: &str, line: u32, file: &str) -> QuoPayload {
    let var_type = std::any::type_name_of_val(&value).to_string();
    let name = name;
    let value = format!("{:?}", value);
    let line = line;
    let file = file;
    let package_name = option_env!("CARGO_PKG_NAME").unwrap_or("Rust project");

    let since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    QuoPayload {
        language: QuoPayloadLanguage::Rust,
        meta: QuoPayloadMeta {
            id: 0,
            uid: since_the_epoch.as_nanos().to_string(),
            origin: package_name.to_string(),
            sender_origin: format!("{}:{}", file, line),
            time_epoch_ms: since_the_epoch.as_millis() as i64,
            variable: QuoPayloadVariable {
                var_type: var_type.clone(),
                name: name.to_string(),
                value: value.clone(),
                // @TODO Detect if is mutable or not
                mutable: true,
                // @TODO Correctly detect const.
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
fn quo<T: Debug>(value: T, name: &str, line: u32, file: &str) -> () {
    #[cfg(debug_assertions)]
    {
        let env_host = option_env!("QUO_HOST").unwrap_or("http://127.0.0.1");
        let env_port = option_env!("QUO_PORT").unwrap_or("7312");

        let quo_server = format!("{}:{}", env_host, env_port);

        let body = quo_create_payload(value, name, line, file);

        let _ = match ureq::post(quo_server).send_json(body) {
            Ok(_response) => {}
            Err(ureq::Error::StatusCode(code)) => {
                // @TODO add extra debugging.
                eprintln!("[Quo] HTTP {} - is Quo running?", code)
            }
            Err(e) => {
                eprintln!("[Quo] error \"{}\" - is Quo running?", e.to_string())
            }
        };
    }
}

#[cfg(debug_assertions)]
#[doc(hidden)]
pub fn __private_quo<T: Debug>(value: T, name: &str, line: u32, file: &str) -> () {
    quo(value, name, line, file)
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
    ($( $var:ident ),*) => {{
        #[cfg(debug_assertions)]
        $(
            $crate::__private_quo(&$var, stringify!($var), line!(), file!());
        )*
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_test() {
        let fn_test_var = 1234;

        let payload = quo_create_payload(&fn_test_var, "fn_test_var", line!(), file!());
        assert_eq!(payload.meta.variable.name, "fn_test_var");
        assert_eq!(payload.meta.variable.value, "1234");
        assert_eq!(payload.meta.variable.var_type, "&i32");
        assert!(payload.meta.variable.mutable);
        assert!(!payload.meta.variable.is_constant);
    }

    #[test]
    fn macro_const_test() {
        const VAR_I32: i32 = -1234;

        quo!(VAR_I32, VAR_I32, VAR_I32);

        let payload = quo_create_payload(&VAR_I32, "VAR_I32", line!(), file!());
        assert_eq!(payload.meta.variable.name, "VAR_I32");
        assert_eq!(payload.meta.variable.value, "-1234");
        assert_eq!(payload.meta.variable.var_type, "&i32");
        assert!(payload.meta.variable.mutable);
        assert!(payload.meta.variable.is_constant);
    }

    #[test]
    fn macro_i32_test() {
        let var_i32: i32 = -1234;

        let payload = quo_create_payload(&var_i32, "var_i32", line!(), file!());
        assert_eq!(payload.meta.variable.name, "var_i32");
        assert_eq!(payload.meta.variable.value, "-1234");
        assert_eq!(payload.meta.variable.var_type, "&i32");
        assert!(payload.meta.variable.mutable);
        assert!(!payload.meta.variable.is_constant);
    }

    #[test]
    fn macro_u32_test() {
        let var_u32: u32 = 1234;

        let payload = quo_create_payload(&var_u32, "var_u32", line!(), file!());
        assert_eq!(payload.meta.variable.name, "var_u32");
        assert_eq!(payload.meta.variable.value, "1234");
        assert_eq!(payload.meta.variable.var_type, "&u32");
        assert!(payload.meta.variable.mutable);
        assert!(!payload.meta.variable.is_constant);
    }

    #[test]
    fn macro_string_test() {
        let var_string: &str = "string";

        let payload = quo_create_payload(&var_string, "var_string", line!(), file!());
        assert_eq!(payload.meta.variable.name, "var_string");
        assert_eq!(payload.meta.variable.value, "\"string\"");
        assert_eq!(payload.meta.variable.var_type, "&&str");
        assert!(payload.meta.variable.mutable);
        assert!(!payload.meta.variable.is_constant);
    }

    #[test]
    fn macro_bool_test() {
        let var_bool: bool = true;

        let payload = quo_create_payload(&var_bool, "var_bool", line!(), file!());
        assert_eq!(payload.meta.variable.name, "var_bool");
        assert_eq!(payload.meta.variable.value, "true");
        assert_eq!(payload.meta.variable.var_type, "&bool");
        assert!(payload.meta.variable.mutable);
        assert!(!payload.meta.variable.is_constant);
    }

    #[test]
    fn macro_tupl_test() {
        let var_tuple: (bool, String, String, String, u32) = (
            false,
            String::from("hope"),
            String::from("this"),
            String::from("works"),
            23423,
        );

        let payload = quo_create_payload(&var_tuple, "var_tuple", line!(), file!());
        assert_eq!(payload.meta.variable.name, "var_tuple");
        assert_eq!(
            payload.meta.variable.value,
            "(false, \"hope\", \"this\", \"works\", 23423)"
        );
        assert_eq!(
            payload.meta.variable.var_type,
            "&(bool, alloc::string::String, alloc::string::String, alloc::string::String, u32)"
        );
        assert!(payload.meta.variable.mutable);
        assert!(!payload.meta.variable.is_constant);
    }

    #[test]
    fn macro_arr_test() {
        let var_array: [String; 3] = [
            String::from("hope"),
            String::from("this"),
            String::from("works"),
        ];

        let payload = quo_create_payload(&var_array, "var_array", line!(), file!());
        assert_eq!(payload.meta.variable.name, "var_array");
        assert_eq!(
            payload.meta.variable.value,
            "[\"hope\", \"this\", \"works\"]"
        );
        assert_eq!(
            payload.meta.variable.var_type,
            "&[alloc::string::String; 3]"
        );
        assert!(payload.meta.variable.mutable);
        assert!(!payload.meta.variable.is_constant);
    }

    #[test]
    fn macro_vec_test() {
        let var_vector: Vec<String> = vec![
            String::from("hope"),
            String::from("this"),
            String::from("works"),
        ];

        let payload = quo_create_payload(&var_vector, "var_vector", line!(), file!());
        assert_eq!(payload.meta.variable.name, "var_vector");
        assert_eq!(
            payload.meta.variable.value,
            "[\"hope\", \"this\", \"works\"]"
        );
        assert_eq!(
            payload.meta.variable.var_type,
            "&alloc::vec::Vec<alloc::string::String>"
        );
        assert!(payload.meta.variable.mutable);
        assert!(!payload.meta.variable.is_constant);
    }

    #[test]
    fn macro_mut_test() {
        let mut var_mut = 1;
        let payload1 = quo_create_payload(&var_mut, "var_mut", line!(), file!());
        assert_eq!(payload1.meta.variable.name, "var_mut");
        assert_eq!(payload1.meta.variable.value, "1");
        assert_eq!(payload1.meta.variable.var_type, "&i32");
        assert!(payload1.meta.variable.mutable);
        assert!(!payload1.meta.variable.is_constant);

        var_mut = 2;
        let payload2 = quo_create_payload(&var_mut, "var_mut", line!(), file!());
        assert_eq!(payload2.meta.variable.name, "var_mut");
        assert_eq!(payload2.meta.variable.value, "2");
        assert_eq!(payload2.meta.variable.var_type, "&i32");
        assert!(payload2.meta.variable.mutable);
        assert!(!payload2.meta.variable.is_constant);
    }

    #[test]
    fn macro_immut_test() {
        let var_immut = 1;
        let payload = quo_create_payload(&var_immut, "var_immut", line!(), file!());
        assert_eq!(payload.meta.variable.name, "var_immut");
        assert_eq!(payload.meta.variable.value, "1");
        assert_eq!(payload.meta.variable.var_type, "&i32");
        assert!(payload.meta.variable.mutable);
        assert!(!payload.meta.variable.is_constant);
    }

    #[test]
    #[allow(dead_code)]
    fn complex_type_test() {
        #[derive(Debug)]
        struct Complex {
            a: i32,
            b: String,
            c: Vec<i32>,
            d: bool,
        }

        let var_complex = Complex {
            a: 1,
            b: String::from("complex"),
            c: vec![1, 2, 3],
            d: true,
        };

        let payload = quo_create_payload(&var_complex, "var_complex", line!(), file!());
        assert_eq!(payload.meta.variable.name, "var_complex");
        assert_eq!(
            payload.meta.variable.var_type,
            "&quo_rust::tests::complex_type_test::Complex"
        );
        assert!(payload.meta.variable.value.contains("a: 1"));
        assert!(payload.meta.variable.value.contains("complex"));
        assert!(payload.meta.variable.mutable);
        assert!(!payload.meta.variable.is_constant);
    }

    #[test]
    #[allow(dead_code)]
    fn large_var_test() {
        #[derive(Debug)]
        struct Large {
            a: i32,
            b: i32,
            c: i32,
            d: i32,
            e: i32,
            f: i32,
            g: i32,
            h: i32,
            i: i32,
            j: i32,
        }

        let var_large = Large {
            a: 1,
            b: 2,
            c: 3,
            d: 4,
            e: 5,
            f: 6,
            g: 7,
            h: 8,
            i: 9,
            j: 10,
        };

        let payload = quo_create_payload(&var_large, "var_large", line!(), file!());
        assert_eq!(payload.meta.variable.name, "var_large");
        assert_eq!(
            payload.meta.variable.var_type,
            "&quo_rust::tests::large_var_test::Large"
        );
        assert!(payload.meta.variable.value.contains("j: 10"));
        assert!(payload.meta.variable.mutable);
        assert!(!payload.meta.variable.is_constant);
    }
}
