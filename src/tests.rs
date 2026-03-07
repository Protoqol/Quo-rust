#[cfg(test)]
mod tests {
    #[cfg(target_family = "wasm")]
    use wasm_bindgen_test::*;
    use crate::{quo, quo_create_payload};

    #[cfg(target_family = "wasm")]
    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn payload_test() {
        let var = 42;
        let payload = quo_create_payload(&var, "var", line!(), file!(), false);

        assert!(payload.meta.time_epoch_ms > 0);
        assert!(!payload.meta.uid.is_empty());
        assert_eq!(payload.meta.variable.name, "var");
        assert_eq!(payload.meta.variable.value, "42");
    }

    #[cfg(target_family = "wasm")]
    #[wasm_bindgen_test]
    fn wasm_payload_test() {
        payload_test();
    }

    #[test]
    fn fn_test() {
        let fn_test_var = 1234;

        let payload = quo_create_payload(&fn_test_var, "fn_test_var", line!(), file!(), false);
        assert_eq!(payload.meta.variable.name, "fn_test_var");
        assert_eq!(payload.meta.variable.value, "1234");
        assert_eq!(payload.meta.variable.var_type, "&i32");
        assert!(!payload.meta.variable.mutable);
        assert!(!payload.meta.variable.is_constant);
    }

    #[test]
    fn macro_const_test() {
        const VAR_I32: i32 = -1234;

        quo!(VAR_I32, VAR_I32, VAR_I32);

        let payload = quo_create_payload(&VAR_I32, "VAR_I32", line!(), file!(), false);
        assert_eq!(payload.meta.variable.name, "VAR_I32");
        assert_eq!(payload.meta.variable.value, "-1234");
        assert_eq!(payload.meta.variable.var_type, "&i32");
        assert!(!payload.meta.variable.mutable);
        assert!(payload.meta.variable.is_constant);
    }

    #[test]
    fn macro_i32_test() {
        let var_i32: i32 = -1234;

        let payload = quo_create_payload(&var_i32, "var_i32", line!(), file!(), false);
        assert_eq!(payload.meta.variable.name, "var_i32");
        assert_eq!(payload.meta.variable.value, "-1234");
        assert_eq!(payload.meta.variable.var_type, "&i32");
        assert!(!payload.meta.variable.mutable);
        assert!(!payload.meta.variable.is_constant);
    }

    #[test]
    fn macro_u32_test() {
        let var_u32: u32 = 1234;

        let payload = quo_create_payload(&var_u32, "var_u32", line!(), file!(), false);
        assert_eq!(payload.meta.variable.name, "var_u32");
        assert_eq!(payload.meta.variable.value, "1234");
        assert_eq!(payload.meta.variable.var_type, "&u32");
        assert!(!payload.meta.variable.mutable);
        assert!(!payload.meta.variable.is_constant);
    }

    #[test]
    fn macro_string_test() {
        let var_string: &str = "string";

        let payload = quo_create_payload(&var_string, "var_string", line!(), file!(), false);
        assert_eq!(payload.meta.variable.name, "var_string");
        assert_eq!(payload.meta.variable.value, "\"string\"");
        assert_eq!(payload.meta.variable.var_type, "&&str");
        assert!(!payload.meta.variable.mutable);
        assert!(!payload.meta.variable.is_constant);
    }

    #[test]
    fn macro_bool_test() {
        let var_bool: bool = true;

        let payload = quo_create_payload(&var_bool, "var_bool", line!(), file!(), false);
        assert_eq!(payload.meta.variable.name, "var_bool");
        assert_eq!(payload.meta.variable.value, "true");
        assert_eq!(payload.meta.variable.var_type, "&bool");
        assert!(!payload.meta.variable.mutable);
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

        let payload = quo_create_payload(&var_tuple, "var_tuple", line!(), file!(), false);
        assert_eq!(payload.meta.variable.name, "var_tuple");
        assert_eq!(
            payload.meta.variable.value,
            "(false, \"hope\", \"this\", \"works\", 23423)"
        );
        assert_eq!(
            payload.meta.variable.var_type,
            "&(bool, alloc::string::String, alloc::string::String, alloc::string::String, u32)"
        );
        assert!(!payload.meta.variable.mutable);
        assert!(!payload.meta.variable.is_constant);
    }

