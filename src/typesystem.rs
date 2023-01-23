// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use std::{
    mem::size_of,
    ops::{Range, RangeInclusive},
};

use crate::{
    ast::{AstStatement, Operator, PouType, TypeNature},
    datalayout::{Bytes, MemoryLocation},
    index::{const_expressions::ConstId, symbol::SymbolLocation, Index},
};

pub const DEFAULT_STRING_LEN: u32 = 80;

// Ranged type check functions names
pub const RANGE_CHECK_S_FN: &str = "CheckRangeSigned";
pub const RANGE_CHECK_LS_FN: &str = "CheckLRangeSigned";
pub const RANGE_CHECK_U_FN: &str = "CheckRangeUnsigned";
pub const RANGE_CHECK_LU_FN: &str = "CheckLRangeUnsigned";

pub type NativeSintType = i8;
pub type NativeIntType = i16;
pub type NativeDintType = i32;
pub type NativeLintType = i64;
pub type NativeByteType = u8;
pub type NativeWordType = u16;
pub type NativeDwordType = u32;
pub type NativeLwordType = u64;
pub type NativeRealType = f32;
pub type NativeLrealType = f64;
pub type NativePointerType = usize;

//TODO should we change this to usize?
pub const U1_SIZE: u32 = 1;
pub const BOOL_SIZE: u32 = BYTE_SIZE;
pub const BYTE_SIZE: u32 = NativeSintType::BITS as u32;
pub const SINT_SIZE: u32 = NativeSintType::BITS as u32;
pub const INT_SIZE: u32 = NativeIntType::BITS as u32;
pub const DINT_SIZE: u32 = NativeDintType::BITS as u32;
pub const LINT_SIZE: u32 = NativeLintType::BITS as u32;
pub const REAL_SIZE: u32 = (size_of::<NativeRealType>() * 8) as u32;
pub const LREAL_SIZE: u32 = (size_of::<NativeLrealType>() * 8) as u32;
pub const DATE_TIME_SIZE: u32 = 64;
pub const POINTER_SIZE: u32 = NativePointerType::BITS as u32;

pub const U1_TYPE: &str = "UINT1";
pub const U8_TYPE: &str = "UINT8";
pub const U16_TYPE: &str = "UINT16";
pub const U32_TYPE: &str = "UINT32";
pub const U64_TYPE: &str = "UINT64";
pub const I8_TYPE: &str = "INT8";
pub const I16_TYPE: &str = "INT16";
pub const I32_TYPE: &str = "INT32";
pub const I64_TYPE: &str = "INT64";

pub const F32_TYPE: &str = "REAL32";
pub const F64_TYPE: &str = "REAL64";

/// used internally for forced casts to u1
pub const BOOL_TYPE: &str = "BOOL";
pub const BYTE_TYPE: &str = "BYTE";
pub const SINT_TYPE: &str = "SINT";
pub const USINT_TYPE: &str = "USINT";
pub const WORD_TYPE: &str = "WORD";
pub const INT_TYPE: &str = "INT";
pub const UINT_TYPE: &str = "UINT";
pub const DWORD_TYPE: &str = "DWORD";
pub const DINT_TYPE: &str = "DINT";
pub const UDINT_TYPE: &str = "UDINT";
pub const LWORD_TYPE: &str = "LWORD";
pub const LINT_TYPE: &str = "LINT";
pub const DATE_TYPE: &str = "DATE";
pub const SHORT_DATE_TYPE: &str = "D";
pub const LONG_DATE_TYPE: &str = "LDATE";
pub const LONG_DATE_TYPE_SHORTENED: &str = "LD";
pub const TIME_TYPE: &str = "TIME";
pub const SHORT_TIME_TYPE: &str = "T";
pub const LONG_TIME_TYPE: &str = "LTIME";
pub const LONG_TIME_TYPE_SHORTENED: &str = "LT";
pub const DATE_AND_TIME_TYPE: &str = "DATE_AND_TIME";
pub const SHORT_DATE_AND_TIME_TYPE: &str = "DT";
pub const LONG_DATE_AND_TIME_TYPE: &str = "LDATE_AND_TIME";
pub const LONG_DATE_AND_TIME_TYPE_SHORTENED: &str = "LDT";
pub const TIME_OF_DAY_TYPE: &str = "TIME_OF_DAY";
pub const SHORT_TIME_OF_DAY_TYPE: &str = "TOD";
pub const LONG_TIME_OF_DAY_TYPE: &str = "LTIME_OF_DAY";
pub const LONG_TIME_OF_DAY_TYPE_SHORTENED: &str = "LTOD";
pub const ULINT_TYPE: &str = "ULINT";
pub const REAL_TYPE: &str = "REAL";
pub const LREAL_TYPE: &str = "LREAL";
pub const STRING_TYPE: &str = "STRING";
pub const WSTRING_TYPE: &str = "WSTRING";
pub const CHAR_TYPE: &str = "CHAR";
pub const WCHAR_TYPE: &str = "WCHAR";
pub const VOID_TYPE: &str = "VOID";

