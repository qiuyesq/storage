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
//! Generated file from `proto/configuration/blockdevice/blockdevice.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_8_2;

#[derive(PartialEq,Clone,Default)]
pub struct FileConfiguration {
    // message fields
    pub path: ::std::string::String,
    pub size_bytes: i64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a FileConfiguration {
    fn default() -> &'a FileConfiguration {
        <FileConfiguration as ::protobuf::Message>::default_instance()
    }
}

impl FileConfiguration {
    pub fn new() -> FileConfiguration {
        ::std::default::Default::default()
    }

    // string path = 1;


    pub fn get_path(&self) -> &str {
        &self.path
    }
    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        &mut self.path
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.path, ::std::string::String::new())
    }

    // int64 size_bytes = 2;


    pub fn get_size_bytes(&self) -> i64 {
        self.size_bytes
    }
    pub fn clear_size_bytes(&mut self) {
        self.size_bytes = 0;
    }

    // Param is passed by value, moved
    pub fn set_size_bytes(&mut self, v: i64) {
        self.size_bytes = v;
    }
}

impl ::protobuf::Message for FileConfiguration {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.path)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.size_bytes = tmp;
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
        if !self.path.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.path);
        }
        if self.size_bytes != 0 {
            my_size += ::protobuf::rt::value_size(2, self.size_bytes, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.path.is_empty() {
            os.write_string(1, &self.path)?;
        }
        if self.size_bytes != 0 {
            os.write_int64(2, self.size_bytes)?;
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

    fn new() -> FileConfiguration {
        FileConfiguration::new()
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
                    "path",
                    |m: &FileConfiguration| { &m.path },
                    |m: &mut FileConfiguration| { &mut m.path },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "size_bytes",
                    |m: &FileConfiguration| { &m.size_bytes },
                    |m: &mut FileConfiguration| { &mut m.size_bytes },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FileConfiguration>(
                    "FileConfiguration",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static FileConfiguration {
        static mut instance: ::protobuf::lazy::Lazy<FileConfiguration> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FileConfiguration,
        };
        unsafe {
            instance.get(FileConfiguration::new)
        }
    }
}

impl ::protobuf::Clear for FileConfiguration {
    fn clear(&mut self) {
        self.path.clear();
        self.size_bytes = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FileConfiguration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FileConfiguration {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Configuration {
    // message oneof groups
    pub source: ::std::option::Option<Configuration_oneof_source>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Configuration {
    fn default() -> &'a Configuration {
        <Configuration as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum Configuration_oneof_source {
    device_path(::std::string::String),
    file(FileConfiguration),
}

impl Configuration {
    pub fn new() -> Configuration {
        ::std::default::Default::default()
    }

    // string device_path = 1;


    pub fn get_device_path(&self) -> &str {
        match self.source {
            ::std::option::Option::Some(Configuration_oneof_source::device_path(ref v)) => v,
            _ => "",
        }
    }
    pub fn clear_device_path(&mut self) {
        self.source = ::std::option::Option::None;
    }

    pub fn has_device_path(&self) -> bool {
        match self.source {
            ::std::option::Option::Some(Configuration_oneof_source::device_path(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_device_path(&mut self, v: ::std::string::String) {
        self.source = ::std::option::Option::Some(Configuration_oneof_source::device_path(v))
    }

    // Mutable pointer to the field.
    pub fn mut_device_path(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(Configuration_oneof_source::device_path(_)) = self.source {
        } else {
            self.source = ::std::option::Option::Some(Configuration_oneof_source::device_path(::std::string::String::new()));
        }
        match self.source {
            ::std::option::Option::Some(Configuration_oneof_source::device_path(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_device_path(&mut self) -> ::std::string::String {
        if self.has_device_path() {
            match self.source.take() {
                ::std::option::Option::Some(Configuration_oneof_source::device_path(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    // .blockdevice.FileConfiguration file = 2;


    pub fn get_file(&self) -> &FileConfiguration {
        match self.source {
            ::std::option::Option::Some(Configuration_oneof_source::file(ref v)) => v,
            _ => FileConfiguration::default_instance(),
        }
    }
    pub fn clear_file(&mut self) {
        self.source = ::std::option::Option::None;
    }

    pub fn has_file(&self) -> bool {
        match self.source {
            ::std::option::Option::Some(Configuration_oneof_source::file(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_file(&mut self, v: FileConfiguration) {
        self.source = ::std::option::Option::Some(Configuration_oneof_source::file(v))
    }

    // Mutable pointer to the field.
    pub fn mut_file(&mut self) -> &mut FileConfiguration {
        if let ::std::option::Option::Some(Configuration_oneof_source::file(_)) = self.source {
        } else {
            self.source = ::std::option::Option::Some(Configuration_oneof_source::file(FileConfiguration::new()));
        }
        match self.source {
            ::std::option::Option::Some(Configuration_oneof_source::file(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_file(&mut self) -> FileConfiguration {
        if self.has_file() {
            match self.source.take() {
                ::std::option::Option::Some(Configuration_oneof_source::file(v)) => v,
                _ => panic!(),
            }
        } else {
            FileConfiguration::new()
        }
    }
}

impl ::protobuf::Message for Configuration {
    fn is_initialized(&self) -> bool {
        if let Some(Configuration_oneof_source::file(ref v)) = self.source {
            if !v.is_initialized() {
                return false;
            }
        }
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
                    self.source = ::std::option::Option::Some(Configuration_oneof_source::device_path(is.read_string()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.source = ::std::option::Option::Some(Configuration_oneof_source::file(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.source {
            match v {
                &Configuration_oneof_source::device_path(ref v) => {
                    my_size += ::protobuf::rt::string_size(1, &v);
                },
                &Configuration_oneof_source::file(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.source {
            match v {
                &Configuration_oneof_source::device_path(ref v) => {
                    os.write_string(1, v)?;
                },
                &Configuration_oneof_source::file(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> Configuration {
        Configuration::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "device_path",
                    Configuration::has_device_path,
                    Configuration::get_device_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, FileConfiguration>(
                    "file",
                    Configuration::has_file,
                    Configuration::get_file,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Configuration>(
                    "Configuration",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Configuration {
        static mut instance: ::protobuf::lazy::Lazy<Configuration> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Configuration,
        };
        unsafe {
            instance.get(Configuration::new)
        }
    }
}

impl ::protobuf::Clear for Configuration {
    fn clear(&mut self) {
        self.source = ::std::option::Option::None;
        self.source = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Configuration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Configuration {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n1proto/configuration/blockdevice/blockdevice.proto\x12\x0bblockdevice\
    \"F\n\x11FileConfiguration\x12\x12\n\x04path\x18\x01\x20\x01(\tR\x04path\
    \x12\x1d\n\nsize_bytes\x18\x02\x20\x01(\x03R\tsizeBytes\"r\n\rConfigurat\
    ion\x12!\n\x0bdevice_path\x18\x01\x20\x01(\tH\0R\ndevicePath\x124\n\x04f\
    ile\x18\x02\x20\x01(\x0b2\x1e.blockdevice.FileConfigurationH\0R\x04fileB\
    \x08\n\x06sourceb\x06proto3\
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
