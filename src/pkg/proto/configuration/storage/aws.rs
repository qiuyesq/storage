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
//! Generated file from `proto/configuration/cloud/aws/aws.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_8_2;

#[derive(PartialEq,Clone,Default)]
pub struct StaticCredentials {
    // message fields
    pub access_key_id: ::std::string::String,
    pub secret_access_key: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a StaticCredentials {
    fn default() -> &'a StaticCredentials {
        <StaticCredentials as ::protobuf::Message>::default_instance()
    }
}

impl StaticCredentials {
    pub fn new() -> StaticCredentials {
        ::std::default::Default::default()
    }

    // string access_key_id = 1;


    pub fn get_access_key_id(&self) -> &str {
        &self.access_key_id
    }
    pub fn clear_access_key_id(&mut self) {
        self.access_key_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_access_key_id(&mut self, v: ::std::string::String) {
        self.access_key_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_access_key_id(&mut self) -> &mut ::std::string::String {
        &mut self.access_key_id
    }

    // Take field
    pub fn take_access_key_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.access_key_id, ::std::string::String::new())
    }

    // string secret_access_key = 2;


    pub fn get_secret_access_key(&self) -> &str {
        &self.secret_access_key
    }
    pub fn clear_secret_access_key(&mut self) {
        self.secret_access_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_secret_access_key(&mut self, v: ::std::string::String) {
        self.secret_access_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_secret_access_key(&mut self) -> &mut ::std::string::String {
        &mut self.secret_access_key
    }

    // Take field
    pub fn take_secret_access_key(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.secret_access_key, ::std::string::String::new())
    }
}

impl ::protobuf::Message for StaticCredentials {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.access_key_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.secret_access_key)?;
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
        if !self.access_key_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.access_key_id);
        }
        if !self.secret_access_key.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.secret_access_key);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.access_key_id.is_empty() {
            os.write_string(1, &self.access_key_id)?;
        }
        if !self.secret_access_key.is_empty() {
            os.write_string(2, &self.secret_access_key)?;
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

    fn new() -> StaticCredentials {
        StaticCredentials::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "access_key_id",
                    |m: &StaticCredentials| { &m.access_key_id },
                    |m: &mut StaticCredentials| { &mut m.access_key_id },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "secret_access_key",
                    |m: &StaticCredentials| { &m.secret_access_key },
                    |m: &mut StaticCredentials| { &mut m.secret_access_key },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StaticCredentials>(
                    "StaticCredentials",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static StaticCredentials {
        static mut instance: ::protobuf::lazy::Lazy<StaticCredentials> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StaticCredentials,
        };
        unsafe {
            instance.get(StaticCredentials::new)
        }
    }
}

impl ::protobuf::Clear for StaticCredentials {
    fn clear(&mut self) {
        self.access_key_id.clear();
        self.secret_access_key.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StaticCredentials {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StaticCredentials {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct WebIdentityRoleCredentials {
    // message fields
    pub role_arn: ::std::string::String,
    pub token_file: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a WebIdentityRoleCredentials {
    fn default() -> &'a WebIdentityRoleCredentials {
        <WebIdentityRoleCredentials as ::protobuf::Message>::default_instance()
    }
}

impl WebIdentityRoleCredentials {
    pub fn new() -> WebIdentityRoleCredentials {
        ::std::default::Default::default()
    }

    // string role_arn = 1;


    pub fn get_role_arn(&self) -> &str {
        &self.role_arn
    }
    pub fn clear_role_arn(&mut self) {
        self.role_arn.clear();
    }

    // Param is passed by value, moved
    pub fn set_role_arn(&mut self, v: ::std::string::String) {
        self.role_arn = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_role_arn(&mut self) -> &mut ::std::string::String {
        &mut self.role_arn
    }

    // Take field
    pub fn take_role_arn(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.role_arn, ::std::string::String::new())
    }

    // string token_file = 2;


    pub fn get_token_file(&self) -> &str {
        &self.token_file
    }
    pub fn clear_token_file(&mut self) {
        self.token_file.clear();
    }

    // Param is passed by value, moved
    pub fn set_token_file(&mut self, v: ::std::string::String) {
        self.token_file = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token_file(&mut self) -> &mut ::std::string::String {
        &mut self.token_file
    }

    // Take field
    pub fn take_token_file(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.token_file, ::std::string::String::new())
    }
}

impl ::protobuf::Message for WebIdentityRoleCredentials {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.role_arn)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.token_file)?;
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
        if !self.role_arn.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.role_arn);
        }
        if !self.token_file.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.token_file);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.role_arn.is_empty() {
            os.write_string(1, &self.role_arn)?;
        }
        if !self.token_file.is_empty() {
            os.write_string(2, &self.token_file)?;
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

    fn new() -> WebIdentityRoleCredentials {
        WebIdentityRoleCredentials::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "role_arn",
                    |m: &WebIdentityRoleCredentials| { &m.role_arn },
                    |m: &mut WebIdentityRoleCredentials| { &mut m.role_arn },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "token_file",
                    |m: &WebIdentityRoleCredentials| { &m.token_file },
                    |m: &mut WebIdentityRoleCredentials| { &mut m.token_file },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WebIdentityRoleCredentials>(
                    "WebIdentityRoleCredentials",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static WebIdentityRoleCredentials {
        static mut instance: ::protobuf::lazy::Lazy<WebIdentityRoleCredentials> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WebIdentityRoleCredentials,
        };
        unsafe {
            instance.get(WebIdentityRoleCredentials::new)
        }
    }
}

impl ::protobuf::Clear for WebIdentityRoleCredentials {
    fn clear(&mut self) {
        self.role_arn.clear();
        self.token_file.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WebIdentityRoleCredentials {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WebIdentityRoleCredentials {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SessionConfiguration {
    // message fields
    pub region: ::std::string::String,
    pub http_client: ::protobuf::SingularPtrField<super::http::ClientConfiguration>,
    // message oneof groups
    pub credentials: ::std::option::Option<SessionConfiguration_oneof_credentials>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SessionConfiguration {
    fn default() -> &'a SessionConfiguration {
        <SessionConfiguration as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum SessionConfiguration_oneof_credentials {
    static_credentials(StaticCredentials),
    web_identity_role_credentials(WebIdentityRoleCredentials),
}

impl SessionConfiguration {
    pub fn new() -> SessionConfiguration {
        ::std::default::Default::default()
    }

    // string region = 2;


    pub fn get_region(&self) -> &str {
        &self.region
    }
    pub fn clear_region(&mut self) {
        self.region.clear();
    }

    // Param is passed by value, moved
    pub fn set_region(&mut self, v: ::std::string::String) {
        self.region = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region(&mut self) -> &mut ::std::string::String {
        &mut self.region
    }

    // Take field
    pub fn take_region(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.region, ::std::string::String::new())
    }

    // .cloud.aws.StaticCredentials static_credentials = 5;


    pub fn get_static_credentials(&self) -> &StaticCredentials {
        match self.credentials {
            ::std::option::Option::Some(SessionConfiguration_oneof_credentials::static_credentials(ref v)) => v,
            _ => StaticCredentials::default_instance(),
        }
    }
    pub fn clear_static_credentials(&mut self) {
        self.credentials = ::std::option::Option::None;
    }

    pub fn has_static_credentials(&self) -> bool {
        match self.credentials {
            ::std::option::Option::Some(SessionConfiguration_oneof_credentials::static_credentials(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_static_credentials(&mut self, v: StaticCredentials) {
        self.credentials = ::std::option::Option::Some(SessionConfiguration_oneof_credentials::static_credentials(v))
    }

    // Mutable pointer to the field.
    pub fn mut_static_credentials(&mut self) -> &mut StaticCredentials {
        if let ::std::option::Option::Some(SessionConfiguration_oneof_credentials::static_credentials(_)) = self.credentials {
        } else {
            self.credentials = ::std::option::Option::Some(SessionConfiguration_oneof_credentials::static_credentials(StaticCredentials::new()));
        }
        match self.credentials {
            ::std::option::Option::Some(SessionConfiguration_oneof_credentials::static_credentials(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_static_credentials(&mut self) -> StaticCredentials {
        if self.has_static_credentials() {
            match self.credentials.take() {
                ::std::option::Option::Some(SessionConfiguration_oneof_credentials::static_credentials(v)) => v,
                _ => panic!(),
            }
        } else {
            StaticCredentials::new()
        }
    }

    // .cloud.aws.WebIdentityRoleCredentials web_identity_role_credentials = 7;


    pub fn get_web_identity_role_credentials(&self) -> &WebIdentityRoleCredentials {
        match self.credentials {
            ::std::option::Option::Some(SessionConfiguration_oneof_credentials::web_identity_role_credentials(ref v)) => v,
            _ => WebIdentityRoleCredentials::default_instance(),
        }
    }
    pub fn clear_web_identity_role_credentials(&mut self) {
        self.credentials = ::std::option::Option::None;
    }

    pub fn has_web_identity_role_credentials(&self) -> bool {
        match self.credentials {
            ::std::option::Option::Some(SessionConfiguration_oneof_credentials::web_identity_role_credentials(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_web_identity_role_credentials(&mut self, v: WebIdentityRoleCredentials) {
        self.credentials = ::std::option::Option::Some(SessionConfiguration_oneof_credentials::web_identity_role_credentials(v))
    }

    // Mutable pointer to the field.
    pub fn mut_web_identity_role_credentials(&mut self) -> &mut WebIdentityRoleCredentials {
        if let ::std::option::Option::Some(SessionConfiguration_oneof_credentials::web_identity_role_credentials(_)) = self.credentials {
        } else {
            self.credentials = ::std::option::Option::Some(SessionConfiguration_oneof_credentials::web_identity_role_credentials(WebIdentityRoleCredentials::new()));
        }
        match self.credentials {
            ::std::option::Option::Some(SessionConfiguration_oneof_credentials::web_identity_role_credentials(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_web_identity_role_credentials(&mut self) -> WebIdentityRoleCredentials {
        if self.has_web_identity_role_credentials() {
            match self.credentials.take() {
                ::std::option::Option::Some(SessionConfiguration_oneof_credentials::web_identity_role_credentials(v)) => v,
                _ => panic!(),
            }
        } else {
            WebIdentityRoleCredentials::new()
        }
    }

    // .http.ClientConfiguration http_client = 6;


    pub fn get_http_client(&self) -> &super::http::ClientConfiguration {
        self.http_client.as_ref().unwrap_or_else(|| super::http::ClientConfiguration::default_instance())
    }
    pub fn clear_http_client(&mut self) {
        self.http_client.clear();
    }

    pub fn has_http_client(&self) -> bool {
        self.http_client.is_some()
    }

    // Param is passed by value, moved
    pub fn set_http_client(&mut self, v: super::http::ClientConfiguration) {
        self.http_client = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_http_client(&mut self) -> &mut super::http::ClientConfiguration {
        if self.http_client.is_none() {
            self.http_client.set_default();
        }
        self.http_client.as_mut().unwrap()
    }

    // Take field
    pub fn take_http_client(&mut self) -> super::http::ClientConfiguration {
        self.http_client.take().unwrap_or_else(|| super::http::ClientConfiguration::new())
    }
}

impl ::protobuf::Message for SessionConfiguration {
    fn is_initialized(&self) -> bool {
        if let Some(SessionConfiguration_oneof_credentials::static_credentials(ref v)) = self.credentials {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(SessionConfiguration_oneof_credentials::web_identity_role_credentials(ref v)) = self.credentials {
            if !v.is_initialized() {
                return false;
            }
        }
        for v in &self.http_client {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.region)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.credentials = ::std::option::Option::Some(SessionConfiguration_oneof_credentials::static_credentials(is.read_message()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.credentials = ::std::option::Option::Some(SessionConfiguration_oneof_credentials::web_identity_role_credentials(is.read_message()?));
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.http_client)?;
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
        if !self.region.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.region);
        }
        if let Some(ref v) = self.http_client.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let ::std::option::Option::Some(ref v) = self.credentials {
            match v {
                &SessionConfiguration_oneof_credentials::static_credentials(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &SessionConfiguration_oneof_credentials::web_identity_role_credentials(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.region.is_empty() {
            os.write_string(2, &self.region)?;
        }
        if let Some(ref v) = self.http_client.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let ::std::option::Option::Some(ref v) = self.credentials {
            match v {
                &SessionConfiguration_oneof_credentials::static_credentials(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &SessionConfiguration_oneof_credentials::web_identity_role_credentials(ref v) => {
                    os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
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

    fn new() -> SessionConfiguration {
        SessionConfiguration::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "region",
                    |m: &SessionConfiguration| { &m.region },
                    |m: &mut SessionConfiguration| { &mut m.region },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, StaticCredentials>(
                    "static_credentials",
                    SessionConfiguration::has_static_credentials,
                    SessionConfiguration::get_static_credentials,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, WebIdentityRoleCredentials>(
                    "web_identity_role_credentials",
                    SessionConfiguration::has_web_identity_role_credentials,
                    SessionConfiguration::get_web_identity_role_credentials,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::http::ClientConfiguration>>(
                    "http_client",
                    |m: &SessionConfiguration| { &m.http_client },
                    |m: &mut SessionConfiguration| { &mut m.http_client },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SessionConfiguration>(
                    "SessionConfiguration",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static SessionConfiguration {
        static mut instance: ::protobuf::lazy::Lazy<SessionConfiguration> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SessionConfiguration,
        };
        unsafe {
            instance.get(SessionConfiguration::new)
        }
    }
}

impl ::protobuf::Clear for SessionConfiguration {
    fn clear(&mut self) {
        self.region.clear();
        self.credentials = ::std::option::Option::None;
        self.credentials = ::std::option::Option::None;
        self.http_client.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SessionConfiguration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SessionConfiguration {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'proto/configuration/cloud/aws/aws.proto\x12\tcloud.aws\x1a#proto/conf\
    iguration/http/http.proto\"c\n\x11StaticCredentials\x12\"\n\raccess_key_\
    id\x18\x01\x20\x01(\tR\x0baccessKeyId\x12*\n\x11secret_access_key\x18\
    \x02\x20\x01(\tR\x0fsecretAccessKey\"V\n\x1aWebIdentityRoleCredentials\
    \x12\x19\n\x08role_arn\x18\x01\x20\x01(\tR\x07roleArn\x12\x1d\n\ntoken_f\
    ile\x18\x02\x20\x01(\tR\ttokenFile\"\xc6\x02\n\x14SessionConfiguration\
    \x12\x16\n\x06region\x18\x02\x20\x01(\tR\x06region\x12M\n\x12static_cred\
    entials\x18\x05\x20\x01(\x0b2\x1c.cloud.aws.StaticCredentialsH\0R\x11sta\
    ticCredentials\x12j\n\x1dweb_identity_role_credentials\x18\x07\x20\x01(\
    \x0b2%.cloud.aws.WebIdentityRoleCredentialsH\0R\x1awebIdentityRoleCreden\
    tials\x12:\n\x0bhttp_client\x18\x06\x20\x01(\x0b2\x19.http.ClientConfigu\
    rationR\nhttpClientB\r\n\x0bcredentialsJ\x04\x08\x01\x10\x02J\x04\x08\
    \x03\x10\x04J\x04\x08\x04\x10\x05b\x06proto3\
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
