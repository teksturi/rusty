use crate::{assert_validation_snapshot, test_utils::tests::parse_and_validate};

static SOURCE: &str = "
    <POU_TYPE> fn : DINT
        VAR_<VAR_TYPE>
            arr : ARRAY[*] OF DINT;
        END_<VAR_TYPE>
    END_<POU_TYPE>

    FUNCTION main : DINT
        VAR
            local : ARRAY[-5..5] OF DINT;
        END_VAR

        fn(local);
    END_FUNCTION
";

#[test]
fn variable_length_array_defined_as_a_global_variable() {
    let src = "
        VAR_GLOBAL
            arr : ARRAY[*] OF DINT;
        END_VAR
    ";

    assert_validation_snapshot!(parse_and_validate(src));
}

mod functions {
    use crate::{
        assert_validation_snapshot, test_utils::tests::parse_and_validate,
        validation::tests::variable_length_array_test::SOURCE,
    };

    #[test]
    fn variable_length_array_function_with_input_output_and_inout() {
        let function = SOURCE.replace("<POU_TYPE>", "FUNCTION");
        assert!(parse_and_validate(&function.replace("<VAR_TYPE>", "INPUT {ref}")).is_empty());
        assert!(parse_and_validate(&function.replace("<VAR_TYPE>", "OUTPUT")).is_empty());
        assert!(parse_and_validate(&function.replace("<VAR_TYPE>", "IN_OUT")).is_empty());
    }

    #[test]
    fn variable_length_array_function_input() {
        let function = SOURCE.replace("<POU_TYPE>", "FUNCTION");
        assert_validation_snapshot!(parse_and_validate(&function.replace("<VAR_TYPE>", "INPUT")));
    }
}

mod program {
    use crate::{
        assert_validation_snapshot, test_utils::tests::parse_and_validate,
        validation::tests::variable_length_array_test::SOURCE,
    };

    #[test]
    fn variable_length_array_program_input() {
        let program = SOURCE.replace("<POU_TYPE>", "PROGRAM");
        let program_input = parse_and_validate(&program.replace("<VAR_TYPE>", "INPUT"));
        assert_validation_snapshot!(program_input);
    }

    #[test]
    fn variable_length_array_program_input_ref() {
        let program = SOURCE.replace("<POU_TYPE>", "PROGRAM");
        let program_input = parse_and_validate(&program.replace("<VAR_TYPE>", "INPUT {ref}"));
        assert_validation_snapshot!(program_input);
    }

    #[test]
    fn variable_length_array_program_output() {
        let program = SOURCE.replace("<POU_TYPE>", "PROGRAM");
        let program_output = parse_and_validate(&program.replace("<VAR_TYPE>", "OUTPUT"));
        assert_validation_snapshot!(program_output);
    }

    #[test]
    fn variable_length_array_program_inout() {
        let program = SOURCE.replace("<POU_TYPE>", "PROGRAM");
        let program_inout = parse_and_validate(&program.replace("<VAR_TYPE>", "IN_OUT"));
        assert_validation_snapshot!(program_inout);
    }
}

mod access {
    use crate::{assert_validation_snapshot, test_utils::tests::parse_and_validate};

    #[test]
    fn variable_length_array_access() {
        let diagnostics = parse_and_validate(
            "
            FUNCTION fn : DINT
                VAR_INPUT {ref}
                    arr : ARRAY[*] OF DINT;
                END_VAR

                arr[0]      := 1;
                arr[0, 0]   := 1; // This should fail (arr is defined as a 1D array)
            END_FUNCTION

            FUNCTION main : DINT
                VAR
                    local_a : ARRAY[0..10] OF DINT;
                    local_b : ARRAY[0..5, 5..10] OF DINT;
                END_VAR

                fn(local_a);
                fn(local_b); // This call should fail, because we expect a 1D array
            END_FUNCTION
        ",
        );

        assert_validation_snapshot!(diagnostics);
    }

    #[test]
    fn variable_length_array_incompatible_datatypes() {
        let diagnostics = parse_and_validate(
            "
            FUNCTION fn : DINT
                VAR_INPUT {ref}
                    arr : ARRAY[*] OF DINT;
                END_VAR
            END_FUNCTION

            FUNCTION main : DINT
                VAR
                    local_int       : ARRAY[0..10] OF INT;
                    local_float     : ARRAY[0..10] OF REAL;
                    local_string    : ARRAY[0..10] OF STRING;
                END_VAR

                fn(local_int);
                fn(local_float);
                fn(local_string);
            END_FUNCTION
        ",
        );

        assert_validation_snapshot!(diagnostics);
    }
}