pub mod iec61131_types;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq)]
pub struct DataType {
    /// the declaration name of the datatype
    pub name: String,
    /// the initial value defined on the TYPE-declaration
    pub initial_value: Option<ConstId>,
    /// the defintion of the type. It holds information about the type's inner structure (numeric, array, string, etc.)
    pub definition: DataTypeDefinition,
    /// the type's nature according to IEC61131-3
    pub nature: TypeNature,
    /// the location of the type's declaration
    pub location: SymbolLocation,
    /// the name of the original type this is aliasing
    pub alias_of: Option<String>,
    /// an optional range limitation of the type (for sub-range types)
    pub sub_range: Option<Range<AstStatement>>,
}

impl DataType {
    pub fn new(
        name: String,
        initial_value: Option<ConstId>,
        information: DataTypeDefinition,
        nature: TypeNature,
        location: SymbolLocation,
    ) -> Self {
        Self {
            name,
            initial_value,
            nature,
            location,
            definition: information,
            alias_of: None,
            sub_range: None,
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_definition(&self) -> &DataTypeDefinition {
        &self.definition
    }

    pub fn has_nature(&self, nature: TypeNature) -> bool {
        self.nature.derives(nature)
    }

    pub fn is_numerical(&self) -> bool {
        self.nature.is_numerical()
    }

    pub fn is_real(&self) -> bool {
        self.nature.is_real()
    }

    pub fn is_char(&self) -> bool {
        self.has_nature(TypeNature::Char)
    }

    /// returns true if this type is an internal, auto-generated type
    pub fn is_internal(&self) -> bool {
        self.location.is_internal()
    }

    /// returns true if this type is an array, struct or string
    pub fn is_aggregate_type(&self) -> bool {
        self.get_definition().is_agregate()
    }

    pub fn create_alias(
        &self,
        new_name: String,
        nature: TypeNature,
        initial_value: Option<ConstId>,
        location: SymbolLocation,
    ) -> DataType {
        DataType {
            name: new_name.clone(),
            initial_value,
            definition: self.definition.clone_with_new_name(new_name),
            nature,
            location,
            alias_of: Some(self.get_name().to_string()),
            sub_range: self.sub_range.clone(),
        }
    }

    pub fn get_display_name(&self, index: &Index) -> String {
        match self.get_definition() {
            DataTypeDefinition::Array { inner_type_name, dimensions } => {
                let dimensions_strings = dimensions
                    .iter()
                    .map(|it| {
                        format!(
                            "[{}]",
                            it.get_range(index)
                                .map(|it| format!("{:?}", it))
                                .unwrap_or_else(|_| "?".to_string())
                        )
                    })
                    .collect::<Vec<_>>();

                let inner_type_name = index.get_type_or_void(inner_type_name).get_display_name(index);
                format!("ARRAY{} OF {inner_type_name}", dimensions_strings.join(","))
            }
            DataTypeDefinition::Pointer { inner_type_name, .. } => {
                let inner_type_name = index.get_type_or_void(inner_type_name).get_display_name(index);
                format!("REF_TO {inner_type_name}")
            }
            DataTypeDefinition::String { encoding: StringEncoding::Utf8, .. } => STRING_TYPE.to_string(),
            DataTypeDefinition::String { encoding: StringEncoding::Utf16, .. } => WSTRING_TYPE.to_string(),
            _ => self.get_name().to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VarArgs {
    Sized(Option<String>),
    Unsized(Option<String>),
}

impl VarArgs {
    pub fn is_sized(&self) -> bool {
        matches!(self, VarArgs::Sized(..))
    }

    pub fn as_typed(&self, new_type: &str) -> VarArgs {
        match self {
            VarArgs::Sized(Some(_)) => VarArgs::Sized(Some(new_type.to_string())),
            VarArgs::Unsized(Some(_)) => VarArgs::Unsized(Some(new_type.to_string())),
            _ => self.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringEncoding {
    Utf8,
    Utf16,
}

impl StringEncoding {
    pub fn get_bytes_per_char(&self) -> u32 {
        match self {
            StringEncoding::Utf8 => 1,
            StringEncoding::Utf16 => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeSize {
    LiteralInteger(i64),
    ConstExpression(ConstId),
}

impl TypeSize {
    pub fn from_literal(v: i64) -> TypeSize {
        TypeSize::LiteralInteger(v)
    }

    pub fn from_expression(id: ConstId) -> TypeSize {
        TypeSize::ConstExpression(id)
    }

    /// tries to compile-time evaluate the size-expression to an i64
    pub fn as_int_value(&self, index: &Index) -> Result<i64, String> {
        match self {
            TypeSize::LiteralInteger(v) => Ok(*v),
            TypeSize::ConstExpression(id) => {
                index.get_const_expressions().get_constant_int_statement_value(id).map(|it| it as i64)
            }
        }
    }

    /// returns the const expression represented by this TypeSize or None if this TypeSize
    /// is a compile-time literal
    pub fn as_const_expression<'i>(&self, index: &'i Index) -> Option<&'i AstStatement> {
        match self {
            TypeSize::LiteralInteger(_) => None,
            TypeSize::ConstExpression(id) => index.get_const_expressions().get_constant_statement(id),
        }
    }
}

/// indicates where this Struct origins from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructSource {
    OriginalDeclaration,
    Pou(PouType),
}

type TypeId = String;

#[derive(Debug, Clone, PartialEq)]
pub enum DataTypeDefinition {
    Struct {
        container_name: String,
        member_names: Vec<String>,
        source: StructSource,
    },
    Array {
        inner_type_name: TypeId,
        dimensions: Vec<Dimension>,
    },
    Pointer {
        inner_type_name: TypeId,
        auto_deref: bool,
    },
    Integer {
        signed: bool,
        /// the number of bit stored in memory
        size: u32,
        /// the numer of bits represented by this type (may differ from the num acutally stored)
        semantic_size: Option<u32>,
    },
    Enum {
        referenced_type: TypeId,
        elements: Vec<String>,
    },
    Float {
        size: u32,
    },
    String {
        size: TypeSize,
        encoding: StringEncoding,
    },
    SubRange {
        referenced_type: TypeId,
        sub_range: Range<AstStatement>,
    },
    Alias {
        referenced_type: TypeId,
    },
    Generic {
        generic_symbol: String,
        nature: TypeNature,
    },
    Void,
}

impl DataTypeDefinition {
    pub fn is_string(&self) -> bool {
        matches!(self, DataTypeDefinition::String { .. })
    }

    //TODO
    // pub fn is_character(&self) -> bool {
    //     match self {
    //         DataTypeDefinition::Integer { name, .. } => name == WCHAR_TYPE || name == CHAR_TYPE,
    //         _ => false,
    //     }
    // }

    pub fn is_int(&self) -> bool {
        // internally an enum is represented as a DINT
        matches!(self, DataTypeDefinition::Integer { .. } | DataTypeDefinition::Enum { .. })
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, DataTypeDefinition::Integer { semantic_size: Some(1), .. })
    }

    pub fn is_pointer(&self) -> bool {
        matches!(self, DataTypeDefinition::Pointer { .. })
    }

    pub fn is_unsigned_int(&self) -> bool {
        matches!(self, DataTypeDefinition::Integer { signed: false, .. })
    }

    pub fn is_signed_int(&self) -> bool {
        matches!(self, DataTypeDefinition::Integer { signed: true, .. })
    }

    pub fn is_float(&self) -> bool {
        matches!(self, DataTypeDefinition::Float { .. })
    }

    pub fn is_struct(&self) -> bool {
        matches!(self, DataTypeDefinition::Struct { .. })
    }

    pub fn is_array(&self) -> bool {
        matches!(self, DataTypeDefinition::Array { .. })
    }

    pub fn is_numerical(&self) -> bool {
        matches!(
            self,
            DataTypeDefinition::Integer { .. }
                | DataTypeDefinition::Float { .. }
                | &DataTypeDefinition::Enum { .. } // internally an enum is represented as a DINT
        )
    }

    pub fn is_generic(&self, index: &Index) -> bool {
        match self {
            DataTypeDefinition::Array { inner_type_name, .. }
            | DataTypeDefinition::Pointer { inner_type_name, .. }
            | DataTypeDefinition::Alias { referenced_type: inner_type_name, .. } => index
                .find_effective_type_by_name(inner_type_name)
                .map(|dt| dt.get_definition().is_generic(index))
                .unwrap_or(false),
            DataTypeDefinition::Generic { .. } => true,
            _ => false,
        }
    }
    /// returns the number of bits of this type, as understood by IEC61131 (may be smaller than get_size(...))
    pub fn get_semantic_size(&self, index: &Index) -> u32 {
        if let DataTypeDefinition::Integer { semantic_size: Some(s), .. } = self {
            return *s;
        }
        self.get_size_in_bits(index)
    }

    /// returns the number of bits used to store this type
    pub fn get_size_in_bits(&self, index: &Index) -> u32 {
        self.get_size(index).bits()
    }

    pub fn get_size(&self, index: &Index) -> Bytes {
        match self {
            DataTypeDefinition::Integer { size, .. } => Bytes::from_bits(*size),
            DataTypeDefinition::Float { size, .. } => Bytes::from_bits(*size),
            DataTypeDefinition::String { size, encoding } => size
                .as_int_value(index)
                .map(|size| encoding.get_bytes_per_char() * size as u32)
                .map(Bytes::from_bits)
                .unwrap(),
            DataTypeDefinition::Struct { member_names, container_name, .. } => member_names
                .iter()
                .filter_map(|it| index.find_member(&container_name, it))
                .map(|it| it.get_type_name())
                .fold(MemoryLocation::new(0), |prev, it| {
                    let type_info = index.get_type_information_or_void(it);
                    let size = type_info.get_size(index).value();
                    let after_align = prev.align_to(type_info.get_alignment(index)).value();
                    let res = after_align + size;
                    MemoryLocation::new(res)
                })
                .into(),
            DataTypeDefinition::Array { inner_type_name, dimensions, .. } => {
                let inner_type = index.get_type_information_or_void(inner_type_name);
                let inner_size = inner_type.get_size_in_bits(index);
                let element_count: u32 =
                    dimensions.iter().map(|dim| dim.get_length(index).unwrap()).product();
                Bytes::from_bits(inner_size * element_count)
            }
            DataTypeDefinition::Pointer { .. } => Bytes::from_bits(POINTER_SIZE),
            DataTypeDefinition::Alias { referenced_type, .. }
            | DataTypeDefinition::SubRange { referenced_type, .. } => {
                let inner_type = index.get_type_information_or_void(referenced_type);
                inner_type.get_size(index)
            }
            DataTypeDefinition::Enum { referenced_type, .. } => index
                .find_effective_type_info(referenced_type)
                .map(|it| it.get_size(index))
                .unwrap_or_else(|| Bytes::from_bits(DINT_SIZE)),
            DataTypeDefinition::Generic { .. } | DataTypeDefinition::Void => Bytes::from_bits(0),
        }
    }

    /// Returns the String encoding's alignment (character)
    pub fn get_string_character_width(&self, index: &Index) -> Bytes {
        let type_layout = index.get_type_layout();
        match self {
            DataTypeDefinition::String { encoding: StringEncoding::Utf8, .. } => type_layout.i8,
            DataTypeDefinition::String { encoding: StringEncoding::Utf16, .. } => type_layout.i16,
            _ => unreachable!("Expected string found {}", self.to_str()),
        }
    }

    pub fn get_alignment(&self, index: &Index) -> Bytes {
        let type_layout = index.get_type_layout();
        match self {
            DataTypeDefinition::Array { inner_type_name, .. } => {
                let inner_type = index.get_type_information_or_void(inner_type_name);
                if inner_type.get_alignment(index) > type_layout.i64 {
                    type_layout.v128
                } else {
                    type_layout.v64
                }
            }
            DataTypeDefinition::Struct { .. } => type_layout.aggregate,
            DataTypeDefinition::String { .. } => type_layout.v64, //Strings are arrays
            DataTypeDefinition::Pointer { .. } => type_layout.p64,
            DataTypeDefinition::Integer { size, semantic_size, .. } => {
                if let Some(1) = semantic_size {
                    type_layout.i1
                } else {
                    match size {
                        8 => type_layout.i8,
                        16 => type_layout.i16,
                        32 => type_layout.i32,
                        64 => type_layout.i64,
                        _ => type_layout.p64,
                    }
                }
            }
            DataTypeDefinition::Enum { referenced_type, .. } => {
                index.get_type_information_or_void(referenced_type).get_alignment(index)
            }
            DataTypeDefinition::Float { size, .. } => match size {
                32 => type_layout.f32,
                64 => type_layout.f64,
                _ => type_layout.p64,
            },
            DataTypeDefinition::SubRange { referenced_type, .. } => {
                index.get_type_information_or_void(referenced_type).get_alignment(index)
            }
            DataTypeDefinition::Alias { referenced_type, .. } => {
                index.get_type_information_or_void(referenced_type).get_alignment(index)
            }
            _ => type_layout.i8,
        }
    }

    fn is_agregate(&self) -> bool {
        matches!(
            self,
            DataTypeDefinition::Struct { .. }
                | DataTypeDefinition::Array { .. }
                | DataTypeDefinition::String { .. }
        )
    }

    fn clone_with_new_name(&self, new_name: String) -> DataTypeDefinition {
        let mut cpy = self.clone();
        // cpy.set_name(new_name);
        cpy
    }

    fn to_str(&self) -> &str {
        match self {
            DataTypeDefinition::Struct { .. } => "Struct",
            DataTypeDefinition::Array { .. } => "Array",
            DataTypeDefinition::Pointer { .. } => "Pointer",
            DataTypeDefinition::Integer { .. } => "Integer",
            DataTypeDefinition::Enum { .. } => "Enum",
            DataTypeDefinition::Float { .. } => "Float",
            DataTypeDefinition::String { .. } => "String",
            DataTypeDefinition::SubRange { .. } => "SubRange",
            DataTypeDefinition::Alias { .. } => "Alias",
            DataTypeDefinition::Generic { .. } => "Generic",
            DataTypeDefinition::Void => "Void",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dimension {
    pub start_offset: TypeSize,
    pub end_offset: TypeSize,
}

impl Dimension {
    pub fn get_length(&self, index: &Index) -> Result<u32, String> {
        let end = self.end_offset.as_int_value(index)?;
        let start = self.start_offset.as_int_value(index)?;
        Ok((end - start + 1) as u32)
    }

    pub fn get_range(&self, index: &Index) -> Result<Range<i64>, String> {
        let start = self.start_offset.as_int_value(index)?;
        let end = self.end_offset.as_int_value(index)?;
        Ok(start..end)
    }

    pub fn get_range_inclusive(&self, index: &Index) -> Result<RangeInclusive<i64>, String> {
        let start = self.start_offset.as_int_value(index)?;
        let end = self.end_offset.as_int_value(index)?;
        Ok(start..=end)
    }
}

pub trait DataTypeInformationProvider<'a>: Into<&'a DataTypeDefinition> {
    fn get_type_information(&self) -> &DataTypeDefinition;
}

impl<'a> DataTypeInformationProvider<'a> for &'a DataTypeDefinition {
    fn get_type_information(&self) -> &'a DataTypeDefinition {
        self
    }
}

impl<'a> From<&'a DataType> for &'a DataTypeDefinition {
    fn from(dt: &'a DataType) -> Self {
        dt.get_definition()
    }
}

impl<'a> DataTypeInformationProvider<'a> for &'a DataType {
    fn get_type_information(&self) -> &DataTypeDefinition {
        DataType::get_definition(self)
    }
}

macro_rules! int_type {
    ($name:expr, $size:expr, $nature: expr, $signed:expr) => {
        DataType {
            name: $name,
            initial_value: None,
            definition: DataTypeDefinition::Integer { signed: $signed, size: $size, semantic_size: None },
            nature: $nature,
            location: SymbolLocation::internal(),
            alias_of: None,
            sub_range: None,
        }
    };
}

pub fn get_builtin_types() -> Vec<DataType> {
    vec![
        DataType::new(
            "VOID".into(),
            None,
            DataTypeDefinition::Void,
            TypeNature::Any,
            SymbolLocation::internal(),
        ),
        int_type!(U1_TYPE.into(), 1, TypeNature::Bit, false),
        int_type!(U8_TYPE.into(), 8, TypeNature::Unsigned, false),
        int_type!(U16_TYPE.into(), 16, TypeNature::Unsigned, false),
        int_type!(U32_TYPE.into(), 32, TypeNature::Unsigned, false),
        int_type!(U64_TYPE.into(), 64, TypeNature::Unsigned, false),
        int_type!(I8_TYPE.into(), 8, TypeNature::Signed, true),
        int_type!(I16_TYPE.into(), 16, TypeNature::Signed, true),
        int_type!(I32_TYPE.into(), 32, TypeNature::Signed, true),
        int_type!(I64_TYPE.into(), 64, TypeNature::Signed, true),
        DataType::new(
            BOOL_TYPE.into(),
            None,
            DataTypeDefinition::Integer { signed: false, size: BOOL_SIZE, semantic_size: Some(1) },
            TypeNature::Bit,
            SymbolLocation::internal(),
        ),
        DataType::new(
            F32_TYPE.into(),
            None,
            DataTypeDefinition::Float { size: 32 },
            TypeNature::Real,
            SymbolLocation::internal(),
        ),
        DataType::new(
            F64_TYPE.into(),
            None,
            DataTypeDefinition::Float { size: 64 },
            TypeNature::Real,
            SymbolLocation::internal(),
        ),
        DataType::new(
            STRING_TYPE.into(),
            None,
            DataTypeDefinition::String {
                size: TypeSize::from_literal((DEFAULT_STRING_LEN + 1).into()),
                encoding: StringEncoding::Utf8,
            },
            TypeNature::String,
            SymbolLocation::internal(),
        ),
        DataType::new(
            WSTRING_TYPE.into(),
            None,
            DataTypeDefinition::String {
                size: TypeSize::from_literal((DEFAULT_STRING_LEN + 1).into()),
                encoding: StringEncoding::Utf16,
            },
            TypeNature::String,
            SymbolLocation::internal(),
        ),
        DataType::new(
            CHAR_TYPE.into(),
            None,
            DataTypeDefinition::Integer { signed: false, size: 8, semantic_size: None },
            TypeNature::Char,
            SymbolLocation::internal(),
        ),
        DataType::new(
            WCHAR_TYPE.into(),
            None,
            DataTypeDefinition::Integer { signed: false, size: 16, semantic_size: None },
            TypeNature::Char,
            SymbolLocation::internal(),
        ),
    ]
}

fn get_rank(type_information: &DataTypeDefinition, index: &Index) -> u32 {
    match type_information {
        DataTypeDefinition::Integer { signed, size, .. } => {
            if *signed {
                *size + 1
            } else {
                *size
            }
        }
        DataTypeDefinition::Float { size, .. } => size + 1000,
        DataTypeDefinition::String { size, .. } => match size {
            TypeSize::LiteralInteger(size) => (*size).try_into().unwrap(),
            TypeSize::ConstExpression(_) => todo!("String rank with CONSTANTS"),
        },
        DataTypeDefinition::Enum { referenced_type, .. } => {
            index.find_effective_type_info(referenced_type).map(|it| get_rank(it, index)).unwrap_or(DINT_SIZE)
        }
        //TODO
        // DataTypeDefinition::SubRange { name, .. } | DataTypeDefinition::Alias { name, .. } => {
        //     get_rank(index.get_intrinsic_type_by_name(name).get_type_information(), index)
        // }
        _ => type_information.get_size_in_bits(index),
    }
}

/// Returns true if provided types have the same type nature
/// i.e. Both are numeric or both are floats
pub fn is_same_type_class(ltype: &DataTypeDefinition, rtype: &DataTypeDefinition, index: &Index) -> bool {
    let ltype = index.find_intrinsic_type(ltype);
    let rtype = index.find_intrinsic_type(rtype);

    match ltype {
        DataTypeDefinition::Integer { .. } => matches!(rtype, DataTypeDefinition::Integer { .. }),
        DataTypeDefinition::Float { .. } => matches!(rtype, DataTypeDefinition::Float { .. }),
        DataTypeDefinition::String { encoding: lenc, .. } => {
            matches!(rtype, DataTypeDefinition::String { encoding, .. } if encoding == lenc)
        }

        // We have to handle 3 different cases here:
        // 1. foo := ADR(bar)
        // 2. foo := REF(bar)
        // 3. foo := &bar
        DataTypeDefinition::Pointer { .. } => match rtype {
            // Case 1: ADR(bar) returns a LWORD value, thus check if we're working with a LWORD
            DataTypeDefinition::Integer { size, .. } => *size == POINTER_SIZE,

            // Case 2 & 3:
            // REF(bar) and &bar returns a pointer, thus deduce their inner types and check if they're equal
            DataTypeDefinition::Pointer { .. } => {
                let ldetails = index.find_elementary_pointer_type(ltype);
                let rdetails = index.find_elementary_pointer_type(rtype);

                ldetails == rdetails
            }

            // If nothing applies we can assume the types to be different
            _ => false,
        },

        _ => ltype == rtype,
    }
}

/// Returns the bigger of the two provided types
pub fn get_bigger_type<'t>(
    left_type: &'t DataType,
    right_type: &'t DataType,
    index: &'t Index,
) -> &'t DataType {
    let lt = left_type.get_definition();
    let rt = right_type.get_definition();

    let ldt = index.get_type(left_type.get_name());
    let rdt = index.get_type(right_type.get_name());

    // if left and right have the same type, check which ranks higher
    if is_same_type_class(lt, rt, index) {
        if get_rank(lt, index) < get_rank(rt, index) {
            return right_type;
        }
    } else if let (Ok(ldt), Ok(rdt)) = (ldt, rdt) {
        // check is_numerical() on TypeNature e.g. DataTypeInformation::Integer is numerical but also used for CHARS which are not considered as numerical
        if (ldt.is_numerical() && rdt.is_numerical()) && (ldt.is_real() || rdt.is_real()) {
            let real_type = index.get_type_or_panic(REAL_TYPE);
            let real_size = real_type.get_definition().get_size_in_bits(index);
            if lt.get_size_in_bits(index) > real_size || rt.get_size_in_bits(index) > real_size {
                return index.get_type_or_panic(LREAL_TYPE);
            } else {
                return real_type;
            }
        }
    }

    left_type
}

/// returns the signed version of the given data_type if its a signed int-type
/// returns the original type if it is no signed int-type
pub fn get_signed_type<'t>(data_type: &'t DataType, index: &'t Index) -> Option<&'t DataType> {
    if data_type.get_definition().is_int() {
        let signed_type = match data_type.get_name() {
            BYTE_TYPE => SINT_TYPE,
            USINT_TYPE => SINT_TYPE,
            WORD_TYPE => INT_TYPE,
            UINT_TYPE => INT_TYPE,
            DWORD_TYPE => DINT_TYPE,
            UDINT_TYPE => DINT_TYPE,
            ULINT_TYPE => LINT_TYPE,
            LWORD_TYPE => LINT_TYPE,
            _ => data_type.get_name(),
        };
        return index.get_type(signed_type).ok();
    }
    Some(data_type)
}

/**
 * returns the compare-function name for the given type and operator.
 * Returns None if the given operator is no comparison operator
 */
pub fn get_equals_function_name_for(type_name: &str, operator: &Operator) -> Option<String> {
    let suffix = match operator {
        Operator::Equal => Some("EQUAL"),
        Operator::Less => Some("LESS"),
        Operator::Greater => Some("GREATER"),
        _ => None,
    };

    suffix.map(|suffix| format!("{}_{}", type_name, suffix))
}

/// returns a name for internally created types using the given prefix and original type name
/// the return name starts with "__"
pub fn create_internal_type_name(prefix: &str, original_type_name: &str) -> String {
    format!("__{}{}", prefix, original_type_name)
}
