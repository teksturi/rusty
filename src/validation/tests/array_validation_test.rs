use crate::test_utils::tests::parse_and_validate;
use crate::Diagnostic;

#[test]
fn array_access_validation() {
    let diagnostics = parse_and_validate(
        "
			VAR_GLOBAL CONSTANT
				start : INT := 1;
				end : INT := 2;
			END_VAR

        	PROGRAM prg
        	VAR
				multi : ARRAY[0..1,2..3] OF INT;
				nested : ARRAY[0..1] OF ARRAY[2..3] OF INT;
				arr : ARRAY[0..1] OF INT;
				negative_start : ARRAY[-2..2] OF INT;
				negative : ARRAY[-3..-1] OF INT;
				const : ARRAY[start..end] OF INT;
				int_ref : INT;
				string_ref : STRING;
        	END_VAR

			// valid
			multi[0,3];
			nested[1][3];
			arr[1];
			negative_start[-1];
			negative[-2];
			const[1];
			arr[int_ref];

			// invalid
			multi[1,4]; // out of range
			nested[1][4]; // out of range
			arr[3]; // out of range
			negative_start[-4]; // out of range
			negative[-4]; // out of range
			const[3]; // out of range
			arr[string_ref]; // invalid type for array access
			int_ref[1]; // not an array
        	END_PROGRAM
       ",
    );

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::incompatible_array_access_range(2..3, (557..558).into()),
            Diagnostic::incompatible_array_access_range(2..3, (590..591).into()),
            Diagnostic::incompatible_array_access_range(0..1, (617..618).into()),
            Diagnostic::incompatible_array_access_range(-2..2, (655..657).into()),
            Diagnostic::incompatible_array_access_range(-3..-1, (688..690).into()),
            Diagnostic::incompatible_array_access_range(1..2, (718..719).into()),
            Diagnostic::incompatible_array_access_type("STRING", (745..755).into()),
            Diagnostic::incompatible_array_access_variable("INT", (802..803).into()),
        ]
    );
}

#[test]
fn array_initialization_validation() {
    let diagnostics = parse_and_validate(
        "
		FUNCTION main : DINT
		VAR
			arr	 : ARRAY[1..2] OF DINT;
			arr2 : ARRAY[1..2] OF DINT := 1, 2; // our parser can handle this, should we validate this ?
			arr3 : ARRAY[1..2] OF myStruct := ((var1 := 1), (var1 := 2, var2 := (1, 2))); // valid
			arr4 : ARRAY[1..2] OF myStruct := ((var1 := 1), (var1 := 2, var2 := 1, 2)); // var2 missing `(`
			x	 : myStruct;
			y	 : myStruct := (var1 := 1, var2 := 3, 4); // var2 missing `(`
		END_VAR
			arr := 1, 2; // missing `(`
			arr := (1, 2); // valid
			x := (var1 := 1, var2 := 3, 4); // var2 missing `(`
		END_FUNCTION
		
		TYPE myStruct : STRUCT
				var1 : DINT;
				var2 : ARRAY[1..2] OF DINT;
			END_STRUCT
		END_TYPE
       ",
    );

    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::array_expected_initializer_list((310..314).into()),
            Diagnostic::array_expected_identifier_or_round_bracket((321..322).into()),
            Diagnostic::array_expected_initializer_list((396..400).into()),
            Diagnostic::array_expected_identifier_or_round_bracket((407..408).into()),
            Diagnostic::array_expected_initializer_list((444..447).into()),
            Diagnostic::array_expected_identifier_or_round_bracket((454..455).into()),
            Diagnostic::array_expected_initializer_list((519..523).into()),
            Diagnostic::array_expected_identifier_or_round_bracket((530..531).into()),
        ]
    );
}

#[test]
fn array_initialization_validation_XXX() {
    let diagnostics = parse_and_validate(
        "
		FUNCTION main : DINT
		VAR
			x	 : myStruct;
			y : ARRAY[0..3] OF DINT;
			z : DINT;
		END_VAR
			x := (var1 := 1, var2 := z); // var2 missing `(`
			y := z;
			y := 3;
			x := 'abc';
		END_FUNCTION
		
		TYPE myStruct : STRUCT
				var1 : DINT;
				var2 : ARRAY[1..2] OF DINT;
			END_STRUCT
		END_TYPE
       ",
    );

    assert_eq!(
        diagnostics,
        vec![
        ]
    );
}

