#[cfg(test)]
mod lib_tests {
    use crate::quo_create_payload;

    #[cfg(target_family = "wasm")]
    use wasm_bindgen_test::*;

    #[cfg(target_family = "wasm")]
    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn test_payload() {
        let var = 42;
        let payload = quo_create_payload(&var, "var", line!(), file!(), false, false, "test");

        assert!(payload.meta.time_epoch_ms > 0);
        assert!(!payload.meta.uid.is_empty());
        assert_eq!(payload.meta.variable.name, "var");
        assert_eq!(payload.meta.variable.value, "42");
        assert!(!payload.meta.variable.is_expression);
    }

    #[cfg(target_family = "wasm")]
    #[wasm_bindgen_test]
    fn test_wasm_payload() {
        test_payload();
    }

    #[test]
    fn test_fn() {
        let fn_test_var = 1234;

        let payload =
            quo_create_payload(&fn_test_var, "fn_test_var", line!(), file!(), false, false, "test");
        assert_eq!(payload.meta.variable.name, "fn_test_var");
        assert_eq!(payload.meta.variable.value, "1234");
        assert_eq!(payload.meta.variable.var_type, "i32");
        assert!(!payload.meta.variable.is_mutable);
        assert!(!payload.meta.variable.is_constant);
        assert!(!payload.meta.variable.is_expression);
    }

    #[test]
    fn test_macro_const() {
        const VAR_I32: i32 = -1234;

        let payload = quo_create_payload(&VAR_I32, "VAR_I32", line!(), file!(), false, false, "test");
        assert_eq!(payload.meta.variable.name, "VAR_I32");
        assert_eq!(payload.meta.variable.value, "-1234");
        assert_eq!(payload.meta.variable.var_type, "i32");
        assert!(!payload.meta.variable.is_mutable);
        assert!(payload.meta.variable.is_constant);
        assert!(!payload.meta.variable.is_expression);
    }

    #[test]
    fn test_macro_i32() {
        let var_i32: i32 = -1234;

        let payload = quo_create_payload(&var_i32, "var_i32", line!(), file!(), false, false, "test");
        assert_eq!(payload.meta.variable.name, "var_i32");
        assert_eq!(payload.meta.variable.value, "-1234");
        assert_eq!(payload.meta.variable.var_type, "i32");
        assert!(!payload.meta.variable.is_mutable);
        assert!(!payload.meta.variable.is_constant);
        assert!(!payload.meta.variable.is_expression);
    }

    #[test]
    fn test_macro_u32() {
        let var_u32: u32 = 1234;

        let payload = quo_create_payload(&var_u32, "var_u32", line!(), file!(), false, false, "test");
        assert_eq!(payload.meta.variable.name, "var_u32");
        assert_eq!(payload.meta.variable.value, "1234");
        assert_eq!(payload.meta.variable.var_type, "u32");
        assert!(!payload.meta.variable.is_mutable);
        assert!(!payload.meta.variable.is_constant);
        assert!(!payload.meta.variable.is_expression);
    }

    #[test]
    fn test_macro_string() {
        let var_string: &str = "string";

        let payload = quo_create_payload(&var_string, "var_string", line!(), file!(), false, false, "test");
        assert_eq!(payload.meta.variable.name, "var_string");
        assert_eq!(payload.meta.variable.value, "\"string\"");
        assert_eq!(payload.meta.variable.var_type, "&str");
        assert!(!payload.meta.variable.is_mutable);
        assert!(!payload.meta.variable.is_constant);
        assert!(!payload.meta.variable.is_expression);
    }

    #[test]
    fn test_macro_bool() {
        let var_bool: bool = true;

        let payload = quo_create_payload(&var_bool, "var_bool", line!(), file!(), false, false, "test");
        assert_eq!(payload.meta.variable.name, "var_bool");
        assert_eq!(payload.meta.variable.value, "true");
        assert_eq!(payload.meta.variable.var_type, "bool");
        assert!(!payload.meta.variable.is_mutable);
        assert!(!payload.meta.variable.is_constant);
        assert!(!payload.meta.variable.is_expression);
    }

    #[test]
    fn test_macro_tuple() {
        let var_tuple: (bool, String, String, String, u32) = (
            false,
            String::from("hope"),
            String::from("this"),
            String::from("works"),
            23423,
        );

        let payload = quo_create_payload(&var_tuple, "var_tuple", line!(), file!(), false, false, "test");
        assert_eq!(payload.meta.variable.name, "var_tuple");
        assert_eq!(
            payload.meta.variable.value,
            "(false, \"hope\", \"this\", \"works\", 23423)"
        );
        assert_eq!(
            payload.meta.variable.var_type,
            "(bool, alloc::string::String, alloc::string::String, alloc::string::String, u32)"
        );
        assert!(!payload.meta.variable.is_mutable);
        assert!(!payload.meta.variable.is_constant);
        assert!(!payload.meta.variable.is_expression);
    }

    #[test]
    fn test_macro_array() {
        let var_array: [String; 3] = [
            String::from("hope"),
            String::from("this"),
            String::from("works"),
        ];

        let payload = quo_create_payload(&var_array, "var_array", line!(), file!(), false, false, "test");
        assert_eq!(payload.meta.variable.name, "var_array");
        assert_eq!(
            payload.meta.variable.value,
            "[\"hope\", \"this\", \"works\"]"
        );
        assert_eq!(payload.meta.variable.var_type, "[alloc::string::String; 3]");
        assert!(!payload.meta.variable.is_mutable);
        assert!(!payload.meta.variable.is_constant);
        assert!(!payload.meta.variable.is_expression);
    }

