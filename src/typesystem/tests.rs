use crate::{
    ast::{Operator, TypeNature},
    index::{symbol::SymbolLocation, Index},
    test_utils::{self, tests::index},
    typesystem::{
        self, get_equals_function_name_for, get_signed_type, Dimension, BOOL_TYPE, BYTE_TYPE, CHAR_TYPE,
        DATE_AND_TIME_TYPE, DATE_TYPE, DINT_TYPE, DWORD_TYPE, INT_TYPE, LINT_TYPE, LREAL_TYPE, LWORD_TYPE,
        REAL_TYPE, SINT_TYPE, STRING_TYPE, TIME_OF_DAY_TYPE, TIME_TYPE, UDINT_TYPE, UINT_TYPE, ULINT_TYPE,
        USINT_TYPE, WCHAR_TYPE, WORD_TYPE, WSTRING_TYPE,
    },
};

use super::{iec61131_types, TypeSize};

macro_rules! assert_signed_type {
    ($expected:expr, $actual:expr, $index:expr) => {
        assert_eq!(
            $index.find_effective_type_by_name($expected),
            get_signed_type($index.find_effective_type_by_name($actual).unwrap(), &$index)
        );
    };
}

#[test]
pub fn signed_types_tests() {
    // Given an initialized index
    let index = get_builtin_index();
    assert_signed_type!(SINT_TYPE, BYTE_TYPE, index);
    assert_signed_type!(SINT_TYPE, USINT_TYPE, index);
    assert_signed_type!(INT_TYPE, WORD_TYPE, index);
    assert_signed_type!(INT_TYPE, UINT_TYPE, index);
    assert_signed_type!(DINT_TYPE, DWORD_TYPE, index);
    assert_signed_type!(DINT_TYPE, UDINT_TYPE, index);
    assert_signed_type!(LINT_TYPE, ULINT_TYPE, index);
    assert_signed_type!(LINT_TYPE, LWORD_TYPE, index);

    let string_type = index.find_effective_type_by_name(STRING_TYPE).unwrap();
    assert_eq!(Some(string_type), get_signed_type(string_type, &index));
}

#[test]
pub fn equal_method_function_names() {
    assert_eq!(Some("STRING_EQUAL".to_string()), get_equals_function_name_for("STRING", &Operator::Equal));
    assert_eq!(Some("MY_TYPE_EQUAL".to_string()), get_equals_function_name_for("MY_TYPE", &Operator::Equal));
    assert_eq!(Some("STRING_LESS".to_string()), get_equals_function_name_for("STRING", &Operator::Less));
    assert_eq!(Some("MY_TYPE_LESS".to_string()), get_equals_function_name_for("MY_TYPE", &Operator::Less));
    assert_eq!(
        Some("STRING_GREATER".to_string()),
        get_equals_function_name_for("STRING", &Operator::Greater)
    );
    assert_eq!(
        Some("MY_TYPE_GREATER".to_string()),
        get_equals_function_name_for("MY_TYPE", &Operator::Greater)
    );
}

#[test]
fn get_bigger_size_integers_test() {
    // Given an initialized index
    let index = get_builtin_index();
    //Given integer types
    let sint_type = index.get_type_or_panic(SINT_TYPE);
    let int_type = index.get_type_or_panic(INT_TYPE);
    let dint_type = index.get_type_or_panic(DINT_TYPE);
    let lint_type = index.get_type_or_panic(LINT_TYPE);
    //Unsigned
    let usint_type = index.get_type_or_panic(USINT_TYPE);
    let uint_type = index.get_type_or_panic(UINT_TYPE);
    let udint_type = index.get_type_or_panic(UDINT_TYPE);
    let ulint_type = index.get_type_or_panic(ULINT_TYPE);

    //The bigger type is the one with the bigger size
    assert_eq!(int_type, typesystem::get_bigger_type(sint_type, int_type, &index));
    assert_eq!(dint_type, typesystem::get_bigger_type(int_type, dint_type, &index));
    assert_eq!(lint_type, typesystem::get_bigger_type(lint_type, dint_type, &index));
    assert_eq!(uint_type, typesystem::get_bigger_type(usint_type, uint_type, &index));
    assert_eq!(udint_type, typesystem::get_bigger_type(uint_type, udint_type, &index));
    assert_eq!(ulint_type, typesystem::get_bigger_type(ulint_type, udint_type, &index));
}

