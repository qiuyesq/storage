// This file is generated by rust-protobuf 2.8.2. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `proto/configuration/jwt/jwt.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_8_2;

#[derive(PartialEq,Clone,Default)]
pub struct AuthorizationHeaderParserConfiguration {
    // message fields
    pub maximum_cache_size: i32,
    pub cache_replacement_policy: super::eviction::CacheReplacementPolicy,
    pub claims_validation_jmespath_expression: ::std::string::String,
    pub metadata_extraction_jmespath_expression: ::std::string::String,
    // message oneof groups
    pub key: ::std::option::Option<AuthorizationHeaderParserConfiguration_oneof_key>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a AuthorizationHeaderParserConfiguration {
    fn default() -> &'a AuthorizationHeaderParserConfiguration {
        <AuthorizationHeaderParserConfiguration as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum AuthorizationHeaderParserConfiguration_oneof_key {
    hmac_key(::std::vec::Vec<u8>),
    public_key(::std::string::String),
}

impl AuthorizationHeaderParserConfiguration {
    pub fn new() -> AuthorizationHeaderParserConfiguration {
        ::std::default::Default::default()
    }

    // bytes hmac_key = 1;


    pub fn get_hmac_key(&self) -> &[u8] {
        match self.key {
            ::std::option::Option::Some(AuthorizationHeaderParserConfiguration_oneof_key::hmac_key(ref v)) => v,
            _ => &[],
        }
    }
    pub fn clear_hmac_key(&mut self) {
        self.key = ::std::option::Option::None;
    }

    pub fn has_hmac_key(&self) -> bool {
        match self.key {
            ::std::option::Option::Some(AuthorizationHeaderParserConfiguration_oneof_key::hmac_key(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_hmac_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::std::option::Option::Some(AuthorizationHeaderParserConfiguration_oneof_key::hmac_key(v))
    }

    // Mutable pointer to the field.
    pub fn mut_hmac_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(AuthorizationHeaderParserConfiguration_oneof_key::hmac_key(_)) = self.key {
        } else {
            self.key = ::std::option::Option::Some(AuthorizationHeaderParserConfiguration_oneof_key::hmac_key(::std::vec::Vec::new()));
        }
        match self.key {
            ::std::option::Option::Some(AuthorizationHeaderParserConfiguration_oneof_key::hmac_key(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_hmac_key(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_hmac_key() {
            match self.key.take() {
                ::std::option::Option::Some(AuthorizationHeaderParserConfiguration_oneof_key::hmac_key(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    // string public_key = 2;


    pub fn get_public_key(&self) -> &str {
        match self.key {
            ::std::option::Option::Some(AuthorizationHeaderParserConfiguration_oneof_key::public_key(ref v)) => v,
            _ => "",
        }
    }
    pub fn clear_public_key(&mut self) {
        self.key = ::std::option::Option::None;
    }

    pub fn has_public_key(&self) -> bool {
        match self.key {
            ::std::option::Option::Some(AuthorizationHeaderParserConfiguration_oneof_key::public_key(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_public_key(&mut self, v: ::std::string::String) {
        self.key = ::std::option::Option::Some(AuthorizationHeaderParserConfiguration_oneof_key::public_key(v))
    }

    // Mutable pointer to the field.
    pub fn mut_public_key(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(AuthorizationHeaderParserConfiguration_oneof_key::public_key(_)) = self.key {
        } else {
            self.key = ::std::option::Option::Some(AuthorizationHeaderParserConfiguration_oneof_key::public_key(::std::string::String::new()));
        }
        match self.key {
            ::std::option::Option::Some(AuthorizationHeaderParserConfiguration_oneof_key::public_key(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_public_key(&mut self) -> ::std::string::String {
        if self.has_public_key() {
            match self.key.take() {
                ::std::option::Option::Some(AuthorizationHeaderParserConfiguration_oneof_key::public_key(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    // int32 maximum_cache_size = 3;


    pub fn get_maximum_cache_size(&self) -> i32 {
        self.maximum_cache_size
    }
    pub fn clear_maximum_cache_size(&mut self) {
        self.maximum_cache_size = 0;
    }

    // Param is passed by value, moved
    pub fn set_maximum_cache_size(&mut self, v: i32) {
        self.maximum_cache_size = v;
    }

    // .eviction.CacheReplacementPolicy cache_replacement_policy = 4;


    pub fn get_cache_replacement_policy(&self) -> super::eviction::CacheReplacementPolicy {
        self.cache_replacement_policy
    }
    pub fn clear_cache_replacement_policy(&mut self) {
        self.cache_replacement_policy = super::eviction::CacheReplacementPolicy::UNKNOWN;
    }

    // Param is passed by value, moved
    pub fn set_cache_replacement_policy(&mut self, v: super::eviction::CacheReplacementPolicy) {
        self.cache_replacement_policy = v;
    }

    // string claims_validation_jmespath_expression = 5;


    pub fn get_claims_validation_jmespath_expression(&self) -> &str {
        &self.claims_validation_jmespath_expression
    }
    pub fn clear_claims_validation_jmespath_expression(&mut self) {
        self.claims_validation_jmespath_expression.clear();
    }

    // Param is passed by value, moved
    pub fn set_claims_validation_jmespath_expression(&mut self, v: ::std::string::String) {
        self.claims_validation_jmespath_expression = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_claims_validation_jmespath_expression(&mut self) -> &mut ::std::string::String {
        &mut self.claims_validation_jmespath_expression
    }

    // Take field
    pub fn take_claims_validation_jmespath_expression(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.claims_validation_jmespath_expression, ::std::string::String::new())
    }

    // string metadata_extraction_jmespath_expression = 6;


    pub fn get_metadata_extraction_jmespath_expression(&self) -> &str {
        &self.metadata_extraction_jmespath_expression
    }
    pub fn clear_metadata_extraction_jmespath_expression(&mut self) {
        self.metadata_extraction_jmespath_expression.clear();
    }

    // Param is passed by value, moved
    pub fn set_metadata_extraction_jmespath_expression(&mut self, v: ::std::string::String) {
        self.metadata_extraction_jmespath_expression = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metadata_extraction_jmespath_expression(&mut self) -> &mut ::std::string::String {
        &mut self.metadata_extraction_jmespath_expression
    }

    // Take field
    pub fn take_metadata_extraction_jmespath_expression(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.metadata_extraction_jmespath_expression, ::std::string::String::new())
    }
}

impl ::protobuf::Message for AuthorizationHeaderParserConfiguration {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.key = ::std::option::Option::Some(AuthorizationHeaderParserConfiguration_oneof_key::hmac_key(is.read_bytes()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.key = ::std::option::Option::Some(AuthorizationHeaderParserConfiguration_oneof_key::public_key(is.read_string()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.maximum_cache_size = tmp;
                },
                4 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.cache_replacement_policy, 4, &mut self.unknown_fields)?
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.claims_validation_jmespath_expression)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.metadata_extraction_jmespath_expression)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.maximum_cache_size != 0 {
            my_size += ::protobuf::rt::value_size(3, self.maximum_cache_size, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.cache_replacement_policy != super::eviction::CacheReplacementPolicy::UNKNOWN {
            my_size += ::protobuf::rt::enum_size(4, self.cache_replacement_policy);
        }
        if !self.claims_validation_jmespath_expression.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.claims_validation_jmespath_expression);
        }
        if !self.metadata_extraction_jmespath_expression.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.metadata_extraction_jmespath_expression);
        }
        if let ::std::option::Option::Some(ref v) = self.key {
            match v {
                &AuthorizationHeaderParserConfiguration_oneof_key::hmac_key(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(1, &v);
                },
                &AuthorizationHeaderParserConfiguration_oneof_key::public_key(ref v) => {
                    my_size += ::protobuf::rt::string_size(2, &v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.maximum_cache_size != 0 {
            os.write_int32(3, self.maximum_cache_size)?;
        }
        if self.cache_replacement_policy != super::eviction::CacheReplacementPolicy::UNKNOWN {
            os.write_enum(4, self.cache_replacement_policy.value())?;
        }
        if !self.claims_validation_jmespath_expression.is_empty() {
            os.write_string(5, &self.claims_validation_jmespath_expression)?;
        }
        if !self.metadata_extraction_jmespath_expression.is_empty() {
            os.write_string(6, &self.metadata_extraction_jmespath_expression)?;
        }
        if let ::std::option::Option::Some(ref v) = self.key {
            match v {
                &AuthorizationHeaderParserConfiguration_oneof_key::hmac_key(ref v) => {
                    os.write_bytes(1, v)?;
                },
                &AuthorizationHeaderParserConfiguration_oneof_key::public_key(ref v) => {
                    os.write_string(2, v)?;
                },
            };
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> AuthorizationHeaderParserConfiguration {
        AuthorizationHeaderParserConfiguration::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor::<_>(
                    "hmac_key",
                    AuthorizationHeaderParserConfiguration::has_hmac_key,
                    AuthorizationHeaderParserConfiguration::get_hmac_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "public_key",
                    AuthorizationHeaderParserConfiguration::has_public_key,
                    AuthorizationHeaderParserConfiguration::get_public_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "maximum_cache_size",
                    |m: &AuthorizationHeaderParserConfiguration| { &m.maximum_cache_size },
                    |m: &mut AuthorizationHeaderParserConfiguration| { &mut m.maximum_cache_size },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::eviction::CacheReplacementPolicy>>(
                    "cache_replacement_policy",
                    |m: &AuthorizationHeaderParserConfiguration| { &m.cache_replacement_policy },
                    |m: &mut AuthorizationHeaderParserConfiguration| { &mut m.cache_replacement_policy },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "claims_validation_jmespath_expression",
                    |m: &AuthorizationHeaderParserConfiguration| { &m.claims_validation_jmespath_expression },
                    |m: &mut AuthorizationHeaderParserConfiguration| { &mut m.claims_validation_jmespath_expression },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "metadata_extraction_jmespath_expression",
                    |m: &AuthorizationHeaderParserConfiguration| { &m.metadata_extraction_jmespath_expression },
                    |m: &mut AuthorizationHeaderParserConfiguration| { &mut m.metadata_extraction_jmespath_expression },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AuthorizationHeaderParserConfiguration>(
                    "AuthorizationHeaderParserConfiguration",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static AuthorizationHeaderParserConfiguration {
        static mut instance: ::protobuf::lazy::Lazy<AuthorizationHeaderParserConfiguration> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AuthorizationHeaderParserConfiguration,
        };
        unsafe {
            instance.get(AuthorizationHeaderParserConfiguration::new)
        }
    }
}

impl ::protobuf::Clear for AuthorizationHeaderParserConfiguration {
    fn clear(&mut self) {
        self.key = ::std::option::Option::None;
        self.key = ::std::option::Option::None;
        self.maximum_cache_size = 0;
        self.cache_replacement_policy = super::eviction::CacheReplacementPolicy::UNKNOWN;
        self.claims_validation_jmespath_expression.clear();
        self.metadata_extraction_jmespath_expression.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AuthorizationHeaderParserConfiguration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthorizationHeaderParserConfiguration {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n!proto/configuration/jwt/jwt.proto\x12\x03jwt\x1a+proto/configuration/\
    eviction/eviction.proto\"\xa1\x03\n&AuthorizationHeaderParserConfigurati\
    on\x12\x1b\n\x08hmac_key\x18\x01\x20\x01(\x0cH\0R\x07hmacKey\x12\x1f\n\n\
    public_key\x18\x02\x20\x01(\tH\0R\tpublicKey\x12,\n\x12maximum_cache_siz\
    e\x18\x03\x20\x01(\x05R\x10maximumCacheSize\x12Z\n\x18cache_replacement_\
    policy\x18\x04\x20\x01(\x0e2\x20.eviction.CacheReplacementPolicyR\x16cac\
    heReplacementPolicy\x12Q\n%claims_validation_jmespath_expression\x18\x05\
    \x20\x01(\tR\"claimsValidationJmespathExpression\x12U\n'metadata_extract\
    ion_jmespath_expression\x18\x06\x20\x01(\tR$metadataExtractionJmespathEx\
    pressionB\x05\n\x03keyb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