    #[test]
    fn macro_arr_test() {
        let var_array: [String; 3] = [
            String::from("hope"),
            String::from("this"),
            String::from("works"),
        ];

        let payload = quo_create_payload(&var_array, "var_array", line!(), file!(), false);
        assert_eq!(payload.meta.variable.name, "var_array");
        assert_eq!(
            payload.meta.variable.value,
            "[\"hope\", \"this\", \"works\"]"
        );
        assert_eq!(
            payload.meta.variable.var_type,
            "&[alloc::string::String; 3]"
        );
        assert!(!payload.meta.variable.mutable);
        assert!(!payload.meta.variable.is_constant);
    }

    #[test]
    fn macro_vec_test() {
        let var_vector: Vec<String> = vec![
            String::from("hope"),
            String::from("this"),
            String::from("works"),
        ];

        let payload = quo_create_payload(&var_vector, "var_vector", line!(), file!(), false);
        assert_eq!(payload.meta.variable.name, "var_vector");
        assert_eq!(
            payload.meta.variable.value,
            "[\"hope\", \"this\", \"works\"]"
        );
        assert_eq!(
            payload.meta.variable.var_type,
            "&alloc::vec::Vec<alloc::string::String>"
        );
        assert!(!payload.meta.variable.mutable);
        assert!(!payload.meta.variable.is_constant);
    }

    #[test]
    fn macro_mut_test() {
        let mut var_mut = 1;
        let payload1 = quo_create_payload(&var_mut, "var_mut", line!(), file!(), true);
        assert_eq!(payload1.meta.variable.name, "var_mut");
        assert_eq!(payload1.meta.variable.value, "1");
        assert_eq!(payload1.meta.variable.var_type, "&i32");
        assert!(payload1.meta.variable.mutable);
        assert!(!payload1.meta.variable.is_constant);

        var_mut = 2;
        let payload2 = quo_create_payload(&var_mut, "var_mut", line!(), file!(), true);
        assert_eq!(payload2.meta.variable.name, "var_mut");
        assert_eq!(payload2.meta.variable.value, "2");
        assert_eq!(payload2.meta.variable.var_type, "&i32");
        assert!(payload2.meta.variable.mutable);
        assert!(!payload2.meta.variable.is_constant);
    }

    #[test]
    fn macro_immut_test() {
        let var_immut = 1;
        let payload = quo_create_payload(&var_immut, "var_immut", line!(), file!(), false);
        assert_eq!(payload.meta.variable.name, "var_immut");
        assert_eq!(payload.meta.variable.value, "1");
        assert_eq!(payload.meta.variable.var_type, "&i32");
        assert!(!payload.meta.variable.mutable);
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

        let payload = quo_create_payload(&var_complex, "var_complex", line!(), file!(), false);
        assert_eq!(payload.meta.variable.name, "var_complex");
        assert_eq!(
            payload.meta.variable.var_type,
            "&quo_rust::tests::tests::complex_type_test::Complex"
        );
        assert!(payload.meta.variable.value.contains("a: 1"));
        assert!(payload.meta.variable.value.contains("complex"));
        assert!(!payload.meta.variable.mutable);
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

        let payload = quo_create_payload(&var_large, "var_large", line!(), file!(), false);
        assert_eq!(payload.meta.variable.name, "var_large");
        assert_eq!(
            payload.meta.variable.var_type,
            "&quo_rust::tests::tests::large_var_test::Large"
        );
        assert!(payload.meta.variable.value.contains("j: 10"));
        assert!(!payload.meta.variable.mutable);
        assert!(!payload.meta.variable.is_constant);
    }

    #[test]
    fn macro_mut_explicit_test() {
        let mut var_mut = 1;
        quo!(mut var_mut);

        let payload = quo_create_payload(&var_mut, "var_mut", line!(), file!(), true);
        assert!(payload.meta.variable.mutable);

        var_mut = 2;
        quo!(var_mut);
    }
}
