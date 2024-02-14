#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Type {
    #[prost(
        oneof = "r#type::Kind",
        tags = "1, 2, 3, 5, 7, 10, 11, 12, 13, 14, 16, 17, 19, 20, 29, 32, 21, 22, 23, 24, 25, 27, 28, 30, 31"
    )]
    pub kind: ::core::option::Option<r#type::Kind>,
}
/// Nested message and enum types in `Type`.
pub mod r#type {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Boolean {
        #[prost(uint32, tag = "1")]
        pub type_variation_reference: u32,
        #[prost(enumeration = "Nullability", tag = "2")]
        pub nullability: i32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct I8 {
        #[prost(uint32, tag = "1")]
        pub type_variation_reference: u32,
        #[prost(enumeration = "Nullability", tag = "2")]
        pub nullability: i32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct I16 {
        #[prost(uint32, tag = "1")]
        pub type_variation_reference: u32,
        #[prost(enumeration = "Nullability", tag = "2")]
        pub nullability: i32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct I32 {
        #[prost(uint32, tag = "1")]
        pub type_variation_reference: u32,
        #[prost(enumeration = "Nullability", tag = "2")]
        pub nullability: i32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct I64 {
        #[prost(uint32, tag = "1")]
        pub type_variation_reference: u32,
        #[prost(enumeration = "Nullability", tag = "2")]
        pub nullability: i32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Fp32 {
        #[prost(uint32, tag = "1")]
        pub type_variation_reference: u32,
        #[prost(enumeration = "Nullability", tag = "2")]
        pub nullability: i32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Fp64 {
        #[prost(uint32, tag = "1")]
        pub type_variation_reference: u32,
        #[prost(enumeration = "Nullability", tag = "2")]
        pub nullability: i32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct String {
        #[prost(uint32, tag = "1")]
        pub type_variation_reference: u32,
        #[prost(enumeration = "Nullability", tag = "2")]
        pub nullability: i32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Binary {
        #[prost(uint32, tag = "1")]
        pub type_variation_reference: u32,
        #[prost(enumeration = "Nullability", tag = "2")]
        pub nullability: i32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Timestamp {
        #[prost(uint32, tag = "1")]
        pub type_variation_reference: u32,
        #[prost(enumeration = "Nullability", tag = "2")]
        pub nullability: i32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Date {
        #[prost(uint32, tag = "1")]
        pub type_variation_reference: u32,
        #[prost(enumeration = "Nullability", tag = "2")]
        pub nullability: i32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Time {
        #[prost(uint32, tag = "1")]
        pub type_variation_reference: u32,
        #[prost(enumeration = "Nullability", tag = "2")]
        pub nullability: i32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TimestampTz {
        #[prost(uint32, tag = "1")]
        pub type_variation_reference: u32,
        #[prost(enumeration = "Nullability", tag = "2")]
        pub nullability: i32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IntervalYear {
        #[prost(uint32, tag = "1")]
        pub type_variation_reference: u32,
        #[prost(enumeration = "Nullability", tag = "2")]
        pub nullability: i32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IntervalDay {
        #[prost(uint32, tag = "1")]
        pub type_variation_reference: u32,
        #[prost(enumeration = "Nullability", tag = "2")]
        pub nullability: i32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Uuid {
        #[prost(uint32, tag = "1")]
        pub type_variation_reference: u32,
        #[prost(enumeration = "Nullability", tag = "2")]
        pub nullability: i32,
    }
    /// Start compound types.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FixedChar {
        #[prost(int32, tag = "1")]
        pub length: i32,
        #[prost(uint32, tag = "2")]
        pub type_variation_reference: u32,
        #[prost(enumeration = "Nullability", tag = "3")]
        pub nullability: i32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct VarChar {
        #[prost(int32, tag = "1")]
        pub length: i32,
        #[prost(uint32, tag = "2")]
        pub type_variation_reference: u32,
        #[prost(enumeration = "Nullability", tag = "3")]
        pub nullability: i32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FixedBinary {
        #[prost(int32, tag = "1")]
        pub length: i32,
        #[prost(uint32, tag = "2")]
        pub type_variation_reference: u32,
        #[prost(enumeration = "Nullability", tag = "3")]
        pub nullability: i32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Decimal {
        #[prost(int32, tag = "1")]
        pub scale: i32,
        #[prost(int32, tag = "2")]
        pub precision: i32,
        #[prost(uint32, tag = "3")]
        pub type_variation_reference: u32,
        #[prost(enumeration = "Nullability", tag = "4")]
        pub nullability: i32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Struct {
        #[prost(message, repeated, tag = "1")]
        pub types: ::prost::alloc::vec::Vec<super::Type>,
        #[prost(uint32, tag = "2")]
        pub type_variation_reference: u32,
        #[prost(enumeration = "Nullability", tag = "3")]
        pub nullability: i32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct List {
        #[prost(message, optional, boxed, tag = "1")]
        pub r#type: ::core::option::Option<::prost::alloc::boxed::Box<super::Type>>,
        #[prost(uint32, tag = "2")]
        pub type_variation_reference: u32,
        #[prost(enumeration = "Nullability", tag = "3")]
        pub nullability: i32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Map {
        #[prost(message, optional, boxed, tag = "1")]
        pub key: ::core::option::Option<::prost::alloc::boxed::Box<super::Type>>,
        #[prost(message, optional, boxed, tag = "2")]
        pub value: ::core::option::Option<::prost::alloc::boxed::Box<super::Type>>,
        #[prost(uint32, tag = "3")]
        pub type_variation_reference: u32,
        #[prost(enumeration = "Nullability", tag = "4")]
        pub nullability: i32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UserDefined {
        #[prost(uint32, tag = "1")]
        pub type_reference: u32,
        #[prost(uint32, tag = "2")]
        pub type_variation_reference: u32,
        #[prost(enumeration = "Nullability", tag = "3")]
        pub nullability: i32,
        #[prost(message, repeated, tag = "4")]
        pub type_parameters: ::prost::alloc::vec::Vec<Parameter>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Parameter {
        #[prost(oneof = "parameter::Parameter", tags = "1, 2, 3, 4, 5, 6")]
        pub parameter: ::core::option::Option<parameter::Parameter>,
    }
    /// Nested message and enum types in `Parameter`.
    pub mod parameter {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Parameter {
            /// Explicitly null/unspecified parameter, to select the default value (if
            /// any).
            #[prost(message, tag = "1")]
            Null(()),
            /// Data type parameters, like the i32 in LIST<i32>.
            #[prost(message, tag = "2")]
            DataType(super::super::Type),
            /// Value parameters, like the 10 in VARCHAR<10>.
            #[prost(bool, tag = "3")]
            Boolean(bool),
            #[prost(int64, tag = "4")]
            Integer(i64),
            #[prost(string, tag = "5")]
            Enum(::prost::alloc::string::String),
            #[prost(string, tag = "6")]
            String(::prost::alloc::string::String),
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Nullability {
        Unspecified = 0,
        Nullable = 1,
        Required = 2,
    }
    impl Nullability {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Nullability::Unspecified => "NULLABILITY_UNSPECIFIED",
                Nullability::Nullable => "NULLABILITY_NULLABLE",
                Nullability::Required => "NULLABILITY_REQUIRED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NULLABILITY_UNSPECIFIED" => Some(Self::Unspecified),
                "NULLABILITY_NULLABLE" => Some(Self::Nullable),
                "NULLABILITY_REQUIRED" => Some(Self::Required),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag = "1")]
        Bool(Boolean),
        #[prost(message, tag = "2")]
        I8(I8),
        #[prost(message, tag = "3")]
        I16(I16),
        #[prost(message, tag = "5")]
        I32(I32),
        #[prost(message, tag = "7")]
        I64(I64),
        #[prost(message, tag = "10")]
        Fp32(Fp32),
        #[prost(message, tag = "11")]
        Fp64(Fp64),
        #[prost(message, tag = "12")]
        String(String),
        #[prost(message, tag = "13")]
        Binary(Binary),
        #[prost(message, tag = "14")]
        Timestamp(Timestamp),
        #[prost(message, tag = "16")]
        Date(Date),
        #[prost(message, tag = "17")]
        Time(Time),
        #[prost(message, tag = "19")]
        IntervalYear(IntervalYear),
        #[prost(message, tag = "20")]
        IntervalDay(IntervalDay),
        #[prost(message, tag = "29")]
        TimestampTz(TimestampTz),
        #[prost(message, tag = "32")]
        Uuid(Uuid),
        #[prost(message, tag = "21")]
        FixedChar(FixedChar),
        #[prost(message, tag = "22")]
        Varchar(VarChar),
        #[prost(message, tag = "23")]
        FixedBinary(FixedBinary),
        #[prost(message, tag = "24")]
        Decimal(Decimal),
        #[prost(message, tag = "25")]
        Struct(Struct),
        #[prost(message, tag = "27")]
        List(::prost::alloc::boxed::Box<List>),
        #[prost(message, tag = "28")]
        Map(::prost::alloc::boxed::Box<Map>),
        #[prost(message, tag = "30")]
        UserDefined(UserDefined),
        /// Deprecated in favor of user_defined, which allows nullability and
        /// variations to be specified. If user_defined_type_reference is
        /// encountered, treat it as being non-nullable and having the default
        /// variation.
        #[prost(uint32, tag = "31")]
        UserDefinedTypeReference(u32),
    }
}
/// A message for modeling name/type pairs.
///
/// Useful for representing relation schemas.
///
/// Notes:
///
/// * The names field is in depth-first order.
///
/// For example a schema such as:
///
/// a: int64
/// b: struct<c: float32, d: string>
///
/// would have a `names` field that looks like:
///
/// \["a", "b", "c", "d"\]
///
/// * Only struct fields are contained in this field's elements,
/// * Map keys should be traversed first, then values when producing/consuming
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamedStruct {
    /// list of names in dfs order
    #[prost(string, repeated, tag = "1")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub r#struct: ::core::option::Option<r#type::Struct>,
}
/// Common fields for all relational operators
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelCommon {
    #[prost(message, optional, tag = "3")]
    pub hint: ::core::option::Option<rel_common::Hint>,
    #[prost(message, optional, tag = "4")]
    pub advanced_extension: ::core::option::Option<extensions::AdvancedExtension>,
    #[prost(oneof = "rel_common::EmitKind", tags = "1, 2")]
    pub emit_kind: ::core::option::Option<rel_common::EmitKind>,
}
/// Nested message and enum types in `RelCommon`.
pub mod rel_common {
    /// Direct indicates no change on presence and ordering of fields in the output
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Direct {}
    /// Remap which fields are output and in which order
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Emit {
        #[prost(int32, repeated, tag = "1")]
        pub output_mapping: ::prost::alloc::vec::Vec<i32>,
    }
    /// Changes to the operation that can influence efficiency/performance but
    /// should not impact correctness.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Hint {
        #[prost(message, optional, tag = "1")]
        pub stats: ::core::option::Option<hint::Stats>,
        #[prost(message, optional, tag = "2")]
        pub constraint: ::core::option::Option<hint::RuntimeConstraint>,
        #[prost(message, optional, tag = "10")]
        pub advanced_extension: ::core::option::Option<
            super::extensions::AdvancedExtension,
        >,
    }
    /// Nested message and enum types in `Hint`.
    pub mod hint {
        /// The statistics related to a hint (physical properties of records)
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Stats {
            #[prost(double, tag = "1")]
            pub row_count: f64,
            #[prost(double, tag = "2")]
            pub record_size: f64,
            #[prost(message, optional, tag = "10")]
            pub advanced_extension: ::core::option::Option<
                super::super::extensions::AdvancedExtension,
            >,
        }
        /// TODO: nodes, cpu threads/%, memory, iops, etc.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct RuntimeConstraint {
            #[prost(message, optional, tag = "10")]
            pub advanced_extension: ::core::option::Option<
                super::super::extensions::AdvancedExtension,
            >,
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EmitKind {
        /// The underlying relation is output as is (no reordering or projection of columns)
        #[prost(message, tag = "1")]
        Direct(Direct),
        /// Allows to control for order and inclusion of fields
        #[prost(message, tag = "2")]
        Emit(Emit),
    }
}
/// The scan operator of base data (physical or virtual), including filtering and projection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadRel {
    #[prost(message, optional, tag = "1")]
    pub common: ::core::option::Option<RelCommon>,
    #[prost(message, optional, tag = "2")]
    pub base_schema: ::core::option::Option<NamedStruct>,
    #[prost(message, optional, boxed, tag = "3")]
    pub filter: ::core::option::Option<::prost::alloc::boxed::Box<Expression>>,
    #[prost(message, optional, boxed, tag = "11")]
    pub best_effort_filter: ::core::option::Option<
        ::prost::alloc::boxed::Box<Expression>,
    >,
    #[prost(message, optional, tag = "4")]
    pub projection: ::core::option::Option<expression::MaskExpression>,
    #[prost(message, optional, tag = "10")]
    pub advanced_extension: ::core::option::Option<extensions::AdvancedExtension>,
    /// Definition of which type of scan operation is to be performed
    #[prost(oneof = "read_rel::ReadType", tags = "5, 6, 7, 8")]
    pub read_type: ::core::option::Option<read_rel::ReadType>,
}
/// Nested message and enum types in `ReadRel`.
pub mod read_rel {
    /// A base table. The list of string is used to represent namespacing (e.g., mydb.mytable).
    /// This assumes shared catalog between systems exchanging a message.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NamedTable {
        #[prost(string, repeated, tag = "1")]
        pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(message, optional, tag = "10")]
        pub advanced_extension: ::core::option::Option<
            super::extensions::AdvancedExtension,
        >,
    }
    /// A table composed of literals.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct VirtualTable {
        #[prost(message, repeated, tag = "1")]
        pub values: ::prost::alloc::vec::Vec<super::expression::literal::Struct>,
    }
    /// A stub type that can be used to extend/introduce new table types outside
    /// the specification.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExtensionTable {
        #[prost(message, optional, tag = "1")]
        pub detail: ::core::option::Option<::prost_types::Any>,
    }
    /// Represents a list of files in input of a scan operation
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LocalFiles {
        #[prost(message, repeated, tag = "1")]
        pub items: ::prost::alloc::vec::Vec<local_files::FileOrFiles>,
        #[prost(message, optional, tag = "10")]
        pub advanced_extension: ::core::option::Option<
            super::extensions::AdvancedExtension,
        >,
    }
    /// Nested message and enum types in `LocalFiles`.
    pub mod local_files {
        /// Many files consist of indivisible chunks (e.g. parquet row groups
        /// or CSV rows).  If a slice partially selects an indivisible chunk
        /// then the consumer should employ some rule to decide which slice to
        /// include the chunk in (e.g. include it in the slice that contains
        /// the midpoint of the chunk)
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FileOrFiles {
            /// The index of the partition this item belongs to
            #[prost(uint64, tag = "6")]
            pub partition_index: u64,
            /// The start position in byte to read from this item
            #[prost(uint64, tag = "7")]
            pub start: u64,
            /// The length in byte to read from this item
            #[prost(uint64, tag = "8")]
            pub length: u64,
            #[prost(oneof = "file_or_files::PathType", tags = "1, 2, 3, 4")]
            pub path_type: ::core::option::Option<file_or_files::PathType>,
            /// The format of the files.
            #[prost(oneof = "file_or_files::FileFormat", tags = "9, 10, 11, 12, 13")]
            pub file_format: ::core::option::Option<file_or_files::FileFormat>,
        }
        /// Nested message and enum types in `FileOrFiles`.
        pub mod file_or_files {
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct ParquetReadOptions {}
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct ArrowReadOptions {}
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct OrcReadOptions {}
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct DwrfReadOptions {}
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum PathType {
                /// A URI that can refer to either a single folder or a single file
                #[prost(string, tag = "1")]
                UriPath(::prost::alloc::string::String),
                /// A URI where the path portion is a glob expression that can
                /// identify zero or more paths.
                /// Consumers should support the POSIX syntax.  The recursive
                /// globstar (**) may not be supported.
                #[prost(string, tag = "2")]
                UriPathGlob(::prost::alloc::string::String),
                /// A URI that refers to a single file
                #[prost(string, tag = "3")]
                UriFile(::prost::alloc::string::String),
                /// A URI that refers to a single folder
                #[prost(string, tag = "4")]
                UriFolder(::prost::alloc::string::String),
            }
            /// The format of the files.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum FileFormat {
                #[prost(message, tag = "9")]
                Parquet(ParquetReadOptions),
                #[prost(message, tag = "10")]
                Arrow(ArrowReadOptions),
                #[prost(message, tag = "11")]
                Orc(OrcReadOptions),
                #[prost(message, tag = "12")]
                Extension(::prost_types::Any),
                #[prost(message, tag = "13")]
                Dwrf(DwrfReadOptions),
            }
        }
    }
    /// Definition of which type of scan operation is to be performed
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ReadType {
        #[prost(message, tag = "5")]
        VirtualTable(VirtualTable),
        #[prost(message, tag = "6")]
        LocalFiles(LocalFiles),
        #[prost(message, tag = "7")]
        NamedTable(NamedTable),
        #[prost(message, tag = "8")]
        ExtensionTable(ExtensionTable),
    }
}
/// This operator allows to represent calculated expressions of fields (e.g., a+b). Direct/Emit are used to represent classical relational projections
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectRel {
    #[prost(message, optional, tag = "1")]
    pub common: ::core::option::Option<RelCommon>,
    #[prost(message, optional, boxed, tag = "2")]
    pub input: ::core::option::Option<::prost::alloc::boxed::Box<Rel>>,
    #[prost(message, repeated, tag = "3")]
    pub expressions: ::prost::alloc::vec::Vec<Expression>,
    #[prost(message, optional, tag = "10")]
    pub advanced_extension: ::core::option::Option<extensions::AdvancedExtension>,
}
/// The binary JOIN relational operator left-join-right, including various join types, a join condition and post_join_filter expression
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinRel {
    #[prost(message, optional, tag = "1")]
    pub common: ::core::option::Option<RelCommon>,
    #[prost(message, optional, boxed, tag = "2")]
    pub left: ::core::option::Option<::prost::alloc::boxed::Box<Rel>>,
    #[prost(message, optional, boxed, tag = "3")]
    pub right: ::core::option::Option<::prost::alloc::boxed::Box<Rel>>,
    #[prost(message, optional, boxed, tag = "4")]
    pub expression: ::core::option::Option<::prost::alloc::boxed::Box<Expression>>,
    #[prost(message, optional, boxed, tag = "5")]
    pub post_join_filter: ::core::option::Option<::prost::alloc::boxed::Box<Expression>>,
    #[prost(enumeration = "join_rel::JoinType", tag = "6")]
    pub r#type: i32,
    #[prost(message, optional, tag = "10")]
    pub advanced_extension: ::core::option::Option<extensions::AdvancedExtension>,
}
/// Nested message and enum types in `JoinRel`.
pub mod join_rel {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum JoinType {
        Unspecified = 0,
        Inner = 1,
        Outer = 2,
        Left = 3,
        Right = 4,
        Semi = 5,
        Anti = 6,
        /// This join is useful for nested sub-queries where we need exactly one record in output (or throw exception)
        /// See Section 3.2 of <https://15721.courses.cs.cmu.edu/spring2018/papers/16-optimizer2/hyperjoins-btw2017.pdf>
        Single = 7,
    }
    impl JoinType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                JoinType::Unspecified => "JOIN_TYPE_UNSPECIFIED",
                JoinType::Inner => "JOIN_TYPE_INNER",
                JoinType::Outer => "JOIN_TYPE_OUTER",
                JoinType::Left => "JOIN_TYPE_LEFT",
                JoinType::Right => "JOIN_TYPE_RIGHT",
                JoinType::Semi => "JOIN_TYPE_SEMI",
                JoinType::Anti => "JOIN_TYPE_ANTI",
                JoinType::Single => "JOIN_TYPE_SINGLE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "JOIN_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "JOIN_TYPE_INNER" => Some(Self::Inner),
                "JOIN_TYPE_OUTER" => Some(Self::Outer),
                "JOIN_TYPE_LEFT" => Some(Self::Left),
                "JOIN_TYPE_RIGHT" => Some(Self::Right),
                "JOIN_TYPE_SEMI" => Some(Self::Semi),
                "JOIN_TYPE_ANTI" => Some(Self::Anti),
                "JOIN_TYPE_SINGLE" => Some(Self::Single),
                _ => None,
            }
        }
    }
}
/// Cartesian product relational operator of two tables (left and right)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrossRel {
    #[prost(message, optional, tag = "1")]
    pub common: ::core::option::Option<RelCommon>,
    #[prost(message, optional, boxed, tag = "2")]
    pub left: ::core::option::Option<::prost::alloc::boxed::Box<Rel>>,
    #[prost(message, optional, boxed, tag = "3")]
    pub right: ::core::option::Option<::prost::alloc::boxed::Box<Rel>>,
    #[prost(message, optional, tag = "10")]
    pub advanced_extension: ::core::option::Option<extensions::AdvancedExtension>,
}
/// The relational operator representing LIMIT/OFFSET or TOP type semantics.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchRel {
    #[prost(message, optional, tag = "1")]
    pub common: ::core::option::Option<RelCommon>,
    #[prost(message, optional, boxed, tag = "2")]
    pub input: ::core::option::Option<::prost::alloc::boxed::Box<Rel>>,
    /// the offset expressed in number of records
    #[prost(int64, tag = "3")]
    pub offset: i64,
    /// the amount of records to return
    #[prost(int64, tag = "4")]
    pub count: i64,
    #[prost(message, optional, tag = "10")]
    pub advanced_extension: ::core::option::Option<extensions::AdvancedExtension>,
}
/// The relational operator representing a GROUP BY Aggregate
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregateRel {
    #[prost(message, optional, tag = "1")]
    pub common: ::core::option::Option<RelCommon>,
    /// Input of the aggregation
    #[prost(message, optional, boxed, tag = "2")]
    pub input: ::core::option::Option<::prost::alloc::boxed::Box<Rel>>,
    /// A list of one or more grouping expression sets that the aggregation measures should be calculated for.
    /// Required if there are no measures.
    #[prost(message, repeated, tag = "3")]
    pub groupings: ::prost::alloc::vec::Vec<aggregate_rel::Grouping>,
    /// A list of one or more aggregate expressions along with an optional filter.
    /// Required if there are no groupings.
    #[prost(message, repeated, tag = "4")]
    pub measures: ::prost::alloc::vec::Vec<aggregate_rel::Measure>,
    #[prost(message, optional, tag = "10")]
    pub advanced_extension: ::core::option::Option<extensions::AdvancedExtension>,
}
/// Nested message and enum types in `AggregateRel`.
pub mod aggregate_rel {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Grouping {
        #[prost(message, repeated, tag = "1")]
        pub grouping_expressions: ::prost::alloc::vec::Vec<super::Expression>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Measure {
        #[prost(message, optional, tag = "1")]
        pub measure: ::core::option::Option<super::AggregateFunction>,
        /// An optional boolean expression that acts to filter which records are
        /// included in the measure. True means include this record for calculation
        /// within the measure.
        /// Helps to support SUM(<c>) FILTER(WHERE...) syntax without masking opportunities for optimization
        #[prost(message, optional, tag = "2")]
        pub filter: ::core::option::Option<super::Expression>,
    }
}
/// ConsistentPartitionWindowRel provides the ability to perform calculations across sets of rows
/// that are related to the current query row. It can be used to execute window functions where
/// all the windows share the same partitioning and ordering.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsistentPartitionWindowRel {
    #[prost(message, optional, tag = "1")]
    pub common: ::core::option::Option<RelCommon>,
    #[prost(message, optional, boxed, tag = "2")]
    pub input: ::core::option::Option<::prost::alloc::boxed::Box<Rel>>,
    #[prost(message, repeated, tag = "3")]
    pub window_functions: ::prost::alloc::vec::Vec<
        consistent_partition_window_rel::WindowRelFunction,
    >,
    #[prost(message, repeated, tag = "4")]
    pub partition_expressions: ::prost::alloc::vec::Vec<Expression>,
    #[prost(message, repeated, tag = "5")]
    pub sorts: ::prost::alloc::vec::Vec<SortField>,
    #[prost(message, optional, tag = "10")]
    pub advanced_extension: ::core::option::Option<extensions::AdvancedExtension>,
}
/// Nested message and enum types in `ConsistentPartitionWindowRel`.
pub mod consistent_partition_window_rel {
    /// This message mirrors the `WindowFunction` message but removes the fields defining the partition,
    /// sorts, and bounds, since those must be consistent across the various functions in this rel.  Refer
    /// to the `WindowFunction` message for a description of these fields.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WindowRelFunction {
        #[prost(uint32, tag = "1")]
        pub function_reference: u32,
        #[prost(message, repeated, tag = "9")]
        pub arguments: ::prost::alloc::vec::Vec<super::FunctionArgument>,
        #[prost(message, repeated, tag = "11")]
        pub options: ::prost::alloc::vec::Vec<super::FunctionOption>,
        #[prost(message, optional, tag = "7")]
        pub output_type: ::core::option::Option<super::Type>,
        #[prost(enumeration = "super::AggregationPhase", tag = "6")]
        pub phase: i32,
        #[prost(
            enumeration = "super::aggregate_function::AggregationInvocation",
            tag = "10"
        )]
        pub invocation: i32,
        #[prost(message, optional, tag = "5")]
        pub lower_bound: ::core::option::Option<
            super::expression::window_function::Bound,
        >,
        #[prost(message, optional, tag = "4")]
        pub upper_bound: ::core::option::Option<
            super::expression::window_function::Bound,
        >,
        #[prost(
            enumeration = "super::expression::window_function::BoundsType",
            tag = "12"
        )]
        pub bounds_type: i32,
    }
}
/// The ORDERY BY (or sorting) relational operator. Beside describing a base relation, it includes a list of fields to sort on
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SortRel {
    #[prost(message, optional, tag = "1")]
    pub common: ::core::option::Option<RelCommon>,
    #[prost(message, optional, boxed, tag = "2")]
    pub input: ::core::option::Option<::prost::alloc::boxed::Box<Rel>>,
    #[prost(message, repeated, tag = "3")]
    pub sorts: ::prost::alloc::vec::Vec<SortField>,
    #[prost(message, optional, tag = "10")]
    pub advanced_extension: ::core::option::Option<extensions::AdvancedExtension>,
}
/// The relational operator capturing simple FILTERs (as in the WHERE clause of SQL)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterRel {
    #[prost(message, optional, tag = "1")]
    pub common: ::core::option::Option<RelCommon>,
    #[prost(message, optional, boxed, tag = "2")]
    pub input: ::core::option::Option<::prost::alloc::boxed::Box<Rel>>,
    #[prost(message, optional, boxed, tag = "3")]
    pub condition: ::core::option::Option<::prost::alloc::boxed::Box<Expression>>,
    #[prost(message, optional, tag = "10")]
    pub advanced_extension: ::core::option::Option<extensions::AdvancedExtension>,
}
/// The relational set operators (intersection/union/etc..)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRel {
    #[prost(message, optional, tag = "1")]
    pub common: ::core::option::Option<RelCommon>,
    /// The first input is the primary input, the remaining are secondary
    /// inputs.  There must be at least two inputs.
    #[prost(message, repeated, tag = "2")]
    pub inputs: ::prost::alloc::vec::Vec<Rel>,
    #[prost(enumeration = "set_rel::SetOp", tag = "3")]
    pub op: i32,
    #[prost(message, optional, tag = "10")]
    pub advanced_extension: ::core::option::Option<extensions::AdvancedExtension>,
}
/// Nested message and enum types in `SetRel`.
pub mod set_rel {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum SetOp {
        Unspecified = 0,
        MinusPrimary = 1,
        MinusMultiset = 2,
        IntersectionPrimary = 3,
        IntersectionMultiset = 4,
        UnionDistinct = 5,
        UnionAll = 6,
    }
    impl SetOp {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SetOp::Unspecified => "SET_OP_UNSPECIFIED",
                SetOp::MinusPrimary => "SET_OP_MINUS_PRIMARY",
                SetOp::MinusMultiset => "SET_OP_MINUS_MULTISET",
                SetOp::IntersectionPrimary => "SET_OP_INTERSECTION_PRIMARY",
                SetOp::IntersectionMultiset => "SET_OP_INTERSECTION_MULTISET",
                SetOp::UnionDistinct => "SET_OP_UNION_DISTINCT",
                SetOp::UnionAll => "SET_OP_UNION_ALL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SET_OP_UNSPECIFIED" => Some(Self::Unspecified),
                "SET_OP_MINUS_PRIMARY" => Some(Self::MinusPrimary),
                "SET_OP_MINUS_MULTISET" => Some(Self::MinusMultiset),
                "SET_OP_INTERSECTION_PRIMARY" => Some(Self::IntersectionPrimary),
                "SET_OP_INTERSECTION_MULTISET" => Some(Self::IntersectionMultiset),
                "SET_OP_UNION_DISTINCT" => Some(Self::UnionDistinct),
                "SET_OP_UNION_ALL" => Some(Self::UnionAll),
                _ => None,
            }
        }
    }
}
/// Stub to support extension with a single input
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionSingleRel {
    #[prost(message, optional, tag = "1")]
    pub common: ::core::option::Option<RelCommon>,
    #[prost(message, optional, boxed, tag = "2")]
    pub input: ::core::option::Option<::prost::alloc::boxed::Box<Rel>>,
    #[prost(message, optional, tag = "3")]
    pub detail: ::core::option::Option<::prost_types::Any>,
}
/// Stub to support extension with a zero inputs
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionLeafRel {
    #[prost(message, optional, tag = "1")]
    pub common: ::core::option::Option<RelCommon>,
    #[prost(message, optional, tag = "2")]
    pub detail: ::core::option::Option<::prost_types::Any>,
}
/// Stub to support extension with multiple inputs
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionMultiRel {
    #[prost(message, optional, tag = "1")]
    pub common: ::core::option::Option<RelCommon>,
    #[prost(message, repeated, tag = "2")]
    pub inputs: ::prost::alloc::vec::Vec<Rel>,
    #[prost(message, optional, tag = "3")]
    pub detail: ::core::option::Option<::prost_types::Any>,
}
/// A redistribution operation
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExchangeRel {
    #[prost(message, optional, tag = "1")]
    pub common: ::core::option::Option<RelCommon>,
    #[prost(message, optional, boxed, tag = "2")]
    pub input: ::core::option::Option<::prost::alloc::boxed::Box<Rel>>,
    #[prost(int32, tag = "3")]
    pub partition_count: i32,
    #[prost(message, repeated, tag = "4")]
    pub targets: ::prost::alloc::vec::Vec<exchange_rel::ExchangeTarget>,
    #[prost(message, optional, tag = "10")]
    pub advanced_extension: ::core::option::Option<extensions::AdvancedExtension>,
    /// the type of exchange used
    #[prost(oneof = "exchange_rel::ExchangeKind", tags = "5, 6, 7, 8, 9")]
    pub exchange_kind: ::core::option::Option<exchange_rel::ExchangeKind>,
}
/// Nested message and enum types in `ExchangeRel`.
pub mod exchange_rel {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ScatterFields {
        #[prost(message, repeated, tag = "1")]
        pub fields: ::prost::alloc::vec::Vec<super::expression::FieldReference>,
    }
    /// Returns a single bucket number per record.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SingleBucketExpression {
        #[prost(message, optional, boxed, tag = "1")]
        pub expression: ::core::option::Option<
            ::prost::alloc::boxed::Box<super::Expression>,
        >,
    }
    /// Returns zero or more bucket numbers per record
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MultiBucketExpression {
        #[prost(message, optional, boxed, tag = "1")]
        pub expression: ::core::option::Option<
            ::prost::alloc::boxed::Box<super::Expression>,
        >,
        #[prost(bool, tag = "2")]
        pub constrained_to_count: bool,
    }
    /// Send all data to every target.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Broadcast {}
    /// Route approximately
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RoundRobin {
        /// whether the round robin behavior is required to exact (per record) or
        /// approximate. Defaults to approximate.
        #[prost(bool, tag = "1")]
        pub exact: bool,
    }
    /// The message to describe partition targets of an exchange
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExchangeTarget {
        /// Describes the partition id(s) to send. If this is empty, all data is sent
        /// to this target.
        #[prost(int32, repeated, tag = "1")]
        pub partition_id: ::prost::alloc::vec::Vec<i32>,
        #[prost(oneof = "exchange_target::TargetType", tags = "2, 3")]
        pub target_type: ::core::option::Option<exchange_target::TargetType>,
    }
    /// Nested message and enum types in `ExchangeTarget`.
    pub mod exchange_target {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum TargetType {
            #[prost(string, tag = "2")]
            Uri(::prost::alloc::string::String),
            #[prost(message, tag = "3")]
            Extended(::prost_types::Any),
        }
    }
    /// the type of exchange used
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ExchangeKind {
        #[prost(message, tag = "5")]
        ScatterByFields(ScatterFields),
        #[prost(message, tag = "6")]
        SingleTarget(::prost::alloc::boxed::Box<SingleBucketExpression>),
        #[prost(message, tag = "7")]
        MultiTarget(::prost::alloc::boxed::Box<MultiBucketExpression>),
        #[prost(message, tag = "8")]
        RoundRobin(RoundRobin),
        #[prost(message, tag = "9")]
        Broadcast(Broadcast),
    }
}
/// Duplicates records by emitting one or more rows per input row.  The number of rows emitted per
/// input row is the same for all input rows.
///
/// In addition to a field being emitted per input field an extra int64 field is emitted which
/// contains a zero-indexed ordinal corresponding to the duplicate definition.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpandRel {
    #[prost(message, optional, tag = "1")]
    pub common: ::core::option::Option<RelCommon>,
    #[prost(message, optional, boxed, tag = "2")]
    pub input: ::core::option::Option<::prost::alloc::boxed::Box<Rel>>,
    /// There should be one definition here for each input field.  Any fields beyond the provided
    /// definitions will be emitted as is (as if a consistent_field record with an identity
    /// expression was provided).
    #[prost(message, repeated, tag = "4")]
    pub fields: ::prost::alloc::vec::Vec<expand_rel::ExpandField>,
}
/// Nested message and enum types in `ExpandRel`.
pub mod expand_rel {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExpandField {
        #[prost(oneof = "expand_field::FieldType", tags = "2, 3")]
        pub field_type: ::core::option::Option<expand_field::FieldType>,
    }
    /// Nested message and enum types in `ExpandField`.
    pub mod expand_field {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum FieldType {
            /// Field that switches output based on which duplicate is being output.  Every
            /// switching_field should contain the same number of duplicates (so that the output rows
            /// are of consistent size and type).  If there are not enough switching field definitions
            /// to match the other field definitions NULL will be returned to fill the extras.
            #[prost(message, tag = "2")]
            SwitchingField(super::SwitchingField),
            /// Field that outputs the same value no matter which duplicate is being output.  Equivalent
            /// to a switching_field that lists the same expression multiple times.
            #[prost(message, tag = "3")]
            ConsistentField(super::super::Expression),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SwitchingField {
        /// All duplicates must return the same type class but may differ in nullability.  The effective
        /// type of the output field will be nullable if any of the duplicate expressions are nullable.
        #[prost(message, repeated, tag = "1")]
        pub duplicates: ::prost::alloc::vec::Vec<super::Expression>,
    }
}
/// A relation with output field names.
///
/// This is for use at the root of a `Rel` tree.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelRoot {
    /// A relation
    #[prost(message, optional, tag = "1")]
    pub input: ::core::option::Option<Rel>,
    /// Field names in depth-first order
    #[prost(string, repeated, tag = "2")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A relation (used internally in a plan)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rel {
    #[prost(
        oneof = "rel::RelType",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 21, 19, 20, 13, 14, 18, 17, 15, 16"
    )]
    pub rel_type: ::core::option::Option<rel::RelType>,
}
/// Nested message and enum types in `Rel`.
pub mod rel {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RelType {
        #[prost(message, tag = "1")]
        Read(::prost::alloc::boxed::Box<super::ReadRel>),
        #[prost(message, tag = "2")]
        Filter(::prost::alloc::boxed::Box<super::FilterRel>),
        #[prost(message, tag = "3")]
        Fetch(::prost::alloc::boxed::Box<super::FetchRel>),
        #[prost(message, tag = "4")]
        Aggregate(::prost::alloc::boxed::Box<super::AggregateRel>),
        #[prost(message, tag = "5")]
        Sort(::prost::alloc::boxed::Box<super::SortRel>),
        #[prost(message, tag = "6")]
        Join(::prost::alloc::boxed::Box<super::JoinRel>),
        #[prost(message, tag = "7")]
        Project(::prost::alloc::boxed::Box<super::ProjectRel>),
        #[prost(message, tag = "8")]
        Set(super::SetRel),
        #[prost(message, tag = "9")]
        ExtensionSingle(::prost::alloc::boxed::Box<super::ExtensionSingleRel>),
        #[prost(message, tag = "10")]
        ExtensionMulti(super::ExtensionMultiRel),
        #[prost(message, tag = "11")]
        ExtensionLeaf(super::ExtensionLeafRel),
        #[prost(message, tag = "12")]
        Cross(::prost::alloc::boxed::Box<super::CrossRel>),
        #[prost(message, tag = "21")]
        Reference(super::ReferenceRel),
        #[prost(message, tag = "19")]
        Write(::prost::alloc::boxed::Box<super::WriteRel>),
        #[prost(message, tag = "20")]
        Ddl(::prost::alloc::boxed::Box<super::DdlRel>),
        /// Physical relations
        #[prost(message, tag = "13")]
        HashJoin(::prost::alloc::boxed::Box<super::HashJoinRel>),
        #[prost(message, tag = "14")]
        MergeJoin(::prost::alloc::boxed::Box<super::MergeJoinRel>),
        #[prost(message, tag = "18")]
        NestedLoopJoin(::prost::alloc::boxed::Box<super::NestedLoopJoinRel>),
        #[prost(message, tag = "17")]
        Window(::prost::alloc::boxed::Box<super::ConsistentPartitionWindowRel>),
        #[prost(message, tag = "15")]
        Exchange(::prost::alloc::boxed::Box<super::ExchangeRel>),
        #[prost(message, tag = "16")]
        Expand(::prost::alloc::boxed::Box<super::ExpandRel>),
    }
}
/// A base object for writing (e.g., a table or a view).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamedObjectWrite {
    /// The list of string is used to represent namespacing (e.g., mydb.mytable).
    /// This assumes shared catalog between systems exchanging a message.
    #[prost(string, repeated, tag = "1")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "10")]
    pub advanced_extension: ::core::option::Option<extensions::AdvancedExtension>,
}
/// A stub type that can be used to extend/introduce new table types outside
/// the specification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionObject {
    #[prost(message, optional, tag = "1")]
    pub detail: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DdlRel {
    /// The columns that will be modified (representing after-image of a schema change)
    #[prost(message, optional, tag = "3")]
    pub table_schema: ::core::option::Option<NamedStruct>,
    /// The default values for the columns (representing after-image of a schema change)
    /// E.g., in case of an ALTER TABLE that changes some of the column default values, we expect
    /// the table_defaults Struct to report a full list of default values reflecting the result of applying
    /// the ALTER TABLE operator successfully
    #[prost(message, optional, tag = "4")]
    pub table_defaults: ::core::option::Option<expression::literal::Struct>,
    /// Which type of object we operate on
    #[prost(enumeration = "ddl_rel::DdlObject", tag = "5")]
    pub object: i32,
    /// The type of operation to perform
    #[prost(enumeration = "ddl_rel::DdlOp", tag = "6")]
    pub op: i32,
    /// The body of the CREATE VIEW
    #[prost(message, optional, boxed, tag = "7")]
    pub view_definition: ::core::option::Option<::prost::alloc::boxed::Box<Rel>>,
    #[prost(message, optional, tag = "8")]
    pub common: ::core::option::Option<RelCommon>,
    /// Definition of which type of object we are operating on
    #[prost(oneof = "ddl_rel::WriteType", tags = "1, 2")]
    pub write_type: ::core::option::Option<ddl_rel::WriteType>,
}
/// Nested message and enum types in `DdlRel`.
pub mod ddl_rel {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum DdlObject {
        Unspecified = 0,
        /// A Table object in the system
        Table = 1,
        /// A View object in the system
        View = 2,
    }
    impl DdlObject {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DdlObject::Unspecified => "DDL_OBJECT_UNSPECIFIED",
                DdlObject::Table => "DDL_OBJECT_TABLE",
                DdlObject::View => "DDL_OBJECT_VIEW",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DDL_OBJECT_UNSPECIFIED" => Some(Self::Unspecified),
                "DDL_OBJECT_TABLE" => Some(Self::Table),
                "DDL_OBJECT_VIEW" => Some(Self::View),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum DdlOp {
        Unspecified = 0,
        /// A create operation (for any object)
        Create = 1,
        /// A create operation if the object does not exist, or replaces it (equivalent to a DROP + CREATE) if the object already exists
        CreateOrReplace = 2,
        /// An operation that modifies the schema (e.g., column names, types, default values) for the target object
        Alter = 3,
        /// An operation that removes an object from the system
        Drop = 4,
        /// An operation that removes an object from the system (without throwing an exception if the object did not exist)
        DropIfExist = 5,
    }
    impl DdlOp {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DdlOp::Unspecified => "DDL_OP_UNSPECIFIED",
                DdlOp::Create => "DDL_OP_CREATE",
                DdlOp::CreateOrReplace => "DDL_OP_CREATE_OR_REPLACE",
                DdlOp::Alter => "DDL_OP_ALTER",
                DdlOp::Drop => "DDL_OP_DROP",
                DdlOp::DropIfExist => "DDL_OP_DROP_IF_EXIST",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DDL_OP_UNSPECIFIED" => Some(Self::Unspecified),
                "DDL_OP_CREATE" => Some(Self::Create),
                "DDL_OP_CREATE_OR_REPLACE" => Some(Self::CreateOrReplace),
                "DDL_OP_ALTER" => Some(Self::Alter),
                "DDL_OP_DROP" => Some(Self::Drop),
                "DDL_OP_DROP_IF_EXIST" => Some(Self::DropIfExist),
                _ => None,
            }
        }
    }
    /// Definition of which type of object we are operating on
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum WriteType {
        #[prost(message, tag = "1")]
        NamedObject(super::NamedObjectWrite),
        #[prost(message, tag = "2")]
        ExtensionObject(super::ExtensionObject),
    }
}
/// The operator that modifies the content of a database (operates on 1 table at a time, but record-selection/source can be
/// based on joining of multiple tables).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteRel {
    /// The schema of the table (must align with Rel input (e.g., number of leaf fields must match))
    #[prost(message, optional, tag = "3")]
    pub table_schema: ::core::option::Option<NamedStruct>,
    /// The type of operation to perform
    #[prost(enumeration = "write_rel::WriteOp", tag = "4")]
    pub op: i32,
    /// The relation that determines the records to add/remove/modify
    /// the schema must match with table_schema. Default values must be explicitly stated
    /// in a ProjectRel at the top of the input. The match must also
    /// occur in case of DELETE to ensure multi-engine plans are unequivocal.
    #[prost(message, optional, boxed, tag = "5")]
    pub input: ::core::option::Option<::prost::alloc::boxed::Box<Rel>>,
    /// Output mode determines what is the output of executing this rel
    #[prost(enumeration = "write_rel::OutputMode", tag = "6")]
    pub output: i32,
    #[prost(message, optional, tag = "7")]
    pub common: ::core::option::Option<RelCommon>,
    /// Definition of which TABLE we are operating on
    #[prost(oneof = "write_rel::WriteType", tags = "1, 2")]
    pub write_type: ::core::option::Option<write_rel::WriteType>,
}
/// Nested message and enum types in `WriteRel`.
pub mod write_rel {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum WriteOp {
        Unspecified = 0,
        /// The insert of new records in a table
        Insert = 1,
        /// The removal of records from a table
        Delete = 2,
        /// The modification of existing records within a table
        Update = 3,
        /// The Creation of a new table, and the insert of new records in the table
        Ctas = 4,
    }
    impl WriteOp {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                WriteOp::Unspecified => "WRITE_OP_UNSPECIFIED",
                WriteOp::Insert => "WRITE_OP_INSERT",
                WriteOp::Delete => "WRITE_OP_DELETE",
                WriteOp::Update => "WRITE_OP_UPDATE",
                WriteOp::Ctas => "WRITE_OP_CTAS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "WRITE_OP_UNSPECIFIED" => Some(Self::Unspecified),
                "WRITE_OP_INSERT" => Some(Self::Insert),
                "WRITE_OP_DELETE" => Some(Self::Delete),
                "WRITE_OP_UPDATE" => Some(Self::Update),
                "WRITE_OP_CTAS" => Some(Self::Ctas),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum OutputMode {
        Unspecified = 0,
        /// return no records at all
        NoOutput = 1,
        /// this mode makes the operator return all the record INSERTED/DELETED/UPDATED by the operator.
        /// The operator returns the AFTER-image of any change. This can be further manipulated by operators upstreams
        /// (e.g., retunring the typical "count of modified records").
        /// For scenarios in which the BEFORE image is required, the user must implement a spool (via references to
        /// subplans in the body of the Rel input) and return those with anounter PlanRel.relations.
        ModifiedRecords = 2,
    }
    impl OutputMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                OutputMode::Unspecified => "OUTPUT_MODE_UNSPECIFIED",
                OutputMode::NoOutput => "OUTPUT_MODE_NO_OUTPUT",
                OutputMode::ModifiedRecords => "OUTPUT_MODE_MODIFIED_RECORDS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OUTPUT_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "OUTPUT_MODE_NO_OUTPUT" => Some(Self::NoOutput),
                "OUTPUT_MODE_MODIFIED_RECORDS" => Some(Self::ModifiedRecords),
                _ => None,
            }
        }
    }
    /// Definition of which TABLE we are operating on
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum WriteType {
        #[prost(message, tag = "1")]
        NamedTable(super::NamedObjectWrite),
        #[prost(message, tag = "2")]
        ExtensionTable(super::ExtensionObject),
    }
}
/// Hash joins and merge joins are a specialization of the general join where the join
/// expression is an series of comparisons between fields that are ANDed together.  The
/// behavior of this comparison is flexible
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComparisonJoinKey {
    /// The key to compare from the left table
    #[prost(message, optional, tag = "1")]
    pub left: ::core::option::Option<expression::FieldReference>,
    /// The key to compare from the right table
    #[prost(message, optional, tag = "2")]
    pub right: ::core::option::Option<expression::FieldReference>,
    /// Describes how to compare the two keys
    #[prost(message, optional, tag = "3")]
    pub comparison: ::core::option::Option<comparison_join_key::ComparisonType>,
}
/// Nested message and enum types in `ComparisonJoinKey`.
pub mod comparison_join_key {
    /// Describes how the relation should consider if two rows are a match
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ComparisonType {
        #[prost(oneof = "comparison_type::InnerType", tags = "1, 2")]
        pub inner_type: ::core::option::Option<comparison_type::InnerType>,
    }
    /// Nested message and enum types in `ComparisonType`.
    pub mod comparison_type {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum InnerType {
            /// One of the simple comparison behaviors is used
            #[prost(enumeration = "super::SimpleComparisonType", tag = "1")]
            Simple(i32),
            /// A custom comparison behavior is used.  This can happen, for example, when using
            /// collations, where we might want to do something like a case-insensitive comparison.
            ///
            /// This must be a binary function with a boolean return type
            #[prost(uint32, tag = "2")]
            CustomFunctionReference(u32),
        }
    }
    /// Most joins will use one of the following behaviors.  To avoid the complexity
    /// of a function lookup we define the common behaviors here
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum SimpleComparisonType {
        Unspecified = 0,
        /// Returns true only if both values are equal and not null
        Eq = 1,
        /// Returns true if both values are equal and not null
        /// Returns true if both values are null
        /// Returns false if one value is null and the other value is not null
        ///
        /// This can be expressed as a = b OR (isnull(a) AND isnull(b))
        IsNotDistinctFrom = 2,
        /// Returns true if both values are equal and not null
        /// Returns true if either value is null
        ///
        /// This can be expressed as a = b OR isnull(a = b)
        MightEqual = 3,
    }
    impl SimpleComparisonType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SimpleComparisonType::Unspecified => "SIMPLE_COMPARISON_TYPE_UNSPECIFIED",
                SimpleComparisonType::Eq => "SIMPLE_COMPARISON_TYPE_EQ",
                SimpleComparisonType::IsNotDistinctFrom => {
                    "SIMPLE_COMPARISON_TYPE_IS_NOT_DISTINCT_FROM"
                }
                SimpleComparisonType::MightEqual => "SIMPLE_COMPARISON_TYPE_MIGHT_EQUAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SIMPLE_COMPARISON_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "SIMPLE_COMPARISON_TYPE_EQ" => Some(Self::Eq),
                "SIMPLE_COMPARISON_TYPE_IS_NOT_DISTINCT_FROM" => {
                    Some(Self::IsNotDistinctFrom)
                }
                "SIMPLE_COMPARISON_TYPE_MIGHT_EQUAL" => Some(Self::MightEqual),
                _ => None,
            }
        }
    }
}
/// The hash equijoin join operator will build a hash table out of the right input based on a set of join keys.
/// It will then probe that hash table for incoming inputs, finding matches.
///
/// Two rows are a match if the comparison function returns true for all keys
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HashJoinRel {
    #[prost(message, optional, tag = "1")]
    pub common: ::core::option::Option<RelCommon>,
    #[prost(message, optional, boxed, tag = "2")]
    pub left: ::core::option::Option<::prost::alloc::boxed::Box<Rel>>,
    #[prost(message, optional, boxed, tag = "3")]
    pub right: ::core::option::Option<::prost::alloc::boxed::Box<Rel>>,
    /// These fields are deprecated in favor of `keys`.  If they are set then
    /// the two lists (left_keys and right_keys) must have the same length and
    /// the comparion function is considered to be SimpleEqualityType::EQ
    #[deprecated]
    #[prost(message, repeated, tag = "4")]
    pub left_keys: ::prost::alloc::vec::Vec<expression::FieldReference>,
    #[deprecated]
    #[prost(message, repeated, tag = "5")]
    pub right_keys: ::prost::alloc::vec::Vec<expression::FieldReference>,
    /// One or more keys to join on.  The relation is invalid if this is empty
    /// (unless the deprecated left_keys/right_keys fields are being used).
    ///
    /// If a custom comparison function is used then it must be consistent with
    /// the hash function used for the keys.
    ///
    /// In other words, the hash function must return the same hash code when the
    /// comparison returns true.  For example, if the comparison function is
    /// "equals ignoring case" then the hash function must return the same hash
    /// code for strings that differ only by case.  Note: the hash function is not
    /// specified here.  It is the responsibility of the consumer to find an appropriate
    /// hash function for a given comparsion function or to reject the plan if it cannot
    /// do so.
    #[prost(message, repeated, tag = "8")]
    pub keys: ::prost::alloc::vec::Vec<ComparisonJoinKey>,
    #[prost(message, optional, boxed, tag = "6")]
    pub post_join_filter: ::core::option::Option<::prost::alloc::boxed::Box<Expression>>,
    #[prost(enumeration = "hash_join_rel::JoinType", tag = "7")]
    pub r#type: i32,
    #[prost(message, optional, tag = "10")]
    pub advanced_extension: ::core::option::Option<extensions::AdvancedExtension>,
}
/// Nested message and enum types in `HashJoinRel`.
pub mod hash_join_rel {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum JoinType {
        Unspecified = 0,
        Inner = 1,
        Outer = 2,
        Left = 3,
        Right = 4,
        LeftSemi = 5,
        RightSemi = 6,
        LeftAnti = 7,
        RightAnti = 8,
    }
    impl JoinType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                JoinType::Unspecified => "JOIN_TYPE_UNSPECIFIED",
                JoinType::Inner => "JOIN_TYPE_INNER",
                JoinType::Outer => "JOIN_TYPE_OUTER",
                JoinType::Left => "JOIN_TYPE_LEFT",
                JoinType::Right => "JOIN_TYPE_RIGHT",
                JoinType::LeftSemi => "JOIN_TYPE_LEFT_SEMI",
                JoinType::RightSemi => "JOIN_TYPE_RIGHT_SEMI",
                JoinType::LeftAnti => "JOIN_TYPE_LEFT_ANTI",
                JoinType::RightAnti => "JOIN_TYPE_RIGHT_ANTI",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "JOIN_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "JOIN_TYPE_INNER" => Some(Self::Inner),
                "JOIN_TYPE_OUTER" => Some(Self::Outer),
                "JOIN_TYPE_LEFT" => Some(Self::Left),
                "JOIN_TYPE_RIGHT" => Some(Self::Right),
                "JOIN_TYPE_LEFT_SEMI" => Some(Self::LeftSemi),
                "JOIN_TYPE_RIGHT_SEMI" => Some(Self::RightSemi),
                "JOIN_TYPE_LEFT_ANTI" => Some(Self::LeftAnti),
                "JOIN_TYPE_RIGHT_ANTI" => Some(Self::RightAnti),
                _ => None,
            }
        }
    }
}
/// The merge equijoin does a join by taking advantage of two sets that are sorted on the join keys.
/// This allows the join operation to be done in a streaming fashion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MergeJoinRel {
    #[prost(message, optional, tag = "1")]
    pub common: ::core::option::Option<RelCommon>,
    #[prost(message, optional, boxed, tag = "2")]
    pub left: ::core::option::Option<::prost::alloc::boxed::Box<Rel>>,
    #[prost(message, optional, boxed, tag = "3")]
    pub right: ::core::option::Option<::prost::alloc::boxed::Box<Rel>>,
    /// These fields are deprecated in favor of `keys`.  If they are set then
    /// the two lists (left_keys and right_keys) must have the same length and
    /// the comparion function is considered to be SimpleEqualityType::EQ
    #[deprecated]
    #[prost(message, repeated, tag = "4")]
    pub left_keys: ::prost::alloc::vec::Vec<expression::FieldReference>,
    #[deprecated]
    #[prost(message, repeated, tag = "5")]
    pub right_keys: ::prost::alloc::vec::Vec<expression::FieldReference>,
    /// One or more keys to join on.  The relation is invalid if this is empty
    /// (unless the deprecated left_keys/right_keys fields are being used).
    ///
    /// If a custom comparison function is used then it must be consistent with
    /// the ordering of the input data.  For example, if the comparison function
    /// is "<" then we generally expect the data to be sorted in ascending order.
    ///
    /// If the comparison function is something like "less than ignoring case" then
    /// the data should be sorted appropriately (e.g. both "A" and "a" should come
    /// before "b")
    ///
    /// The sort order is not specified here.  It is typically the responsibility of
    /// the producer to ensure the plan sorts the data if needed (although the consumer
    /// is free to do so as well).  If possible, the consumer should verify the sort
    /// order and reject invalid plans.
    #[prost(message, repeated, tag = "8")]
    pub keys: ::prost::alloc::vec::Vec<ComparisonJoinKey>,
    #[prost(message, optional, boxed, tag = "6")]
    pub post_join_filter: ::core::option::Option<::prost::alloc::boxed::Box<Expression>>,
    #[prost(enumeration = "merge_join_rel::JoinType", tag = "7")]
    pub r#type: i32,
    #[prost(message, optional, tag = "10")]
    pub advanced_extension: ::core::option::Option<extensions::AdvancedExtension>,
}
/// Nested message and enum types in `MergeJoinRel`.
pub mod merge_join_rel {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum JoinType {
        Unspecified = 0,
        Inner = 1,
        Outer = 2,
        Left = 3,
        Right = 4,
        LeftSemi = 5,
        RightSemi = 6,
        LeftAnti = 7,
        RightAnti = 8,
    }
    impl JoinType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                JoinType::Unspecified => "JOIN_TYPE_UNSPECIFIED",
                JoinType::Inner => "JOIN_TYPE_INNER",
                JoinType::Outer => "JOIN_TYPE_OUTER",
                JoinType::Left => "JOIN_TYPE_LEFT",
                JoinType::Right => "JOIN_TYPE_RIGHT",
                JoinType::LeftSemi => "JOIN_TYPE_LEFT_SEMI",
                JoinType::RightSemi => "JOIN_TYPE_RIGHT_SEMI",
                JoinType::LeftAnti => "JOIN_TYPE_LEFT_ANTI",
                JoinType::RightAnti => "JOIN_TYPE_RIGHT_ANTI",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "JOIN_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "JOIN_TYPE_INNER" => Some(Self::Inner),
                "JOIN_TYPE_OUTER" => Some(Self::Outer),
                "JOIN_TYPE_LEFT" => Some(Self::Left),
                "JOIN_TYPE_RIGHT" => Some(Self::Right),
                "JOIN_TYPE_LEFT_SEMI" => Some(Self::LeftSemi),
                "JOIN_TYPE_RIGHT_SEMI" => Some(Self::RightSemi),
                "JOIN_TYPE_LEFT_ANTI" => Some(Self::LeftAnti),
                "JOIN_TYPE_RIGHT_ANTI" => Some(Self::RightAnti),
                _ => None,
            }
        }
    }
}
/// The nested loop join (NLJ) operator will hold the entire right input and iterate over it using the
/// left input, evaluating the join expression on the Cartesian product of all rows.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NestedLoopJoinRel {
    #[prost(message, optional, tag = "1")]
    pub common: ::core::option::Option<RelCommon>,
    #[prost(message, optional, boxed, tag = "2")]
    pub left: ::core::option::Option<::prost::alloc::boxed::Box<Rel>>,
    #[prost(message, optional, boxed, tag = "3")]
    pub right: ::core::option::Option<::prost::alloc::boxed::Box<Rel>>,
    /// optional, defaults to true (a cartesian join)
    #[prost(message, optional, boxed, tag = "4")]
    pub expression: ::core::option::Option<::prost::alloc::boxed::Box<Expression>>,
    #[prost(enumeration = "nested_loop_join_rel::JoinType", tag = "5")]
    pub r#type: i32,
    #[prost(message, optional, tag = "10")]
    pub advanced_extension: ::core::option::Option<extensions::AdvancedExtension>,
}
/// Nested message and enum types in `NestedLoopJoinRel`.
pub mod nested_loop_join_rel {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum JoinType {
        Unspecified = 0,
        Inner = 1,
        Outer = 2,
        Left = 3,
        Right = 4,
        LeftSemi = 5,
        RightSemi = 6,
        LeftAnti = 7,
        RightAnti = 8,
    }
    impl JoinType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                JoinType::Unspecified => "JOIN_TYPE_UNSPECIFIED",
                JoinType::Inner => "JOIN_TYPE_INNER",
                JoinType::Outer => "JOIN_TYPE_OUTER",
                JoinType::Left => "JOIN_TYPE_LEFT",
                JoinType::Right => "JOIN_TYPE_RIGHT",
                JoinType::LeftSemi => "JOIN_TYPE_LEFT_SEMI",
                JoinType::RightSemi => "JOIN_TYPE_RIGHT_SEMI",
                JoinType::LeftAnti => "JOIN_TYPE_LEFT_ANTI",
                JoinType::RightAnti => "JOIN_TYPE_RIGHT_ANTI",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "JOIN_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "JOIN_TYPE_INNER" => Some(Self::Inner),
                "JOIN_TYPE_OUTER" => Some(Self::Outer),
                "JOIN_TYPE_LEFT" => Some(Self::Left),
                "JOIN_TYPE_RIGHT" => Some(Self::Right),
                "JOIN_TYPE_LEFT_SEMI" => Some(Self::LeftSemi),
                "JOIN_TYPE_RIGHT_SEMI" => Some(Self::RightSemi),
                "JOIN_TYPE_LEFT_ANTI" => Some(Self::LeftAnti),
                "JOIN_TYPE_RIGHT_ANTI" => Some(Self::RightAnti),
                _ => None,
            }
        }
    }
}
/// The argument of a function
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionArgument {
    #[prost(oneof = "function_argument::ArgType", tags = "1, 2, 3")]
    pub arg_type: ::core::option::Option<function_argument::ArgType>,
}
/// Nested message and enum types in `FunctionArgument`.
pub mod function_argument {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ArgType {
        #[prost(string, tag = "1")]
        Enum(::prost::alloc::string::String),
        #[prost(message, tag = "2")]
        Type(super::Type),
        #[prost(message, tag = "3")]
        Value(super::Expression),
    }
}
/// An optional function argument.  Typically used for specifying behavior in
/// invalid or corner cases.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionOption {
    /// Name of the option to set. If the consumer does not recognize the
    /// option, it must reject the plan. The name is matched case-insensitively
    /// with option names defined for the function.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// List of behavior options allowed by the producer. At least one must be
    /// specified; to leave an option unspecified, simply don't add an entry to
    /// `options`. The consumer must use the first option from the list that it
    /// supports. If the consumer supports none of the specified options, it
    /// must reject the plan. The name is matched case-insensitively and must
    /// match one of the option values defined for the option.
    #[prost(string, repeated, tag = "2")]
    pub preference: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Expression {
    #[prost(
        oneof = "expression::RexType",
        tags = "1, 2, 3, 5, 6, 7, 8, 9, 11, 12, 13, 10"
    )]
    pub rex_type: ::core::option::Option<expression::RexType>,
}
/// Nested message and enum types in `Expression`.
pub mod expression {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Enum {
        #[prost(oneof = "r#enum::EnumKind", tags = "1, 2")]
        pub enum_kind: ::core::option::Option<r#enum::EnumKind>,
    }
    /// Nested message and enum types in `Enum`.
    pub mod r#enum {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Empty {}
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum EnumKind {
            #[prost(string, tag = "1")]
            Specified(::prost::alloc::string::String),
            #[prost(message, tag = "2")]
            Unspecified(Empty),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Literal {
        /// whether the literal type should be treated as a nullable type. Applies to
        /// all members of union other than the Typed null (which should directly
        /// declare nullability).
        #[prost(bool, tag = "50")]
        pub nullable: bool,
        /// optionally points to a type_variation_anchor defined in this plan.
        /// Applies to all members of union other than the Typed null (which should
        /// directly declare the type variation).
        #[prost(uint32, tag = "51")]
        pub type_variation_reference: u32,
        #[prost(
            oneof = "literal::LiteralType",
            tags = "1, 2, 3, 5, 7, 10, 11, 12, 13, 14, 16, 17, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33"
        )]
        pub literal_type: ::core::option::Option<literal::LiteralType>,
    }
    /// Nested message and enum types in `Literal`.
    pub mod literal {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct VarChar {
            #[prost(string, tag = "1")]
            pub value: ::prost::alloc::string::String,
            #[prost(uint32, tag = "2")]
            pub length: u32,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Decimal {
            /// little-endian twos-complement integer representation of complete value
            /// (ignoring precision) Always 16 bytes in length
            #[prost(bytes = "vec", tag = "1")]
            pub value: ::prost::alloc::vec::Vec<u8>,
            /// The maximum number of digits allowed in the value.
            /// the maximum precision is 38.
            #[prost(int32, tag = "2")]
            pub precision: i32,
            /// declared scale of decimal literal
            #[prost(int32, tag = "3")]
            pub scale: i32,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Map {
            #[prost(message, repeated, tag = "1")]
            pub key_values: ::prost::alloc::vec::Vec<map::KeyValue>,
        }
        /// Nested message and enum types in `Map`.
        pub mod map {
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct KeyValue {
                #[prost(message, optional, tag = "1")]
                pub key: ::core::option::Option<super::super::Literal>,
                #[prost(message, optional, tag = "2")]
                pub value: ::core::option::Option<super::super::Literal>,
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct IntervalYearToMonth {
            #[prost(int32, tag = "1")]
            pub years: i32,
            #[prost(int32, tag = "2")]
            pub months: i32,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct IntervalDayToSecond {
            #[prost(int32, tag = "1")]
            pub days: i32,
            #[prost(int32, tag = "2")]
            pub seconds: i32,
            #[prost(int32, tag = "3")]
            pub microseconds: i32,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Struct {
            /// A possibly heterogeneously typed list of literals
            #[prost(message, repeated, tag = "1")]
            pub fields: ::prost::alloc::vec::Vec<super::Literal>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct List {
            /// A homogeneously typed list of literals
            #[prost(message, repeated, tag = "1")]
            pub values: ::prost::alloc::vec::Vec<super::Literal>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct UserDefined {
            /// points to a type_anchor defined in this plan
            #[prost(uint32, tag = "1")]
            pub type_reference: u32,
            /// The parameters to be bound to the type class, if the type class is
            /// parameterizable.
            #[prost(message, repeated, tag = "3")]
            pub type_parameters: ::prost::alloc::vec::Vec<
                super::super::r#type::Parameter,
            >,
            /// the value of the literal, serialized using some type-specific
            /// protobuf message
            #[prost(message, optional, tag = "2")]
            pub value: ::core::option::Option<::prost_types::Any>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum LiteralType {
            #[prost(bool, tag = "1")]
            Boolean(bool),
            #[prost(int32, tag = "2")]
            I8(i32),
            #[prost(int32, tag = "3")]
            I16(i32),
            #[prost(int32, tag = "5")]
            I32(i32),
            #[prost(int64, tag = "7")]
            I64(i64),
            #[prost(float, tag = "10")]
            Fp32(f32),
            #[prost(double, tag = "11")]
            Fp64(f64),
            #[prost(string, tag = "12")]
            String(::prost::alloc::string::String),
            #[prost(bytes, tag = "13")]
            Binary(::prost::alloc::vec::Vec<u8>),
            /// Timestamp in units of microseconds since the UNIX epoch.
            #[prost(int64, tag = "14")]
            Timestamp(i64),
            /// Date in units of days since the UNIX epoch.
            #[prost(int32, tag = "16")]
            Date(i32),
            /// Time in units of microseconds past midnight
            #[prost(int64, tag = "17")]
            Time(i64),
            #[prost(message, tag = "19")]
            IntervalYearToMonth(IntervalYearToMonth),
            #[prost(message, tag = "20")]
            IntervalDayToSecond(IntervalDayToSecond),
            #[prost(string, tag = "21")]
            FixedChar(::prost::alloc::string::String),
            #[prost(message, tag = "22")]
            VarChar(VarChar),
            #[prost(bytes, tag = "23")]
            FixedBinary(::prost::alloc::vec::Vec<u8>),
            #[prost(message, tag = "24")]
            Decimal(Decimal),
            #[prost(message, tag = "25")]
            Struct(Struct),
            #[prost(message, tag = "26")]
            Map(Map),
            /// Timestamp in units of microseconds since the UNIX epoch.
            #[prost(int64, tag = "27")]
            TimestampTz(i64),
            #[prost(bytes, tag = "28")]
            Uuid(::prost::alloc::vec::Vec<u8>),
            /// a typed null literal
            #[prost(message, tag = "29")]
            Null(super::super::Type),
            #[prost(message, tag = "30")]
            List(List),
            #[prost(message, tag = "31")]
            EmptyList(super::super::r#type::List),
            #[prost(message, tag = "32")]
            EmptyMap(super::super::r#type::Map),
            #[prost(message, tag = "33")]
            UserDefined(UserDefined),
        }
    }
    /// Expression to dynamically construct nested types.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Nested {
        /// Whether the returned nested type is nullable.
        #[prost(bool, tag = "1")]
        pub nullable: bool,
        /// Optionally points to a type_variation_anchor defined in this plan for
        /// the returned nested type.
        #[prost(uint32, tag = "2")]
        pub type_variation_reference: u32,
        #[prost(oneof = "nested::NestedType", tags = "3, 4, 5")]
        pub nested_type: ::core::option::Option<nested::NestedType>,
    }
    /// Nested message and enum types in `Nested`.
    pub mod nested {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Map {
            /// One or more key-value pairs. To specify an empty map, use
            /// Literal.empty_map (otherwise type information would be missing).
            #[prost(message, repeated, tag = "1")]
            pub key_values: ::prost::alloc::vec::Vec<map::KeyValue>,
        }
        /// Nested message and enum types in `Map`.
        pub mod map {
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct KeyValue {
                /// Mandatory key/value expressions.
                #[prost(message, optional, tag = "1")]
                pub key: ::core::option::Option<super::super::super::Expression>,
                #[prost(message, optional, tag = "2")]
                pub value: ::core::option::Option<super::super::super::Expression>,
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Struct {
            /// Zero or more possibly heterogeneously-typed list of expressions that
            /// form the struct fields.
            #[prost(message, repeated, tag = "1")]
            pub fields: ::prost::alloc::vec::Vec<super::super::Expression>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct List {
            /// A homogeneously-typed list of one or more expressions that form the
            /// list entries. To specify an empty list, use Literal.empty_list
            /// (otherwise type information would be missing).
            #[prost(message, repeated, tag = "1")]
            pub values: ::prost::alloc::vec::Vec<super::super::Expression>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum NestedType {
            #[prost(message, tag = "3")]
            Struct(Struct),
            #[prost(message, tag = "4")]
            List(List),
            #[prost(message, tag = "5")]
            Map(Map),
        }
    }
    /// A scalar function call.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ScalarFunction {
        /// Points to a function_anchor defined in this plan, which must refer
        /// to a scalar function in the associated YAML file. Required; avoid
        /// using anchor/reference zero.
        #[prost(uint32, tag = "1")]
        pub function_reference: u32,
        /// The arguments to be bound to the function. This must have exactly the
        /// number of arguments specified in the function definition, and the
        /// argument types must also match exactly:
        ///
        ///   - Value arguments must be bound using FunctionArgument.value, and
        ///     the expression in that must yield a value of a type that a function
        ///     overload is defined for.
        ///   - Type arguments must be bound using FunctionArgument.type.
        ///   - Enum arguments must be bound using FunctionArgument.enum
        ///     followed by Enum.specified, with a string that case-insensitively
        ///     matches one of the allowed options.
        #[prost(message, repeated, tag = "4")]
        pub arguments: ::prost::alloc::vec::Vec<super::FunctionArgument>,
        /// Options to specify behavior for corner cases, or leave behavior
        /// unspecified if the consumer does not need specific behavior in these
        /// cases.
        #[prost(message, repeated, tag = "5")]
        pub options: ::prost::alloc::vec::Vec<super::FunctionOption>,
        /// Must be set to the return type of the function, exactly as derived
        /// using the declaration in the extension.
        #[prost(message, optional, tag = "3")]
        pub output_type: ::core::option::Option<super::Type>,
        /// Deprecated; use arguments instead.
        #[deprecated]
        #[prost(message, repeated, tag = "2")]
        pub args: ::prost::alloc::vec::Vec<super::Expression>,
    }
    /// A window function call.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WindowFunction {
        /// Points to a function_anchor defined in this plan. The function must be:
        ///   - a window function
        ///   - an aggregate function
        ///
        /// An aggregate function referenced here should be treated as a window
        /// function with Window Type STREAMING
        ///
        /// Required; 0 is considered to be a valid anchor/reference.
        #[prost(uint32, tag = "1")]
        pub function_reference: u32,
        /// The arguments to be bound to the function. This must have exactly the
        /// number of arguments specified in the function definition, and the
        /// argument types must also match exactly:
        ///
        ///   - Value arguments must be bound using FunctionArgument.value, and
        ///     the expression in that must yield a value of a type that a function
        ///     overload is defined for.
        ///   - Type arguments must be bound using FunctionArgument.type, and a
        ///     function overload must be defined for that type.
        ///   - Enum arguments must be bound using FunctionArgument.enum
        ///     followed by Enum.specified, with a string that case-insensitively
        ///     matches one of the allowed options.
        #[prost(message, repeated, tag = "9")]
        pub arguments: ::prost::alloc::vec::Vec<super::FunctionArgument>,
        /// Options to specify behavior for corner cases, or leave behavior
        /// unspecified if the consumer does not need specific behavior in these
        /// cases.
        #[prost(message, repeated, tag = "11")]
        pub options: ::prost::alloc::vec::Vec<super::FunctionOption>,
        /// Must be set to the return type of the function, exactly as derived
        /// using the declaration in the extension.
        #[prost(message, optional, tag = "7")]
        pub output_type: ::core::option::Option<super::Type>,
        /// Describes which part of the window function to perform within the
        /// context of distributed algorithms. Required. Must be set to
        /// INITIAL_TO_RESULT for window functions that are not decomposable.
        #[prost(enumeration = "super::AggregationPhase", tag = "6")]
        pub phase: i32,
        /// If specified, the records that are part of the window defined by
        /// upper_bound and lower_bound are ordered according to this list
        /// before they are aggregated. The first sort field has the highest
        /// priority; only if a sort field determines two records to be equivalent
        /// is the next field queried. This field is optional, and is only allowed
        /// if the window function is defined to support sorting.
        #[prost(message, repeated, tag = "3")]
        pub sorts: ::prost::alloc::vec::Vec<super::SortField>,
        /// Specifies whether equivalent records are merged before being aggregated.
        /// Optional, defaults to AGGREGATION_INVOCATION_ALL.
        #[prost(
            enumeration = "super::aggregate_function::AggregationInvocation",
            tag = "10"
        )]
        pub invocation: i32,
        /// When one or more partition expressions are specified, two records are
        /// considered to be in the same partition if and only if these expressions
        /// yield an equal record of values for both. When computing the window
        /// function, only the subset of records within the bounds that are also in
        /// the same partition as the current record are aggregated.
        #[prost(message, repeated, tag = "2")]
        pub partitions: ::prost::alloc::vec::Vec<super::Expression>,
        /// Defines the bounds type: ROWS, RANGE
        #[prost(enumeration = "window_function::BoundsType", tag = "12")]
        pub bounds_type: i32,
        /// Defines the record relative to the current record from which the window
        /// extends. The bound is inclusive. If the lower bound indexes a record
        /// greater than the upper bound, TODO (null range/no records passed?
        /// wrapping around as if lower/upper were swapped? error? null?).
        /// Optional; defaults to the start of the partition.
        #[prost(message, optional, tag = "5")]
        pub lower_bound: ::core::option::Option<window_function::Bound>,
        /// Defines the record relative to the current record up to which the window
        /// extends. The bound is inclusive. If the upper bound indexes a record
        /// less than the lower bound, TODO (null range/no records passed?
        /// wrapping around as if lower/upper were swapped? error? null?).
        /// Optional; defaults to the end of the partition.
        #[prost(message, optional, tag = "4")]
        pub upper_bound: ::core::option::Option<window_function::Bound>,
        /// Deprecated; use arguments instead.
        #[deprecated]
        #[prost(message, repeated, tag = "8")]
        pub args: ::prost::alloc::vec::Vec<super::Expression>,
    }
    /// Nested message and enum types in `WindowFunction`.
    pub mod window_function {
        /// Defines one of the two boundaries for the window of a window function.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Bound {
            #[prost(oneof = "bound::Kind", tags = "1, 2, 3, 4")]
            pub kind: ::core::option::Option<bound::Kind>,
        }
        /// Nested message and enum types in `Bound`.
        pub mod bound {
            /// Defines that the bound extends this far back from the current record.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Preceding {
                /// A strictly positive integer specifying the number of records that
                /// the window extends back from the current record. Required. Use
                /// CurrentRow for offset zero and Following for negative offsets.
                #[prost(int64, tag = "1")]
                pub offset: i64,
            }
            /// Defines that the bound extends this far ahead of the current record.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Following {
                /// A strictly positive integer specifying the number of records that
                /// the window extends ahead of the current record. Required. Use
                /// CurrentRow for offset zero and Preceding for negative offsets.
                #[prost(int64, tag = "1")]
                pub offset: i64,
            }
            /// Defines that the bound extends to or from the current record.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct CurrentRow {}
            /// Defines an "unbounded bound": for lower bounds this means the start
            /// of the partition, and for upper bounds this means the end of the
            /// partition.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Unbounded {}
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Kind {
                /// The bound extends some number of records behind the current record.
                #[prost(message, tag = "1")]
                Preceding(Preceding),
                /// The bound extends some number of records ahead of the current
                /// record.
                #[prost(message, tag = "2")]
                Following(Following),
                /// The bound extends to the current record.
                #[prost(message, tag = "3")]
                CurrentRow(CurrentRow),
                /// The bound extends to the start of the partition or the end of the
                /// partition, depending on whether this represents the upper or lower
                /// bound.
                #[prost(message, tag = "4")]
                Unbounded(Unbounded),
            }
        }
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum BoundsType {
            Unspecified = 0,
            /// The lower and upper bound specify how many rows before and after the current row
            /// the window should extend.
            Rows = 1,
            /// The lower and upper bound describe a range of values.  The window should include all rows
            /// where the value of the ordering column is greater than or equal to (current_value - lower bound)
            /// and less than or equal to (current_value + upper bound).  This bounds type is only valid if there
            /// is a single ordering column.
            Range = 2,
        }
        impl BoundsType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    BoundsType::Unspecified => "BOUNDS_TYPE_UNSPECIFIED",
                    BoundsType::Rows => "BOUNDS_TYPE_ROWS",
                    BoundsType::Range => "BOUNDS_TYPE_RANGE",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "BOUNDS_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                    "BOUNDS_TYPE_ROWS" => Some(Self::Rows),
                    "BOUNDS_TYPE_RANGE" => Some(Self::Range),
                    _ => None,
                }
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IfThen {
        /// A list of one or more IfClauses
        #[prost(message, repeated, tag = "1")]
        pub ifs: ::prost::alloc::vec::Vec<if_then::IfClause>,
        /// The returned Expression if no IfClauses are satisified
        #[prost(message, optional, boxed, tag = "2")]
        pub r#else: ::core::option::Option<
            ::prost::alloc::boxed::Box<super::Expression>,
        >,
    }
    /// Nested message and enum types in `IfThen`.
    pub mod if_then {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct IfClause {
            #[prost(message, optional, tag = "1")]
            pub r#if: ::core::option::Option<super::super::Expression>,
            #[prost(message, optional, tag = "2")]
            pub then: ::core::option::Option<super::super::Expression>,
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Cast {
        #[prost(message, optional, tag = "1")]
        pub r#type: ::core::option::Option<super::Type>,
        #[prost(message, optional, boxed, tag = "2")]
        pub input: ::core::option::Option<::prost::alloc::boxed::Box<super::Expression>>,
        #[prost(enumeration = "cast::FailureBehavior", tag = "3")]
        pub failure_behavior: i32,
    }
    /// Nested message and enum types in `Cast`.
    pub mod cast {
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum FailureBehavior {
            Unspecified = 0,
            ReturnNull = 1,
            ThrowException = 2,
        }
        impl FailureBehavior {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    FailureBehavior::Unspecified => "FAILURE_BEHAVIOR_UNSPECIFIED",
                    FailureBehavior::ReturnNull => "FAILURE_BEHAVIOR_RETURN_NULL",
                    FailureBehavior::ThrowException => "FAILURE_BEHAVIOR_THROW_EXCEPTION",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "FAILURE_BEHAVIOR_UNSPECIFIED" => Some(Self::Unspecified),
                    "FAILURE_BEHAVIOR_RETURN_NULL" => Some(Self::ReturnNull),
                    "FAILURE_BEHAVIOR_THROW_EXCEPTION" => Some(Self::ThrowException),
                    _ => None,
                }
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SwitchExpression {
        #[prost(message, optional, boxed, tag = "3")]
        pub r#match: ::core::option::Option<
            ::prost::alloc::boxed::Box<super::Expression>,
        >,
        #[prost(message, repeated, tag = "1")]
        pub ifs: ::prost::alloc::vec::Vec<switch_expression::IfValue>,
        #[prost(message, optional, boxed, tag = "2")]
        pub r#else: ::core::option::Option<
            ::prost::alloc::boxed::Box<super::Expression>,
        >,
    }
    /// Nested message and enum types in `SwitchExpression`.
    pub mod switch_expression {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct IfValue {
            #[prost(message, optional, tag = "1")]
            pub r#if: ::core::option::Option<super::Literal>,
            #[prost(message, optional, tag = "2")]
            pub then: ::core::option::Option<super::super::Expression>,
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SingularOrList {
        #[prost(message, optional, boxed, tag = "1")]
        pub value: ::core::option::Option<::prost::alloc::boxed::Box<super::Expression>>,
        #[prost(message, repeated, tag = "2")]
        pub options: ::prost::alloc::vec::Vec<super::Expression>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MultiOrList {
        #[prost(message, repeated, tag = "1")]
        pub value: ::prost::alloc::vec::Vec<super::Expression>,
        #[prost(message, repeated, tag = "2")]
        pub options: ::prost::alloc::vec::Vec<multi_or_list::Record>,
    }
    /// Nested message and enum types in `MultiOrList`.
    pub mod multi_or_list {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Record {
            #[prost(message, repeated, tag = "1")]
            pub fields: ::prost::alloc::vec::Vec<super::super::Expression>,
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EmbeddedFunction {
        #[prost(message, repeated, tag = "1")]
        pub arguments: ::prost::alloc::vec::Vec<super::Expression>,
        #[prost(message, optional, tag = "2")]
        pub output_type: ::core::option::Option<super::Type>,
        #[prost(oneof = "embedded_function::Kind", tags = "3, 4")]
        pub kind: ::core::option::Option<embedded_function::Kind>,
    }
    /// Nested message and enum types in `EmbeddedFunction`.
    pub mod embedded_function {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct PythonPickleFunction {
            #[prost(bytes = "vec", tag = "1")]
            pub function: ::prost::alloc::vec::Vec<u8>,
            #[prost(string, repeated, tag = "2")]
            pub prerequisite: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct WebAssemblyFunction {
            #[prost(bytes = "vec", tag = "1")]
            pub script: ::prost::alloc::vec::Vec<u8>,
            #[prost(string, repeated, tag = "2")]
            pub prerequisite: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Kind {
            #[prost(message, tag = "3")]
            PythonPickleFunction(PythonPickleFunction),
            #[prost(message, tag = "4")]
            WebAssemblyFunction(WebAssemblyFunction),
        }
    }
    /// A way to reference the inner property of a complex record. Can reference
    /// either a map key by literal, a struct field by the ordinal position of
    /// the desired field or a particular element in an array. Supports
    /// expressions that would roughly translate to something similar to:
    /// a.b\[2\].c\['my_map_key'\].x where a,b,c and x are struct field references
    /// (ordinalized in the internal representation here), \[2\] is a list offset
    /// and \['my_map_key'\] is a reference into a map field.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ReferenceSegment {
        #[prost(oneof = "reference_segment::ReferenceType", tags = "1, 2, 3")]
        pub reference_type: ::core::option::Option<reference_segment::ReferenceType>,
    }
    /// Nested message and enum types in `ReferenceSegment`.
    pub mod reference_segment {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MapKey {
            /// literal based reference to specific possible value in map.
            #[prost(message, optional, tag = "1")]
            pub map_key: ::core::option::Option<super::Literal>,
            /// Optional child segment
            #[prost(message, optional, boxed, tag = "2")]
            pub child: ::core::option::Option<
                ::prost::alloc::boxed::Box<super::ReferenceSegment>,
            >,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct StructField {
            /// zero-indexed ordinal position of field in struct
            #[prost(int32, tag = "1")]
            pub field: i32,
            /// Optional child segment
            #[prost(message, optional, boxed, tag = "2")]
            pub child: ::core::option::Option<
                ::prost::alloc::boxed::Box<super::ReferenceSegment>,
            >,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ListElement {
            /// zero-indexed ordinal position of element in list
            #[prost(int32, tag = "1")]
            pub offset: i32,
            /// Optional child segment
            #[prost(message, optional, boxed, tag = "2")]
            pub child: ::core::option::Option<
                ::prost::alloc::boxed::Box<super::ReferenceSegment>,
            >,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ReferenceType {
            #[prost(message, tag = "1")]
            MapKey(::prost::alloc::boxed::Box<MapKey>),
            #[prost(message, tag = "2")]
            StructField(::prost::alloc::boxed::Box<StructField>),
            #[prost(message, tag = "3")]
            ListElement(::prost::alloc::boxed::Box<ListElement>),
        }
    }
    /// A reference that takes an existing subtype and selectively removes fields
    /// from it. For example, one might initially have an inner struct with 100
    /// fields but a a particular operation only needs to interact with only 2 of
    /// those 100 fields. In this situation, one would use a mask expression to
    /// eliminate the 98 fields that are not relevant to the rest of the operation
    /// pipeline.
    ///
    /// Note that this does not fundamentally alter the structure of data beyond
    /// the elimination of unnecessary elements.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MaskExpression {
        #[prost(message, optional, tag = "1")]
        pub select: ::core::option::Option<mask_expression::StructSelect>,
        #[prost(bool, tag = "2")]
        pub maintain_singular_struct: bool,
    }
    /// Nested message and enum types in `MaskExpression`.
    pub mod mask_expression {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Select {
            #[prost(oneof = "select::Type", tags = "1, 2, 3")]
            pub r#type: ::core::option::Option<select::Type>,
        }
        /// Nested message and enum types in `Select`.
        pub mod select {
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Type {
                #[prost(message, tag = "1")]
                Struct(super::StructSelect),
                #[prost(message, tag = "2")]
                List(::prost::alloc::boxed::Box<super::ListSelect>),
                #[prost(message, tag = "3")]
                Map(::prost::alloc::boxed::Box<super::MapSelect>),
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct StructSelect {
            #[prost(message, repeated, tag = "1")]
            pub struct_items: ::prost::alloc::vec::Vec<StructItem>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct StructItem {
            #[prost(int32, tag = "1")]
            pub field: i32,
            #[prost(message, optional, tag = "2")]
            pub child: ::core::option::Option<Select>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ListSelect {
            #[prost(message, repeated, tag = "1")]
            pub selection: ::prost::alloc::vec::Vec<list_select::ListSelectItem>,
            #[prost(message, optional, boxed, tag = "2")]
            pub child: ::core::option::Option<::prost::alloc::boxed::Box<Select>>,
        }
        /// Nested message and enum types in `ListSelect`.
        pub mod list_select {
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct ListSelectItem {
                #[prost(oneof = "list_select_item::Type", tags = "1, 2")]
                pub r#type: ::core::option::Option<list_select_item::Type>,
            }
            /// Nested message and enum types in `ListSelectItem`.
            pub mod list_select_item {
                #[allow(clippy::derive_partial_eq_without_eq)]
                #[derive(Clone, PartialEq, ::prost::Message)]
                pub struct ListElement {
                    #[prost(int32, tag = "1")]
                    pub field: i32,
                }
                #[allow(clippy::derive_partial_eq_without_eq)]
                #[derive(Clone, PartialEq, ::prost::Message)]
                pub struct ListSlice {
                    #[prost(int32, tag = "1")]
                    pub start: i32,
                    #[prost(int32, tag = "2")]
                    pub end: i32,
                }
                #[allow(clippy::derive_partial_eq_without_eq)]
                #[derive(Clone, PartialEq, ::prost::Oneof)]
                pub enum Type {
                    #[prost(message, tag = "1")]
                    Item(ListElement),
                    #[prost(message, tag = "2")]
                    Slice(ListSlice),
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MapSelect {
            #[prost(message, optional, boxed, tag = "3")]
            pub child: ::core::option::Option<::prost::alloc::boxed::Box<Select>>,
            #[prost(oneof = "map_select::Select", tags = "1, 2")]
            pub select: ::core::option::Option<map_select::Select>,
        }
        /// Nested message and enum types in `MapSelect`.
        pub mod map_select {
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct MapKey {
                #[prost(string, tag = "1")]
                pub map_key: ::prost::alloc::string::String,
            }
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct MapKeyExpression {
                #[prost(string, tag = "1")]
                pub map_key_expression: ::prost::alloc::string::String,
            }
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Select {
                #[prost(message, tag = "1")]
                Key(MapKey),
                #[prost(message, tag = "2")]
                Expression(MapKeyExpression),
            }
        }
    }
    /// A reference to an inner part of a complex object. Can reference reference a
    /// single element or a masked version of elements
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FieldReference {
        /// Whether this is composed of a single element reference or a masked
        /// element subtree
        #[prost(oneof = "field_reference::ReferenceType", tags = "1, 2")]
        pub reference_type: ::core::option::Option<field_reference::ReferenceType>,
        /// Whether this reference has an origin of a root struct or is based on the
        /// ouput of an expression. When this is a RootReference and direct_reference
        /// above is used, the direct_reference must be of a type StructField.
        #[prost(oneof = "field_reference::RootType", tags = "3, 4, 5")]
        pub root_type: ::core::option::Option<field_reference::RootType>,
    }
    /// Nested message and enum types in `FieldReference`.
    pub mod field_reference {
        /// Singleton that expresses this FieldReference is rooted off the root
        /// incoming record type
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct RootReference {}
        /// A root reference for the outer relation's subquery
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct OuterReference {
            /// number of subquery boundaries to traverse up for this field's reference
            ///
            /// This value must be >= 1
            #[prost(uint32, tag = "1")]
            pub steps_out: u32,
        }
        /// Whether this is composed of a single element reference or a masked
        /// element subtree
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ReferenceType {
            #[prost(message, tag = "1")]
            DirectReference(super::ReferenceSegment),
            #[prost(message, tag = "2")]
            MaskedReference(super::MaskExpression),
        }
        /// Whether this reference has an origin of a root struct or is based on the
        /// ouput of an expression. When this is a RootReference and direct_reference
        /// above is used, the direct_reference must be of a type StructField.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum RootType {
            #[prost(message, tag = "3")]
            Expression(::prost::alloc::boxed::Box<super::super::Expression>),
            #[prost(message, tag = "4")]
            RootReference(RootReference),
            #[prost(message, tag = "5")]
            OuterReference(OuterReference),
        }
    }
    /// Subquery relation expression
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Subquery {
        #[prost(oneof = "subquery::SubqueryType", tags = "1, 2, 3, 4")]
        pub subquery_type: ::core::option::Option<subquery::SubqueryType>,
    }
    /// Nested message and enum types in `Subquery`.
    pub mod subquery {
        /// A subquery with one row and one column. This is often an aggregate
        /// though not required to be.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Scalar {
            #[prost(message, optional, boxed, tag = "1")]
            pub input: ::core::option::Option<
                ::prost::alloc::boxed::Box<super::super::Rel>,
            >,
        }
        /// Predicate checking that the left expression is contained in the right
        /// subquery
        ///
        /// Examples:
        ///
        /// x IN (SELECT * FROM t)
        /// (x, y) IN (SELECT a, b FROM t)
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct InPredicate {
            #[prost(message, repeated, tag = "1")]
            pub needles: ::prost::alloc::vec::Vec<super::super::Expression>,
            #[prost(message, optional, boxed, tag = "2")]
            pub haystack: ::core::option::Option<
                ::prost::alloc::boxed::Box<super::super::Rel>,
            >,
        }
        /// A predicate over a set of rows in the form of a subquery
        /// EXISTS and UNIQUE are common SQL forms of this operation.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SetPredicate {
            /// TODO: should allow expressions
            #[prost(enumeration = "set_predicate::PredicateOp", tag = "1")]
            pub predicate_op: i32,
            #[prost(message, optional, boxed, tag = "2")]
            pub tuples: ::core::option::Option<
                ::prost::alloc::boxed::Box<super::super::Rel>,
            >,
        }
        /// Nested message and enum types in `SetPredicate`.
        pub mod set_predicate {
            #[derive(
                Clone,
                Copy,
                Debug,
                PartialEq,
                Eq,
                Hash,
                PartialOrd,
                Ord,
                ::prost::Enumeration
            )]
            #[repr(i32)]
            pub enum PredicateOp {
                Unspecified = 0,
                Exists = 1,
                Unique = 2,
            }
            impl PredicateOp {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        PredicateOp::Unspecified => "PREDICATE_OP_UNSPECIFIED",
                        PredicateOp::Exists => "PREDICATE_OP_EXISTS",
                        PredicateOp::Unique => "PREDICATE_OP_UNIQUE",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "PREDICATE_OP_UNSPECIFIED" => Some(Self::Unspecified),
                        "PREDICATE_OP_EXISTS" => Some(Self::Exists),
                        "PREDICATE_OP_UNIQUE" => Some(Self::Unique),
                        _ => None,
                    }
                }
            }
        }
        /// A subquery comparison using ANY or ALL.
        /// Examples:
        ///
        /// SELECT *
        /// FROM t1
        /// WHERE x < ANY(SELECT y from t2)
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SetComparison {
            /// ANY or ALL
            #[prost(enumeration = "set_comparison::ReductionOp", tag = "1")]
            pub reduction_op: i32,
            /// A comparison operator
            #[prost(enumeration = "set_comparison::ComparisonOp", tag = "2")]
            pub comparison_op: i32,
            /// left side of the expression
            #[prost(message, optional, boxed, tag = "3")]
            pub left: ::core::option::Option<
                ::prost::alloc::boxed::Box<super::super::Expression>,
            >,
            /// right side of the expression
            #[prost(message, optional, boxed, tag = "4")]
            pub right: ::core::option::Option<
                ::prost::alloc::boxed::Box<super::super::Rel>,
            >,
        }
        /// Nested message and enum types in `SetComparison`.
        pub mod set_comparison {
            #[derive(
                Clone,
                Copy,
                Debug,
                PartialEq,
                Eq,
                Hash,
                PartialOrd,
                Ord,
                ::prost::Enumeration
            )]
            #[repr(i32)]
            pub enum ComparisonOp {
                Unspecified = 0,
                Eq = 1,
                Ne = 2,
                Lt = 3,
                Gt = 4,
                Le = 5,
                Ge = 6,
            }
            impl ComparisonOp {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        ComparisonOp::Unspecified => "COMPARISON_OP_UNSPECIFIED",
                        ComparisonOp::Eq => "COMPARISON_OP_EQ",
                        ComparisonOp::Ne => "COMPARISON_OP_NE",
                        ComparisonOp::Lt => "COMPARISON_OP_LT",
                        ComparisonOp::Gt => "COMPARISON_OP_GT",
                        ComparisonOp::Le => "COMPARISON_OP_LE",
                        ComparisonOp::Ge => "COMPARISON_OP_GE",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "COMPARISON_OP_UNSPECIFIED" => Some(Self::Unspecified),
                        "COMPARISON_OP_EQ" => Some(Self::Eq),
                        "COMPARISON_OP_NE" => Some(Self::Ne),
                        "COMPARISON_OP_LT" => Some(Self::Lt),
                        "COMPARISON_OP_GT" => Some(Self::Gt),
                        "COMPARISON_OP_LE" => Some(Self::Le),
                        "COMPARISON_OP_GE" => Some(Self::Ge),
                        _ => None,
                    }
                }
            }
            #[derive(
                Clone,
                Copy,
                Debug,
                PartialEq,
                Eq,
                Hash,
                PartialOrd,
                Ord,
                ::prost::Enumeration
            )]
            #[repr(i32)]
            pub enum ReductionOp {
                Unspecified = 0,
                Any = 1,
                All = 2,
            }
            impl ReductionOp {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        ReductionOp::Unspecified => "REDUCTION_OP_UNSPECIFIED",
                        ReductionOp::Any => "REDUCTION_OP_ANY",
                        ReductionOp::All => "REDUCTION_OP_ALL",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "REDUCTION_OP_UNSPECIFIED" => Some(Self::Unspecified),
                        "REDUCTION_OP_ANY" => Some(Self::Any),
                        "REDUCTION_OP_ALL" => Some(Self::All),
                        _ => None,
                    }
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum SubqueryType {
            /// Scalar subquery
            #[prost(message, tag = "1")]
            Scalar(::prost::alloc::boxed::Box<Scalar>),
            /// x IN y predicate
            #[prost(message, tag = "2")]
            InPredicate(::prost::alloc::boxed::Box<InPredicate>),
            /// EXISTS/UNIQUE predicate
            #[prost(message, tag = "3")]
            SetPredicate(::prost::alloc::boxed::Box<SetPredicate>),
            /// ANY/ALL predicate
            #[prost(message, tag = "4")]
            SetComparison(::prost::alloc::boxed::Box<SetComparison>),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RexType {
        #[prost(message, tag = "1")]
        Literal(Literal),
        #[prost(message, tag = "2")]
        Selection(::prost::alloc::boxed::Box<FieldReference>),
        #[prost(message, tag = "3")]
        ScalarFunction(ScalarFunction),
        #[prost(message, tag = "5")]
        WindowFunction(WindowFunction),
        #[prost(message, tag = "6")]
        IfThen(::prost::alloc::boxed::Box<IfThen>),
        #[prost(message, tag = "7")]
        SwitchExpression(::prost::alloc::boxed::Box<SwitchExpression>),
        #[prost(message, tag = "8")]
        SingularOrList(::prost::alloc::boxed::Box<SingularOrList>),
        #[prost(message, tag = "9")]
        MultiOrList(MultiOrList),
        #[prost(message, tag = "11")]
        Cast(::prost::alloc::boxed::Box<Cast>),
        #[prost(message, tag = "12")]
        Subquery(::prost::alloc::boxed::Box<Subquery>),
        #[prost(message, tag = "13")]
        Nested(Nested),
        /// deprecated: enum literals are only sensible in the context of
        /// function arguments, for which FunctionArgument should now be
        /// used
        #[prost(message, tag = "10")]
        Enum(Enum),
    }
}
/// The description of a field to sort on (including the direction of sorting and null semantics)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SortField {
    #[prost(message, optional, tag = "1")]
    pub expr: ::core::option::Option<Expression>,
    #[prost(oneof = "sort_field::SortKind", tags = "2, 3")]
    pub sort_kind: ::core::option::Option<sort_field::SortKind>,
}
/// Nested message and enum types in `SortField`.
pub mod sort_field {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum SortDirection {
        Unspecified = 0,
        AscNullsFirst = 1,
        AscNullsLast = 2,
        DescNullsFirst = 3,
        DescNullsLast = 4,
        Clustered = 5,
    }
    impl SortDirection {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SortDirection::Unspecified => "SORT_DIRECTION_UNSPECIFIED",
                SortDirection::AscNullsFirst => "SORT_DIRECTION_ASC_NULLS_FIRST",
                SortDirection::AscNullsLast => "SORT_DIRECTION_ASC_NULLS_LAST",
                SortDirection::DescNullsFirst => "SORT_DIRECTION_DESC_NULLS_FIRST",
                SortDirection::DescNullsLast => "SORT_DIRECTION_DESC_NULLS_LAST",
                SortDirection::Clustered => "SORT_DIRECTION_CLUSTERED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SORT_DIRECTION_UNSPECIFIED" => Some(Self::Unspecified),
                "SORT_DIRECTION_ASC_NULLS_FIRST" => Some(Self::AscNullsFirst),
                "SORT_DIRECTION_ASC_NULLS_LAST" => Some(Self::AscNullsLast),
                "SORT_DIRECTION_DESC_NULLS_FIRST" => Some(Self::DescNullsFirst),
                "SORT_DIRECTION_DESC_NULLS_LAST" => Some(Self::DescNullsLast),
                "SORT_DIRECTION_CLUSTERED" => Some(Self::Clustered),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SortKind {
        #[prost(enumeration = "SortDirection", tag = "2")]
        Direction(i32),
        #[prost(uint32, tag = "3")]
        ComparisonFunctionReference(u32),
    }
}
/// An aggregate function.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregateFunction {
    /// Points to a function_anchor defined in this plan, which must refer
    /// to an aggregate function in the associated YAML file. Required; 0 is
    /// considered to be a valid anchor/reference.
    #[prost(uint32, tag = "1")]
    pub function_reference: u32,
    /// The arguments to be bound to the function. This must have exactly the
    /// number of arguments specified in the function definition, and the
    /// argument types must also match exactly:
    ///
    ///   - Value arguments must be bound using FunctionArgument.value, and
    ///     the expression in that must yield a value of a type that a function
    ///     overload is defined for.
    ///   - Type arguments must be bound using FunctionArgument.type, and a
    ///     function overload must be defined for that type.
    ///   - Enum arguments must be bound using FunctionArgument.enum
    ///     followed by Enum.specified, with a string that case-insensitively
    ///     matches one of the allowed options.
    ///   - Optional enum arguments must be bound using FunctionArgument.enum
    ///     followed by either Enum.specified or Enum.unspecified. If specified,
    ///     the string must case-insensitively match one of the allowed options.
    #[prost(message, repeated, tag = "7")]
    pub arguments: ::prost::alloc::vec::Vec<FunctionArgument>,
    /// Options to specify behavior for corner cases, or leave behavior
    /// unspecified if the consumer does not need specific behavior in these
    /// cases.
    #[prost(message, repeated, tag = "8")]
    pub options: ::prost::alloc::vec::Vec<FunctionOption>,
    /// Must be set to the return type of the function, exactly as derived
    /// using the declaration in the extension.
    #[prost(message, optional, tag = "5")]
    pub output_type: ::core::option::Option<Type>,
    /// Describes which part of the aggregation to perform within the context of
    /// distributed algorithms. Required. Must be set to INITIAL_TO_RESULT for
    /// aggregate functions that are not decomposable.
    #[prost(enumeration = "AggregationPhase", tag = "4")]
    pub phase: i32,
    /// If specified, the aggregated records are ordered according to this list
    /// before they are aggregated. The first sort field has the highest
    /// priority; only if a sort field determines two records to be equivalent is
    /// the next field queried. This field is optional.
    #[prost(message, repeated, tag = "3")]
    pub sorts: ::prost::alloc::vec::Vec<SortField>,
    /// Specifies whether equivalent records are merged before being aggregated.
    /// Optional, defaults to AGGREGATION_INVOCATION_ALL.
    #[prost(enumeration = "aggregate_function::AggregationInvocation", tag = "6")]
    pub invocation: i32,
    /// deprecated; use arguments instead
    #[deprecated]
    #[prost(message, repeated, tag = "2")]
    pub args: ::prost::alloc::vec::Vec<Expression>,
}
/// Nested message and enum types in `AggregateFunction`.
pub mod aggregate_function {
    /// Method in which equivalent records are merged before being aggregated.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum AggregationInvocation {
        /// This default value implies AGGREGATION_INVOCATION_ALL.
        Unspecified = 0,
        /// Use all values in the aggregation calculation.
        All = 1,
        /// Use only distinct values in the aggregation calculation.
        Distinct = 2,
    }
    impl AggregationInvocation {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AggregationInvocation::Unspecified => {
                    "AGGREGATION_INVOCATION_UNSPECIFIED"
                }
                AggregationInvocation::All => "AGGREGATION_INVOCATION_ALL",
                AggregationInvocation::Distinct => "AGGREGATION_INVOCATION_DISTINCT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "AGGREGATION_INVOCATION_UNSPECIFIED" => Some(Self::Unspecified),
                "AGGREGATION_INVOCATION_ALL" => Some(Self::All),
                "AGGREGATION_INVOCATION_DISTINCT" => Some(Self::Distinct),
                _ => None,
            }
        }
    }
}
/// This rel is used  to create references,
/// in case we refer to a RelRoot field names will be ignored
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReferenceRel {
    #[prost(int32, tag = "1")]
    pub subtree_ordinal: i32,
}
/// Describes which part of an aggregation or window function to perform within
/// the context of distributed algorithms.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AggregationPhase {
    /// Implies `INTERMEDIATE_TO_RESULT`.
    Unspecified = 0,
    /// Specifies that the function should be run only up to the point of
    /// generating an intermediate value, to be further aggregated later using
    /// INTERMEDIATE_TO_INTERMEDIATE or INTERMEDIATE_TO_RESULT.
    InitialToIntermediate = 1,
    /// Specifies that the inputs of the aggregate or window function are the
    /// intermediate values of the function, and that the output should also be
    /// an intermediate value, to be further aggregated later using
    /// INTERMEDIATE_TO_INTERMEDIATE or INTERMEDIATE_TO_RESULT.
    IntermediateToIntermediate = 2,
    /// A complete invocation: the function should aggregate the given set of
    /// inputs to yield a single return value. This style must be used for
    /// aggregate or window functions that are not decomposable.
    InitialToResult = 3,
    /// Specifies that the inputs of the aggregate or window function are the
    /// intermediate values of the function, generated previously using
    /// INITIAL_TO_INTERMEDIATE and possibly INTERMEDIATE_TO_INTERMEDIATE calls.
    /// This call should combine the intermediate values to yield the final
    /// return value.
    IntermediateToResult = 4,
}
impl AggregationPhase {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AggregationPhase::Unspecified => "AGGREGATION_PHASE_UNSPECIFIED",
            AggregationPhase::InitialToIntermediate => {
                "AGGREGATION_PHASE_INITIAL_TO_INTERMEDIATE"
            }
            AggregationPhase::IntermediateToIntermediate => {
                "AGGREGATION_PHASE_INTERMEDIATE_TO_INTERMEDIATE"
            }
            AggregationPhase::InitialToResult => "AGGREGATION_PHASE_INITIAL_TO_RESULT",
            AggregationPhase::IntermediateToResult => {
                "AGGREGATION_PHASE_INTERMEDIATE_TO_RESULT"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AGGREGATION_PHASE_UNSPECIFIED" => Some(Self::Unspecified),
            "AGGREGATION_PHASE_INITIAL_TO_INTERMEDIATE" => {
                Some(Self::InitialToIntermediate)
            }
            "AGGREGATION_PHASE_INTERMEDIATE_TO_INTERMEDIATE" => {
                Some(Self::IntermediateToIntermediate)
            }
            "AGGREGATION_PHASE_INITIAL_TO_RESULT" => Some(Self::InitialToResult),
            "AGGREGATION_PHASE_INTERMEDIATE_TO_RESULT" => {
                Some(Self::IntermediateToResult)
            }
            _ => None,
        }
    }
}
/// Either a relation or root relation
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanRel {
    #[prost(oneof = "plan_rel::RelType", tags = "1, 2")]
    pub rel_type: ::core::option::Option<plan_rel::RelType>,
}
/// Nested message and enum types in `PlanRel`.
pub mod plan_rel {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RelType {
        /// Any relation (used for references and CTEs)
        #[prost(message, tag = "1")]
        Rel(super::Rel),
        /// The root of a relation tree
        #[prost(message, tag = "2")]
        Root(super::RelRoot),
    }
}
/// Describe a set of operations to complete.
/// For compactness sake, identifiers are normalized at the plan level.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Plan {
    /// Substrait version of the plan. Optional up to 0.17.0, required for later
    /// versions.
    #[prost(message, optional, tag = "6")]
    pub version: ::core::option::Option<Version>,
    /// a list of yaml specifications this plan may depend on
    #[prost(message, repeated, tag = "1")]
    pub extension_uris: ::prost::alloc::vec::Vec<extensions::SimpleExtensionUri>,
    /// a list of extensions this plan may depend on
    #[prost(message, repeated, tag = "2")]
    pub extensions: ::prost::alloc::vec::Vec<extensions::SimpleExtensionDeclaration>,
    /// one or more relation trees that are associated with this plan.
    #[prost(message, repeated, tag = "3")]
    pub relations: ::prost::alloc::vec::Vec<PlanRel>,
    /// additional extensions associated with this plan.
    #[prost(message, optional, tag = "4")]
    pub advanced_extensions: ::core::option::Option<extensions::AdvancedExtension>,
    /// A list of com.google.Any entities that this plan may use. Can be used to
    /// warn if some embedded message types are unknown. Note that this list may
    /// include message types that are ignorable (optimizations) or that are
    /// unused. In many cases, a consumer may be able to work with a plan even if
    /// one or more message types defined here are unknown.
    #[prost(string, repeated, tag = "5")]
    pub expected_type_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// This message type can be used to deserialize only the version of a Substrait
/// Plan message. This prevents deserialization errors when there were breaking
/// changes between the Substrait version of the tool that produced the plan and
/// the Substrait version used to deserialize it, such that a consumer can emit
/// a more helpful error message in this case.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanVersion {
    #[prost(message, optional, tag = "6")]
    pub version: ::core::option::Option<Version>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Version {
    /// Substrait version number.
    #[prost(uint32, tag = "1")]
    pub major_number: u32,
    #[prost(uint32, tag = "2")]
    pub minor_number: u32,
    #[prost(uint32, tag = "3")]
    pub patch_number: u32,
    /// If a particular version of Substrait is used that does not correspond to
    /// a version number exactly (for example when using an unofficial fork or
    /// using a version that is not yet released or is between versions), set this
    /// to the full git hash of the utilized commit of
    /// <https://github.com/substrait-io/substrait> (or fork thereof), represented
    /// using a lowercase hex ASCII string 40 characters in length. The version
    /// number above should be set to the most recent version tag in the history
    /// of that commit.
    #[prost(string, tag = "4")]
    pub git_hash: ::prost::alloc::string::String,
    /// Identifying information for the producer that created this plan. Under
    /// ideal circumstances, consumers should not need this information. However,
    /// it is foreseen that consumers may need to work around bugs in particular
    /// producers in practice, and therefore may need to know which producer
    /// created the plan.
    #[prost(string, tag = "5")]
    pub producer: ::prost::alloc::string::String,
}