fn get_builtin_index() -> Index {
    let (_, index) = index("");
    index
}

#[test]
fn get_bigger_size_integers_mix_test() {
    // Given an initialized index
    let index = get_builtin_index();
    //Given integer types
    let sint_type = index.get_type_or_panic(SINT_TYPE);
    let int_type = index.get_type_or_panic(INT_TYPE);
    let dint_type = index.get_type_or_panic(DINT_TYPE);
    let lint_type = index.get_type_or_panic(LINT_TYPE);
    //Unsigned
    let usint_type = index.get_type_or_panic(USINT_TYPE);
    let uint_type = index.get_type_or_panic(UINT_TYPE);
    let udint_type = index.get_type_or_panic(UDINT_TYPE);
    let ulint_type = index.get_type_or_panic(ULINT_TYPE);

    assert_eq!(int_type, typesystem::get_bigger_type(sint_type, int_type, &index));
    assert_eq!(dint_type, typesystem::get_bigger_type(int_type, dint_type, &index));
    assert_eq!(lint_type, typesystem::get_bigger_type(lint_type, dint_type, &index));
    assert_eq!(uint_type, typesystem::get_bigger_type(usint_type, uint_type, &index));
    assert_eq!(udint_type, typesystem::get_bigger_type(uint_type, udint_type, &index));
    assert_eq!(ulint_type, typesystem::get_bigger_type(ulint_type, udint_type, &index));
    //The bigger type is the signed
    assert_eq!(sint_type, typesystem::get_bigger_type(sint_type, usint_type, &index));
    assert_eq!(int_type, typesystem::get_bigger_type(int_type, uint_type, &index));
    assert_eq!(dint_type, typesystem::get_bigger_type(dint_type, udint_type, &index));
}

#[test]
fn get_bigger_size_real_test() {
    // Given an initialized index
    let index = get_builtin_index();
    //Given two float numbers (REAL/LREAL)
    let real_type = index.get_type_or_panic(REAL_TYPE);
    let lreal_type = index.get_type_or_panic(LREAL_TYPE);
    //LREAL is bigger than REAL
    assert_eq!(lreal_type, typesystem::get_bigger_type(lreal_type, real_type, &index));
}

#[test]
fn get_bigger_size_numeric_test() {
    // Given an initialized index
    let index = get_builtin_index();
    //Given a float and an int
    //integer types
    let int_type = index.get_type_or_panic(INT_TYPE);
    let dint_type = index.get_type_or_panic(DINT_TYPE);
    let lint_type = index.get_type_or_panic(LINT_TYPE);

    //Float types
    let real_type = index.get_type_or_panic(REAL_TYPE);
    let lreal_type = index.get_type_or_panic(LREAL_TYPE);
    //The bigger type is the float
    assert_eq!(real_type, typesystem::get_bigger_type(real_type, int_type, &index));
    assert_eq!(real_type, typesystem::get_bigger_type(real_type, dint_type, &index));
    //Given an int that is bigger than a float in size (LINT)
    //The bigger type is an LREAL
    assert_eq!(lreal_type, typesystem::get_bigger_type(lint_type, real_type, &index));
}

#[test]
fn get_bigger_size_string_test() {
    // Given an initialized index
    let index = get_builtin_index();
    //Given two STRING
    let string_1024 = typesystem::DataType {
        name: "STRING_1024".into(),
        initial_value: None,
        definition: typesystem::DataTypeDefinition::String {
            size: TypeSize::LiteralInteger(1024),
            encoding: typesystem::StringEncoding::Utf8,
        },

        nature: TypeNature::String,
        location: SymbolLocation::internal(),
        alias_of: None,
    };
    let string_30 = typesystem::DataType {
        name: "STRING_30".into(),
        initial_value: None,
        definition: typesystem::DataTypeDefinition::String {
            size: TypeSize::LiteralInteger(30),
            encoding: typesystem::StringEncoding::Utf8,
        },
        nature: TypeNature::String,
        location: SymbolLocation::internal(),
        alias_of: None,
    };
    //The string with the bigger length is the bigger string
    assert_eq!(&string_1024, typesystem::get_bigger_type(&string_1024, &string_30, &index));
    assert_eq!(&string_1024, typesystem::get_bigger_type(&string_30, &string_1024, &index));

    //TODO : Strings with constant sizes
}