    #[test]
    fn test_macro_vec() {
        let var_vector: Vec<String> = vec![
            String::from("hope"),
            String::from("this"),
            String::from("works"),
        ];

        let payload = quo_create_payload(&var_vector, "var_vector", line!(), file!(), false, false, "test");
        assert_eq!(payload.meta.variable.name, "var_vector");
        assert_eq!(
            payload.meta.variable.value,
            "[\"hope\", \"this\", \"works\"]"
        );
        assert_eq!(
            payload.meta.variable.var_type,
            "alloc::vec::Vec<alloc::string::String>"
        );
        assert!(!payload.meta.variable.is_mutable);
        assert!(!payload.meta.variable.is_constant);
        assert!(!payload.meta.variable.is_expression);
    }

    #[test]
    fn test_macro_mut() {
        let mut var_mut = 1;
        let payload1 = quo_create_payload(&var_mut, "var_mut", line!(), file!(), true, false, "test");
        assert_eq!(payload1.meta.variable.name, "var_mut");
        assert_eq!(payload1.meta.variable.value, "1");
        assert_eq!(payload1.meta.variable.var_type, "i32");
        assert!(payload1.meta.variable.is_mutable);
        assert!(!payload1.meta.variable.is_constant);
        assert!(!payload1.meta.variable.is_expression);

        var_mut = 2;
        let payload2 = quo_create_payload(&var_mut, "var_mut", line!(), file!(), true, false, "test");
        assert_eq!(payload2.meta.variable.name, "var_mut");
        assert_eq!(payload2.meta.variable.value, "2");
        assert_eq!(payload2.meta.variable.var_type, "i32");
        assert!(payload2.meta.variable.is_mutable);
        assert!(!payload2.meta.variable.is_constant);
        assert!(!payload2.meta.variable.is_expression);
    }

    #[test]
    fn test_macro_immut() {
        let var_immut = 1;
        let payload = quo_create_payload(&var_immut, "var_immut", line!(), file!(), false, false, "test");
        assert_eq!(payload.meta.variable.name, "var_immut");
        assert_eq!(payload.meta.variable.value, "1");
        assert_eq!(payload.meta.variable.var_type, "i32");
        assert!(!payload.meta.variable.is_mutable);
        assert!(!payload.meta.variable.is_constant);
        assert!(!payload.meta.variable.is_expression);
    }

    #[test]
    #[allow(dead_code)]
    fn test_complex_type() {
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

        let payload =
            quo_create_payload(&var_complex, "var_complex", line!(), file!(), false, false, "test");
        assert_eq!(payload.meta.variable.name, "var_complex");
        assert_eq!(
            payload.meta.variable.var_type,
            "quo_rust::tests::lib_tests::test_complex_type::Complex"
        );
        assert!(payload.meta.variable.value.contains("a: 1"));
        assert!(payload.meta.variable.value.contains("complex"));
        assert!(!payload.meta.variable.is_mutable);
        assert!(!payload.meta.variable.is_constant);
        assert!(!payload.meta.variable.is_expression);
    }

    #[test]
    #[allow(dead_code)]
    fn test_large_var() {
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

        let payload = quo_create_payload(&var_large, "var_large", line!(), file!(), false, false, "test");
        assert_eq!(payload.meta.variable.name, "var_large");
        assert_eq!(
            payload.meta.variable.var_type,
            "quo_rust::tests::lib_tests::test_large_var::Large"
        );
        assert!(payload.meta.variable.value.contains("j: 10"));
        assert!(!payload.meta.variable.is_mutable);
        assert!(!payload.meta.variable.is_constant);
        assert!(!payload.meta.variable.is_expression);
    }

    #[test]
    fn test_macro_mut_explicit() {
        #[allow(unused_mut)]
        let mut var_mut = 1;

        let payload = quo_create_payload(&var_mut, "var_mut", line!(), file!(), true, false, "test");
        assert!(payload.meta.variable.is_mutable);
        assert!(!payload.meta.variable.is_expression);
    }

    #[test]
    fn test_macro_mut_binding() {
        let mut big_number = 170141183460469231731687303715884105727i128;
        
        // This is what quo!(big_number) does (pattern 2 in macros.rs)
        let payload = quo_create_payload(&big_number, "big_number", line!(), file!(), false, false, "test");
        
        // This will be FALSE because declarative macros cannot see if a binding is mutable.
        // T will be &i128 here or i128 depending on how it's called.
        // The macro currently does &big_number, which makes T = &i128.
        assert!(!payload.meta.variable.is_mutable);

        // However, if we pass it as an expression &mut big_number:
        // This is what quo!(&mut big_number) does (pattern 3 in macros.rs)
        let payload2 = quo_create_payload(&mut big_number, "&mut big_number", line!(), file!(), false, true, "test");
        
        // This should be TRUE because we detect &mut in the type name.
        assert!(payload2.meta.variable.is_mutable);
    }

    #[test]
    fn test_macro_mut_ref_auto() {
        let mut y = 1;
        let x = &mut y;

        // Automatically detected even if is_mutable parameter is false
        let payload = quo_create_payload(x, "x", line!(), file!(), false, false, "test");
        assert!(payload.meta.variable.is_mutable);
    }

    #[test]
    fn test_expression() {
        let payload = quo_create_payload(&(1 + 1), "1 + 1", line!(), file!(), false, true, "test");
        assert_eq!(payload.meta.variable.name, "1 + 1");
        assert_eq!(payload.meta.variable.value, "2");
        assert!(payload.meta.variable.is_expression);
    }
}