#[test]
fn get_bigger_size_array_test_returns_first() {
    // Given an initialized index
    let index = get_builtin_index();
    //Given two ARRAY of the same type and dimensions
    let array_1024 = typesystem::DataType {
        name: "ARRAY_1024".into(),
        initial_value: None,
        definition: typesystem::DataTypeDefinition::Array {
            inner_type_name: "INT".into(),
            dimensions: vec![Dimension {
                start_offset: TypeSize::LiteralInteger(0),
                end_offset: TypeSize::LiteralInteger(1023),
            }],
        },
        nature: TypeNature::Any,
        location: SymbolLocation::internal(),
        alias_of: None,
    };
    let array_30 = typesystem::DataType {
        name: "ARRAY_30".into(),
        initial_value: None,
        definition: typesystem::DataTypeDefinition::Array {
            inner_type_name: "INT".into(),
            dimensions: vec![Dimension {
                start_offset: TypeSize::LiteralInteger(1),
                end_offset: TypeSize::LiteralInteger(30),
            }],
        },
        nature: TypeNature::Any,
        location: SymbolLocation::internal(),
        alias_of: None,
    };
    //The array with the most elements is bigger
    assert_eq!(&array_1024, typesystem::get_bigger_type(&array_1024, &array_30, &index));
    assert_eq!(&array_30, typesystem::get_bigger_type(&array_30, &array_1024, &index));
}

#[test]
fn get_bigger_size_mixed_test_no_() {
    // Given an initialized index
    let index = get_builtin_index();
    //Int
    let int_type = index.get_type_or_panic(INT_TYPE);
    //String
    let string_1024 = typesystem::DataType {
        name: "STRING_1024".into(),
        initial_value: None,
        definition: typesystem::DataTypeDefinition::String {
            size: TypeSize::LiteralInteger(1024),
            encoding: typesystem::StringEncoding::Utf8,
        },
        nature: TypeNature::String,
        location: SymbolLocation::internal(),
        alias_of: None,
    };
    let wstring_1024 = typesystem::DataType {
        name: "WSTRING_1024".into(),
        initial_value: None,
        definition: typesystem::DataTypeDefinition::String {
            size: TypeSize::LiteralInteger(1024),
            encoding: typesystem::StringEncoding::Utf16,
        },
        nature: TypeNature::String,
        location: SymbolLocation::internal(),
        alias_of: None,
    };
    //Array of string
    let array_string_30 = typesystem::DataType {
        name: "ARRAY_STRING_30".into(),
        initial_value: None,
        definition: typesystem::DataTypeDefinition::Array {
            inner_type_name: "STRING".into(),
            dimensions: vec![Dimension {
                start_offset: TypeSize::LiteralInteger(1),
                end_offset: TypeSize::LiteralInteger(30),
            }],
        },
        nature: TypeNature::Any,
        location: SymbolLocation::internal(),
        alias_of: None,
    };
    //Array of int
    let array_30 = typesystem::DataType {
        name: "ARRAY_30".into(),
        initial_value: None,
        definition: typesystem::DataTypeDefinition::Array {
            inner_type_name: "INT".into(),
            dimensions: vec![Dimension {
                start_offset: TypeSize::LiteralInteger(1),
                end_offset: TypeSize::LiteralInteger(30),
            }],
        },
        nature: TypeNature::Any,
        location: SymbolLocation::internal(),
        alias_of: None,
    };
    //2-dim array of int
    let array_30_30 = typesystem::DataType {
        name: "ARRAY_30_30".into(),
        initial_value: None,
        definition: typesystem::DataTypeDefinition::Array {
            inner_type_name: "INT".into(),
            dimensions: vec![
                Dimension {
                    start_offset: TypeSize::LiteralInteger(1),
                    end_offset: TypeSize::LiteralInteger(30),
                },
                Dimension {
                    start_offset: TypeSize::LiteralInteger(1),
                    end_offset: TypeSize::LiteralInteger(30),
                },
            ],
        },
        nature: TypeNature::Any,
        location: SymbolLocation::internal(),
        alias_of: None,
    };

    //Given two incompatible types
    //The first given type is returned
    assert_eq!(&array_30, typesystem::get_bigger_type(&array_30, &array_30_30, &index));
    assert_eq!(&string_1024, typesystem::get_bigger_type(&string_1024, &array_30, &index));
    assert_eq!(&string_1024, typesystem::get_bigger_type(&string_1024, &wstring_1024, &index));
    assert_eq!(&wstring_1024, typesystem::get_bigger_type(&wstring_1024, &string_1024, &index));
    assert_eq!(&array_string_30, typesystem::get_bigger_type(&array_string_30, &array_30, &index));
    assert_eq!(int_type, typesystem::get_bigger_type(int_type, &array_30, &index));
}

fn get_index() -> Index {
    let (_, mut index) = test_utils::tests::index(iec61131_types::get_alias_types().as_str());
    for t in typesystem::get_builtin_types() {
        index.register_type(t)
    }
    index.resolve_alias_types();
    index
}

#[test]
fn any_signed_type_test() {
    let index = get_index();
    let sint = index.get_type_or_panic(SINT_TYPE);
    let int = index.get_type_or_panic(INT_TYPE);
    let dint = index.get_type_or_panic(DINT_TYPE);
    let lint = index.get_type_or_panic(LINT_TYPE);

    assert!(sint.has_nature(TypeNature::Signed));
    assert!(int.has_nature(TypeNature::Signed));
    assert!(dint.has_nature(TypeNature::Signed));
    assert!(lint.has_nature(TypeNature::Signed));

    assert!(sint.has_nature(TypeNature::Int));
    assert!(int.has_nature(TypeNature::Int));
    assert!(dint.has_nature(TypeNature::Int));
    assert!(lint.has_nature(TypeNature::Int));

    assert!(sint.has_nature(TypeNature::Num));
    assert!(int.has_nature(TypeNature::Num));
    assert!(dint.has_nature(TypeNature::Num));
    assert!(lint.has_nature(TypeNature::Num));

    assert!(sint.has_nature(TypeNature::Magnitude));
    assert!(int.has_nature(TypeNature::Magnitude));
    assert!(dint.has_nature(TypeNature::Magnitude));
    assert!(lint.has_nature(TypeNature::Magnitude));

    assert!(sint.has_nature(TypeNature::Elementary));
    assert!(int.has_nature(TypeNature::Elementary));
    assert!(dint.has_nature(TypeNature::Elementary));
    assert!(lint.has_nature(TypeNature::Elementary));

    assert!(sint.has_nature(TypeNature::Any));
    assert!(int.has_nature(TypeNature::Any));
    assert!(dint.has_nature(TypeNature::Any));
    assert!(lint.has_nature(TypeNature::Any));
}

#[test]
fn any_unsigned_type_test() {
    let index = get_index();
    let usint = index.get_type_or_panic(USINT_TYPE);
    let uint = index.get_type_or_panic(UINT_TYPE);
    let udint = index.get_type_or_panic(UDINT_TYPE);
    let ulint = index.get_type_or_panic(ULINT_TYPE);

    assert!(usint.has_nature(TypeNature::Unsigned));
    assert!(uint.has_nature(TypeNature::Unsigned));
    assert!(udint.has_nature(TypeNature::Unsigned));
    assert!(ulint.has_nature(TypeNature::Unsigned));

    assert!(usint.has_nature(TypeNature::Int));
    assert!(uint.has_nature(TypeNature::Int));
    assert!(udint.has_nature(TypeNature::Int));
    assert!(ulint.has_nature(TypeNature::Int));

    assert!(usint.has_nature(TypeNature::Num));
    assert!(uint.has_nature(TypeNature::Num));
    assert!(udint.has_nature(TypeNature::Num));
    assert!(ulint.has_nature(TypeNature::Num));

    assert!(usint.has_nature(TypeNature::Magnitude));
    assert!(uint.has_nature(TypeNature::Magnitude));
    assert!(udint.has_nature(TypeNature::Magnitude));
    assert!(ulint.has_nature(TypeNature::Magnitude));

    assert!(usint.has_nature(TypeNature::Elementary));
    assert!(uint.has_nature(TypeNature::Elementary));
    assert!(udint.has_nature(TypeNature::Elementary));
    assert!(ulint.has_nature(TypeNature::Elementary));

    assert!(usint.has_nature(TypeNature::Any));
    assert!(uint.has_nature(TypeNature::Any));
    assert!(udint.has_nature(TypeNature::Any));
    assert!(ulint.has_nature(TypeNature::Any));
}

#[test]
fn any_real_type_test() {
    let index = get_index();
    let real = index.get_type_or_panic(REAL_TYPE);
    let lreal = index.get_type_or_panic(LREAL_TYPE);

    assert!(real.has_nature(TypeNature::Real));
    assert!(lreal.has_nature(TypeNature::Real));

    assert!(real.has_nature(TypeNature::Num));
    assert!(lreal.has_nature(TypeNature::Num));

    assert!(real.has_nature(TypeNature::Magnitude));
    assert!(lreal.has_nature(TypeNature::Magnitude));

    assert!(real.has_nature(TypeNature::Elementary));
    assert!(lreal.has_nature(TypeNature::Elementary));

    assert!(real.has_nature(TypeNature::Any));
    assert!(lreal.has_nature(TypeNature::Any));
}

#[test]
fn any_duration_type_test() {
    let index = get_index();
    let time = index.get_type_or_panic(TIME_TYPE);
    // let ltime = index.get_type_or_panic(LTIME_TYTE);

    assert!(time.has_nature(TypeNature::Duration));

    assert!(time.has_nature(TypeNature::Magnitude));

    assert!(time.has_nature(TypeNature::Elementary));

    assert!(time.has_nature(TypeNature::Any));
}

#[test]
fn any_bit_type_test() {
    let index = get_index();
    let bool_type = index.get_type_or_panic(BOOL_TYPE);
    let byte = index.get_type_or_panic(BYTE_TYPE);
    let word = index.get_type_or_panic(WORD_TYPE);
    let dword = index.get_type_or_panic(DWORD_TYPE);
    let lword = index.get_type_or_panic(LWORD_TYPE);

    assert!(bool_type.has_nature(TypeNature::Bit));
    assert!(byte.has_nature(TypeNature::Bit));
    assert!(word.has_nature(TypeNature::Bit));
    assert!(dword.has_nature(TypeNature::Bit));
    assert!(lword.has_nature(TypeNature::Bit));

    assert!(bool_type.has_nature(TypeNature::Elementary));
    assert!(byte.has_nature(TypeNature::Elementary));
    assert!(word.has_nature(TypeNature::Elementary));
    assert!(dword.has_nature(TypeNature::Elementary));
    assert!(lword.has_nature(TypeNature::Elementary));

    assert!(bool_type.has_nature(TypeNature::Any));
    assert!(byte.has_nature(TypeNature::Any));
    assert!(word.has_nature(TypeNature::Any));
    assert!(dword.has_nature(TypeNature::Any));
    assert!(lword.has_nature(TypeNature::Any));
}

#[test]
fn any_string_type_test() {
    let index = get_index();
    let string = index.get_type_or_panic(STRING_TYPE);
    let wstring = index.get_type_or_panic(WSTRING_TYPE);

    assert!(string.has_nature(TypeNature::Chars));
    assert!(wstring.has_nature(TypeNature::Chars));

    assert!(string.has_nature(TypeNature::String));
    assert!(wstring.has_nature(TypeNature::String));

    assert!(string.has_nature(TypeNature::Elementary));
    assert!(wstring.has_nature(TypeNature::Elementary));

    assert!(string.has_nature(TypeNature::Any));
    assert!(wstring.has_nature(TypeNature::Any));
}

#[test]
fn any_char_type_test() {
    let index = get_index();
    let char = index.get_type_or_panic(CHAR_TYPE);
    let wchar = index.get_type_or_panic(WCHAR_TYPE);

    assert!(char.has_nature(TypeNature::Chars));
    assert!(wchar.has_nature(TypeNature::Chars));

    assert!(char.has_nature(TypeNature::Char));
    assert!(wchar.has_nature(TypeNature::Char));

    assert!(char.has_nature(TypeNature::Elementary));
    assert!(wchar.has_nature(TypeNature::Elementary));

    assert!(char.has_nature(TypeNature::Any));
    assert!(wchar.has_nature(TypeNature::Any));
}

#[test]
fn any_date_type_test() {
    let index = get_index();
    let date = index.get_type_or_panic(DATE_TYPE);
    let date_time = index.get_type_or_panic(DATE_AND_TIME_TYPE);
    let tod = index.get_type_or_panic(TIME_OF_DAY_TYPE);

    assert!(date.has_nature(TypeNature::Date));
    assert!(date_time.has_nature(TypeNature::Date));
    assert!(tod.has_nature(TypeNature::Date));

    assert!(date.has_nature(TypeNature::Elementary));
    assert!(date_time.has_nature(TypeNature::Elementary));
    assert!(tod.has_nature(TypeNature::Elementary));

    assert!(date.has_nature(TypeNature::Any));
    assert!(date_time.has_nature(TypeNature::Any));
    assert!(tod.has_nature(TypeNature::Any));
}

#[test]
fn array_size_single_dim_tests() {
    let index = get_index();
    //Given an ARRAY[1..20] OF INT
    let array_20 = typesystem::DataType {
        name: "ARRAY_20".into(),
        initial_value: None,
        definition: typesystem::DataTypeDefinition::Array {
            inner_type_name: "INT".into(),
            dimensions: vec![Dimension {
                start_offset: TypeSize::LiteralInteger(1),
                end_offset: TypeSize::LiteralInteger(20),
            }],
        },
        nature: TypeNature::Any,
        location: SymbolLocation::internal(),
        alias_of: None,
    };
    //the size of the array is 20*size(int)
    assert_eq!(320, array_20.get_definition().get_size_in_bits(&index));
}

#[test]
fn array_size_multi_dim_tests() {
    let index = get_index();
    //Given an ARRAY[1..20] OF INT
    let array_20_20 = typesystem::DataType {
        name: "ARRAY_20_20".into(),
        initial_value: None,
        definition: typesystem::DataTypeDefinition::Array {
            inner_type_name: "INT".into(),
            dimensions: vec![
                Dimension {
                    start_offset: TypeSize::LiteralInteger(1),
                    end_offset: TypeSize::LiteralInteger(20),
                },
                Dimension {
                    start_offset: TypeSize::LiteralInteger(-1),
                    end_offset: TypeSize::LiteralInteger(18),
                },
            ],
        },
        nature: TypeNature::Any,
        location: SymbolLocation::internal(),
        alias_of: None,
    };
    //the size of the array is 20*size(int)
    assert_eq!(6400, array_20_20.get_definition().get_size_in_bits(&index));
}

#[test]
fn array_size_nested_tests() {
    let mut index = get_index();
    //Given an ARRAY[1..20] OF INT
    let array_20 = typesystem::DataType {
        name: "ARRAY_20".into(),
        initial_value: None,
        definition: typesystem::DataTypeDefinition::Array {
            inner_type_name: "INT".into(),
            dimensions: vec![Dimension {
                start_offset: TypeSize::LiteralInteger(1),
                end_offset: TypeSize::LiteralInteger(20),
            }],
        },
        nature: TypeNature::Any,
        location: SymbolLocation::internal(),
        alias_of: None,
    };
    index.register_type(array_20);
    let nested_array = typesystem::DataType {
        name: "NESTED_ARRAY".into(),
        initial_value: None,
        definition: typesystem::DataTypeDefinition::Array {
            inner_type_name: "ARRAY_20".into(),
            dimensions: vec![Dimension {
                start_offset: TypeSize::LiteralInteger(1),
                end_offset: TypeSize::LiteralInteger(20),
            }],
        },
        nature: TypeNature::Any,
        location: SymbolLocation::internal(),
        alias_of: None,
    };

    //the size of the array is 20*size(int)
    assert_eq!(6400, nested_array.get_definition().get_size_in_bits(&index));
}
