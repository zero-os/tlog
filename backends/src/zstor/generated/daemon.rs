// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

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

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Permission {
    // message fields
    pub read: bool,
    pub write: bool,
    pub delete: bool,
    pub admin: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Permission {}

impl Permission {
    pub fn new() -> Permission {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Permission {
        static mut instance: ::protobuf::lazy::Lazy<Permission> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Permission,
        };
        unsafe {
            instance.get(Permission::new)
        }
    }

    // bool read = 1;

    pub fn clear_read(&mut self) {
        self.read = false;
    }

    // Param is passed by value, moved
    pub fn set_read(&mut self, v: bool) {
        self.read = v;
    }

    pub fn get_read(&self) -> bool {
        self.read
    }

    fn get_read_for_reflect(&self) -> &bool {
        &self.read
    }

    fn mut_read_for_reflect(&mut self) -> &mut bool {
        &mut self.read
    }

    // bool write = 2;

    pub fn clear_write(&mut self) {
        self.write = false;
    }

    // Param is passed by value, moved
    pub fn set_write(&mut self, v: bool) {
        self.write = v;
    }

    pub fn get_write(&self) -> bool {
        self.write
    }

    fn get_write_for_reflect(&self) -> &bool {
        &self.write
    }

    fn mut_write_for_reflect(&mut self) -> &mut bool {
        &mut self.write
    }

    // bool delete = 3;

    pub fn clear_delete(&mut self) {
        self.delete = false;
    }

    // Param is passed by value, moved
    pub fn set_delete(&mut self, v: bool) {
        self.delete = v;
    }

    pub fn get_delete(&self) -> bool {
        self.delete
    }

    fn get_delete_for_reflect(&self) -> &bool {
        &self.delete
    }

    fn mut_delete_for_reflect(&mut self) -> &mut bool {
        &mut self.delete
    }

    // bool admin = 4;

    pub fn clear_admin(&mut self) {
        self.admin = false;
    }

    // Param is passed by value, moved
    pub fn set_admin(&mut self, v: bool) {
        self.admin = v;
    }

    pub fn get_admin(&self) -> bool {
        self.admin
    }

    fn get_admin_for_reflect(&self) -> &bool {
        &self.admin
    }

    fn mut_admin_for_reflect(&mut self) -> &mut bool {
        &mut self.admin
    }
}

impl ::protobuf::Message for Permission {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.read = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.write = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.delete = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.admin = tmp;
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
        if self.read != false {
            my_size += 2;
        }
        if self.write != false {
            my_size += 2;
        }
        if self.delete != false {
            my_size += 2;
        }
        if self.admin != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.read != false {
            os.write_bool(1, self.read)?;
        }
        if self.write != false {
            os.write_bool(2, self.write)?;
        }
        if self.delete != false {
            os.write_bool(3, self.delete)?;
        }
        if self.admin != false {
            os.write_bool(4, self.admin)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Permission {
    fn new() -> Permission {
        Permission::new()
    }

    fn descriptor_static(_: ::std::option::Option<Permission>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "read",
                    Permission::get_read_for_reflect,
                    Permission::mut_read_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "write",
                    Permission::get_write_for_reflect,
                    Permission::mut_write_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "delete",
                    Permission::get_delete_for_reflect,
                    Permission::mut_delete_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "admin",
                    Permission::get_admin_for_reflect,
                    Permission::mut_admin_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Permission>(
                    "Permission",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Permission {
    fn clear(&mut self) {
        self.clear_read();
        self.clear_write();
        self.clear_delete();
        self.clear_admin();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Permission {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Permission {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CreateNamespaceRequest {
    // message fields
    pub namespace: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CreateNamespaceRequest {}

impl CreateNamespaceRequest {
    pub fn new() -> CreateNamespaceRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CreateNamespaceRequest {
        static mut instance: ::protobuf::lazy::Lazy<CreateNamespaceRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CreateNamespaceRequest,
        };
        unsafe {
            instance.get(CreateNamespaceRequest::new)
        }
    }

    // string namespace = 1;

    pub fn clear_namespace(&mut self) {
        self.namespace.clear();
    }

    // Param is passed by value, moved
    pub fn set_namespace(&mut self, v: ::std::string::String) {
        self.namespace = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_namespace(&mut self) -> &mut ::std::string::String {
        &mut self.namespace
    }

    // Take field
    pub fn take_namespace(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.namespace, ::std::string::String::new())
    }

    pub fn get_namespace(&self) -> &str {
        &self.namespace
    }

    fn get_namespace_for_reflect(&self) -> &::std::string::String {
        &self.namespace
    }

    fn mut_namespace_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.namespace
    }
}

impl ::protobuf::Message for CreateNamespaceRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.namespace)?;
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
        if !self.namespace.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.namespace);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.namespace.is_empty() {
            os.write_string(1, &self.namespace)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CreateNamespaceRequest {
    fn new() -> CreateNamespaceRequest {
        CreateNamespaceRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CreateNamespaceRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "namespace",
                    CreateNamespaceRequest::get_namespace_for_reflect,
                    CreateNamespaceRequest::mut_namespace_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CreateNamespaceRequest>(
                    "CreateNamespaceRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CreateNamespaceRequest {
    fn clear(&mut self) {
        self.clear_namespace();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CreateNamespaceRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreateNamespaceRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CreateNamespaceResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CreateNamespaceResponse {}

impl CreateNamespaceResponse {
    pub fn new() -> CreateNamespaceResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CreateNamespaceResponse {
        static mut instance: ::protobuf::lazy::Lazy<CreateNamespaceResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CreateNamespaceResponse,
        };
        unsafe {
            instance.get(CreateNamespaceResponse::new)
        }
    }
}

impl ::protobuf::Message for CreateNamespaceResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CreateNamespaceResponse {
    fn new() -> CreateNamespaceResponse {
        CreateNamespaceResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CreateNamespaceResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CreateNamespaceResponse>(
                    "CreateNamespaceResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CreateNamespaceResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CreateNamespaceResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreateNamespaceResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteNamespaceRequest {
    // message fields
    pub namespace: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeleteNamespaceRequest {}

impl DeleteNamespaceRequest {
    pub fn new() -> DeleteNamespaceRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteNamespaceRequest {
        static mut instance: ::protobuf::lazy::Lazy<DeleteNamespaceRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteNamespaceRequest,
        };
        unsafe {
            instance.get(DeleteNamespaceRequest::new)
        }
    }

    // string namespace = 1;

    pub fn clear_namespace(&mut self) {
        self.namespace.clear();
    }

    // Param is passed by value, moved
    pub fn set_namespace(&mut self, v: ::std::string::String) {
        self.namespace = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_namespace(&mut self) -> &mut ::std::string::String {
        &mut self.namespace
    }

    // Take field
    pub fn take_namespace(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.namespace, ::std::string::String::new())
    }

    pub fn get_namespace(&self) -> &str {
        &self.namespace
    }

    fn get_namespace_for_reflect(&self) -> &::std::string::String {
        &self.namespace
    }

    fn mut_namespace_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.namespace
    }
}

impl ::protobuf::Message for DeleteNamespaceRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.namespace)?;
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
        if !self.namespace.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.namespace);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.namespace.is_empty() {
            os.write_string(1, &self.namespace)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DeleteNamespaceRequest {
    fn new() -> DeleteNamespaceRequest {
        DeleteNamespaceRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteNamespaceRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "namespace",
                    DeleteNamespaceRequest::get_namespace_for_reflect,
                    DeleteNamespaceRequest::mut_namespace_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeleteNamespaceRequest>(
                    "DeleteNamespaceRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteNamespaceRequest {
    fn clear(&mut self) {
        self.clear_namespace();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeleteNamespaceRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteNamespaceRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteNamespaceResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeleteNamespaceResponse {}

impl DeleteNamespaceResponse {
    pub fn new() -> DeleteNamespaceResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteNamespaceResponse {
        static mut instance: ::protobuf::lazy::Lazy<DeleteNamespaceResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteNamespaceResponse,
        };
        unsafe {
            instance.get(DeleteNamespaceResponse::new)
        }
    }
}

impl ::protobuf::Message for DeleteNamespaceResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DeleteNamespaceResponse {
    fn new() -> DeleteNamespaceResponse {
        DeleteNamespaceResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteNamespaceResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<DeleteNamespaceResponse>(
                    "DeleteNamespaceResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteNamespaceResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeleteNamespaceResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteNamespaceResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetPermissionRequest {
    // message fields
    pub namespace: ::std::string::String,
    pub userID: ::std::string::String,
    pub permission: ::protobuf::SingularPtrField<Permission>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetPermissionRequest {}

impl SetPermissionRequest {
    pub fn new() -> SetPermissionRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetPermissionRequest {
        static mut instance: ::protobuf::lazy::Lazy<SetPermissionRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetPermissionRequest,
        };
        unsafe {
            instance.get(SetPermissionRequest::new)
        }
    }

    // string namespace = 1;

    pub fn clear_namespace(&mut self) {
        self.namespace.clear();
    }

    // Param is passed by value, moved
    pub fn set_namespace(&mut self, v: ::std::string::String) {
        self.namespace = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_namespace(&mut self) -> &mut ::std::string::String {
        &mut self.namespace
    }

    // Take field
    pub fn take_namespace(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.namespace, ::std::string::String::new())
    }

    pub fn get_namespace(&self) -> &str {
        &self.namespace
    }

    fn get_namespace_for_reflect(&self) -> &::std::string::String {
        &self.namespace
    }

    fn mut_namespace_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.namespace
    }

    // string userID = 2;

    pub fn clear_userID(&mut self) {
        self.userID.clear();
    }

    // Param is passed by value, moved
    pub fn set_userID(&mut self, v: ::std::string::String) {
        self.userID = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_userID(&mut self) -> &mut ::std::string::String {
        &mut self.userID
    }

    // Take field
    pub fn take_userID(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.userID, ::std::string::String::new())
    }

    pub fn get_userID(&self) -> &str {
        &self.userID
    }

    fn get_userID_for_reflect(&self) -> &::std::string::String {
        &self.userID
    }

    fn mut_userID_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.userID
    }

    // .schema.Permission permission = 3;

    pub fn clear_permission(&mut self) {
        self.permission.clear();
    }

    pub fn has_permission(&self) -> bool {
        self.permission.is_some()
    }

    // Param is passed by value, moved
    pub fn set_permission(&mut self, v: Permission) {
        self.permission = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_permission(&mut self) -> &mut Permission {
        if self.permission.is_none() {
            self.permission.set_default();
        }
        self.permission.as_mut().unwrap()
    }

    // Take field
    pub fn take_permission(&mut self) -> Permission {
        self.permission.take().unwrap_or_else(|| Permission::new())
    }

    pub fn get_permission(&self) -> &Permission {
        self.permission.as_ref().unwrap_or_else(|| Permission::default_instance())
    }

    fn get_permission_for_reflect(&self) -> &::protobuf::SingularPtrField<Permission> {
        &self.permission
    }

    fn mut_permission_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Permission> {
        &mut self.permission
    }
}

impl ::protobuf::Message for SetPermissionRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.permission {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.namespace)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.userID)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.permission)?;
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
        if !self.namespace.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.namespace);
        }
        if !self.userID.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.userID);
        }
        if let Some(ref v) = self.permission.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.namespace.is_empty() {
            os.write_string(1, &self.namespace)?;
        }
        if !self.userID.is_empty() {
            os.write_string(2, &self.userID)?;
        }
        if let Some(ref v) = self.permission.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SetPermissionRequest {
    fn new() -> SetPermissionRequest {
        SetPermissionRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetPermissionRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "namespace",
                    SetPermissionRequest::get_namespace_for_reflect,
                    SetPermissionRequest::mut_namespace_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "userID",
                    SetPermissionRequest::get_userID_for_reflect,
                    SetPermissionRequest::mut_userID_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Permission>>(
                    "permission",
                    SetPermissionRequest::get_permission_for_reflect,
                    SetPermissionRequest::mut_permission_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetPermissionRequest>(
                    "SetPermissionRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetPermissionRequest {
    fn clear(&mut self) {
        self.clear_namespace();
        self.clear_userID();
        self.clear_permission();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SetPermissionRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetPermissionRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetPermissionResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetPermissionResponse {}

impl SetPermissionResponse {
    pub fn new() -> SetPermissionResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetPermissionResponse {
        static mut instance: ::protobuf::lazy::Lazy<SetPermissionResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetPermissionResponse,
        };
        unsafe {
            instance.get(SetPermissionResponse::new)
        }
    }
}

impl ::protobuf::Message for SetPermissionResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SetPermissionResponse {
    fn new() -> SetPermissionResponse {
        SetPermissionResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetPermissionResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<SetPermissionResponse>(
                    "SetPermissionResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetPermissionResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SetPermissionResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetPermissionResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetPermissionRequest {
    // message fields
    pub namespace: ::std::string::String,
    pub userID: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetPermissionRequest {}

impl GetPermissionRequest {
    pub fn new() -> GetPermissionRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetPermissionRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetPermissionRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetPermissionRequest,
        };
        unsafe {
            instance.get(GetPermissionRequest::new)
        }
    }

    // string namespace = 1;

    pub fn clear_namespace(&mut self) {
        self.namespace.clear();
    }

    // Param is passed by value, moved
    pub fn set_namespace(&mut self, v: ::std::string::String) {
        self.namespace = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_namespace(&mut self) -> &mut ::std::string::String {
        &mut self.namespace
    }

    // Take field
    pub fn take_namespace(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.namespace, ::std::string::String::new())
    }

    pub fn get_namespace(&self) -> &str {
        &self.namespace
    }

    fn get_namespace_for_reflect(&self) -> &::std::string::String {
        &self.namespace
    }

    fn mut_namespace_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.namespace
    }

    // string userID = 2;

    pub fn clear_userID(&mut self) {
        self.userID.clear();
    }

    // Param is passed by value, moved
    pub fn set_userID(&mut self, v: ::std::string::String) {
        self.userID = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_userID(&mut self) -> &mut ::std::string::String {
        &mut self.userID
    }

    // Take field
    pub fn take_userID(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.userID, ::std::string::String::new())
    }

    pub fn get_userID(&self) -> &str {
        &self.userID
    }

    fn get_userID_for_reflect(&self) -> &::std::string::String {
        &self.userID
    }

    fn mut_userID_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.userID
    }
}

impl ::protobuf::Message for GetPermissionRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.namespace)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.userID)?;
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
        if !self.namespace.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.namespace);
        }
        if !self.userID.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.userID);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.namespace.is_empty() {
            os.write_string(1, &self.namespace)?;
        }
        if !self.userID.is_empty() {
            os.write_string(2, &self.userID)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetPermissionRequest {
    fn new() -> GetPermissionRequest {
        GetPermissionRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetPermissionRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "namespace",
                    GetPermissionRequest::get_namespace_for_reflect,
                    GetPermissionRequest::mut_namespace_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "userID",
                    GetPermissionRequest::get_userID_for_reflect,
                    GetPermissionRequest::mut_userID_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetPermissionRequest>(
                    "GetPermissionRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetPermissionRequest {
    fn clear(&mut self) {
        self.clear_namespace();
        self.clear_userID();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetPermissionRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetPermissionRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetPermissionResponse {
    // message fields
    pub permission: ::protobuf::SingularPtrField<Permission>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetPermissionResponse {}

impl GetPermissionResponse {
    pub fn new() -> GetPermissionResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetPermissionResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetPermissionResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetPermissionResponse,
        };
        unsafe {
            instance.get(GetPermissionResponse::new)
        }
    }

    // .schema.Permission permission = 1;

    pub fn clear_permission(&mut self) {
        self.permission.clear();
    }

    pub fn has_permission(&self) -> bool {
        self.permission.is_some()
    }

    // Param is passed by value, moved
    pub fn set_permission(&mut self, v: Permission) {
        self.permission = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_permission(&mut self) -> &mut Permission {
        if self.permission.is_none() {
            self.permission.set_default();
        }
        self.permission.as_mut().unwrap()
    }

    // Take field
    pub fn take_permission(&mut self) -> Permission {
        self.permission.take().unwrap_or_else(|| Permission::new())
    }

    pub fn get_permission(&self) -> &Permission {
        self.permission.as_ref().unwrap_or_else(|| Permission::default_instance())
    }

    fn get_permission_for_reflect(&self) -> &::protobuf::SingularPtrField<Permission> {
        &self.permission
    }

    fn mut_permission_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Permission> {
        &mut self.permission
    }
}

impl ::protobuf::Message for GetPermissionResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.permission {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.permission)?;
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
        if let Some(ref v) = self.permission.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.permission.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetPermissionResponse {
    fn new() -> GetPermissionResponse {
        GetPermissionResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetPermissionResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Permission>>(
                    "permission",
                    GetPermissionResponse::get_permission_for_reflect,
                    GetPermissionResponse::mut_permission_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetPermissionResponse>(
                    "GetPermissionResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetPermissionResponse {
    fn clear(&mut self) {
        self.clear_permission();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetPermissionResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetPermissionResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Metadata {
    // message fields
    pub key: ::std::vec::Vec<u8>,
    pub totalSize: i64,
    pub creationEpoch: i64,
    pub lastWriteEpoch: i64,
    pub chunks: ::protobuf::RepeatedField<Chunk>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Metadata {}

impl Metadata {
    pub fn new() -> Metadata {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Metadata {
        static mut instance: ::protobuf::lazy::Lazy<Metadata> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Metadata,
        };
        unsafe {
            instance.get(Metadata::new)
        }
    }

    // bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // int64 totalSize = 2;

    pub fn clear_totalSize(&mut self) {
        self.totalSize = 0;
    }

    // Param is passed by value, moved
    pub fn set_totalSize(&mut self, v: i64) {
        self.totalSize = v;
    }

    pub fn get_totalSize(&self) -> i64 {
        self.totalSize
    }

    fn get_totalSize_for_reflect(&self) -> &i64 {
        &self.totalSize
    }

    fn mut_totalSize_for_reflect(&mut self) -> &mut i64 {
        &mut self.totalSize
    }

    // int64 creationEpoch = 3;

    pub fn clear_creationEpoch(&mut self) {
        self.creationEpoch = 0;
    }

    // Param is passed by value, moved
    pub fn set_creationEpoch(&mut self, v: i64) {
        self.creationEpoch = v;
    }

    pub fn get_creationEpoch(&self) -> i64 {
        self.creationEpoch
    }

    fn get_creationEpoch_for_reflect(&self) -> &i64 {
        &self.creationEpoch
    }

    fn mut_creationEpoch_for_reflect(&mut self) -> &mut i64 {
        &mut self.creationEpoch
    }

    // int64 lastWriteEpoch = 4;

    pub fn clear_lastWriteEpoch(&mut self) {
        self.lastWriteEpoch = 0;
    }

    // Param is passed by value, moved
    pub fn set_lastWriteEpoch(&mut self, v: i64) {
        self.lastWriteEpoch = v;
    }

    pub fn get_lastWriteEpoch(&self) -> i64 {
        self.lastWriteEpoch
    }

    fn get_lastWriteEpoch_for_reflect(&self) -> &i64 {
        &self.lastWriteEpoch
    }

    fn mut_lastWriteEpoch_for_reflect(&mut self) -> &mut i64 {
        &mut self.lastWriteEpoch
    }

    // repeated .schema.Chunk chunks = 5;

    pub fn clear_chunks(&mut self) {
        self.chunks.clear();
    }

    // Param is passed by value, moved
    pub fn set_chunks(&mut self, v: ::protobuf::RepeatedField<Chunk>) {
        self.chunks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_chunks(&mut self) -> &mut ::protobuf::RepeatedField<Chunk> {
        &mut self.chunks
    }

    // Take field
    pub fn take_chunks(&mut self) -> ::protobuf::RepeatedField<Chunk> {
        ::std::mem::replace(&mut self.chunks, ::protobuf::RepeatedField::new())
    }

    pub fn get_chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    fn get_chunks_for_reflect(&self) -> &::protobuf::RepeatedField<Chunk> {
        &self.chunks
    }

    fn mut_chunks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Chunk> {
        &mut self.chunks
    }
}

impl ::protobuf::Message for Metadata {
    fn is_initialized(&self) -> bool {
        for v in &self.chunks {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.totalSize = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.creationEpoch = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.lastWriteEpoch = tmp;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.chunks)?;
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
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.key);
        }
        if self.totalSize != 0 {
            my_size += ::protobuf::rt::value_size(2, self.totalSize, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.creationEpoch != 0 {
            my_size += ::protobuf::rt::value_size(3, self.creationEpoch, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.lastWriteEpoch != 0 {
            my_size += ::protobuf::rt::value_size(4, self.lastWriteEpoch, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.chunks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_bytes(1, &self.key)?;
        }
        if self.totalSize != 0 {
            os.write_int64(2, self.totalSize)?;
        }
        if self.creationEpoch != 0 {
            os.write_int64(3, self.creationEpoch)?;
        }
        if self.lastWriteEpoch != 0 {
            os.write_int64(4, self.lastWriteEpoch)?;
        }
        for v in &self.chunks {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Metadata {
    fn new() -> Metadata {
        Metadata::new()
    }

    fn descriptor_static(_: ::std::option::Option<Metadata>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    Metadata::get_key_for_reflect,
                    Metadata::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "totalSize",
                    Metadata::get_totalSize_for_reflect,
                    Metadata::mut_totalSize_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "creationEpoch",
                    Metadata::get_creationEpoch_for_reflect,
                    Metadata::mut_creationEpoch_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "lastWriteEpoch",
                    Metadata::get_lastWriteEpoch_for_reflect,
                    Metadata::mut_lastWriteEpoch_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Chunk>>(
                    "chunks",
                    Metadata::get_chunks_for_reflect,
                    Metadata::mut_chunks_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Metadata>(
                    "Metadata",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Metadata {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_totalSize();
        self.clear_creationEpoch();
        self.clear_lastWriteEpoch();
        self.clear_chunks();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Metadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Metadata {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Chunk {
    // message fields
    pub chunkSize: i64,
    pub objects: ::protobuf::RepeatedField<Object>,
    pub hash: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Chunk {}

impl Chunk {
    pub fn new() -> Chunk {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Chunk {
        static mut instance: ::protobuf::lazy::Lazy<Chunk> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Chunk,
        };
        unsafe {
            instance.get(Chunk::new)
        }
    }

    // int64 chunkSize = 1;

    pub fn clear_chunkSize(&mut self) {
        self.chunkSize = 0;
    }

    // Param is passed by value, moved
    pub fn set_chunkSize(&mut self, v: i64) {
        self.chunkSize = v;
    }

    pub fn get_chunkSize(&self) -> i64 {
        self.chunkSize
    }

    fn get_chunkSize_for_reflect(&self) -> &i64 {
        &self.chunkSize
    }

    fn mut_chunkSize_for_reflect(&mut self) -> &mut i64 {
        &mut self.chunkSize
    }

    // repeated .schema.Object objects = 2;

    pub fn clear_objects(&mut self) {
        self.objects.clear();
    }

    // Param is passed by value, moved
    pub fn set_objects(&mut self, v: ::protobuf::RepeatedField<Object>) {
        self.objects = v;
    }

    // Mutable pointer to the field.
    pub fn mut_objects(&mut self) -> &mut ::protobuf::RepeatedField<Object> {
        &mut self.objects
    }

    // Take field
    pub fn take_objects(&mut self) -> ::protobuf::RepeatedField<Object> {
        ::std::mem::replace(&mut self.objects, ::protobuf::RepeatedField::new())
    }

    pub fn get_objects(&self) -> &[Object] {
        &self.objects
    }

    fn get_objects_for_reflect(&self) -> &::protobuf::RepeatedField<Object> {
        &self.objects
    }

    fn mut_objects_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Object> {
        &mut self.objects
    }

    // bytes hash = 3;

    pub fn clear_hash(&mut self) {
        self.hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.hash
    }

    // Take field
    pub fn take_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.hash, ::std::vec::Vec::new())
    }

    pub fn get_hash(&self) -> &[u8] {
        &self.hash
    }

    fn get_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.hash
    }

    fn mut_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.hash
    }
}

impl ::protobuf::Message for Chunk {
    fn is_initialized(&self) -> bool {
        for v in &self.objects {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.chunkSize = tmp;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.objects)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.hash)?;
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
        if self.chunkSize != 0 {
            my_size += ::protobuf::rt::value_size(1, self.chunkSize, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.objects {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.hash);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.chunkSize != 0 {
            os.write_int64(1, self.chunkSize)?;
        }
        for v in &self.objects {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.hash.is_empty() {
            os.write_bytes(3, &self.hash)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Chunk {
    fn new() -> Chunk {
        Chunk::new()
    }

    fn descriptor_static(_: ::std::option::Option<Chunk>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "chunkSize",
                    Chunk::get_chunkSize_for_reflect,
                    Chunk::mut_chunkSize_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Object>>(
                    "objects",
                    Chunk::get_objects_for_reflect,
                    Chunk::mut_objects_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "hash",
                    Chunk::get_hash_for_reflect,
                    Chunk::mut_hash_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Chunk>(
                    "Chunk",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Chunk {
    fn clear(&mut self) {
        self.clear_chunkSize();
        self.clear_objects();
        self.clear_hash();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Chunk {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Chunk {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Object {
    // message fields
    pub key: ::std::vec::Vec<u8>,
    pub shardID: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Object {}

impl Object {
    pub fn new() -> Object {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Object {
        static mut instance: ::protobuf::lazy::Lazy<Object> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Object,
        };
        unsafe {
            instance.get(Object::new)
        }
    }

    // bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // string shardID = 2;

    pub fn clear_shardID(&mut self) {
        self.shardID.clear();
    }

    // Param is passed by value, moved
    pub fn set_shardID(&mut self, v: ::std::string::String) {
        self.shardID = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_shardID(&mut self) -> &mut ::std::string::String {
        &mut self.shardID
    }

    // Take field
    pub fn take_shardID(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.shardID, ::std::string::String::new())
    }

    pub fn get_shardID(&self) -> &str {
        &self.shardID
    }

    fn get_shardID_for_reflect(&self) -> &::std::string::String {
        &self.shardID
    }

    fn mut_shardID_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.shardID
    }
}

impl ::protobuf::Message for Object {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.shardID)?;
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
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.key);
        }
        if !self.shardID.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.shardID);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_bytes(1, &self.key)?;
        }
        if !self.shardID.is_empty() {
            os.write_string(2, &self.shardID)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Object {
    fn new() -> Object {
        Object::new()
    }

    fn descriptor_static(_: ::std::option::Option<Object>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    Object::get_key_for_reflect,
                    Object::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "shardID",
                    Object::get_shardID_for_reflect,
                    Object::mut_shardID_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Object>(
                    "Object",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Object {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_shardID();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Object {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Object {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct WriteRequest {
    // message fields
    pub key: ::std::vec::Vec<u8>,
    pub data: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WriteRequest {}

impl WriteRequest {
    pub fn new() -> WriteRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WriteRequest {
        static mut instance: ::protobuf::lazy::Lazy<WriteRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WriteRequest,
        };
        unsafe {
            instance.get(WriteRequest::new)
        }
    }

    // bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // bytes data = 2;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }
}

impl ::protobuf::Message for WriteRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
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
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.key);
        }
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_bytes(1, &self.key)?;
        }
        if !self.data.is_empty() {
            os.write_bytes(2, &self.data)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WriteRequest {
    fn new() -> WriteRequest {
        WriteRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<WriteRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    WriteRequest::get_key_for_reflect,
                    WriteRequest::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    WriteRequest::get_data_for_reflect,
                    WriteRequest::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WriteRequest>(
                    "WriteRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WriteRequest {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WriteRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WriteRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct WriteResponse {
    // message fields
    pub metadata: ::protobuf::SingularPtrField<Metadata>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WriteResponse {}

impl WriteResponse {
    pub fn new() -> WriteResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WriteResponse {
        static mut instance: ::protobuf::lazy::Lazy<WriteResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WriteResponse,
        };
        unsafe {
            instance.get(WriteResponse::new)
        }
    }

    // .schema.Metadata metadata = 1;

    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    pub fn has_metadata(&self) -> bool {
        self.metadata.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: Metadata) {
        self.metadata = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metadata(&mut self) -> &mut Metadata {
        if self.metadata.is_none() {
            self.metadata.set_default();
        }
        self.metadata.as_mut().unwrap()
    }

    // Take field
    pub fn take_metadata(&mut self) -> Metadata {
        self.metadata.take().unwrap_or_else(|| Metadata::new())
    }

    pub fn get_metadata(&self) -> &Metadata {
        self.metadata.as_ref().unwrap_or_else(|| Metadata::default_instance())
    }

    fn get_metadata_for_reflect(&self) -> &::protobuf::SingularPtrField<Metadata> {
        &self.metadata
    }

    fn mut_metadata_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Metadata> {
        &mut self.metadata
    }
}

impl ::protobuf::Message for WriteResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.metadata {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.metadata)?;
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
        if let Some(ref v) = self.metadata.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.metadata.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WriteResponse {
    fn new() -> WriteResponse {
        WriteResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<WriteResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Metadata>>(
                    "metadata",
                    WriteResponse::get_metadata_for_reflect,
                    WriteResponse::mut_metadata_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WriteResponse>(
                    "WriteResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WriteResponse {
    fn clear(&mut self) {
        self.clear_metadata();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WriteResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WriteResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct WriteFileRequest {
    // message fields
    pub key: ::std::vec::Vec<u8>,
    pub filePath: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WriteFileRequest {}

impl WriteFileRequest {
    pub fn new() -> WriteFileRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WriteFileRequest {
        static mut instance: ::protobuf::lazy::Lazy<WriteFileRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WriteFileRequest,
        };
        unsafe {
            instance.get(WriteFileRequest::new)
        }
    }

    // bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // string filePath = 2;

    pub fn clear_filePath(&mut self) {
        self.filePath.clear();
    }

    // Param is passed by value, moved
    pub fn set_filePath(&mut self, v: ::std::string::String) {
        self.filePath = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_filePath(&mut self) -> &mut ::std::string::String {
        &mut self.filePath
    }

    // Take field
    pub fn take_filePath(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.filePath, ::std::string::String::new())
    }

    pub fn get_filePath(&self) -> &str {
        &self.filePath
    }

    fn get_filePath_for_reflect(&self) -> &::std::string::String {
        &self.filePath
    }

    fn mut_filePath_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.filePath
    }
}

impl ::protobuf::Message for WriteFileRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.filePath)?;
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
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.key);
        }
        if !self.filePath.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.filePath);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_bytes(1, &self.key)?;
        }
        if !self.filePath.is_empty() {
            os.write_string(2, &self.filePath)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WriteFileRequest {
    fn new() -> WriteFileRequest {
        WriteFileRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<WriteFileRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    WriteFileRequest::get_key_for_reflect,
                    WriteFileRequest::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "filePath",
                    WriteFileRequest::get_filePath_for_reflect,
                    WriteFileRequest::mut_filePath_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WriteFileRequest>(
                    "WriteFileRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WriteFileRequest {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_filePath();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WriteFileRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WriteFileRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct WriteFileResponse {
    // message fields
    pub metadata: ::protobuf::SingularPtrField<Metadata>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WriteFileResponse {}

impl WriteFileResponse {
    pub fn new() -> WriteFileResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WriteFileResponse {
        static mut instance: ::protobuf::lazy::Lazy<WriteFileResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WriteFileResponse,
        };
        unsafe {
            instance.get(WriteFileResponse::new)
        }
    }

    // .schema.Metadata metadata = 1;

    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    pub fn has_metadata(&self) -> bool {
        self.metadata.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: Metadata) {
        self.metadata = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metadata(&mut self) -> &mut Metadata {
        if self.metadata.is_none() {
            self.metadata.set_default();
        }
        self.metadata.as_mut().unwrap()
    }

    // Take field
    pub fn take_metadata(&mut self) -> Metadata {
        self.metadata.take().unwrap_or_else(|| Metadata::new())
    }

    pub fn get_metadata(&self) -> &Metadata {
        self.metadata.as_ref().unwrap_or_else(|| Metadata::default_instance())
    }

    fn get_metadata_for_reflect(&self) -> &::protobuf::SingularPtrField<Metadata> {
        &self.metadata
    }

    fn mut_metadata_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Metadata> {
        &mut self.metadata
    }
}

impl ::protobuf::Message for WriteFileResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.metadata {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.metadata)?;
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
        if let Some(ref v) = self.metadata.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.metadata.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WriteFileResponse {
    fn new() -> WriteFileResponse {
        WriteFileResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<WriteFileResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Metadata>>(
                    "metadata",
                    WriteFileResponse::get_metadata_for_reflect,
                    WriteFileResponse::mut_metadata_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WriteFileResponse>(
                    "WriteFileResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WriteFileResponse {
    fn clear(&mut self) {
        self.clear_metadata();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WriteFileResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WriteFileResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct WriteStreamRequest {
    // message oneof groups
    input: ::std::option::Option<WriteStreamRequest_oneof_input>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WriteStreamRequest {}

#[derive(Clone,PartialEq)]
pub enum WriteStreamRequest_oneof_input {
    metadata(WriteStreamRequest_Metadata),
    data(WriteStreamRequest_Data),
}

impl WriteStreamRequest {
    pub fn new() -> WriteStreamRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WriteStreamRequest {
        static mut instance: ::protobuf::lazy::Lazy<WriteStreamRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WriteStreamRequest,
        };
        unsafe {
            instance.get(WriteStreamRequest::new)
        }
    }

    // .schema.WriteStreamRequest.Metadata metadata = 1;

    pub fn clear_metadata(&mut self) {
        self.input = ::std::option::Option::None;
    }

    pub fn has_metadata(&self) -> bool {
        match self.input {
            ::std::option::Option::Some(WriteStreamRequest_oneof_input::metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: WriteStreamRequest_Metadata) {
        self.input = ::std::option::Option::Some(WriteStreamRequest_oneof_input::metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_metadata(&mut self) -> &mut WriteStreamRequest_Metadata {
        if let ::std::option::Option::Some(WriteStreamRequest_oneof_input::metadata(_)) = self.input {
        } else {
            self.input = ::std::option::Option::Some(WriteStreamRequest_oneof_input::metadata(WriteStreamRequest_Metadata::new()));
        }
        match self.input {
            ::std::option::Option::Some(WriteStreamRequest_oneof_input::metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_metadata(&mut self) -> WriteStreamRequest_Metadata {
        if self.has_metadata() {
            match self.input.take() {
                ::std::option::Option::Some(WriteStreamRequest_oneof_input::metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            WriteStreamRequest_Metadata::new()
        }
    }

    pub fn get_metadata(&self) -> &WriteStreamRequest_Metadata {
        match self.input {
            ::std::option::Option::Some(WriteStreamRequest_oneof_input::metadata(ref v)) => v,
            _ => WriteStreamRequest_Metadata::default_instance(),
        }
    }

    // .schema.WriteStreamRequest.Data data = 2;

    pub fn clear_data(&mut self) {
        self.input = ::std::option::Option::None;
    }

    pub fn has_data(&self) -> bool {
        match self.input {
            ::std::option::Option::Some(WriteStreamRequest_oneof_input::data(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: WriteStreamRequest_Data) {
        self.input = ::std::option::Option::Some(WriteStreamRequest_oneof_input::data(v))
    }

    // Mutable pointer to the field.
    pub fn mut_data(&mut self) -> &mut WriteStreamRequest_Data {
        if let ::std::option::Option::Some(WriteStreamRequest_oneof_input::data(_)) = self.input {
        } else {
            self.input = ::std::option::Option::Some(WriteStreamRequest_oneof_input::data(WriteStreamRequest_Data::new()));
        }
        match self.input {
            ::std::option::Option::Some(WriteStreamRequest_oneof_input::data(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_data(&mut self) -> WriteStreamRequest_Data {
        if self.has_data() {
            match self.input.take() {
                ::std::option::Option::Some(WriteStreamRequest_oneof_input::data(v)) => v,
                _ => panic!(),
            }
        } else {
            WriteStreamRequest_Data::new()
        }
    }

    pub fn get_data(&self) -> &WriteStreamRequest_Data {
        match self.input {
            ::std::option::Option::Some(WriteStreamRequest_oneof_input::data(ref v)) => v,
            _ => WriteStreamRequest_Data::default_instance(),
        }
    }
}

impl ::protobuf::Message for WriteStreamRequest {
    fn is_initialized(&self) -> bool {
        if let Some(WriteStreamRequest_oneof_input::metadata(ref v)) = self.input {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(WriteStreamRequest_oneof_input::data(ref v)) = self.input {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.input = ::std::option::Option::Some(WriteStreamRequest_oneof_input::metadata(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.input = ::std::option::Option::Some(WriteStreamRequest_oneof_input::data(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.input {
            match v {
                &WriteStreamRequest_oneof_input::metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &WriteStreamRequest_oneof_input::data(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.input {
            match v {
                &WriteStreamRequest_oneof_input::metadata(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &WriteStreamRequest_oneof_input::data(ref v) => {
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WriteStreamRequest {
    fn new() -> WriteStreamRequest {
        WriteStreamRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<WriteStreamRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, WriteStreamRequest_Metadata>(
                    "metadata",
                    WriteStreamRequest::has_metadata,
                    WriteStreamRequest::get_metadata,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, WriteStreamRequest_Data>(
                    "data",
                    WriteStreamRequest::has_data,
                    WriteStreamRequest::get_data,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WriteStreamRequest>(
                    "WriteStreamRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WriteStreamRequest {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WriteStreamRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WriteStreamRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct WriteStreamRequest_Metadata {
    // message fields
    pub key: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WriteStreamRequest_Metadata {}

impl WriteStreamRequest_Metadata {
    pub fn new() -> WriteStreamRequest_Metadata {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WriteStreamRequest_Metadata {
        static mut instance: ::protobuf::lazy::Lazy<WriteStreamRequest_Metadata> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WriteStreamRequest_Metadata,
        };
        unsafe {
            instance.get(WriteStreamRequest_Metadata::new)
        }
    }

    // bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }
}

impl ::protobuf::Message for WriteStreamRequest_Metadata {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
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
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.key);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_bytes(1, &self.key)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WriteStreamRequest_Metadata {
    fn new() -> WriteStreamRequest_Metadata {
        WriteStreamRequest_Metadata::new()
    }

    fn descriptor_static(_: ::std::option::Option<WriteStreamRequest_Metadata>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    WriteStreamRequest_Metadata::get_key_for_reflect,
                    WriteStreamRequest_Metadata::mut_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WriteStreamRequest_Metadata>(
                    "WriteStreamRequest_Metadata",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WriteStreamRequest_Metadata {
    fn clear(&mut self) {
        self.clear_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WriteStreamRequest_Metadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WriteStreamRequest_Metadata {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct WriteStreamRequest_Data {
    // message fields
    pub dataChunk: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WriteStreamRequest_Data {}

impl WriteStreamRequest_Data {
    pub fn new() -> WriteStreamRequest_Data {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WriteStreamRequest_Data {
        static mut instance: ::protobuf::lazy::Lazy<WriteStreamRequest_Data> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WriteStreamRequest_Data,
        };
        unsafe {
            instance.get(WriteStreamRequest_Data::new)
        }
    }

    // bytes dataChunk = 2;

    pub fn clear_dataChunk(&mut self) {
        self.dataChunk.clear();
    }

    // Param is passed by value, moved
    pub fn set_dataChunk(&mut self, v: ::std::vec::Vec<u8>) {
        self.dataChunk = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dataChunk(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.dataChunk
    }

    // Take field
    pub fn take_dataChunk(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.dataChunk, ::std::vec::Vec::new())
    }

    pub fn get_dataChunk(&self) -> &[u8] {
        &self.dataChunk
    }

    fn get_dataChunk_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.dataChunk
    }

    fn mut_dataChunk_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.dataChunk
    }
}

impl ::protobuf::Message for WriteStreamRequest_Data {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.dataChunk)?;
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
        if !self.dataChunk.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.dataChunk);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.dataChunk.is_empty() {
            os.write_bytes(2, &self.dataChunk)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WriteStreamRequest_Data {
    fn new() -> WriteStreamRequest_Data {
        WriteStreamRequest_Data::new()
    }

    fn descriptor_static(_: ::std::option::Option<WriteStreamRequest_Data>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "dataChunk",
                    WriteStreamRequest_Data::get_dataChunk_for_reflect,
                    WriteStreamRequest_Data::mut_dataChunk_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WriteStreamRequest_Data>(
                    "WriteStreamRequest_Data",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WriteStreamRequest_Data {
    fn clear(&mut self) {
        self.clear_dataChunk();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WriteStreamRequest_Data {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WriteStreamRequest_Data {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct WriteStreamResponse {
    // message fields
    pub metadata: ::protobuf::SingularPtrField<Metadata>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WriteStreamResponse {}

impl WriteStreamResponse {
    pub fn new() -> WriteStreamResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WriteStreamResponse {
        static mut instance: ::protobuf::lazy::Lazy<WriteStreamResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WriteStreamResponse,
        };
        unsafe {
            instance.get(WriteStreamResponse::new)
        }
    }

    // .schema.Metadata metadata = 1;

    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    pub fn has_metadata(&self) -> bool {
        self.metadata.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: Metadata) {
        self.metadata = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metadata(&mut self) -> &mut Metadata {
        if self.metadata.is_none() {
            self.metadata.set_default();
        }
        self.metadata.as_mut().unwrap()
    }

    // Take field
    pub fn take_metadata(&mut self) -> Metadata {
        self.metadata.take().unwrap_or_else(|| Metadata::new())
    }

    pub fn get_metadata(&self) -> &Metadata {
        self.metadata.as_ref().unwrap_or_else(|| Metadata::default_instance())
    }

    fn get_metadata_for_reflect(&self) -> &::protobuf::SingularPtrField<Metadata> {
        &self.metadata
    }

    fn mut_metadata_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Metadata> {
        &mut self.metadata
    }
}

impl ::protobuf::Message for WriteStreamResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.metadata {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.metadata)?;
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
        if let Some(ref v) = self.metadata.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.metadata.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WriteStreamResponse {
    fn new() -> WriteStreamResponse {
        WriteStreamResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<WriteStreamResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Metadata>>(
                    "metadata",
                    WriteStreamResponse::get_metadata_for_reflect,
                    WriteStreamResponse::mut_metadata_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WriteStreamResponse>(
                    "WriteStreamResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WriteStreamResponse {
    fn clear(&mut self) {
        self.clear_metadata();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WriteStreamResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WriteStreamResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReadRequest {
    // message oneof groups
    input: ::std::option::Option<ReadRequest_oneof_input>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReadRequest {}

#[derive(Clone,PartialEq)]
pub enum ReadRequest_oneof_input {
    key(::std::vec::Vec<u8>),
    metadata(Metadata),
}

impl ReadRequest {
    pub fn new() -> ReadRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReadRequest {
        static mut instance: ::protobuf::lazy::Lazy<ReadRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReadRequest,
        };
        unsafe {
            instance.get(ReadRequest::new)
        }
    }

    // bytes key = 1;

    pub fn clear_key(&mut self) {
        self.input = ::std::option::Option::None;
    }

    pub fn has_key(&self) -> bool {
        match self.input {
            ::std::option::Option::Some(ReadRequest_oneof_input::key(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.input = ::std::option::Option::Some(ReadRequest_oneof_input::key(v))
    }

    // Mutable pointer to the field.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(ReadRequest_oneof_input::key(_)) = self.input {
        } else {
            self.input = ::std::option::Option::Some(ReadRequest_oneof_input::key(::std::vec::Vec::new()));
        }
        match self.input {
            ::std::option::Option::Some(ReadRequest_oneof_input::key(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_key() {
            match self.input.take() {
                ::std::option::Option::Some(ReadRequest_oneof_input::key(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    pub fn get_key(&self) -> &[u8] {
        match self.input {
            ::std::option::Option::Some(ReadRequest_oneof_input::key(ref v)) => v,
            _ => &[],
        }
    }

    // .schema.Metadata metadata = 2;

    pub fn clear_metadata(&mut self) {
        self.input = ::std::option::Option::None;
    }

    pub fn has_metadata(&self) -> bool {
        match self.input {
            ::std::option::Option::Some(ReadRequest_oneof_input::metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: Metadata) {
        self.input = ::std::option::Option::Some(ReadRequest_oneof_input::metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_metadata(&mut self) -> &mut Metadata {
        if let ::std::option::Option::Some(ReadRequest_oneof_input::metadata(_)) = self.input {
        } else {
            self.input = ::std::option::Option::Some(ReadRequest_oneof_input::metadata(Metadata::new()));
        }
        match self.input {
            ::std::option::Option::Some(ReadRequest_oneof_input::metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_metadata(&mut self) -> Metadata {
        if self.has_metadata() {
            match self.input.take() {
                ::std::option::Option::Some(ReadRequest_oneof_input::metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            Metadata::new()
        }
    }

    pub fn get_metadata(&self) -> &Metadata {
        match self.input {
            ::std::option::Option::Some(ReadRequest_oneof_input::metadata(ref v)) => v,
            _ => Metadata::default_instance(),
        }
    }
}

impl ::protobuf::Message for ReadRequest {
    fn is_initialized(&self) -> bool {
        if let Some(ReadRequest_oneof_input::metadata(ref v)) = self.input {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.input = ::std::option::Option::Some(ReadRequest_oneof_input::key(is.read_bytes()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.input = ::std::option::Option::Some(ReadRequest_oneof_input::metadata(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.input {
            match v {
                &ReadRequest_oneof_input::key(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(1, &v);
                },
                &ReadRequest_oneof_input::metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.input {
            match v {
                &ReadRequest_oneof_input::key(ref v) => {
                    os.write_bytes(1, v)?;
                },
                &ReadRequest_oneof_input::metadata(ref v) => {
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReadRequest {
    fn new() -> ReadRequest {
        ReadRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReadRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor::<_>(
                    "key",
                    ReadRequest::has_key,
                    ReadRequest::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Metadata>(
                    "metadata",
                    ReadRequest::has_metadata,
                    ReadRequest::get_metadata,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReadRequest>(
                    "ReadRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReadRequest {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_metadata();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReadRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReadRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReadResponse {
    // message fields
    pub data: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReadResponse {}

impl ReadResponse {
    pub fn new() -> ReadResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReadResponse {
        static mut instance: ::protobuf::lazy::Lazy<ReadResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReadResponse,
        };
        unsafe {
            instance.get(ReadResponse::new)
        }
    }

    // bytes data = 1;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }
}

impl ::protobuf::Message for ReadResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
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
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.data.is_empty() {
            os.write_bytes(1, &self.data)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReadResponse {
    fn new() -> ReadResponse {
        ReadResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReadResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    ReadResponse::get_data_for_reflect,
                    ReadResponse::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReadResponse>(
                    "ReadResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReadResponse {
    fn clear(&mut self) {
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReadResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReadResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReadFileRequest {
    // message fields
    pub filePath: ::std::string::String,
    pub fileMode: FileMode,
    pub synchronousIO: bool,
    // message oneof groups
    input: ::std::option::Option<ReadFileRequest_oneof_input>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReadFileRequest {}

#[derive(Clone,PartialEq)]
pub enum ReadFileRequest_oneof_input {
    key(::std::vec::Vec<u8>),
    metadata(Metadata),
}

impl ReadFileRequest {
    pub fn new() -> ReadFileRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReadFileRequest {
        static mut instance: ::protobuf::lazy::Lazy<ReadFileRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReadFileRequest,
        };
        unsafe {
            instance.get(ReadFileRequest::new)
        }
    }

    // bytes key = 1;

    pub fn clear_key(&mut self) {
        self.input = ::std::option::Option::None;
    }

    pub fn has_key(&self) -> bool {
        match self.input {
            ::std::option::Option::Some(ReadFileRequest_oneof_input::key(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.input = ::std::option::Option::Some(ReadFileRequest_oneof_input::key(v))
    }

    // Mutable pointer to the field.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(ReadFileRequest_oneof_input::key(_)) = self.input {
        } else {
            self.input = ::std::option::Option::Some(ReadFileRequest_oneof_input::key(::std::vec::Vec::new()));
        }
        match self.input {
            ::std::option::Option::Some(ReadFileRequest_oneof_input::key(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_key() {
            match self.input.take() {
                ::std::option::Option::Some(ReadFileRequest_oneof_input::key(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    pub fn get_key(&self) -> &[u8] {
        match self.input {
            ::std::option::Option::Some(ReadFileRequest_oneof_input::key(ref v)) => v,
            _ => &[],
        }
    }

    // .schema.Metadata metadata = 2;

    pub fn clear_metadata(&mut self) {
        self.input = ::std::option::Option::None;
    }

    pub fn has_metadata(&self) -> bool {
        match self.input {
            ::std::option::Option::Some(ReadFileRequest_oneof_input::metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: Metadata) {
        self.input = ::std::option::Option::Some(ReadFileRequest_oneof_input::metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_metadata(&mut self) -> &mut Metadata {
        if let ::std::option::Option::Some(ReadFileRequest_oneof_input::metadata(_)) = self.input {
        } else {
            self.input = ::std::option::Option::Some(ReadFileRequest_oneof_input::metadata(Metadata::new()));
        }
        match self.input {
            ::std::option::Option::Some(ReadFileRequest_oneof_input::metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_metadata(&mut self) -> Metadata {
        if self.has_metadata() {
            match self.input.take() {
                ::std::option::Option::Some(ReadFileRequest_oneof_input::metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            Metadata::new()
        }
    }

    pub fn get_metadata(&self) -> &Metadata {
        match self.input {
            ::std::option::Option::Some(ReadFileRequest_oneof_input::metadata(ref v)) => v,
            _ => Metadata::default_instance(),
        }
    }

    // string filePath = 3;

    pub fn clear_filePath(&mut self) {
        self.filePath.clear();
    }

    // Param is passed by value, moved
    pub fn set_filePath(&mut self, v: ::std::string::String) {
        self.filePath = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_filePath(&mut self) -> &mut ::std::string::String {
        &mut self.filePath
    }

    // Take field
    pub fn take_filePath(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.filePath, ::std::string::String::new())
    }

    pub fn get_filePath(&self) -> &str {
        &self.filePath
    }

    fn get_filePath_for_reflect(&self) -> &::std::string::String {
        &self.filePath
    }

    fn mut_filePath_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.filePath
    }

    // .schema.FileMode fileMode = 4;

    pub fn clear_fileMode(&mut self) {
        self.fileMode = FileMode::FileModeTruncate;
    }

    // Param is passed by value, moved
    pub fn set_fileMode(&mut self, v: FileMode) {
        self.fileMode = v;
    }

    pub fn get_fileMode(&self) -> FileMode {
        self.fileMode
    }

    fn get_fileMode_for_reflect(&self) -> &FileMode {
        &self.fileMode
    }

    fn mut_fileMode_for_reflect(&mut self) -> &mut FileMode {
        &mut self.fileMode
    }

    // bool synchronousIO = 5;

    pub fn clear_synchronousIO(&mut self) {
        self.synchronousIO = false;
    }

    // Param is passed by value, moved
    pub fn set_synchronousIO(&mut self, v: bool) {
        self.synchronousIO = v;
    }

    pub fn get_synchronousIO(&self) -> bool {
        self.synchronousIO
    }

    fn get_synchronousIO_for_reflect(&self) -> &bool {
        &self.synchronousIO
    }

    fn mut_synchronousIO_for_reflect(&mut self) -> &mut bool {
        &mut self.synchronousIO
    }
}

impl ::protobuf::Message for ReadFileRequest {
    fn is_initialized(&self) -> bool {
        if let Some(ReadFileRequest_oneof_input::metadata(ref v)) = self.input {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.input = ::std::option::Option::Some(ReadFileRequest_oneof_input::key(is.read_bytes()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.input = ::std::option::Option::Some(ReadFileRequest_oneof_input::metadata(is.read_message()?));
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.filePath)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.fileMode = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.synchronousIO = tmp;
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
        if !self.filePath.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.filePath);
        }
        if self.fileMode != FileMode::FileModeTruncate {
            my_size += ::protobuf::rt::enum_size(4, self.fileMode);
        }
        if self.synchronousIO != false {
            my_size += 2;
        }
        if let ::std::option::Option::Some(ref v) = self.input {
            match v {
                &ReadFileRequest_oneof_input::key(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(1, &v);
                },
                &ReadFileRequest_oneof_input::metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.filePath.is_empty() {
            os.write_string(3, &self.filePath)?;
        }
        if self.fileMode != FileMode::FileModeTruncate {
            os.write_enum(4, self.fileMode.value())?;
        }
        if self.synchronousIO != false {
            os.write_bool(5, self.synchronousIO)?;
        }
        if let ::std::option::Option::Some(ref v) = self.input {
            match v {
                &ReadFileRequest_oneof_input::key(ref v) => {
                    os.write_bytes(1, v)?;
                },
                &ReadFileRequest_oneof_input::metadata(ref v) => {
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReadFileRequest {
    fn new() -> ReadFileRequest {
        ReadFileRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReadFileRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor::<_>(
                    "key",
                    ReadFileRequest::has_key,
                    ReadFileRequest::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Metadata>(
                    "metadata",
                    ReadFileRequest::has_metadata,
                    ReadFileRequest::get_metadata,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "filePath",
                    ReadFileRequest::get_filePath_for_reflect,
                    ReadFileRequest::mut_filePath_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<FileMode>>(
                    "fileMode",
                    ReadFileRequest::get_fileMode_for_reflect,
                    ReadFileRequest::mut_fileMode_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "synchronousIO",
                    ReadFileRequest::get_synchronousIO_for_reflect,
                    ReadFileRequest::mut_synchronousIO_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReadFileRequest>(
                    "ReadFileRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReadFileRequest {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_metadata();
        self.clear_filePath();
        self.clear_fileMode();
        self.clear_synchronousIO();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReadFileRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReadFileRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReadFileResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReadFileResponse {}

impl ReadFileResponse {
    pub fn new() -> ReadFileResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReadFileResponse {
        static mut instance: ::protobuf::lazy::Lazy<ReadFileResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReadFileResponse,
        };
        unsafe {
            instance.get(ReadFileResponse::new)
        }
    }
}

impl ::protobuf::Message for ReadFileResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReadFileResponse {
    fn new() -> ReadFileResponse {
        ReadFileResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReadFileResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ReadFileResponse>(
                    "ReadFileResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReadFileResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReadFileResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReadFileResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReadStreamRequest {
    // message fields
    pub chunkSize: i64,
    // message oneof groups
    input: ::std::option::Option<ReadStreamRequest_oneof_input>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReadStreamRequest {}

#[derive(Clone,PartialEq)]
pub enum ReadStreamRequest_oneof_input {
    key(::std::vec::Vec<u8>),
    metadata(Metadata),
}

impl ReadStreamRequest {
    pub fn new() -> ReadStreamRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReadStreamRequest {
        static mut instance: ::protobuf::lazy::Lazy<ReadStreamRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReadStreamRequest,
        };
        unsafe {
            instance.get(ReadStreamRequest::new)
        }
    }

    // bytes key = 1;

    pub fn clear_key(&mut self) {
        self.input = ::std::option::Option::None;
    }

    pub fn has_key(&self) -> bool {
        match self.input {
            ::std::option::Option::Some(ReadStreamRequest_oneof_input::key(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.input = ::std::option::Option::Some(ReadStreamRequest_oneof_input::key(v))
    }

    // Mutable pointer to the field.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(ReadStreamRequest_oneof_input::key(_)) = self.input {
        } else {
            self.input = ::std::option::Option::Some(ReadStreamRequest_oneof_input::key(::std::vec::Vec::new()));
        }
        match self.input {
            ::std::option::Option::Some(ReadStreamRequest_oneof_input::key(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_key() {
            match self.input.take() {
                ::std::option::Option::Some(ReadStreamRequest_oneof_input::key(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    pub fn get_key(&self) -> &[u8] {
        match self.input {
            ::std::option::Option::Some(ReadStreamRequest_oneof_input::key(ref v)) => v,
            _ => &[],
        }
    }

    // .schema.Metadata metadata = 2;

    pub fn clear_metadata(&mut self) {
        self.input = ::std::option::Option::None;
    }

    pub fn has_metadata(&self) -> bool {
        match self.input {
            ::std::option::Option::Some(ReadStreamRequest_oneof_input::metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: Metadata) {
        self.input = ::std::option::Option::Some(ReadStreamRequest_oneof_input::metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_metadata(&mut self) -> &mut Metadata {
        if let ::std::option::Option::Some(ReadStreamRequest_oneof_input::metadata(_)) = self.input {
        } else {
            self.input = ::std::option::Option::Some(ReadStreamRequest_oneof_input::metadata(Metadata::new()));
        }
        match self.input {
            ::std::option::Option::Some(ReadStreamRequest_oneof_input::metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_metadata(&mut self) -> Metadata {
        if self.has_metadata() {
            match self.input.take() {
                ::std::option::Option::Some(ReadStreamRequest_oneof_input::metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            Metadata::new()
        }
    }

    pub fn get_metadata(&self) -> &Metadata {
        match self.input {
            ::std::option::Option::Some(ReadStreamRequest_oneof_input::metadata(ref v)) => v,
            _ => Metadata::default_instance(),
        }
    }

    // int64 chunkSize = 3;

    pub fn clear_chunkSize(&mut self) {
        self.chunkSize = 0;
    }

    // Param is passed by value, moved
    pub fn set_chunkSize(&mut self, v: i64) {
        self.chunkSize = v;
    }

    pub fn get_chunkSize(&self) -> i64 {
        self.chunkSize
    }

    fn get_chunkSize_for_reflect(&self) -> &i64 {
        &self.chunkSize
    }

    fn mut_chunkSize_for_reflect(&mut self) -> &mut i64 {
        &mut self.chunkSize
    }
}

impl ::protobuf::Message for ReadStreamRequest {
    fn is_initialized(&self) -> bool {
        if let Some(ReadStreamRequest_oneof_input::metadata(ref v)) = self.input {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.input = ::std::option::Option::Some(ReadStreamRequest_oneof_input::key(is.read_bytes()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.input = ::std::option::Option::Some(ReadStreamRequest_oneof_input::metadata(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.chunkSize = tmp;
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
        if self.chunkSize != 0 {
            my_size += ::protobuf::rt::value_size(3, self.chunkSize, ::protobuf::wire_format::WireTypeVarint);
        }
        if let ::std::option::Option::Some(ref v) = self.input {
            match v {
                &ReadStreamRequest_oneof_input::key(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(1, &v);
                },
                &ReadStreamRequest_oneof_input::metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.chunkSize != 0 {
            os.write_int64(3, self.chunkSize)?;
        }
        if let ::std::option::Option::Some(ref v) = self.input {
            match v {
                &ReadStreamRequest_oneof_input::key(ref v) => {
                    os.write_bytes(1, v)?;
                },
                &ReadStreamRequest_oneof_input::metadata(ref v) => {
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReadStreamRequest {
    fn new() -> ReadStreamRequest {
        ReadStreamRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReadStreamRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor::<_>(
                    "key",
                    ReadStreamRequest::has_key,
                    ReadStreamRequest::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Metadata>(
                    "metadata",
                    ReadStreamRequest::has_metadata,
                    ReadStreamRequest::get_metadata,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "chunkSize",
                    ReadStreamRequest::get_chunkSize_for_reflect,
                    ReadStreamRequest::mut_chunkSize_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReadStreamRequest>(
                    "ReadStreamRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReadStreamRequest {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_metadata();
        self.clear_chunkSize();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReadStreamRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReadStreamRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReadStreamResponse {
    // message fields
    pub dataChunk: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReadStreamResponse {}

impl ReadStreamResponse {
    pub fn new() -> ReadStreamResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReadStreamResponse {
        static mut instance: ::protobuf::lazy::Lazy<ReadStreamResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReadStreamResponse,
        };
        unsafe {
            instance.get(ReadStreamResponse::new)
        }
    }

    // bytes dataChunk = 1;

    pub fn clear_dataChunk(&mut self) {
        self.dataChunk.clear();
    }

    // Param is passed by value, moved
    pub fn set_dataChunk(&mut self, v: ::std::vec::Vec<u8>) {
        self.dataChunk = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dataChunk(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.dataChunk
    }

    // Take field
    pub fn take_dataChunk(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.dataChunk, ::std::vec::Vec::new())
    }

    pub fn get_dataChunk(&self) -> &[u8] {
        &self.dataChunk
    }

    fn get_dataChunk_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.dataChunk
    }

    fn mut_dataChunk_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.dataChunk
    }
}

impl ::protobuf::Message for ReadStreamResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.dataChunk)?;
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
        if !self.dataChunk.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.dataChunk);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.dataChunk.is_empty() {
            os.write_bytes(1, &self.dataChunk)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReadStreamResponse {
    fn new() -> ReadStreamResponse {
        ReadStreamResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReadStreamResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "dataChunk",
                    ReadStreamResponse::get_dataChunk_for_reflect,
                    ReadStreamResponse::mut_dataChunk_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReadStreamResponse>(
                    "ReadStreamResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReadStreamResponse {
    fn clear(&mut self) {
        self.clear_dataChunk();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReadStreamResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReadStreamResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteRequest {
    // message oneof groups
    input: ::std::option::Option<DeleteRequest_oneof_input>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeleteRequest {}

#[derive(Clone,PartialEq)]
pub enum DeleteRequest_oneof_input {
    key(::std::vec::Vec<u8>),
    metadata(Metadata),
}

impl DeleteRequest {
    pub fn new() -> DeleteRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteRequest {
        static mut instance: ::protobuf::lazy::Lazy<DeleteRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteRequest,
        };
        unsafe {
            instance.get(DeleteRequest::new)
        }
    }

    // bytes key = 1;

    pub fn clear_key(&mut self) {
        self.input = ::std::option::Option::None;
    }

    pub fn has_key(&self) -> bool {
        match self.input {
            ::std::option::Option::Some(DeleteRequest_oneof_input::key(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.input = ::std::option::Option::Some(DeleteRequest_oneof_input::key(v))
    }

    // Mutable pointer to the field.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(DeleteRequest_oneof_input::key(_)) = self.input {
        } else {
            self.input = ::std::option::Option::Some(DeleteRequest_oneof_input::key(::std::vec::Vec::new()));
        }
        match self.input {
            ::std::option::Option::Some(DeleteRequest_oneof_input::key(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_key() {
            match self.input.take() {
                ::std::option::Option::Some(DeleteRequest_oneof_input::key(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    pub fn get_key(&self) -> &[u8] {
        match self.input {
            ::std::option::Option::Some(DeleteRequest_oneof_input::key(ref v)) => v,
            _ => &[],
        }
    }

    // .schema.Metadata metadata = 2;

    pub fn clear_metadata(&mut self) {
        self.input = ::std::option::Option::None;
    }

    pub fn has_metadata(&self) -> bool {
        match self.input {
            ::std::option::Option::Some(DeleteRequest_oneof_input::metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: Metadata) {
        self.input = ::std::option::Option::Some(DeleteRequest_oneof_input::metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_metadata(&mut self) -> &mut Metadata {
        if let ::std::option::Option::Some(DeleteRequest_oneof_input::metadata(_)) = self.input {
        } else {
            self.input = ::std::option::Option::Some(DeleteRequest_oneof_input::metadata(Metadata::new()));
        }
        match self.input {
            ::std::option::Option::Some(DeleteRequest_oneof_input::metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_metadata(&mut self) -> Metadata {
        if self.has_metadata() {
            match self.input.take() {
                ::std::option::Option::Some(DeleteRequest_oneof_input::metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            Metadata::new()
        }
    }

    pub fn get_metadata(&self) -> &Metadata {
        match self.input {
            ::std::option::Option::Some(DeleteRequest_oneof_input::metadata(ref v)) => v,
            _ => Metadata::default_instance(),
        }
    }
}

impl ::protobuf::Message for DeleteRequest {
    fn is_initialized(&self) -> bool {
        if let Some(DeleteRequest_oneof_input::metadata(ref v)) = self.input {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.input = ::std::option::Option::Some(DeleteRequest_oneof_input::key(is.read_bytes()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.input = ::std::option::Option::Some(DeleteRequest_oneof_input::metadata(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.input {
            match v {
                &DeleteRequest_oneof_input::key(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(1, &v);
                },
                &DeleteRequest_oneof_input::metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.input {
            match v {
                &DeleteRequest_oneof_input::key(ref v) => {
                    os.write_bytes(1, v)?;
                },
                &DeleteRequest_oneof_input::metadata(ref v) => {
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DeleteRequest {
    fn new() -> DeleteRequest {
        DeleteRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor::<_>(
                    "key",
                    DeleteRequest::has_key,
                    DeleteRequest::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Metadata>(
                    "metadata",
                    DeleteRequest::has_metadata,
                    DeleteRequest::get_metadata,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeleteRequest>(
                    "DeleteRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteRequest {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_metadata();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeleteRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeleteResponse {}

impl DeleteResponse {
    pub fn new() -> DeleteResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteResponse {
        static mut instance: ::protobuf::lazy::Lazy<DeleteResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteResponse,
        };
        unsafe {
            instance.get(DeleteResponse::new)
        }
    }
}

impl ::protobuf::Message for DeleteResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DeleteResponse {
    fn new() -> DeleteResponse {
        DeleteResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<DeleteResponse>(
                    "DeleteResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeleteResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CheckRequest {
    // message fields
    pub fast: bool,
    // message oneof groups
    input: ::std::option::Option<CheckRequest_oneof_input>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CheckRequest {}

#[derive(Clone,PartialEq)]
pub enum CheckRequest_oneof_input {
    key(::std::vec::Vec<u8>),
    metadata(Metadata),
}

impl CheckRequest {
    pub fn new() -> CheckRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CheckRequest {
        static mut instance: ::protobuf::lazy::Lazy<CheckRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CheckRequest,
        };
        unsafe {
            instance.get(CheckRequest::new)
        }
    }

    // bytes key = 1;

    pub fn clear_key(&mut self) {
        self.input = ::std::option::Option::None;
    }

    pub fn has_key(&self) -> bool {
        match self.input {
            ::std::option::Option::Some(CheckRequest_oneof_input::key(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.input = ::std::option::Option::Some(CheckRequest_oneof_input::key(v))
    }

    // Mutable pointer to the field.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(CheckRequest_oneof_input::key(_)) = self.input {
        } else {
            self.input = ::std::option::Option::Some(CheckRequest_oneof_input::key(::std::vec::Vec::new()));
        }
        match self.input {
            ::std::option::Option::Some(CheckRequest_oneof_input::key(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_key() {
            match self.input.take() {
                ::std::option::Option::Some(CheckRequest_oneof_input::key(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    pub fn get_key(&self) -> &[u8] {
        match self.input {
            ::std::option::Option::Some(CheckRequest_oneof_input::key(ref v)) => v,
            _ => &[],
        }
    }

    // .schema.Metadata metadata = 2;

    pub fn clear_metadata(&mut self) {
        self.input = ::std::option::Option::None;
    }

    pub fn has_metadata(&self) -> bool {
        match self.input {
            ::std::option::Option::Some(CheckRequest_oneof_input::metadata(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: Metadata) {
        self.input = ::std::option::Option::Some(CheckRequest_oneof_input::metadata(v))
    }

    // Mutable pointer to the field.
    pub fn mut_metadata(&mut self) -> &mut Metadata {
        if let ::std::option::Option::Some(CheckRequest_oneof_input::metadata(_)) = self.input {
        } else {
            self.input = ::std::option::Option::Some(CheckRequest_oneof_input::metadata(Metadata::new()));
        }
        match self.input {
            ::std::option::Option::Some(CheckRequest_oneof_input::metadata(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_metadata(&mut self) -> Metadata {
        if self.has_metadata() {
            match self.input.take() {
                ::std::option::Option::Some(CheckRequest_oneof_input::metadata(v)) => v,
                _ => panic!(),
            }
        } else {
            Metadata::new()
        }
    }

    pub fn get_metadata(&self) -> &Metadata {
        match self.input {
            ::std::option::Option::Some(CheckRequest_oneof_input::metadata(ref v)) => v,
            _ => Metadata::default_instance(),
        }
    }

    // bool fast = 3;

    pub fn clear_fast(&mut self) {
        self.fast = false;
    }

    // Param is passed by value, moved
    pub fn set_fast(&mut self, v: bool) {
        self.fast = v;
    }

    pub fn get_fast(&self) -> bool {
        self.fast
    }

    fn get_fast_for_reflect(&self) -> &bool {
        &self.fast
    }

    fn mut_fast_for_reflect(&mut self) -> &mut bool {
        &mut self.fast
    }
}

impl ::protobuf::Message for CheckRequest {
    fn is_initialized(&self) -> bool {
        if let Some(CheckRequest_oneof_input::metadata(ref v)) = self.input {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.input = ::std::option::Option::Some(CheckRequest_oneof_input::key(is.read_bytes()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.input = ::std::option::Option::Some(CheckRequest_oneof_input::metadata(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.fast = tmp;
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
        if self.fast != false {
            my_size += 2;
        }
        if let ::std::option::Option::Some(ref v) = self.input {
            match v {
                &CheckRequest_oneof_input::key(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(1, &v);
                },
                &CheckRequest_oneof_input::metadata(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.fast != false {
            os.write_bool(3, self.fast)?;
        }
        if let ::std::option::Option::Some(ref v) = self.input {
            match v {
                &CheckRequest_oneof_input::key(ref v) => {
                    os.write_bytes(1, v)?;
                },
                &CheckRequest_oneof_input::metadata(ref v) => {
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CheckRequest {
    fn new() -> CheckRequest {
        CheckRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CheckRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor::<_>(
                    "key",
                    CheckRequest::has_key,
                    CheckRequest::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Metadata>(
                    "metadata",
                    CheckRequest::has_metadata,
                    CheckRequest::get_metadata,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "fast",
                    CheckRequest::get_fast_for_reflect,
                    CheckRequest::mut_fast_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CheckRequest>(
                    "CheckRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CheckRequest {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_metadata();
        self.clear_fast();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CheckRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CheckRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CheckResponse {
    // message fields
    pub status: CheckStatus,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CheckResponse {}

impl CheckResponse {
    pub fn new() -> CheckResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CheckResponse {
        static mut instance: ::protobuf::lazy::Lazy<CheckResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CheckResponse,
        };
        unsafe {
            instance.get(CheckResponse::new)
        }
    }

    // .schema.CheckStatus status = 1;

    pub fn clear_status(&mut self) {
        self.status = CheckStatus::CheckStatusInvalid;
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: CheckStatus) {
        self.status = v;
    }

    pub fn get_status(&self) -> CheckStatus {
        self.status
    }

    fn get_status_for_reflect(&self) -> &CheckStatus {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut CheckStatus {
        &mut self.status
    }
}

impl ::protobuf::Message for CheckResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.status = tmp;
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
        if self.status != CheckStatus::CheckStatusInvalid {
            my_size += ::protobuf::rt::enum_size(1, self.status);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.status != CheckStatus::CheckStatusInvalid {
            os.write_enum(1, self.status.value())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CheckResponse {
    fn new() -> CheckResponse {
        CheckResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CheckResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CheckStatus>>(
                    "status",
                    CheckResponse::get_status_for_reflect,
                    CheckResponse::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CheckResponse>(
                    "CheckResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CheckResponse {
    fn clear(&mut self) {
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CheckResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CheckResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RepairRequest {
    // message fields
    pub key: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RepairRequest {}

impl RepairRequest {
    pub fn new() -> RepairRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RepairRequest {
        static mut instance: ::protobuf::lazy::Lazy<RepairRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RepairRequest,
        };
        unsafe {
            instance.get(RepairRequest::new)
        }
    }

    // bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }
}

impl ::protobuf::Message for RepairRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
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
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.key);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_bytes(1, &self.key)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RepairRequest {
    fn new() -> RepairRequest {
        RepairRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<RepairRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    RepairRequest::get_key_for_reflect,
                    RepairRequest::mut_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RepairRequest>(
                    "RepairRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RepairRequest {
    fn clear(&mut self) {
        self.clear_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RepairRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RepairRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RepairResponse {
    // message fields
    pub metadata: ::protobuf::SingularPtrField<Metadata>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RepairResponse {}

impl RepairResponse {
    pub fn new() -> RepairResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RepairResponse {
        static mut instance: ::protobuf::lazy::Lazy<RepairResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RepairResponse,
        };
        unsafe {
            instance.get(RepairResponse::new)
        }
    }

    // .schema.Metadata metadata = 1;

    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    pub fn has_metadata(&self) -> bool {
        self.metadata.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: Metadata) {
        self.metadata = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metadata(&mut self) -> &mut Metadata {
        if self.metadata.is_none() {
            self.metadata.set_default();
        }
        self.metadata.as_mut().unwrap()
    }

    // Take field
    pub fn take_metadata(&mut self) -> Metadata {
        self.metadata.take().unwrap_or_else(|| Metadata::new())
    }

    pub fn get_metadata(&self) -> &Metadata {
        self.metadata.as_ref().unwrap_or_else(|| Metadata::default_instance())
    }

    fn get_metadata_for_reflect(&self) -> &::protobuf::SingularPtrField<Metadata> {
        &self.metadata
    }

    fn mut_metadata_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Metadata> {
        &mut self.metadata
    }
}

impl ::protobuf::Message for RepairResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.metadata {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.metadata)?;
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
        if let Some(ref v) = self.metadata.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.metadata.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RepairResponse {
    fn new() -> RepairResponse {
        RepairResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<RepairResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Metadata>>(
                    "metadata",
                    RepairResponse::get_metadata_for_reflect,
                    RepairResponse::mut_metadata_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RepairResponse>(
                    "RepairResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RepairResponse {
    fn clear(&mut self) {
        self.clear_metadata();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RepairResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RepairResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetMetadataRequest {
    // message fields
    pub metadata: ::protobuf::SingularPtrField<Metadata>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetMetadataRequest {}

impl SetMetadataRequest {
    pub fn new() -> SetMetadataRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetMetadataRequest {
        static mut instance: ::protobuf::lazy::Lazy<SetMetadataRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetMetadataRequest,
        };
        unsafe {
            instance.get(SetMetadataRequest::new)
        }
    }

    // .schema.Metadata metadata = 1;

    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    pub fn has_metadata(&self) -> bool {
        self.metadata.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: Metadata) {
        self.metadata = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metadata(&mut self) -> &mut Metadata {
        if self.metadata.is_none() {
            self.metadata.set_default();
        }
        self.metadata.as_mut().unwrap()
    }

    // Take field
    pub fn take_metadata(&mut self) -> Metadata {
        self.metadata.take().unwrap_or_else(|| Metadata::new())
    }

    pub fn get_metadata(&self) -> &Metadata {
        self.metadata.as_ref().unwrap_or_else(|| Metadata::default_instance())
    }

    fn get_metadata_for_reflect(&self) -> &::protobuf::SingularPtrField<Metadata> {
        &self.metadata
    }

    fn mut_metadata_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Metadata> {
        &mut self.metadata
    }
}

impl ::protobuf::Message for SetMetadataRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.metadata {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.metadata)?;
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
        if let Some(ref v) = self.metadata.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.metadata.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SetMetadataRequest {
    fn new() -> SetMetadataRequest {
        SetMetadataRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetMetadataRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Metadata>>(
                    "metadata",
                    SetMetadataRequest::get_metadata_for_reflect,
                    SetMetadataRequest::mut_metadata_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetMetadataRequest>(
                    "SetMetadataRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetMetadataRequest {
    fn clear(&mut self) {
        self.clear_metadata();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SetMetadataRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetMetadataRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetMetadataResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetMetadataResponse {}

impl SetMetadataResponse {
    pub fn new() -> SetMetadataResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetMetadataResponse {
        static mut instance: ::protobuf::lazy::Lazy<SetMetadataResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetMetadataResponse,
        };
        unsafe {
            instance.get(SetMetadataResponse::new)
        }
    }
}

impl ::protobuf::Message for SetMetadataResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SetMetadataResponse {
    fn new() -> SetMetadataResponse {
        SetMetadataResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetMetadataResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<SetMetadataResponse>(
                    "SetMetadataResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetMetadataResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SetMetadataResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetMetadataResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetMetadataRequest {
    // message fields
    pub key: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetMetadataRequest {}

impl GetMetadataRequest {
    pub fn new() -> GetMetadataRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetMetadataRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetMetadataRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetMetadataRequest,
        };
        unsafe {
            instance.get(GetMetadataRequest::new)
        }
    }

    // bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }
}

impl ::protobuf::Message for GetMetadataRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
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
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.key);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_bytes(1, &self.key)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetMetadataRequest {
    fn new() -> GetMetadataRequest {
        GetMetadataRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetMetadataRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    GetMetadataRequest::get_key_for_reflect,
                    GetMetadataRequest::mut_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetMetadataRequest>(
                    "GetMetadataRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetMetadataRequest {
    fn clear(&mut self) {
        self.clear_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetMetadataRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetMetadataRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetMetadataResponse {
    // message fields
    pub metadata: ::protobuf::SingularPtrField<Metadata>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetMetadataResponse {}

impl GetMetadataResponse {
    pub fn new() -> GetMetadataResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetMetadataResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetMetadataResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetMetadataResponse,
        };
        unsafe {
            instance.get(GetMetadataResponse::new)
        }
    }

    // .schema.Metadata metadata = 1;

    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    pub fn has_metadata(&self) -> bool {
        self.metadata.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: Metadata) {
        self.metadata = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metadata(&mut self) -> &mut Metadata {
        if self.metadata.is_none() {
            self.metadata.set_default();
        }
        self.metadata.as_mut().unwrap()
    }

    // Take field
    pub fn take_metadata(&mut self) -> Metadata {
        self.metadata.take().unwrap_or_else(|| Metadata::new())
    }

    pub fn get_metadata(&self) -> &Metadata {
        self.metadata.as_ref().unwrap_or_else(|| Metadata::default_instance())
    }

    fn get_metadata_for_reflect(&self) -> &::protobuf::SingularPtrField<Metadata> {
        &self.metadata
    }

    fn mut_metadata_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Metadata> {
        &mut self.metadata
    }
}

impl ::protobuf::Message for GetMetadataResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.metadata {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.metadata)?;
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
        if let Some(ref v) = self.metadata.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.metadata.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetMetadataResponse {
    fn new() -> GetMetadataResponse {
        GetMetadataResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetMetadataResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Metadata>>(
                    "metadata",
                    GetMetadataResponse::get_metadata_for_reflect,
                    GetMetadataResponse::mut_metadata_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetMetadataResponse>(
                    "GetMetadataResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetMetadataResponse {
    fn clear(&mut self) {
        self.clear_metadata();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetMetadataResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetMetadataResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteMetadataRequest {
    // message fields
    pub key: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeleteMetadataRequest {}

impl DeleteMetadataRequest {
    pub fn new() -> DeleteMetadataRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteMetadataRequest {
        static mut instance: ::protobuf::lazy::Lazy<DeleteMetadataRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteMetadataRequest,
        };
        unsafe {
            instance.get(DeleteMetadataRequest::new)
        }
    }

    // bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }
}

impl ::protobuf::Message for DeleteMetadataRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
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
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.key);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_bytes(1, &self.key)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DeleteMetadataRequest {
    fn new() -> DeleteMetadataRequest {
        DeleteMetadataRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteMetadataRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    DeleteMetadataRequest::get_key_for_reflect,
                    DeleteMetadataRequest::mut_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeleteMetadataRequest>(
                    "DeleteMetadataRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteMetadataRequest {
    fn clear(&mut self) {
        self.clear_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeleteMetadataRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteMetadataRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteMetadataResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeleteMetadataResponse {}

impl DeleteMetadataResponse {
    pub fn new() -> DeleteMetadataResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteMetadataResponse {
        static mut instance: ::protobuf::lazy::Lazy<DeleteMetadataResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteMetadataResponse,
        };
        unsafe {
            instance.get(DeleteMetadataResponse::new)
        }
    }
}

impl ::protobuf::Message for DeleteMetadataResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DeleteMetadataResponse {
    fn new() -> DeleteMetadataResponse {
        DeleteMetadataResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteMetadataResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<DeleteMetadataResponse>(
                    "DeleteMetadataResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteMetadataResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeleteMetadataResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteMetadataResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DataWriteRequest {
    // message fields
    pub data: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DataWriteRequest {}

impl DataWriteRequest {
    pub fn new() -> DataWriteRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DataWriteRequest {
        static mut instance: ::protobuf::lazy::Lazy<DataWriteRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DataWriteRequest,
        };
        unsafe {
            instance.get(DataWriteRequest::new)
        }
    }

    // bytes data = 1;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }
}

impl ::protobuf::Message for DataWriteRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
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
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.data.is_empty() {
            os.write_bytes(1, &self.data)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DataWriteRequest {
    fn new() -> DataWriteRequest {
        DataWriteRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<DataWriteRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    DataWriteRequest::get_data_for_reflect,
                    DataWriteRequest::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DataWriteRequest>(
                    "DataWriteRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DataWriteRequest {
    fn clear(&mut self) {
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DataWriteRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DataWriteRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DataWriteResponse {
    // message fields
    pub chunks: ::protobuf::RepeatedField<Chunk>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DataWriteResponse {}

impl DataWriteResponse {
    pub fn new() -> DataWriteResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DataWriteResponse {
        static mut instance: ::protobuf::lazy::Lazy<DataWriteResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DataWriteResponse,
        };
        unsafe {
            instance.get(DataWriteResponse::new)
        }
    }

    // repeated .schema.Chunk chunks = 1;

    pub fn clear_chunks(&mut self) {
        self.chunks.clear();
    }

    // Param is passed by value, moved
    pub fn set_chunks(&mut self, v: ::protobuf::RepeatedField<Chunk>) {
        self.chunks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_chunks(&mut self) -> &mut ::protobuf::RepeatedField<Chunk> {
        &mut self.chunks
    }

    // Take field
    pub fn take_chunks(&mut self) -> ::protobuf::RepeatedField<Chunk> {
        ::std::mem::replace(&mut self.chunks, ::protobuf::RepeatedField::new())
    }

    pub fn get_chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    fn get_chunks_for_reflect(&self) -> &::protobuf::RepeatedField<Chunk> {
        &self.chunks
    }

    fn mut_chunks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Chunk> {
        &mut self.chunks
    }
}

impl ::protobuf::Message for DataWriteResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.chunks {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.chunks)?;
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
        for value in &self.chunks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.chunks {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DataWriteResponse {
    fn new() -> DataWriteResponse {
        DataWriteResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<DataWriteResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Chunk>>(
                    "chunks",
                    DataWriteResponse::get_chunks_for_reflect,
                    DataWriteResponse::mut_chunks_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DataWriteResponse>(
                    "DataWriteResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DataWriteResponse {
    fn clear(&mut self) {
        self.clear_chunks();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DataWriteResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DataWriteResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DataWriteFileRequest {
    // message fields
    pub filePath: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DataWriteFileRequest {}

impl DataWriteFileRequest {
    pub fn new() -> DataWriteFileRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DataWriteFileRequest {
        static mut instance: ::protobuf::lazy::Lazy<DataWriteFileRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DataWriteFileRequest,
        };
        unsafe {
            instance.get(DataWriteFileRequest::new)
        }
    }

    // string filePath = 1;

    pub fn clear_filePath(&mut self) {
        self.filePath.clear();
    }

    // Param is passed by value, moved
    pub fn set_filePath(&mut self, v: ::std::string::String) {
        self.filePath = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_filePath(&mut self) -> &mut ::std::string::String {
        &mut self.filePath
    }

    // Take field
    pub fn take_filePath(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.filePath, ::std::string::String::new())
    }

    pub fn get_filePath(&self) -> &str {
        &self.filePath
    }

    fn get_filePath_for_reflect(&self) -> &::std::string::String {
        &self.filePath
    }

    fn mut_filePath_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.filePath
    }
}

impl ::protobuf::Message for DataWriteFileRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.filePath)?;
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
        if !self.filePath.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.filePath);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.filePath.is_empty() {
            os.write_string(1, &self.filePath)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DataWriteFileRequest {
    fn new() -> DataWriteFileRequest {
        DataWriteFileRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<DataWriteFileRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "filePath",
                    DataWriteFileRequest::get_filePath_for_reflect,
                    DataWriteFileRequest::mut_filePath_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DataWriteFileRequest>(
                    "DataWriteFileRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DataWriteFileRequest {
    fn clear(&mut self) {
        self.clear_filePath();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DataWriteFileRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DataWriteFileRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DataWriteFileResponse {
    // message fields
    pub chunks: ::protobuf::RepeatedField<Chunk>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DataWriteFileResponse {}

impl DataWriteFileResponse {
    pub fn new() -> DataWriteFileResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DataWriteFileResponse {
        static mut instance: ::protobuf::lazy::Lazy<DataWriteFileResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DataWriteFileResponse,
        };
        unsafe {
            instance.get(DataWriteFileResponse::new)
        }
    }

    // repeated .schema.Chunk chunks = 1;

    pub fn clear_chunks(&mut self) {
        self.chunks.clear();
    }

    // Param is passed by value, moved
    pub fn set_chunks(&mut self, v: ::protobuf::RepeatedField<Chunk>) {
        self.chunks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_chunks(&mut self) -> &mut ::protobuf::RepeatedField<Chunk> {
        &mut self.chunks
    }

    // Take field
    pub fn take_chunks(&mut self) -> ::protobuf::RepeatedField<Chunk> {
        ::std::mem::replace(&mut self.chunks, ::protobuf::RepeatedField::new())
    }

    pub fn get_chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    fn get_chunks_for_reflect(&self) -> &::protobuf::RepeatedField<Chunk> {
        &self.chunks
    }

    fn mut_chunks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Chunk> {
        &mut self.chunks
    }
}

impl ::protobuf::Message for DataWriteFileResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.chunks {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.chunks)?;
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
        for value in &self.chunks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.chunks {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DataWriteFileResponse {
    fn new() -> DataWriteFileResponse {
        DataWriteFileResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<DataWriteFileResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Chunk>>(
                    "chunks",
                    DataWriteFileResponse::get_chunks_for_reflect,
                    DataWriteFileResponse::mut_chunks_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DataWriteFileResponse>(
                    "DataWriteFileResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DataWriteFileResponse {
    fn clear(&mut self) {
        self.clear_chunks();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DataWriteFileResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DataWriteFileResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DataWriteStreamRequest {
    // message fields
    pub dataChunk: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DataWriteStreamRequest {}

impl DataWriteStreamRequest {
    pub fn new() -> DataWriteStreamRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DataWriteStreamRequest {
        static mut instance: ::protobuf::lazy::Lazy<DataWriteStreamRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DataWriteStreamRequest,
        };
        unsafe {
            instance.get(DataWriteStreamRequest::new)
        }
    }

    // bytes dataChunk = 1;

    pub fn clear_dataChunk(&mut self) {
        self.dataChunk.clear();
    }

    // Param is passed by value, moved
    pub fn set_dataChunk(&mut self, v: ::std::vec::Vec<u8>) {
        self.dataChunk = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dataChunk(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.dataChunk
    }

    // Take field
    pub fn take_dataChunk(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.dataChunk, ::std::vec::Vec::new())
    }

    pub fn get_dataChunk(&self) -> &[u8] {
        &self.dataChunk
    }

    fn get_dataChunk_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.dataChunk
    }

    fn mut_dataChunk_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.dataChunk
    }
}

impl ::protobuf::Message for DataWriteStreamRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.dataChunk)?;
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
        if !self.dataChunk.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.dataChunk);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.dataChunk.is_empty() {
            os.write_bytes(1, &self.dataChunk)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DataWriteStreamRequest {
    fn new() -> DataWriteStreamRequest {
        DataWriteStreamRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<DataWriteStreamRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "dataChunk",
                    DataWriteStreamRequest::get_dataChunk_for_reflect,
                    DataWriteStreamRequest::mut_dataChunk_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DataWriteStreamRequest>(
                    "DataWriteStreamRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DataWriteStreamRequest {
    fn clear(&mut self) {
        self.clear_dataChunk();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DataWriteStreamRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DataWriteStreamRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DataWriteStreamResponse {
    // message fields
    pub chunks: ::protobuf::RepeatedField<Chunk>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DataWriteStreamResponse {}

impl DataWriteStreamResponse {
    pub fn new() -> DataWriteStreamResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DataWriteStreamResponse {
        static mut instance: ::protobuf::lazy::Lazy<DataWriteStreamResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DataWriteStreamResponse,
        };
        unsafe {
            instance.get(DataWriteStreamResponse::new)
        }
    }

    // repeated .schema.Chunk chunks = 1;

    pub fn clear_chunks(&mut self) {
        self.chunks.clear();
    }

    // Param is passed by value, moved
    pub fn set_chunks(&mut self, v: ::protobuf::RepeatedField<Chunk>) {
        self.chunks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_chunks(&mut self) -> &mut ::protobuf::RepeatedField<Chunk> {
        &mut self.chunks
    }

    // Take field
    pub fn take_chunks(&mut self) -> ::protobuf::RepeatedField<Chunk> {
        ::std::mem::replace(&mut self.chunks, ::protobuf::RepeatedField::new())
    }

    pub fn get_chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    fn get_chunks_for_reflect(&self) -> &::protobuf::RepeatedField<Chunk> {
        &self.chunks
    }

    fn mut_chunks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Chunk> {
        &mut self.chunks
    }
}

impl ::protobuf::Message for DataWriteStreamResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.chunks {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.chunks)?;
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
        for value in &self.chunks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.chunks {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DataWriteStreamResponse {
    fn new() -> DataWriteStreamResponse {
        DataWriteStreamResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<DataWriteStreamResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Chunk>>(
                    "chunks",
                    DataWriteStreamResponse::get_chunks_for_reflect,
                    DataWriteStreamResponse::mut_chunks_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DataWriteStreamResponse>(
                    "DataWriteStreamResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DataWriteStreamResponse {
    fn clear(&mut self) {
        self.clear_chunks();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DataWriteStreamResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DataWriteStreamResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DataReadRequest {
    // message fields
    pub chunks: ::protobuf::RepeatedField<Chunk>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DataReadRequest {}

impl DataReadRequest {
    pub fn new() -> DataReadRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DataReadRequest {
        static mut instance: ::protobuf::lazy::Lazy<DataReadRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DataReadRequest,
        };
        unsafe {
            instance.get(DataReadRequest::new)
        }
    }

    // repeated .schema.Chunk chunks = 1;

    pub fn clear_chunks(&mut self) {
        self.chunks.clear();
    }

    // Param is passed by value, moved
    pub fn set_chunks(&mut self, v: ::protobuf::RepeatedField<Chunk>) {
        self.chunks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_chunks(&mut self) -> &mut ::protobuf::RepeatedField<Chunk> {
        &mut self.chunks
    }

    // Take field
    pub fn take_chunks(&mut self) -> ::protobuf::RepeatedField<Chunk> {
        ::std::mem::replace(&mut self.chunks, ::protobuf::RepeatedField::new())
    }

    pub fn get_chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    fn get_chunks_for_reflect(&self) -> &::protobuf::RepeatedField<Chunk> {
        &self.chunks
    }

    fn mut_chunks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Chunk> {
        &mut self.chunks
    }
}

impl ::protobuf::Message for DataReadRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.chunks {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.chunks)?;
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
        for value in &self.chunks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.chunks {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DataReadRequest {
    fn new() -> DataReadRequest {
        DataReadRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<DataReadRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Chunk>>(
                    "chunks",
                    DataReadRequest::get_chunks_for_reflect,
                    DataReadRequest::mut_chunks_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DataReadRequest>(
                    "DataReadRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DataReadRequest {
    fn clear(&mut self) {
        self.clear_chunks();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DataReadRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DataReadRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DataReadResponse {
    // message fields
    pub data: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DataReadResponse {}

impl DataReadResponse {
    pub fn new() -> DataReadResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DataReadResponse {
        static mut instance: ::protobuf::lazy::Lazy<DataReadResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DataReadResponse,
        };
        unsafe {
            instance.get(DataReadResponse::new)
        }
    }

    // bytes data = 1;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }
}

impl ::protobuf::Message for DataReadResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
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
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.data.is_empty() {
            os.write_bytes(1, &self.data)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DataReadResponse {
    fn new() -> DataReadResponse {
        DataReadResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<DataReadResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    DataReadResponse::get_data_for_reflect,
                    DataReadResponse::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DataReadResponse>(
                    "DataReadResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DataReadResponse {
    fn clear(&mut self) {
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DataReadResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DataReadResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DataReadFileRequest {
    // message fields
    pub chunks: ::protobuf::RepeatedField<Chunk>,
    pub filePath: ::std::string::String,
    pub fileMode: FileMode,
    pub synchronousIO: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DataReadFileRequest {}

impl DataReadFileRequest {
    pub fn new() -> DataReadFileRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DataReadFileRequest {
        static mut instance: ::protobuf::lazy::Lazy<DataReadFileRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DataReadFileRequest,
        };
        unsafe {
            instance.get(DataReadFileRequest::new)
        }
    }

    // repeated .schema.Chunk chunks = 1;

    pub fn clear_chunks(&mut self) {
        self.chunks.clear();
    }

    // Param is passed by value, moved
    pub fn set_chunks(&mut self, v: ::protobuf::RepeatedField<Chunk>) {
        self.chunks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_chunks(&mut self) -> &mut ::protobuf::RepeatedField<Chunk> {
        &mut self.chunks
    }

    // Take field
    pub fn take_chunks(&mut self) -> ::protobuf::RepeatedField<Chunk> {
        ::std::mem::replace(&mut self.chunks, ::protobuf::RepeatedField::new())
    }

    pub fn get_chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    fn get_chunks_for_reflect(&self) -> &::protobuf::RepeatedField<Chunk> {
        &self.chunks
    }

    fn mut_chunks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Chunk> {
        &mut self.chunks
    }

    // string filePath = 2;

    pub fn clear_filePath(&mut self) {
        self.filePath.clear();
    }

    // Param is passed by value, moved
    pub fn set_filePath(&mut self, v: ::std::string::String) {
        self.filePath = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_filePath(&mut self) -> &mut ::std::string::String {
        &mut self.filePath
    }

    // Take field
    pub fn take_filePath(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.filePath, ::std::string::String::new())
    }

    pub fn get_filePath(&self) -> &str {
        &self.filePath
    }

    fn get_filePath_for_reflect(&self) -> &::std::string::String {
        &self.filePath
    }

    fn mut_filePath_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.filePath
    }

    // .schema.FileMode fileMode = 3;

    pub fn clear_fileMode(&mut self) {
        self.fileMode = FileMode::FileModeTruncate;
    }

    // Param is passed by value, moved
    pub fn set_fileMode(&mut self, v: FileMode) {
        self.fileMode = v;
    }

    pub fn get_fileMode(&self) -> FileMode {
        self.fileMode
    }

    fn get_fileMode_for_reflect(&self) -> &FileMode {
        &self.fileMode
    }

    fn mut_fileMode_for_reflect(&mut self) -> &mut FileMode {
        &mut self.fileMode
    }

    // bool synchronousIO = 4;

    pub fn clear_synchronousIO(&mut self) {
        self.synchronousIO = false;
    }

    // Param is passed by value, moved
    pub fn set_synchronousIO(&mut self, v: bool) {
        self.synchronousIO = v;
    }

    pub fn get_synchronousIO(&self) -> bool {
        self.synchronousIO
    }

    fn get_synchronousIO_for_reflect(&self) -> &bool {
        &self.synchronousIO
    }

    fn mut_synchronousIO_for_reflect(&mut self) -> &mut bool {
        &mut self.synchronousIO
    }
}

impl ::protobuf::Message for DataReadFileRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.chunks {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.chunks)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.filePath)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.fileMode = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.synchronousIO = tmp;
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
        for value in &self.chunks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.filePath.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.filePath);
        }
        if self.fileMode != FileMode::FileModeTruncate {
            my_size += ::protobuf::rt::enum_size(3, self.fileMode);
        }
        if self.synchronousIO != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.chunks {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.filePath.is_empty() {
            os.write_string(2, &self.filePath)?;
        }
        if self.fileMode != FileMode::FileModeTruncate {
            os.write_enum(3, self.fileMode.value())?;
        }
        if self.synchronousIO != false {
            os.write_bool(4, self.synchronousIO)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DataReadFileRequest {
    fn new() -> DataReadFileRequest {
        DataReadFileRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<DataReadFileRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Chunk>>(
                    "chunks",
                    DataReadFileRequest::get_chunks_for_reflect,
                    DataReadFileRequest::mut_chunks_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "filePath",
                    DataReadFileRequest::get_filePath_for_reflect,
                    DataReadFileRequest::mut_filePath_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<FileMode>>(
                    "fileMode",
                    DataReadFileRequest::get_fileMode_for_reflect,
                    DataReadFileRequest::mut_fileMode_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "synchronousIO",
                    DataReadFileRequest::get_synchronousIO_for_reflect,
                    DataReadFileRequest::mut_synchronousIO_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DataReadFileRequest>(
                    "DataReadFileRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DataReadFileRequest {
    fn clear(&mut self) {
        self.clear_chunks();
        self.clear_filePath();
        self.clear_fileMode();
        self.clear_synchronousIO();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DataReadFileRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DataReadFileRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DataReadFileResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DataReadFileResponse {}

impl DataReadFileResponse {
    pub fn new() -> DataReadFileResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DataReadFileResponse {
        static mut instance: ::protobuf::lazy::Lazy<DataReadFileResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DataReadFileResponse,
        };
        unsafe {
            instance.get(DataReadFileResponse::new)
        }
    }
}

impl ::protobuf::Message for DataReadFileResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DataReadFileResponse {
    fn new() -> DataReadFileResponse {
        DataReadFileResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<DataReadFileResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<DataReadFileResponse>(
                    "DataReadFileResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DataReadFileResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DataReadFileResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DataReadFileResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DataReadStreamRequest {
    // message fields
    pub chunks: ::protobuf::RepeatedField<Chunk>,
    pub chunkSize: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DataReadStreamRequest {}

impl DataReadStreamRequest {
    pub fn new() -> DataReadStreamRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DataReadStreamRequest {
        static mut instance: ::protobuf::lazy::Lazy<DataReadStreamRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DataReadStreamRequest,
        };
        unsafe {
            instance.get(DataReadStreamRequest::new)
        }
    }

    // repeated .schema.Chunk chunks = 1;

    pub fn clear_chunks(&mut self) {
        self.chunks.clear();
    }

    // Param is passed by value, moved
    pub fn set_chunks(&mut self, v: ::protobuf::RepeatedField<Chunk>) {
        self.chunks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_chunks(&mut self) -> &mut ::protobuf::RepeatedField<Chunk> {
        &mut self.chunks
    }

    // Take field
    pub fn take_chunks(&mut self) -> ::protobuf::RepeatedField<Chunk> {
        ::std::mem::replace(&mut self.chunks, ::protobuf::RepeatedField::new())
    }

    pub fn get_chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    fn get_chunks_for_reflect(&self) -> &::protobuf::RepeatedField<Chunk> {
        &self.chunks
    }

    fn mut_chunks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Chunk> {
        &mut self.chunks
    }

    // int64 chunkSize = 2;

    pub fn clear_chunkSize(&mut self) {
        self.chunkSize = 0;
    }

    // Param is passed by value, moved
    pub fn set_chunkSize(&mut self, v: i64) {
        self.chunkSize = v;
    }

    pub fn get_chunkSize(&self) -> i64 {
        self.chunkSize
    }

    fn get_chunkSize_for_reflect(&self) -> &i64 {
        &self.chunkSize
    }

    fn mut_chunkSize_for_reflect(&mut self) -> &mut i64 {
        &mut self.chunkSize
    }
}

impl ::protobuf::Message for DataReadStreamRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.chunks {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.chunks)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.chunkSize = tmp;
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
        for value in &self.chunks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.chunkSize != 0 {
            my_size += ::protobuf::rt::value_size(2, self.chunkSize, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.chunks {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.chunkSize != 0 {
            os.write_int64(2, self.chunkSize)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DataReadStreamRequest {
    fn new() -> DataReadStreamRequest {
        DataReadStreamRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<DataReadStreamRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Chunk>>(
                    "chunks",
                    DataReadStreamRequest::get_chunks_for_reflect,
                    DataReadStreamRequest::mut_chunks_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "chunkSize",
                    DataReadStreamRequest::get_chunkSize_for_reflect,
                    DataReadStreamRequest::mut_chunkSize_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DataReadStreamRequest>(
                    "DataReadStreamRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DataReadStreamRequest {
    fn clear(&mut self) {
        self.clear_chunks();
        self.clear_chunkSize();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DataReadStreamRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DataReadStreamRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DataReadStreamResponse {
    // message fields
    pub dataChunk: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DataReadStreamResponse {}

impl DataReadStreamResponse {
    pub fn new() -> DataReadStreamResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DataReadStreamResponse {
        static mut instance: ::protobuf::lazy::Lazy<DataReadStreamResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DataReadStreamResponse,
        };
        unsafe {
            instance.get(DataReadStreamResponse::new)
        }
    }

    // bytes dataChunk = 1;

    pub fn clear_dataChunk(&mut self) {
        self.dataChunk.clear();
    }

    // Param is passed by value, moved
    pub fn set_dataChunk(&mut self, v: ::std::vec::Vec<u8>) {
        self.dataChunk = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dataChunk(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.dataChunk
    }

    // Take field
    pub fn take_dataChunk(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.dataChunk, ::std::vec::Vec::new())
    }

    pub fn get_dataChunk(&self) -> &[u8] {
        &self.dataChunk
    }

    fn get_dataChunk_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.dataChunk
    }

    fn mut_dataChunk_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.dataChunk
    }
}

impl ::protobuf::Message for DataReadStreamResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.dataChunk)?;
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
        if !self.dataChunk.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.dataChunk);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.dataChunk.is_empty() {
            os.write_bytes(1, &self.dataChunk)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DataReadStreamResponse {
    fn new() -> DataReadStreamResponse {
        DataReadStreamResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<DataReadStreamResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "dataChunk",
                    DataReadStreamResponse::get_dataChunk_for_reflect,
                    DataReadStreamResponse::mut_dataChunk_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DataReadStreamResponse>(
                    "DataReadStreamResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DataReadStreamResponse {
    fn clear(&mut self) {
        self.clear_dataChunk();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DataReadStreamResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DataReadStreamResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DataDeleteRequest {
    // message fields
    pub chunks: ::protobuf::RepeatedField<Chunk>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DataDeleteRequest {}

impl DataDeleteRequest {
    pub fn new() -> DataDeleteRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DataDeleteRequest {
        static mut instance: ::protobuf::lazy::Lazy<DataDeleteRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DataDeleteRequest,
        };
        unsafe {
            instance.get(DataDeleteRequest::new)
        }
    }

    // repeated .schema.Chunk chunks = 1;

    pub fn clear_chunks(&mut self) {
        self.chunks.clear();
    }

    // Param is passed by value, moved
    pub fn set_chunks(&mut self, v: ::protobuf::RepeatedField<Chunk>) {
        self.chunks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_chunks(&mut self) -> &mut ::protobuf::RepeatedField<Chunk> {
        &mut self.chunks
    }

    // Take field
    pub fn take_chunks(&mut self) -> ::protobuf::RepeatedField<Chunk> {
        ::std::mem::replace(&mut self.chunks, ::protobuf::RepeatedField::new())
    }

    pub fn get_chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    fn get_chunks_for_reflect(&self) -> &::protobuf::RepeatedField<Chunk> {
        &self.chunks
    }

    fn mut_chunks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Chunk> {
        &mut self.chunks
    }
}

impl ::protobuf::Message for DataDeleteRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.chunks {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.chunks)?;
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
        for value in &self.chunks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.chunks {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DataDeleteRequest {
    fn new() -> DataDeleteRequest {
        DataDeleteRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<DataDeleteRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Chunk>>(
                    "chunks",
                    DataDeleteRequest::get_chunks_for_reflect,
                    DataDeleteRequest::mut_chunks_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DataDeleteRequest>(
                    "DataDeleteRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DataDeleteRequest {
    fn clear(&mut self) {
        self.clear_chunks();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DataDeleteRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DataDeleteRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DataDeleteResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DataDeleteResponse {}

impl DataDeleteResponse {
    pub fn new() -> DataDeleteResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DataDeleteResponse {
        static mut instance: ::protobuf::lazy::Lazy<DataDeleteResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DataDeleteResponse,
        };
        unsafe {
            instance.get(DataDeleteResponse::new)
        }
    }
}

impl ::protobuf::Message for DataDeleteResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DataDeleteResponse {
    fn new() -> DataDeleteResponse {
        DataDeleteResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<DataDeleteResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<DataDeleteResponse>(
                    "DataDeleteResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DataDeleteResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DataDeleteResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DataDeleteResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DataCheckRequest {
    // message fields
    pub chunks: ::protobuf::RepeatedField<Chunk>,
    pub fast: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DataCheckRequest {}

impl DataCheckRequest {
    pub fn new() -> DataCheckRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DataCheckRequest {
        static mut instance: ::protobuf::lazy::Lazy<DataCheckRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DataCheckRequest,
        };
        unsafe {
            instance.get(DataCheckRequest::new)
        }
    }

    // repeated .schema.Chunk chunks = 1;

    pub fn clear_chunks(&mut self) {
        self.chunks.clear();
    }

    // Param is passed by value, moved
    pub fn set_chunks(&mut self, v: ::protobuf::RepeatedField<Chunk>) {
        self.chunks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_chunks(&mut self) -> &mut ::protobuf::RepeatedField<Chunk> {
        &mut self.chunks
    }

    // Take field
    pub fn take_chunks(&mut self) -> ::protobuf::RepeatedField<Chunk> {
        ::std::mem::replace(&mut self.chunks, ::protobuf::RepeatedField::new())
    }

    pub fn get_chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    fn get_chunks_for_reflect(&self) -> &::protobuf::RepeatedField<Chunk> {
        &self.chunks
    }

    fn mut_chunks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Chunk> {
        &mut self.chunks
    }

    // bool fast = 2;

    pub fn clear_fast(&mut self) {
        self.fast = false;
    }

    // Param is passed by value, moved
    pub fn set_fast(&mut self, v: bool) {
        self.fast = v;
    }

    pub fn get_fast(&self) -> bool {
        self.fast
    }

    fn get_fast_for_reflect(&self) -> &bool {
        &self.fast
    }

    fn mut_fast_for_reflect(&mut self) -> &mut bool {
        &mut self.fast
    }
}

impl ::protobuf::Message for DataCheckRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.chunks {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.chunks)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.fast = tmp;
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
        for value in &self.chunks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.fast != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.chunks {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.fast != false {
            os.write_bool(2, self.fast)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DataCheckRequest {
    fn new() -> DataCheckRequest {
        DataCheckRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<DataCheckRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Chunk>>(
                    "chunks",
                    DataCheckRequest::get_chunks_for_reflect,
                    DataCheckRequest::mut_chunks_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "fast",
                    DataCheckRequest::get_fast_for_reflect,
                    DataCheckRequest::mut_fast_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DataCheckRequest>(
                    "DataCheckRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DataCheckRequest {
    fn clear(&mut self) {
        self.clear_chunks();
        self.clear_fast();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DataCheckRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DataCheckRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DataCheckResponse {
    // message fields
    pub status: CheckStatus,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DataCheckResponse {}

impl DataCheckResponse {
    pub fn new() -> DataCheckResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DataCheckResponse {
        static mut instance: ::protobuf::lazy::Lazy<DataCheckResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DataCheckResponse,
        };
        unsafe {
            instance.get(DataCheckResponse::new)
        }
    }

    // .schema.CheckStatus status = 1;

    pub fn clear_status(&mut self) {
        self.status = CheckStatus::CheckStatusInvalid;
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: CheckStatus) {
        self.status = v;
    }

    pub fn get_status(&self) -> CheckStatus {
        self.status
    }

    fn get_status_for_reflect(&self) -> &CheckStatus {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut CheckStatus {
        &mut self.status
    }
}

impl ::protobuf::Message for DataCheckResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.status = tmp;
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
        if self.status != CheckStatus::CheckStatusInvalid {
            my_size += ::protobuf::rt::enum_size(1, self.status);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.status != CheckStatus::CheckStatusInvalid {
            os.write_enum(1, self.status.value())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DataCheckResponse {
    fn new() -> DataCheckResponse {
        DataCheckResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<DataCheckResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CheckStatus>>(
                    "status",
                    DataCheckResponse::get_status_for_reflect,
                    DataCheckResponse::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DataCheckResponse>(
                    "DataCheckResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DataCheckResponse {
    fn clear(&mut self) {
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DataCheckResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DataCheckResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DataRepairRequest {
    // message fields
    pub chunks: ::protobuf::RepeatedField<Chunk>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DataRepairRequest {}

impl DataRepairRequest {
    pub fn new() -> DataRepairRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DataRepairRequest {
        static mut instance: ::protobuf::lazy::Lazy<DataRepairRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DataRepairRequest,
        };
        unsafe {
            instance.get(DataRepairRequest::new)
        }
    }

    // repeated .schema.Chunk chunks = 1;

    pub fn clear_chunks(&mut self) {
        self.chunks.clear();
    }

    // Param is passed by value, moved
    pub fn set_chunks(&mut self, v: ::protobuf::RepeatedField<Chunk>) {
        self.chunks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_chunks(&mut self) -> &mut ::protobuf::RepeatedField<Chunk> {
        &mut self.chunks
    }

    // Take field
    pub fn take_chunks(&mut self) -> ::protobuf::RepeatedField<Chunk> {
        ::std::mem::replace(&mut self.chunks, ::protobuf::RepeatedField::new())
    }

    pub fn get_chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    fn get_chunks_for_reflect(&self) -> &::protobuf::RepeatedField<Chunk> {
        &self.chunks
    }

    fn mut_chunks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Chunk> {
        &mut self.chunks
    }
}

impl ::protobuf::Message for DataRepairRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.chunks {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.chunks)?;
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
        for value in &self.chunks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.chunks {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DataRepairRequest {
    fn new() -> DataRepairRequest {
        DataRepairRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<DataRepairRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Chunk>>(
                    "chunks",
                    DataRepairRequest::get_chunks_for_reflect,
                    DataRepairRequest::mut_chunks_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DataRepairRequest>(
                    "DataRepairRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DataRepairRequest {
    fn clear(&mut self) {
        self.clear_chunks();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DataRepairRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DataRepairRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DataRepairResponse {
    // message fields
    pub chunks: ::protobuf::RepeatedField<Chunk>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DataRepairResponse {}

impl DataRepairResponse {
    pub fn new() -> DataRepairResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DataRepairResponse {
        static mut instance: ::protobuf::lazy::Lazy<DataRepairResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DataRepairResponse,
        };
        unsafe {
            instance.get(DataRepairResponse::new)
        }
    }

    // repeated .schema.Chunk chunks = 1;

    pub fn clear_chunks(&mut self) {
        self.chunks.clear();
    }

    // Param is passed by value, moved
    pub fn set_chunks(&mut self, v: ::protobuf::RepeatedField<Chunk>) {
        self.chunks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_chunks(&mut self) -> &mut ::protobuf::RepeatedField<Chunk> {
        &mut self.chunks
    }

    // Take field
    pub fn take_chunks(&mut self) -> ::protobuf::RepeatedField<Chunk> {
        ::std::mem::replace(&mut self.chunks, ::protobuf::RepeatedField::new())
    }

    pub fn get_chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    fn get_chunks_for_reflect(&self) -> &::protobuf::RepeatedField<Chunk> {
        &self.chunks
    }

    fn mut_chunks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Chunk> {
        &mut self.chunks
    }
}

impl ::protobuf::Message for DataRepairResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.chunks {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.chunks)?;
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
        for value in &self.chunks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.chunks {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DataRepairResponse {
    fn new() -> DataRepairResponse {
        DataRepairResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<DataRepairResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Chunk>>(
                    "chunks",
                    DataRepairResponse::get_chunks_for_reflect,
                    DataRepairResponse::mut_chunks_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DataRepairResponse>(
                    "DataRepairResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DataRepairResponse {
    fn clear(&mut self) {
        self.clear_chunks();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DataRepairResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DataRepairResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CheckStatus {
    CheckStatusInvalid = 0,
    CheckStatusValid = 1,
    CheckStatusOptimal = 2,
}

impl ::protobuf::ProtobufEnum for CheckStatus {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CheckStatus> {
        match value {
            0 => ::std::option::Option::Some(CheckStatus::CheckStatusInvalid),
            1 => ::std::option::Option::Some(CheckStatus::CheckStatusValid),
            2 => ::std::option::Option::Some(CheckStatus::CheckStatusOptimal),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CheckStatus] = &[
            CheckStatus::CheckStatusInvalid,
            CheckStatus::CheckStatusValid,
            CheckStatus::CheckStatusOptimal,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<CheckStatus>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CheckStatus", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CheckStatus {
}

impl ::std::default::Default for CheckStatus {
    fn default() -> Self {
        CheckStatus::CheckStatusInvalid
    }
}

impl ::protobuf::reflect::ProtobufValue for CheckStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum FileMode {
    FileModeTruncate = 0,
    FileModeAppend = 1,
    FileModeExclusive = 2,
}

impl ::protobuf::ProtobufEnum for FileMode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<FileMode> {
        match value {
            0 => ::std::option::Option::Some(FileMode::FileModeTruncate),
            1 => ::std::option::Option::Some(FileMode::FileModeAppend),
            2 => ::std::option::Option::Some(FileMode::FileModeExclusive),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [FileMode] = &[
            FileMode::FileModeTruncate,
            FileMode::FileModeAppend,
            FileMode::FileModeExclusive,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<FileMode>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("FileMode", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for FileMode {
}

impl ::std::default::Default for FileMode {
    fn default() -> Self {
        FileMode::FileModeTruncate
    }
}

impl ::protobuf::reflect::ProtobufValue for FileMode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0cdaemon.proto\x12\x06schema\"d\n\nPermission\x12\x12\n\x04read\x18\
    \x01\x20\x01(\x08R\x04read\x12\x14\n\x05write\x18\x02\x20\x01(\x08R\x05w\
    rite\x12\x16\n\x06delete\x18\x03\x20\x01(\x08R\x06delete\x12\x14\n\x05ad\
    min\x18\x04\x20\x01(\x08R\x05admin\"6\n\x16CreateNamespaceRequest\x12\
    \x1c\n\tnamespace\x18\x01\x20\x01(\tR\tnamespace\"\x19\n\x17CreateNamesp\
    aceResponse\"6\n\x16DeleteNamespaceRequest\x12\x1c\n\tnamespace\x18\x01\
    \x20\x01(\tR\tnamespace\"\x19\n\x17DeleteNamespaceResponse\"\x80\x01\n\
    \x14SetPermissionRequest\x12\x1c\n\tnamespace\x18\x01\x20\x01(\tR\tnames\
    pace\x12\x16\n\x06userID\x18\x02\x20\x01(\tR\x06userID\x122\n\npermissio\
    n\x18\x03\x20\x01(\x0b2\x12.schema.PermissionR\npermission\"\x17\n\x15Se\
    tPermissionResponse\"L\n\x14GetPermissionRequest\x12\x1c\n\tnamespace\
    \x18\x01\x20\x01(\tR\tnamespace\x12\x16\n\x06userID\x18\x02\x20\x01(\tR\
    \x06userID\"K\n\x15GetPermissionResponse\x122\n\npermission\x18\x01\x20\
    \x01(\x0b2\x12.schema.PermissionR\npermission\"\xaf\x01\n\x08Metadata\
    \x12\x10\n\x03key\x18\x01\x20\x01(\x0cR\x03key\x12\x1c\n\ttotalSize\x18\
    \x02\x20\x01(\x03R\ttotalSize\x12$\n\rcreationEpoch\x18\x03\x20\x01(\x03\
    R\rcreationEpoch\x12&\n\x0elastWriteEpoch\x18\x04\x20\x01(\x03R\x0elastW\
    riteEpoch\x12%\n\x06chunks\x18\x05\x20\x03(\x0b2\r.schema.ChunkR\x06chun\
    ks\"c\n\x05Chunk\x12\x1c\n\tchunkSize\x18\x01\x20\x01(\x03R\tchunkSize\
    \x12(\n\x07objects\x18\x02\x20\x03(\x0b2\x0e.schema.ObjectR\x07objects\
    \x12\x12\n\x04hash\x18\x03\x20\x01(\x0cR\x04hash\"4\n\x06Object\x12\x10\
    \n\x03key\x18\x01\x20\x01(\x0cR\x03key\x12\x18\n\x07shardID\x18\x02\x20\
    \x01(\tR\x07shardID\"4\n\x0cWriteRequest\x12\x10\n\x03key\x18\x01\x20\
    \x01(\x0cR\x03key\x12\x12\n\x04data\x18\x02\x20\x01(\x0cR\x04data\"=\n\r\
    WriteResponse\x12,\n\x08metadata\x18\x01\x20\x01(\x0b2\x10.schema.Metada\
    taR\x08metadata\"@\n\x10WriteFileRequest\x12\x10\n\x03key\x18\x01\x20\
    \x01(\x0cR\x03key\x12\x1a\n\x08filePath\x18\x02\x20\x01(\tR\x08filePath\
    \"A\n\x11WriteFileResponse\x12,\n\x08metadata\x18\x01\x20\x01(\x0b2\x10.\
    schema.MetadataR\x08metadata\"\xdb\x01\n\x12WriteStreamRequest\x12A\n\
    \x08metadata\x18\x01\x20\x01(\x0b2#.schema.WriteStreamRequest.MetadataH\
    \0R\x08metadata\x125\n\x04data\x18\x02\x20\x01(\x0b2\x1f.schema.WriteStr\
    eamRequest.DataH\0R\x04data\x1a\x1c\n\x08Metadata\x12\x10\n\x03key\x18\
    \x01\x20\x01(\x0cR\x03key\x1a$\n\x04Data\x12\x1c\n\tdataChunk\x18\x02\
    \x20\x01(\x0cR\tdataChunkB\x07\n\x05input\"C\n\x13WriteStreamResponse\
    \x12,\n\x08metadata\x18\x01\x20\x01(\x0b2\x10.schema.MetadataR\x08metada\
    ta\"Z\n\x0bReadRequest\x12\x12\n\x03key\x18\x01\x20\x01(\x0cH\0R\x03key\
    \x12.\n\x08metadata\x18\x02\x20\x01(\x0b2\x10.schema.MetadataH\0R\x08met\
    adataB\x07\n\x05input\"\"\n\x0cReadResponse\x12\x12\n\x04data\x18\x01\
    \x20\x01(\x0cR\x04data\"\xce\x01\n\x0fReadFileRequest\x12\x12\n\x03key\
    \x18\x01\x20\x01(\x0cH\0R\x03key\x12.\n\x08metadata\x18\x02\x20\x01(\x0b\
    2\x10.schema.MetadataH\0R\x08metadata\x12\x1a\n\x08filePath\x18\x03\x20\
    \x01(\tR\x08filePath\x12,\n\x08fileMode\x18\x04\x20\x01(\x0e2\x10.schema\
    .FileModeR\x08fileMode\x12$\n\rsynchronousIO\x18\x05\x20\x01(\x08R\rsync\
    hronousIOB\x07\n\x05input\"\x12\n\x10ReadFileResponse\"~\n\x11ReadStream\
    Request\x12\x12\n\x03key\x18\x01\x20\x01(\x0cH\0R\x03key\x12.\n\x08metad\
    ata\x18\x02\x20\x01(\x0b2\x10.schema.MetadataH\0R\x08metadata\x12\x1c\n\
    \tchunkSize\x18\x03\x20\x01(\x03R\tchunkSizeB\x07\n\x05input\"2\n\x12Rea\
    dStreamResponse\x12\x1c\n\tdataChunk\x18\x01\x20\x01(\x0cR\tdataChunk\"\
    \\\n\rDeleteRequest\x12\x12\n\x03key\x18\x01\x20\x01(\x0cH\0R\x03key\x12\
    .\n\x08metadata\x18\x02\x20\x01(\x0b2\x10.schema.MetadataH\0R\x08metadat\
    aB\x07\n\x05input\"\x10\n\x0eDeleteResponse\"o\n\x0cCheckRequest\x12\x12\
    \n\x03key\x18\x01\x20\x01(\x0cH\0R\x03key\x12.\n\x08metadata\x18\x02\x20\
    \x01(\x0b2\x10.schema.MetadataH\0R\x08metadata\x12\x12\n\x04fast\x18\x03\
    \x20\x01(\x08R\x04fastB\x07\n\x05input\"<\n\rCheckResponse\x12+\n\x06sta\
    tus\x18\x01\x20\x01(\x0e2\x13.schema.CheckStatusR\x06status\"!\n\rRepair\
    Request\x12\x10\n\x03key\x18\x01\x20\x01(\x0cR\x03key\">\n\x0eRepairResp\
    onse\x12,\n\x08metadata\x18\x01\x20\x01(\x0b2\x10.schema.MetadataR\x08me\
    tadata\"B\n\x12SetMetadataRequest\x12,\n\x08metadata\x18\x01\x20\x01(\
    \x0b2\x10.schema.MetadataR\x08metadata\"\x15\n\x13SetMetadataResponse\"&\
    \n\x12GetMetadataRequest\x12\x10\n\x03key\x18\x01\x20\x01(\x0cR\x03key\"\
    C\n\x13GetMetadataResponse\x12,\n\x08metadata\x18\x01\x20\x01(\x0b2\x10.\
    schema.MetadataR\x08metadata\")\n\x15DeleteMetadataRequest\x12\x10\n\x03\
    key\x18\x01\x20\x01(\x0cR\x03key\"\x18\n\x16DeleteMetadataResponse\"&\n\
    \x10DataWriteRequest\x12\x12\n\x04data\x18\x01\x20\x01(\x0cR\x04data\":\
    \n\x11DataWriteResponse\x12%\n\x06chunks\x18\x01\x20\x03(\x0b2\r.schema.\
    ChunkR\x06chunks\"2\n\x14DataWriteFileRequest\x12\x1a\n\x08filePath\x18\
    \x01\x20\x01(\tR\x08filePath\">\n\x15DataWriteFileResponse\x12%\n\x06chu\
    nks\x18\x01\x20\x03(\x0b2\r.schema.ChunkR\x06chunks\"6\n\x16DataWriteStr\
    eamRequest\x12\x1c\n\tdataChunk\x18\x01\x20\x01(\x0cR\tdataChunk\"@\n\
    \x17DataWriteStreamResponse\x12%\n\x06chunks\x18\x01\x20\x03(\x0b2\r.sch\
    ema.ChunkR\x06chunks\"8\n\x0fDataReadRequest\x12%\n\x06chunks\x18\x01\
    \x20\x03(\x0b2\r.schema.ChunkR\x06chunks\"&\n\x10DataReadResponse\x12\
    \x12\n\x04data\x18\x01\x20\x01(\x0cR\x04data\"\xac\x01\n\x13DataReadFile\
    Request\x12%\n\x06chunks\x18\x01\x20\x03(\x0b2\r.schema.ChunkR\x06chunks\
    \x12\x1a\n\x08filePath\x18\x02\x20\x01(\tR\x08filePath\x12,\n\x08fileMod\
    e\x18\x03\x20\x01(\x0e2\x10.schema.FileModeR\x08fileMode\x12$\n\rsynchro\
    nousIO\x18\x04\x20\x01(\x08R\rsynchronousIO\"\x16\n\x14DataReadFileRespo\
    nse\"\\\n\x15DataReadStreamRequest\x12%\n\x06chunks\x18\x01\x20\x03(\x0b\
    2\r.schema.ChunkR\x06chunks\x12\x1c\n\tchunkSize\x18\x02\x20\x01(\x03R\t\
    chunkSize\"6\n\x16DataReadStreamResponse\x12\x1c\n\tdataChunk\x18\x01\
    \x20\x01(\x0cR\tdataChunk\":\n\x11DataDeleteRequest\x12%\n\x06chunks\x18\
    \x01\x20\x03(\x0b2\r.schema.ChunkR\x06chunks\"\x14\n\x12DataDeleteRespon\
    se\"M\n\x10DataCheckRequest\x12%\n\x06chunks\x18\x01\x20\x03(\x0b2\r.sch\
    ema.ChunkR\x06chunks\x12\x12\n\x04fast\x18\x02\x20\x01(\x08R\x04fast\"@\
    \n\x11DataCheckResponse\x12+\n\x06status\x18\x01\x20\x01(\x0e2\x13.schem\
    a.CheckStatusR\x06status\":\n\x11DataRepairRequest\x12%\n\x06chunks\x18\
    \x01\x20\x03(\x0b2\r.schema.ChunkR\x06chunks\";\n\x12DataRepairResponse\
    \x12%\n\x06chunks\x18\x01\x20\x03(\x0b2\r.schema.ChunkR\x06chunks*S\n\
    \x0bCheckStatus\x12\x16\n\x12CheckStatusInvalid\x10\0\x12\x14\n\x10Check\
    StatusValid\x10\x01\x12\x16\n\x12CheckStatusOptimal\x10\x02*K\n\x08FileM\
    ode\x12\x14\n\x10FileModeTruncate\x10\0\x12\x12\n\x0eFileModeAppend\x10\
    \x01\x12\x15\n\x11FileModeExclusive\x10\x022\xde\x02\n\x10NamespaceServi\
    ce\x12T\n\x0fCreateNamespace\x12\x1e.schema.CreateNamespaceRequest\x1a\
    \x1f.schema.CreateNamespaceResponse\"\0\x12T\n\x0fDeleteNamespace\x12\
    \x1e.schema.DeleteNamespaceRequest\x1a\x1f.schema.DeleteNamespaceRespons\
    e\"\0\x12N\n\rSetPermission\x12\x1c.schema.SetPermissionRequest\x1a\x1d.\
    schema.SetPermissionResponse\"\0\x12N\n\rGetPermission\x12\x1c.schema.Ge\
    tPermissionRequest\x1a\x1d.schema.GetPermissionResponse\"\02\xc2\x04\n\
    \x0bFileService\x126\n\x05Write\x12\x14.schema.WriteRequest\x1a\x15.sche\
    ma.WriteResponse\"\0\x12B\n\tWriteFile\x12\x18.schema.WriteFileRequest\
    \x1a\x19.schema.WriteFileResponse\"\0\x12J\n\x0bWriteStream\x12\x1a.sche\
    ma.WriteStreamRequest\x1a\x1b.schema.WriteStreamResponse\"\0(\x01\x123\n\
    \x04Read\x12\x13.schema.ReadRequest\x1a\x14.schema.ReadResponse\"\0\x12?\
    \n\x08ReadFile\x12\x17.schema.ReadFileRequest\x1a\x18.schema.ReadFileRes\
    ponse\"\0\x12G\n\nReadStream\x12\x19.schema.ReadStreamRequest\x1a\x1a.sc\
    hema.ReadStreamResponse\"\00\x01\x129\n\x06Delete\x12\x15.schema.DeleteR\
    equest\x1a\x16.schema.DeleteResponse\"\0\x126\n\x05Check\x12\x14.schema.\
    CheckRequest\x1a\x15.schema.CheckResponse\"\0\x129\n\x06Repair\x12\x15.s\
    chema.RepairRequest\x1a\x16.schema.RepairResponse\"\02\xf8\x01\n\x0fMeta\
    dataService\x12H\n\x0bSetMetadata\x12\x1a.schema.SetMetadataRequest\x1a\
    \x1b.schema.SetMetadataResponse\"\0\x12H\n\x0bGetMetadata\x12\x1a.schema\
    .GetMetadataRequest\x1a\x1b.schema.GetMetadataResponse\"\0\x12Q\n\x0eDel\
    eteMetadata\x12\x1d.schema.DeleteMetadataRequest\x1a\x1e.schema.DeleteMe\
    tadataResponse\"\02\x8a\x05\n\x0bDataService\x12>\n\x05Write\x12\x18.sch\
    ema.DataWriteRequest\x1a\x19.schema.DataWriteResponse\"\0\x12J\n\tWriteF\
    ile\x12\x1c.schema.DataWriteFileRequest\x1a\x1d.schema.DataWriteFileResp\
    onse\"\0\x12R\n\x0bWriteStream\x12\x1e.schema.DataWriteStreamRequest\x1a\
    \x1f.schema.DataWriteStreamResponse\"\0(\x01\x12;\n\x04Read\x12\x17.sche\
    ma.DataReadRequest\x1a\x18.schema.DataReadResponse\"\0\x12G\n\x08ReadFil\
    e\x12\x1b.schema.DataReadFileRequest\x1a\x1c.schema.DataReadFileResponse\
    \"\0\x12O\n\nReadStream\x12\x1d.schema.DataReadStreamRequest\x1a\x1e.sch\
    ema.DataReadStreamResponse\"\00\x01\x12A\n\x06Delete\x12\x19.schema.Data\
    DeleteRequest\x1a\x1a.schema.DataDeleteResponse\"\0\x12>\n\x05Check\x12\
    \x18.schema.DataCheckRequest\x1a\x19.schema.DataCheckResponse\"\0\x12A\n\
    \x06Repair\x12\x19.schema.DataRepairRequest\x1a\x1a.schema.DataRepairRes\
    ponse\"\0J\xdb`\n\x07\x12\x05\x10\0\xe9\x02\x01\n\xdd\x04\n\x01\x0c\x12\
    \x03\x10\0\x122\xd2\x04\n\x20Copyright\x20(C)\x202017-2018\x20GIG\x20Tec\
    hnology\x20NV\x20and\x20Contributors\n\n\x20Licensed\x20under\x20the\x20\
    Apache\x20License,\x20Version\x202.0\x20(the\x20\"License\");\n\x20you\
    \x20may\x20not\x20use\x20this\x20file\x20except\x20in\x20compliance\x20w\
    ith\x20the\x20License.\n\x20You\x20may\x20obtain\x20a\x20copy\x20of\x20t\
    he\x20License\x20at\n\n\x20\x20\x20\x20http://www.apache.org/licenses/LI\
    CENSE-2.0\n\n\x20Unless\x20required\x20by\x20applicable\x20law\x20or\x20\
    agreed\x20to\x20in\x20writing,\x20software\n\x20distributed\x20under\x20\
    the\x20License\x20is\x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS\
    ,\n\x20WITHOUT\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\
    \x20either\x20express\x20or\x20implied.\n\x20See\x20the\x20License\x20fo\
    r\x20the\x20specific\x20language\x20governing\x20permissions\x20and\n\
    \x20limitations\x20under\x20the\x20License.\n\n\x08\n\x01\x02\x12\x03\
    \x12\x08\x0e\n\x87\x01\n\x02\x06\0\x12\x04\x16\0\x1b\x01\x1a{\x20Namespa\
    ceService\x20is\x20the\x20service\x20to\x20be\x20used\x20to\x20manage\
    \x20namespaces\n\x20and\x20the\x20permissions\x20of\x20users\x20linked\
    \x20to\x20those\x20namespaces.\n\n\n\n\x03\x06\0\x01\x12\x03\x16\x08\x18\
    \n\x0b\n\x04\x06\0\x02\0\x12\x03\x17\x08V\n\x0c\n\x05\x06\0\x02\0\x01\
    \x12\x03\x17\x0c\x1b\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\x17\x1c2\n\x0c\
    \n\x05\x06\0\x02\0\x03\x12\x03\x17<S\n\x0b\n\x04\x06\0\x02\x01\x12\x03\
    \x18\x08V\n\x0c\n\x05\x06\0\x02\x01\x01\x12\x03\x18\x0c\x1b\n\x0c\n\x05\
    \x06\0\x02\x01\x02\x12\x03\x18\x1c2\n\x0c\n\x05\x06\0\x02\x01\x03\x12\
    \x03\x18<S\n\x0b\n\x04\x06\0\x02\x02\x12\x03\x19\x08P\n\x0c\n\x05\x06\0\
    \x02\x02\x01\x12\x03\x19\x0c\x19\n\x0c\n\x05\x06\0\x02\x02\x02\x12\x03\
    \x19\x1a.\n\x0c\n\x05\x06\0\x02\x02\x03\x12\x03\x198M\n\x0b\n\x04\x06\0\
    \x02\x03\x12\x03\x1a\x08P\n\x0c\n\x05\x06\0\x02\x03\x01\x12\x03\x1a\x0c\
    \x19\n\x0c\n\x05\x06\0\x02\x03\x02\x12\x03\x1a\x1a.\n\x0c\n\x05\x06\0\
    \x02\x03\x03\x12\x03\x1a8M\n\n\n\x02\x04\0\x12\x04\x1d\0\"\x01\n\n\n\x03\
    \x04\0\x01\x12\x03\x1d\x08\x12\n\x0b\n\x04\x04\0\x02\0\x12\x03\x1e\x08\
    \x16\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x1e\x08\x1d\x14\n\x0c\n\x05\x04\0\
    \x02\0\x05\x12\x03\x1e\x08\x0c\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x1e\r\
    \x11\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x1e\x14\x15\n\x0b\n\x04\x04\0\
    \x02\x01\x12\x03\x1f\x08\x17\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x1f\x08\
    \x1e\x16\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x1f\x08\x0c\n\x0c\n\x05\
    \x04\0\x02\x01\x01\x12\x03\x1f\r\x12\n\x0c\n\x05\x04\0\x02\x01\x03\x12\
    \x03\x1f\x15\x16\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x20\x08\x18\n\r\n\x05\
    \x04\0\x02\x02\x04\x12\x04\x20\x08\x1f\x17\n\x0c\n\x05\x04\0\x02\x02\x05\
    \x12\x03\x20\x08\x0c\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x20\r\x13\n\
    \x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x20\x16\x17\n\x0b\n\x04\x04\0\x02\
    \x03\x12\x03!\x08\x17\n\r\n\x05\x04\0\x02\x03\x04\x12\x04!\x08\x20\x18\n\
    \x0c\n\x05\x04\0\x02\x03\x05\x12\x03!\x08\x0c\n\x0c\n\x05\x04\0\x02\x03\
    \x01\x12\x03!\r\x12\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03!\x15\x16\n\n\n\
    \x02\x04\x01\x12\x04$\0&\x01\n\n\n\x03\x04\x01\x01\x12\x03$\x08\x1e\n\
    \x0b\n\x04\x04\x01\x02\0\x12\x03%\x08\x1d\n\r\n\x05\x04\x01\x02\0\x04\
    \x12\x04%\x08$\x20\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03%\x08\x0e\n\x0c\
    \n\x05\x04\x01\x02\0\x01\x12\x03%\x0f\x18\n\x0c\n\x05\x04\x01\x02\0\x03\
    \x12\x03%\x1b\x1c\n\n\n\x02\x04\x02\x12\x04'\0(\x01\n\n\n\x03\x04\x02\
    \x01\x12\x03'\x08\x1f\n\n\n\x02\x04\x03\x12\x04*\0,\x01\n\n\n\x03\x04\
    \x03\x01\x12\x03*\x08\x1e\n\x0b\n\x04\x04\x03\x02\0\x12\x03+\x08\x1d\n\r\
    \n\x05\x04\x03\x02\0\x04\x12\x04+\x08*\x20\n\x0c\n\x05\x04\x03\x02\0\x05\
    \x12\x03+\x08\x0e\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03+\x0f\x18\n\x0c\n\
    \x05\x04\x03\x02\0\x03\x12\x03+\x1b\x1c\n\n\n\x02\x04\x04\x12\x04-\0.\
    \x01\n\n\n\x03\x04\x04\x01\x12\x03-\x08\x1f\n\n\n\x02\x04\x05\x12\x040\0\
    4\x01\n\n\n\x03\x04\x05\x01\x12\x030\x08\x1c\n\x0b\n\x04\x04\x05\x02\0\
    \x12\x031\x08\x1d\n\r\n\x05\x04\x05\x02\0\x04\x12\x041\x080\x1e\n\x0c\n\
    \x05\x04\x05\x02\0\x05\x12\x031\x08\x0e\n\x0c\n\x05\x04\x05\x02\0\x01\
    \x12\x031\x0f\x18\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x031\x1b\x1c\n\x0b\n\
    \x04\x04\x05\x02\x01\x12\x032\x08\x1a\n\r\n\x05\x04\x05\x02\x01\x04\x12\
    \x042\x081\x1d\n\x0c\n\x05\x04\x05\x02\x01\x05\x12\x032\x08\x0e\n\x0c\n\
    \x05\x04\x05\x02\x01\x01\x12\x032\x0f\x15\n\x0c\n\x05\x04\x05\x02\x01\
    \x03\x12\x032\x18\x19\n\x0b\n\x04\x04\x05\x02\x02\x12\x033\x08\"\n\r\n\
    \x05\x04\x05\x02\x02\x04\x12\x043\x082\x1a\n\x0c\n\x05\x04\x05\x02\x02\
    \x06\x12\x033\x08\x12\n\x0c\n\x05\x04\x05\x02\x02\x01\x12\x033\x13\x1d\n\
    \x0c\n\x05\x04\x05\x02\x02\x03\x12\x033\x20!\n\n\n\x02\x04\x06\x12\x045\
    \06\x01\n\n\n\x03\x04\x06\x01\x12\x035\x08\x1d\n\n\n\x02\x04\x07\x12\x04\
    8\0;\x01\n\n\n\x03\x04\x07\x01\x12\x038\x08\x1c\n\x0b\n\x04\x04\x07\x02\
    \0\x12\x039\x08\x1d\n\r\n\x05\x04\x07\x02\0\x04\x12\x049\x088\x1e\n\x0c\
    \n\x05\x04\x07\x02\0\x05\x12\x039\x08\x0e\n\x0c\n\x05\x04\x07\x02\0\x01\
    \x12\x039\x0f\x18\n\x0c\n\x05\x04\x07\x02\0\x03\x12\x039\x1b\x1c\n\x0b\n\
    \x04\x04\x07\x02\x01\x12\x03:\x08\x1a\n\r\n\x05\x04\x07\x02\x01\x04\x12\
    \x04:\x089\x1d\n\x0c\n\x05\x04\x07\x02\x01\x05\x12\x03:\x08\x0e\n\x0c\n\
    \x05\x04\x07\x02\x01\x01\x12\x03:\x0f\x15\n\x0c\n\x05\x04\x07\x02\x01\
    \x03\x12\x03:\x18\x19\n\n\n\x02\x04\x08\x12\x04<\0>\x01\n\n\n\x03\x04\
    \x08\x01\x12\x03<\x08\x1d\n\x0b\n\x04\x04\x08\x02\0\x12\x03=\x08\"\n\r\n\
    \x05\x04\x08\x02\0\x04\x12\x04=\x08<\x1f\n\x0c\n\x05\x04\x08\x02\0\x06\
    \x12\x03=\x08\x12\n\x0c\n\x05\x04\x08\x02\0\x01\x12\x03=\x13\x1d\n\x0c\n\
    \x05\x04\x08\x02\0\x03\x12\x03=\x20!\n\xf7\x03\n\x02\x06\x01\x12\x04G\0T\
    \x01\x1a\xea\x03\x20FileService\x20is\x20the\x20service\x20to\x20be\x20u\
    sed\x20to\x20write,\x20read,\x20delete,\x20check\x20and\x20repair\x20fil\
    es.\n\x20The\x20fileService\x20follows\x20the\x20principle\x20of\x20ever\
    ything\x20is\x20a\x20file.\n\x20All\x20files\x20are\x20written\x20as\x20\
    raw\x20binary\x20data,\x20but\x20also\x20have\n\x20metadata\x20bound\x20\
    to\x20them,\x20which\x20identify\x20where\x20and\x20how\x20they\x20are\
    \x20stored.\n\x20This\x20metadata\x20is\x20returned,\x20when\x20creating\
    \x20or\x20altering\x20the\x20metadata,\n\x20however\x20if\x20not\x20need\
    ed\x20for\x20anything\x20it\x20can\x20be\x20ignored,\x20as\x20it\x20is\
    \x20stored\x20by\x20the\x20daemon\x20(server),\n\x20already,\x20using\
    \x20the\x20underlying\x20metastor\x20client.\n\n\n\n\x03\x06\x01\x01\x12\
    \x03G\x08\x13\n\x0b\n\x04\x06\x01\x02\0\x12\x03H\x08:\n\x0c\n\x05\x06\
    \x01\x02\0\x01\x12\x03H\x0c\x11\n\x0c\n\x05\x06\x01\x02\0\x02\x12\x03H\
    \x12\x1e\n\x0c\n\x05\x06\x01\x02\0\x03\x12\x03H)6\n\x0b\n\x04\x06\x01\
    \x02\x01\x12\x03I\x08F\n\x0c\n\x05\x06\x01\x02\x01\x01\x12\x03I\x0c\x15\
    \n\x0c\n\x05\x06\x01\x02\x01\x02\x12\x03I\x16&\n\x0c\n\x05\x06\x01\x02\
    \x01\x03\x12\x03I1B\n\x0b\n\x04\x06\x01\x02\x02\x12\x03J\x08S\n\x0c\n\
    \x05\x06\x01\x02\x02\x01\x12\x03J\x0c\x17\n\x0c\n\x05\x06\x01\x02\x02\
    \x05\x12\x03J\x18\x1e\n\x0c\n\x05\x06\x01\x02\x02\x02\x12\x03J\x1f1\n\
    \x0c\n\x05\x06\x01\x02\x02\x03\x12\x03J<O\n\x0b\n\x04\x06\x01\x02\x03\
    \x12\x03L\x087\n\x0c\n\x05\x06\x01\x02\x03\x01\x12\x03L\x0c\x10\n\x0c\n\
    \x05\x06\x01\x02\x03\x02\x12\x03L\x11\x1c\n\x0c\n\x05\x06\x01\x02\x03\
    \x03\x12\x03L'3\n\x0b\n\x04\x06\x01\x02\x04\x12\x03M\x08C\n\x0c\n\x05\
    \x06\x01\x02\x04\x01\x12\x03M\x0c\x14\n\x0c\n\x05\x06\x01\x02\x04\x02\
    \x12\x03M\x15$\n\x0c\n\x05\x06\x01\x02\x04\x03\x12\x03M/?\n\x0b\n\x04\
    \x06\x01\x02\x05\x12\x03N\x08P\n\x0c\n\x05\x06\x01\x02\x05\x01\x12\x03N\
    \x0c\x16\n\x0c\n\x05\x06\x01\x02\x05\x02\x12\x03N\x17(\n\x0c\n\x05\x06\
    \x01\x02\x05\x06\x12\x03N39\n\x0c\n\x05\x06\x01\x02\x05\x03\x12\x03N:L\n\
    \x0b\n\x04\x06\x01\x02\x06\x12\x03P\x08=\n\x0c\n\x05\x06\x01\x02\x06\x01\
    \x12\x03P\x0c\x12\n\x0c\n\x05\x06\x01\x02\x06\x02\x12\x03P\x13\x20\n\x0c\
    \n\x05\x06\x01\x02\x06\x03\x12\x03P+9\n\x0b\n\x04\x06\x01\x02\x07\x12\
    \x03R\x08:\n\x0c\n\x05\x06\x01\x02\x07\x01\x12\x03R\x0c\x11\n\x0c\n\x05\
    \x06\x01\x02\x07\x02\x12\x03R\x12\x1e\n\x0c\n\x05\x06\x01\x02\x07\x03\
    \x12\x03R)6\n\x0b\n\x04\x06\x01\x02\x08\x12\x03S\x08=\n\x0c\n\x05\x06\
    \x01\x02\x08\x01\x12\x03S\x0c\x12\n\x0c\n\x05\x06\x01\x02\x08\x02\x12\
    \x03S\x13\x20\n\x0c\n\x05\x06\x01\x02\x08\x03\x12\x03S+9\n\n\n\x02\x04\t\
    \x12\x04V\0j\x01\n\n\n\x03\x04\t\x01\x12\x03V\x08\x10\nY\n\x04\x04\t\x02\
    \0\x12\x03Y\x04\x12\x1aL\x20key\x20defines\x20the\x20key\x20of\x20the\
    \x20data,\n\x20and\x20is\x20chosen\x20by\x20the\x20owner\x20of\x20this\
    \x20data.\n\n\r\n\x05\x04\t\x02\0\x04\x12\x04Y\x04V\x12\n\x0c\n\x05\x04\
    \t\x02\0\x05\x12\x03Y\x04\t\n\x0c\n\x05\x04\t\x02\0\x01\x12\x03Y\n\r\n\
    \x0c\n\x05\x04\t\x02\0\x03\x12\x03Y\x10\x11\ns\n\x04\x04\t\x02\x01\x12\
    \x03]\x04\x18\x1af\x20totalSize\x20in\x20bytes\x20represents\x20the\x20t\
    otal\x20size\x20of\x20all\x20chunks,\n\x20that\x20make\x20up\x20the\x20s\
    tored\x20data,\x20combined.\n\n\r\n\x05\x04\t\x02\x01\x04\x12\x04]\x04Y\
    \x12\n\x0c\n\x05\x04\t\x02\x01\x05\x12\x03]\x04\t\n\x0c\n\x05\x04\t\x02\
    \x01\x01\x12\x03]\n\x13\n\x0c\n\x05\x04\t\x02\x01\x03\x12\x03]\x16\x17\n\
    z\n\x04\x04\t\x02\x02\x12\x03a\x04\x1c\x1am\x20creationEpoch\x20defines\
    \x20the\x20time\x20this\x20data\x20was\x20initially\x20created,\n\x20in\
    \x20the\x20Unix\x20epoch\x20format,\x20in\x20nano\x20seconds.\n\n\r\n\
    \x05\x04\t\x02\x02\x04\x12\x04a\x04]\x18\n\x0c\n\x05\x04\t\x02\x02\x05\
    \x12\x03a\x04\t\n\x0c\n\x05\x04\t\x02\x02\x01\x12\x03a\n\x17\n\x0c\n\x05\
    \x04\t\x02\x02\x03\x12\x03a\x1a\x1b\n\x88\x01\n\x04\x04\t\x02\x03\x12\
    \x03e\x04\x1d\x1a{\x20lastWriteEpoch\x20defines\x20the\x20time\x20this\
    \x20data\n\x20was\x20last\x20modified\x20(e.g.\x20repaired),\n\x20in\x20\
    the\x20Unix\x20epoch\x20format,\x20in\x20nano\x20seconds.\n\n\r\n\x05\
    \x04\t\x02\x03\x04\x12\x04e\x04a\x1c\n\x0c\n\x05\x04\t\x02\x03\x05\x12\
    \x03e\x04\t\n\x0c\n\x05\x04\t\x02\x03\x01\x12\x03e\n\x18\n\x0c\n\x05\x04\
    \t\x02\x03\x03\x12\x03e\x1b\x1c\n_\n\x04\x04\t\x02\x04\x12\x03i\x04\x1e\
    \x1aR\x20chunks\x20is\x20the\x20metadata\x20list\x20of\x20all\x20chunks\
    \n\x20that\x20make\x20up\x20the\x20data,\x20when\x20combined.\n\n\x0c\n\
    \x05\x04\t\x02\x04\x04\x12\x03i\x04\x0c\n\x0c\n\x05\x04\t\x02\x04\x06\
    \x12\x03i\r\x12\n\x0c\n\x05\x04\t\x02\x04\x01\x12\x03i\x13\x19\n\x0c\n\
    \x05\x04\t\x02\x04\x03\x12\x03i\x1c\x1d\n\n\n\x02\x04\n\x12\x04k\0v\x01\
    \n\n\n\x03\x04\n\x01\x12\x03k\x08\r\n.\n\x04\x04\n\x02\0\x12\x03m\x04\
    \x18\x1a!\x20chunkSize\x20of\x20the\x20chunk\x20in\x20bytes\n\n\r\n\x05\
    \x04\n\x02\0\x04\x12\x04m\x04k\x0f\n\x0c\n\x05\x04\n\x02\0\x05\x12\x03m\
    \x04\t\n\x0c\n\x05\x04\n\x02\0\x01\x12\x03m\n\x13\n\x0c\n\x05\x04\n\x02\
    \0\x03\x12\x03m\x16\x17\nT\n\x04\x04\n\x02\x01\x12\x03q\x04\x20\x1aG\x20\
    objects\x20defines\x20the\x20metadata\x20of\x20the\x20objects\n\x20that\
    \x20make\x20up\x20this\x20chunk.\n\n\x0c\n\x05\x04\n\x02\x01\x04\x12\x03\
    q\x04\x0c\n\x0c\n\x05\x04\n\x02\x01\x06\x12\x03q\r\x13\n\x0c\n\x05\x04\n\
    \x02\x01\x01\x12\x03q\x14\x1b\n\x0c\n\x05\x04\n\x02\x01\x03\x12\x03q\x1e\
    \x1f\n\x83\x01\n\x04\x04\n\x02\x02\x12\x03u\x04\x13\x1av\x20hash\x20cont\
    ains\x20the\x20checksum/signature\x20of\x20the\x20chunk\x20(data),\n\x20\
    meaning\x20the\x20data\x20of\x20all\x20objects\x20(of\x20this\x20chunk)\
    \x20combined.\n\n\r\n\x05\x04\n\x02\x02\x04\x12\x04u\x04q\x20\n\x0c\n\
    \x05\x04\n\x02\x02\x05\x12\x03u\x04\t\n\x0c\n\x05\x04\n\x02\x02\x01\x12\
    \x03u\n\x0e\n\x0c\n\x05\x04\n\x02\x02\x03\x12\x03u\x11\x12\n\n\n\x02\x04\
    \x0b\x12\x04w\0}\x01\n\n\n\x03\x04\x0b\x01\x12\x03w\x08\x0e\n\x20\n\x04\
    \x04\x0b\x02\0\x12\x03y\x04\x12\x1a\x13\x20key\x20of\x20the\x20Object\n\
    \n\r\n\x05\x04\x0b\x02\0\x04\x12\x04y\x04w\x10\n\x0c\n\x05\x04\x0b\x02\0\
    \x05\x12\x03y\x04\t\n\x0c\n\x05\x04\x0b\x02\0\x01\x12\x03y\n\r\n\x0c\n\
    \x05\x04\x0b\x02\0\x03\x12\x03y\x10\x11\nJ\n\x04\x04\x0b\x02\x01\x12\x03\
    |\x04\x17\x1a=\x20shardID\x20defines\x20the\x20ID\x20of\x20the\x20shard\
    \x20the\x20object\x20is\x20stored\x20on\n\n\r\n\x05\x04\x0b\x02\x01\x04\
    \x12\x04|\x04y\x12\n\x0c\n\x05\x04\x0b\x02\x01\x05\x12\x03|\x04\n\n\x0c\
    \n\x05\x04\x0b\x02\x01\x01\x12\x03|\x0b\x12\n\x0c\n\x05\x04\x0b\x02\x01\
    \x03\x12\x03|\x15\x16\n\x0b\n\x02\x05\0\x12\x05\x7f\0\x83\x01\x01\n\n\n\
    \x03\x05\0\x01\x12\x03\x7f\x05\x10\n\x0c\n\x04\x05\0\x02\0\x12\x04\x80\
    \x01\x08\x1f\n\r\n\x05\x05\0\x02\0\x01\x12\x04\x80\x01\x08\x1a\n\r\n\x05\
    \x05\0\x02\0\x02\x12\x04\x80\x01\x1d\x1e\n\x0c\n\x04\x05\0\x02\x01\x12\
    \x04\x81\x01\x08\x1d\n\r\n\x05\x05\0\x02\x01\x01\x12\x04\x81\x01\x08\x18\
    \n\r\n\x05\x05\0\x02\x01\x02\x12\x04\x81\x01\x1b\x1c\n\x0c\n\x04\x05\0\
    \x02\x02\x12\x04\x82\x01\x08\x1f\n\r\n\x05\x05\0\x02\x02\x01\x12\x04\x82\
    \x01\x08\x1a\n\r\n\x05\x05\0\x02\x02\x02\x12\x04\x82\x01\x1d\x1e\n\x0c\n\
    \x02\x05\x01\x12\x06\x85\x01\0\x8c\x01\x01\n\x0b\n\x03\x05\x01\x01\x12\
    \x04\x85\x01\x05\r\nS\n\x04\x05\x01\x02\0\x12\x04\x87\x01\x08\x1d\x1aE\
    \x20if\x20the\x20file\x20already\x20exists,\x20truncate\x20it\x20to\x200\
    \x20bytes\x20prior\x20to\x20writing\n\n\r\n\x05\x05\x01\x02\0\x01\x12\
    \x04\x87\x01\x08\x18\n\r\n\x05\x05\x01\x02\0\x02\x12\x04\x87\x01\x1b\x1c\
    \nP\n\x04\x05\x01\x02\x01\x12\x04\x89\x01\x08\x1b\x1aB\x20append\x20to\
    \x20the\x20file\x20should\x20it\x20already\x20exists,\x20otherwise\x20cr\
    eate\x20it\n\n\r\n\x05\x05\x01\x02\x01\x01\x12\x04\x89\x01\x08\x16\n\r\n\
    \x05\x05\x01\x02\x01\x02\x12\x04\x89\x01\x19\x1a\n:\n\x04\x05\x01\x02\
    \x02\x12\x04\x8b\x01\x08\x1e\x1a,\x20create\x20a\x20non-existing\x20file\
    \x20and\x20write\x20to\x20it\n\n\r\n\x05\x05\x01\x02\x02\x01\x12\x04\x8b\
    \x01\x08\x19\n\r\n\x05\x05\x01\x02\x02\x02\x12\x04\x8b\x01\x1c\x1d\n\x0c\
    \n\x02\x04\x0c\x12\x06\x8e\x01\0\x91\x01\x01\n\x0b\n\x03\x04\x0c\x01\x12\
    \x04\x8e\x01\x08\x14\n\x0c\n\x04\x04\x0c\x02\0\x12\x04\x8f\x01\x08\x16\n\
    \x0f\n\x05\x04\x0c\x02\0\x04\x12\x06\x8f\x01\x08\x8e\x01\x16\n\r\n\x05\
    \x04\x0c\x02\0\x05\x12\x04\x8f\x01\x08\r\n\r\n\x05\x04\x0c\x02\0\x01\x12\
    \x04\x8f\x01\x0e\x11\n\r\n\x05\x04\x0c\x02\0\x03\x12\x04\x8f\x01\x14\x15\
    \n\x0c\n\x04\x04\x0c\x02\x01\x12\x04\x90\x01\x08\x17\n\x0f\n\x05\x04\x0c\
    \x02\x01\x04\x12\x06\x90\x01\x08\x8f\x01\x16\n\r\n\x05\x04\x0c\x02\x01\
    \x05\x12\x04\x90\x01\x08\r\n\r\n\x05\x04\x0c\x02\x01\x01\x12\x04\x90\x01\
    \x0e\x12\n\r\n\x05\x04\x0c\x02\x01\x03\x12\x04\x90\x01\x15\x16\n\x0c\n\
    \x02\x04\r\x12\x06\x92\x01\0\x94\x01\x01\n\x0b\n\x03\x04\r\x01\x12\x04\
    \x92\x01\x08\x15\n\x0c\n\x04\x04\r\x02\0\x12\x04\x93\x01\x08\x1e\n\x0f\n\
    \x05\x04\r\x02\0\x04\x12\x06\x93\x01\x08\x92\x01\x17\n\r\n\x05\x04\r\x02\
    \0\x06\x12\x04\x93\x01\x08\x10\n\r\n\x05\x04\r\x02\0\x01\x12\x04\x93\x01\
    \x11\x19\n\r\n\x05\x04\r\x02\0\x03\x12\x04\x93\x01\x1c\x1d\n\x0c\n\x02\
    \x04\x0e\x12\x06\x96\x01\0\x99\x01\x01\n\x0b\n\x03\x04\x0e\x01\x12\x04\
    \x96\x01\x08\x18\n\x0c\n\x04\x04\x0e\x02\0\x12\x04\x97\x01\x08\x16\n\x0f\
    \n\x05\x04\x0e\x02\0\x04\x12\x06\x97\x01\x08\x96\x01\x1a\n\r\n\x05\x04\
    \x0e\x02\0\x05\x12\x04\x97\x01\x08\r\n\r\n\x05\x04\x0e\x02\0\x01\x12\x04\
    \x97\x01\x0e\x11\n\r\n\x05\x04\x0e\x02\0\x03\x12\x04\x97\x01\x14\x15\n\
    \x0c\n\x04\x04\x0e\x02\x01\x12\x04\x98\x01\x08\x1c\n\x0f\n\x05\x04\x0e\
    \x02\x01\x04\x12\x06\x98\x01\x08\x97\x01\x16\n\r\n\x05\x04\x0e\x02\x01\
    \x05\x12\x04\x98\x01\x08\x0e\n\r\n\x05\x04\x0e\x02\x01\x01\x12\x04\x98\
    \x01\x0f\x17\n\r\n\x05\x04\x0e\x02\x01\x03\x12\x04\x98\x01\x1a\x1b\n\x0c\
    \n\x02\x04\x0f\x12\x06\x9a\x01\0\x9c\x01\x01\n\x0b\n\x03\x04\x0f\x01\x12\
    \x04\x9a\x01\x08\x19\n\x0c\n\x04\x04\x0f\x02\0\x12\x04\x9b\x01\x08\x1e\n\
    \x0f\n\x05\x04\x0f\x02\0\x04\x12\x06\x9b\x01\x08\x9a\x01\x1b\n\r\n\x05\
    \x04\x0f\x02\0\x06\x12\x04\x9b\x01\x08\x10\n\r\n\x05\x04\x0f\x02\0\x01\
    \x12\x04\x9b\x01\x11\x19\n\r\n\x05\x04\x0f\x02\0\x03\x12\x04\x9b\x01\x1c\
    \x1d\n-\n\x02\x04\x10\x12\x06\x9f\x01\0\xab\x01\x01\x1a\x1f\x20key\x20is\
    \x20send\x20as\x20part\x20of\x20header\n\n\x0b\n\x03\x04\x10\x01\x12\x04\
    \x9f\x01\x08\x1a\n\x0e\n\x04\x04\x10\x08\0\x12\x06\xa0\x01\x08\xa3\x01\t\
    \n\r\n\x05\x04\x10\x08\0\x01\x12\x04\xa0\x01\x0e\x13\n4\n\x04\x04\x10\
    \x02\0\x12\x04\xa1\x01\x10&\"&\x20first\x20message\x20should\x20be\x20th\
    e\x20metadata\n\n\r\n\x05\x04\x10\x02\0\x06\x12\x04\xa1\x01\x10\x18\n\r\
    \n\x05\x04\x10\x02\0\x01\x12\x04\xa1\x01\x19!\n\r\n\x05\x04\x10\x02\0\
    \x03\x12\x04\xa1\x01$%\nG\n\x04\x04\x10\x02\x01\x12\x04\xa2\x01\x10\x1e\
    \"9\x20all\x20other\x20messages\x20(at\x20least\x20one)\x20should\x20be\
    \x20data\x20itself\n\n\r\n\x05\x04\x10\x02\x01\x06\x12\x04\xa2\x01\x10\
    \x14\n\r\n\x05\x04\x10\x02\x01\x01\x12\x04\xa2\x01\x15\x19\n\r\n\x05\x04\
    \x10\x02\x01\x03\x12\x04\xa2\x01\x1c\x1d\n\x0e\n\x04\x04\x10\x03\0\x12\
    \x06\xa5\x01\x08\xa7\x01\t\n\r\n\x05\x04\x10\x03\0\x01\x12\x04\xa5\x01\
    \x10\x18\n\x0e\n\x06\x04\x10\x03\0\x02\0\x12\x04\xa6\x01\x10\x1e\n\x11\n\
    \x07\x04\x10\x03\0\x02\0\x04\x12\x06\xa6\x01\x10\xa5\x01\x1a\n\x0f\n\x07\
    \x04\x10\x03\0\x02\0\x05\x12\x04\xa6\x01\x10\x15\n\x0f\n\x07\x04\x10\x03\
    \0\x02\0\x01\x12\x04\xa6\x01\x16\x19\n\x0f\n\x07\x04\x10\x03\0\x02\0\x03\
    \x12\x04\xa6\x01\x1c\x1d\n\x0e\n\x04\x04\x10\x03\x01\x12\x06\xa8\x01\x08\
    \xaa\x01\t\n\r\n\x05\x04\x10\x03\x01\x01\x12\x04\xa8\x01\x10\x14\n\x0e\n\
    \x06\x04\x10\x03\x01\x02\0\x12\x04\xa9\x01\x10$\n\x11\n\x07\x04\x10\x03\
    \x01\x02\0\x04\x12\x06\xa9\x01\x10\xa8\x01\x16\n\x0f\n\x07\x04\x10\x03\
    \x01\x02\0\x05\x12\x04\xa9\x01\x10\x15\n\x0f\n\x07\x04\x10\x03\x01\x02\0\
    \x01\x12\x04\xa9\x01\x16\x1f\n\x0f\n\x07\x04\x10\x03\x01\x02\0\x03\x12\
    \x04\xa9\x01\"#\n\x0c\n\x02\x04\x11\x12\x06\xac\x01\0\xae\x01\x01\n\x0b\
    \n\x03\x04\x11\x01\x12\x04\xac\x01\x08\x1b\n\x0c\n\x04\x04\x11\x02\0\x12\
    \x04\xad\x01\x08\x1e\n\x0f\n\x05\x04\x11\x02\0\x04\x12\x06\xad\x01\x08\
    \xac\x01\x1d\n\r\n\x05\x04\x11\x02\0\x06\x12\x04\xad\x01\x08\x10\n\r\n\
    \x05\x04\x11\x02\0\x01\x12\x04\xad\x01\x11\x19\n\r\n\x05\x04\x11\x02\0\
    \x03\x12\x04\xad\x01\x1c\x1d\n\x0c\n\x02\x04\x12\x12\x06\xb0\x01\0\xb5\
    \x01\x01\n\x0b\n\x03\x04\x12\x01\x12\x04\xb0\x01\x08\x13\n\x0e\n\x04\x04\
    \x12\x08\0\x12\x06\xb1\x01\x08\xb4\x01\t\n\r\n\x05\x04\x12\x08\0\x01\x12\
    \x04\xb1\x01\x0e\x13\n\x0c\n\x04\x04\x12\x02\0\x12\x04\xb2\x01\x10\x1e\n\
    \r\n\x05\x04\x12\x02\0\x05\x12\x04\xb2\x01\x10\x15\n\r\n\x05\x04\x12\x02\
    \0\x01\x12\x04\xb2\x01\x16\x19\n\r\n\x05\x04\x12\x02\0\x03\x12\x04\xb2\
    \x01\x1c\x1d\n\x0c\n\x04\x04\x12\x02\x01\x12\x04\xb3\x01\x10&\n\r\n\x05\
    \x04\x12\x02\x01\x06\x12\x04\xb3\x01\x10\x18\n\r\n\x05\x04\x12\x02\x01\
    \x01\x12\x04\xb3\x01\x19!\n\r\n\x05\x04\x12\x02\x01\x03\x12\x04\xb3\x01$\
    %\n\x0c\n\x02\x04\x13\x12\x06\xb6\x01\0\xb8\x01\x01\n\x0b\n\x03\x04\x13\
    \x01\x12\x04\xb6\x01\x08\x14\n\x0c\n\x04\x04\x13\x02\0\x12\x04\xb7\x01\
    \x08\x17\n\x0f\n\x05\x04\x13\x02\0\x04\x12\x06\xb7\x01\x08\xb6\x01\x16\n\
    \r\n\x05\x04\x13\x02\0\x05\x12\x04\xb7\x01\x08\r\n\r\n\x05\x04\x13\x02\0\
    \x01\x12\x04\xb7\x01\x0e\x12\n\r\n\x05\x04\x13\x02\0\x03\x12\x04\xb7\x01\
    \x15\x16\n\x0c\n\x02\x04\x14\x12\x06\xba\x01\0\xc5\x01\x01\n\x0b\n\x03\
    \x04\x14\x01\x12\x04\xba\x01\x08\x17\n\x0e\n\x04\x04\x14\x08\0\x12\x06\
    \xbb\x01\x08\xbe\x01\t\n\r\n\x05\x04\x14\x08\0\x01\x12\x04\xbb\x01\x0e\
    \x13\n\x0c\n\x04\x04\x14\x02\0\x12\x04\xbc\x01\x10\x1e\n\r\n\x05\x04\x14\
    \x02\0\x05\x12\x04\xbc\x01\x10\x15\n\r\n\x05\x04\x14\x02\0\x01\x12\x04\
    \xbc\x01\x16\x19\n\r\n\x05\x04\x14\x02\0\x03\x12\x04\xbc\x01\x1c\x1d\n\
    \x0c\n\x04\x04\x14\x02\x01\x12\x04\xbd\x01\x10&\n\r\n\x05\x04\x14\x02\
    \x01\x06\x12\x04\xbd\x01\x10\x18\n\r\n\x05\x04\x14\x02\x01\x01\x12\x04\
    \xbd\x01\x19!\n\r\n\x05\x04\x14\x02\x01\x03\x12\x04\xbd\x01$%\n6\n\x04\
    \x04\x14\x02\x02\x12\x04\xc1\x01\x08\x1c\x1a(\x20destination\x20file\x20\
    and\x20its\x20configuration\n\n\x0f\n\x05\x04\x14\x02\x02\x04\x12\x06\
    \xc1\x01\x08\xbe\x01\t\n\r\n\x05\x04\x14\x02\x02\x05\x12\x04\xc1\x01\x08\
    \x0e\n\r\n\x05\x04\x14\x02\x02\x01\x12\x04\xc1\x01\x0f\x17\n\r\n\x05\x04\
    \x14\x02\x02\x03\x12\x04\xc1\x01\x1a\x1b\n\x0c\n\x04\x04\x14\x02\x03\x12\
    \x04\xc2\x01\x08\x1e\n\x0f\n\x05\x04\x14\x02\x03\x04\x12\x06\xc2\x01\x08\
    \xc1\x01\x1c\n\r\n\x05\x04\x14\x02\x03\x06\x12\x04\xc2\x01\x08\x10\n\r\n\
    \x05\x04\x14\x02\x03\x01\x12\x04\xc2\x01\x11\x19\n\r\n\x05\x04\x14\x02\
    \x03\x03\x12\x04\xc2\x01\x1c\x1d\n-\n\x04\x04\x14\x02\x04\x12\x04\xc3\
    \x01\x08\x1f\"\x1f\x20open\x20file\x20for\x20synchronous\x20I/O\n\n\x0f\
    \n\x05\x04\x14\x02\x04\x04\x12\x06\xc3\x01\x08\xc2\x01\x1e\n\r\n\x05\x04\
    \x14\x02\x04\x05\x12\x04\xc3\x01\x08\x0c\n\r\n\x05\x04\x14\x02\x04\x01\
    \x12\x04\xc3\x01\r\x1a\n\r\n\x05\x04\x14\x02\x04\x03\x12\x04\xc3\x01\x1d\
    \x1e\n\x0c\n\x02\x04\x15\x12\x06\xc6\x01\0\xc7\x01\x01\n\x0b\n\x03\x04\
    \x15\x01\x12\x04\xc6\x01\x08\x18\n\x0c\n\x02\x04\x16\x12\x06\xc9\x01\0\
    \xcf\x01\x01\n\x0b\n\x03\x04\x16\x01\x12\x04\xc9\x01\x08\x19\n\x0e\n\x04\
    \x04\x16\x08\0\x12\x06\xca\x01\x08\xcd\x01\t\n\r\n\x05\x04\x16\x08\0\x01\
    \x12\x04\xca\x01\x0e\x13\n\x0c\n\x04\x04\x16\x02\0\x12\x04\xcb\x01\x10\
    \x1e\n\r\n\x05\x04\x16\x02\0\x05\x12\x04\xcb\x01\x10\x15\n\r\n\x05\x04\
    \x16\x02\0\x01\x12\x04\xcb\x01\x16\x19\n\r\n\x05\x04\x16\x02\0\x03\x12\
    \x04\xcb\x01\x1c\x1d\n\x0c\n\x04\x04\x16\x02\x01\x12\x04\xcc\x01\x10&\n\
    \r\n\x05\x04\x16\x02\x01\x06\x12\x04\xcc\x01\x10\x18\n\r\n\x05\x04\x16\
    \x02\x01\x01\x12\x04\xcc\x01\x19!\n\r\n\x05\x04\x16\x02\x01\x03\x12\x04\
    \xcc\x01$%\n\x0c\n\x04\x04\x16\x02\x02\x12\x04\xce\x01\x08\x1c\n\x0f\n\
    \x05\x04\x16\x02\x02\x04\x12\x06\xce\x01\x08\xcd\x01\t\n\r\n\x05\x04\x16\
    \x02\x02\x05\x12\x04\xce\x01\x08\r\n\r\n\x05\x04\x16\x02\x02\x01\x12\x04\
    \xce\x01\x0e\x17\n\r\n\x05\x04\x16\x02\x02\x03\x12\x04\xce\x01\x1a\x1b\n\
    \x0c\n\x02\x04\x17\x12\x06\xd0\x01\0\xd2\x01\x01\n\x0b\n\x03\x04\x17\x01\
    \x12\x04\xd0\x01\x08\x1a\n\x0c\n\x04\x04\x17\x02\0\x12\x04\xd1\x01\x08\
    \x1c\n\x0f\n\x05\x04\x17\x02\0\x04\x12\x06\xd1\x01\x08\xd0\x01\x1c\n\r\n\
    \x05\x04\x17\x02\0\x05\x12\x04\xd1\x01\x08\r\n\r\n\x05\x04\x17\x02\0\x01\
    \x12\x04\xd1\x01\x0e\x17\n\r\n\x05\x04\x17\x02\0\x03\x12\x04\xd1\x01\x1a\
    \x1b\n\x0c\n\x02\x04\x18\x12\x06\xd4\x01\0\xd9\x01\x01\n\x0b\n\x03\x04\
    \x18\x01\x12\x04\xd4\x01\x08\x15\n\x0e\n\x04\x04\x18\x08\0\x12\x06\xd5\
    \x01\x08\xd8\x01\t\n\r\n\x05\x04\x18\x08\0\x01\x12\x04\xd5\x01\x0e\x13\n\
    \x0c\n\x04\x04\x18\x02\0\x12\x04\xd6\x01\x10\x1e\n\r\n\x05\x04\x18\x02\0\
    \x05\x12\x04\xd6\x01\x10\x15\n\r\n\x05\x04\x18\x02\0\x01\x12\x04\xd6\x01\
    \x16\x19\n\r\n\x05\x04\x18\x02\0\x03\x12\x04\xd6\x01\x1c\x1d\n\x0c\n\x04\
    \x04\x18\x02\x01\x12\x04\xd7\x01\x10&\n\r\n\x05\x04\x18\x02\x01\x06\x12\
    \x04\xd7\x01\x10\x18\n\r\n\x05\x04\x18\x02\x01\x01\x12\x04\xd7\x01\x19!\
    \n\r\n\x05\x04\x18\x02\x01\x03\x12\x04\xd7\x01$%\n\x0c\n\x02\x04\x19\x12\
    \x06\xda\x01\0\xdb\x01\x01\n\x0b\n\x03\x04\x19\x01\x12\x04\xda\x01\x08\
    \x16\n\x0c\n\x02\x04\x1a\x12\x06\xdd\x01\0\xe3\x01\x01\n\x0b\n\x03\x04\
    \x1a\x01\x12\x04\xdd\x01\x08\x14\n\x0e\n\x04\x04\x1a\x08\0\x12\x06\xde\
    \x01\x08\xe1\x01\t\n\r\n\x05\x04\x1a\x08\0\x01\x12\x04\xde\x01\x0e\x13\n\
    \x0c\n\x04\x04\x1a\x02\0\x12\x04\xdf\x01\x10\x1e\n\r\n\x05\x04\x1a\x02\0\
    \x05\x12\x04\xdf\x01\x10\x15\n\r\n\x05\x04\x1a\x02\0\x01\x12\x04\xdf\x01\
    \x16\x19\n\r\n\x05\x04\x1a\x02\0\x03\x12\x04\xdf\x01\x1c\x1d\n\x0c\n\x04\
    \x04\x1a\x02\x01\x12\x04\xe0\x01\x10&\n\r\n\x05\x04\x1a\x02\x01\x06\x12\
    \x04\xe0\x01\x10\x18\n\r\n\x05\x04\x1a\x02\x01\x01\x12\x04\xe0\x01\x19!\
    \n\r\n\x05\x04\x1a\x02\x01\x03\x12\x04\xe0\x01$%\n\x0c\n\x04\x04\x1a\x02\
    \x02\x12\x04\xe2\x01\x08\x16\n\x0f\n\x05\x04\x1a\x02\x02\x04\x12\x06\xe2\
    \x01\x08\xe1\x01\t\n\r\n\x05\x04\x1a\x02\x02\x05\x12\x04\xe2\x01\x08\x0c\
    \n\r\n\x05\x04\x1a\x02\x02\x01\x12\x04\xe2\x01\r\x11\n\r\n\x05\x04\x1a\
    \x02\x02\x03\x12\x04\xe2\x01\x14\x15\n\x0c\n\x02\x04\x1b\x12\x06\xe4\x01\
    \0\xe6\x01\x01\n\x0b\n\x03\x04\x1b\x01\x12\x04\xe4\x01\x08\x15\n\x0c\n\
    \x04\x04\x1b\x02\0\x12\x04\xe5\x01\x08\x1f\n\x0f\n\x05\x04\x1b\x02\0\x04\
    \x12\x06\xe5\x01\x08\xe4\x01\x17\n\r\n\x05\x04\x1b\x02\0\x06\x12\x04\xe5\
    \x01\x08\x13\n\r\n\x05\x04\x1b\x02\0\x01\x12\x04\xe5\x01\x14\x1a\n\r\n\
    \x05\x04\x1b\x02\0\x03\x12\x04\xe5\x01\x1d\x1e\n\x0c\n\x02\x04\x1c\x12\
    \x06\xe8\x01\0\xea\x01\x01\n\x0b\n\x03\x04\x1c\x01\x12\x04\xe8\x01\x08\
    \x15\n\x0c\n\x04\x04\x1c\x02\0\x12\x04\xe9\x01\x08\x16\n\x0f\n\x05\x04\
    \x1c\x02\0\x04\x12\x06\xe9\x01\x08\xe8\x01\x17\n\r\n\x05\x04\x1c\x02\0\
    \x05\x12\x04\xe9\x01\x08\r\n\r\n\x05\x04\x1c\x02\0\x01\x12\x04\xe9\x01\
    \x0e\x11\n\r\n\x05\x04\x1c\x02\0\x03\x12\x04\xe9\x01\x14\x15\n\x0c\n\x02\
    \x04\x1d\x12\x06\xeb\x01\0\xed\x01\x01\n\x0b\n\x03\x04\x1d\x01\x12\x04\
    \xeb\x01\x08\x16\n\x0c\n\x04\x04\x1d\x02\0\x12\x04\xec\x01\x08\x1e\n\x0f\
    \n\x05\x04\x1d\x02\0\x04\x12\x06\xec\x01\x08\xeb\x01\x18\n\r\n\x05\x04\
    \x1d\x02\0\x06\x12\x04\xec\x01\x08\x10\n\r\n\x05\x04\x1d\x02\0\x01\x12\
    \x04\xec\x01\x11\x19\n\r\n\x05\x04\x1d\x02\0\x03\x12\x04\xec\x01\x1c\x1d\
    \n\x7f\n\x02\x06\x02\x12\x06\xf1\x01\0\xfb\x01\x01\x1aq\x20MetadataServi\
    ce\x20is\x20used\x20to\x20set,\x20get\x20and\x20delete\x20metadata\x20di\
    rectly\n\x20using\x20the\x20underlying\x20metadata\x20storage\x20client.\
    \n\n\x0b\n\x03\x06\x02\x01\x12\x04\xf1\x01\x08\x17\n\x0c\n\x04\x06\x02\
    \x02\0\x12\x04\xf2\x01\x08L\n\r\n\x05\x06\x02\x02\0\x01\x12\x04\xf2\x01\
    \x0c\x17\n\r\n\x05\x06\x02\x02\0\x02\x12\x04\xf2\x01\x18*\n\r\n\x05\x06\
    \x02\x02\0\x03\x12\x04\xf2\x015H\n\x0c\n\x04\x06\x02\x02\x01\x12\x04\xf3\
    \x01\x08L\n\r\n\x05\x06\x02\x02\x01\x01\x12\x04\xf3\x01\x0c\x17\n\r\n\
    \x05\x06\x02\x02\x01\x02\x12\x04\xf3\x01\x18*\n\r\n\x05\x06\x02\x02\x01\
    \x03\x12\x04\xf3\x015H\n\x0c\n\x04\x06\x02\x02\x02\x12\x04\xf4\x01\x08U\
    \n\r\n\x05\x06\x02\x02\x02\x01\x12\x04\xf4\x01\x0c\x1a\n\r\n\x05\x06\x02\
    \x02\x02\x02\x12\x04\xf4\x01\x1b0\n\r\n\x05\x06\x02\x02\x02\x03\x12\x04\
    \xf4\x01;Q\n\x0c\n\x02\x04\x1e\x12\x06\xfd\x01\0\xff\x01\x01\n\x0b\n\x03\
    \x04\x1e\x01\x12\x04\xfd\x01\x08\x1a\n\x0c\n\x04\x04\x1e\x02\0\x12\x04\
    \xfe\x01\x08\x1e\n\x0f\n\x05\x04\x1e\x02\0\x04\x12\x06\xfe\x01\x08\xfd\
    \x01\x1c\n\r\n\x05\x04\x1e\x02\0\x06\x12\x04\xfe\x01\x08\x10\n\r\n\x05\
    \x04\x1e\x02\0\x01\x12\x04\xfe\x01\x11\x19\n\r\n\x05\x04\x1e\x02\0\x03\
    \x12\x04\xfe\x01\x1c\x1d\n\x0c\n\x02\x04\x1f\x12\x06\x80\x02\0\x81\x02\
    \x01\n\x0b\n\x03\x04\x1f\x01\x12\x04\x80\x02\x08\x1b\n\x0c\n\x02\x04\x20\
    \x12\x06\x83\x02\0\x85\x02\x01\n\x0b\n\x03\x04\x20\x01\x12\x04\x83\x02\
    \x08\x1a\n\x0c\n\x04\x04\x20\x02\0\x12\x04\x84\x02\x08\x16\n\x0f\n\x05\
    \x04\x20\x02\0\x04\x12\x06\x84\x02\x08\x83\x02\x1c\n\r\n\x05\x04\x20\x02\
    \0\x05\x12\x04\x84\x02\x08\r\n\r\n\x05\x04\x20\x02\0\x01\x12\x04\x84\x02\
    \x0e\x11\n\r\n\x05\x04\x20\x02\0\x03\x12\x04\x84\x02\x14\x15\n\x0c\n\x02\
    \x04!\x12\x06\x86\x02\0\x88\x02\x01\n\x0b\n\x03\x04!\x01\x12\x04\x86\x02\
    \x08\x1b\n\x0c\n\x04\x04!\x02\0\x12\x04\x87\x02\x08\x1e\n\x0f\n\x05\x04!\
    \x02\0\x04\x12\x06\x87\x02\x08\x86\x02\x1d\n\r\n\x05\x04!\x02\0\x06\x12\
    \x04\x87\x02\x08\x10\n\r\n\x05\x04!\x02\0\x01\x12\x04\x87\x02\x11\x19\n\
    \r\n\x05\x04!\x02\0\x03\x12\x04\x87\x02\x1c\x1d\n\x0c\n\x02\x04\"\x12\
    \x06\x8a\x02\0\x8c\x02\x01\n\x0b\n\x03\x04\"\x01\x12\x04\x8a\x02\x08\x1d\
    \n\x0c\n\x04\x04\"\x02\0\x12\x04\x8b\x02\x08\x16\n\x0f\n\x05\x04\"\x02\0\
    \x04\x12\x06\x8b\x02\x08\x8a\x02\x1f\n\r\n\x05\x04\"\x02\0\x05\x12\x04\
    \x8b\x02\x08\r\n\r\n\x05\x04\"\x02\0\x01\x12\x04\x8b\x02\x0e\x11\n\r\n\
    \x05\x04\"\x02\0\x03\x12\x04\x8b\x02\x14\x15\n\x0c\n\x02\x04#\x12\x06\
    \x8d\x02\0\x8e\x02\x01\n\x0b\n\x03\x04#\x01\x12\x04\x8d\x02\x08\x1e\n\
    \xca\x04\n\x02\x06\x03\x12\x06\x99\x02\0\xa6\x02\x01\x1a\xbb\x04\x20Data\
    Service\x20is\x20used\x20write,\x20read,\x20delete,\x20check\x20and\x20r\
    epair\x20(processed)\x20data.\n\x20as\x20data\x20can\x20be\x20written\
    \x20to\x20multiple\x20servers\x20and/or\x20be\x20split\x20over\x20multip\
    le\x20chunks,\n\x20some\x20metadata\x20is\x20returned\x20which\x20the\
    \x20user\x20is\x20expected\x20to\x20store\x20somewhere,\n\x20as\x20that\
    \x20metadata\x20is\x20required\x20to\x20read\x20and\x20manage\x20the\x20\
    data,\x20later\x20on.\n\n\x20Should\x20you\x20not\x20want\x20to\x20have\
    \x20to\x20deal\x20with\x20this\x20metadata\x20yourself,\n\x20you\x20shou\
    ld\x20use\x20the\x20FileService\x20instead,\x20which\x20is\x20one\x20lev\
    el\x20higher,\n\x20and\x20takes\x20care\x20of\x20managing\x20and\x20stor\
    ing\x20the\x20metadata\x20for\x20you,\n\x20as\x20well\x20as\x20generatin\
    g\x20extra\x20metadata,\x20such\x20as\x20the\x20creation\x20epoch\x20tim\
    e.\n\n\x0b\n\x03\x06\x03\x01\x12\x04\x99\x02\x08\x13\n\x0c\n\x04\x06\x03\
    \x02\0\x12\x04\x9a\x02\x08B\n\r\n\x05\x06\x03\x02\0\x01\x12\x04\x9a\x02\
    \x0c\x11\n\r\n\x05\x06\x03\x02\0\x02\x12\x04\x9a\x02\x12\"\n\r\n\x05\x06\
    \x03\x02\0\x03\x12\x04\x9a\x02->\n\x0c\n\x04\x06\x03\x02\x01\x12\x04\x9b\
    \x02\x08N\n\r\n\x05\x06\x03\x02\x01\x01\x12\x04\x9b\x02\x0c\x15\n\r\n\
    \x05\x06\x03\x02\x01\x02\x12\x04\x9b\x02\x16*\n\r\n\x05\x06\x03\x02\x01\
    \x03\x12\x04\x9b\x025J\n\x0c\n\x04\x06\x03\x02\x02\x12\x04\x9c\x02\x08[\
    \n\r\n\x05\x06\x03\x02\x02\x01\x12\x04\x9c\x02\x0c\x17\n\r\n\x05\x06\x03\
    \x02\x02\x05\x12\x04\x9c\x02\x18\x1e\n\r\n\x05\x06\x03\x02\x02\x02\x12\
    \x04\x9c\x02\x1f5\n\r\n\x05\x06\x03\x02\x02\x03\x12\x04\x9c\x02@W\n\x0c\
    \n\x04\x06\x03\x02\x03\x12\x04\x9e\x02\x08?\n\r\n\x05\x06\x03\x02\x03\
    \x01\x12\x04\x9e\x02\x0c\x10\n\r\n\x05\x06\x03\x02\x03\x02\x12\x04\x9e\
    \x02\x11\x20\n\r\n\x05\x06\x03\x02\x03\x03\x12\x04\x9e\x02+;\n\x0c\n\x04\
    \x06\x03\x02\x04\x12\x04\x9f\x02\x08K\n\r\n\x05\x06\x03\x02\x04\x01\x12\
    \x04\x9f\x02\x0c\x14\n\r\n\x05\x06\x03\x02\x04\x02\x12\x04\x9f\x02\x15(\
    \n\r\n\x05\x06\x03\x02\x04\x03\x12\x04\x9f\x023G\n\x0c\n\x04\x06\x03\x02\
    \x05\x12\x04\xa0\x02\x08X\n\r\n\x05\x06\x03\x02\x05\x01\x12\x04\xa0\x02\
    \x0c\x16\n\r\n\x05\x06\x03\x02\x05\x02\x12\x04\xa0\x02\x17,\n\r\n\x05\
    \x06\x03\x02\x05\x06\x12\x04\xa0\x027=\n\r\n\x05\x06\x03\x02\x05\x03\x12\
    \x04\xa0\x02>T\n\x0c\n\x04\x06\x03\x02\x06\x12\x04\xa2\x02\x08E\n\r\n\
    \x05\x06\x03\x02\x06\x01\x12\x04\xa2\x02\x0c\x12\n\r\n\x05\x06\x03\x02\
    \x06\x02\x12\x04\xa2\x02\x13$\n\r\n\x05\x06\x03\x02\x06\x03\x12\x04\xa2\
    \x02/A\n\x0c\n\x04\x06\x03\x02\x07\x12\x04\xa4\x02\x08B\n\r\n\x05\x06\
    \x03\x02\x07\x01\x12\x04\xa4\x02\x0c\x11\n\r\n\x05\x06\x03\x02\x07\x02\
    \x12\x04\xa4\x02\x12\"\n\r\n\x05\x06\x03\x02\x07\x03\x12\x04\xa4\x02->\n\
    \x0c\n\x04\x06\x03\x02\x08\x12\x04\xa5\x02\x08E\n\r\n\x05\x06\x03\x02\
    \x08\x01\x12\x04\xa5\x02\x0c\x12\n\r\n\x05\x06\x03\x02\x08\x02\x12\x04\
    \xa5\x02\x13$\n\r\n\x05\x06\x03\x02\x08\x03\x12\x04\xa5\x02/A\n\x0c\n\
    \x02\x04$\x12\x06\xa8\x02\0\xaa\x02\x01\n\x0b\n\x03\x04$\x01\x12\x04\xa8\
    \x02\x08\x18\n\x0c\n\x04\x04$\x02\0\x12\x04\xa9\x02\x08\x17\n\x0f\n\x05\
    \x04$\x02\0\x04\x12\x06\xa9\x02\x08\xa8\x02\x1a\n\r\n\x05\x04$\x02\0\x05\
    \x12\x04\xa9\x02\x08\r\n\r\n\x05\x04$\x02\0\x01\x12\x04\xa9\x02\x0e\x12\
    \n\r\n\x05\x04$\x02\0\x03\x12\x04\xa9\x02\x15\x16\n\x0c\n\x02\x04%\x12\
    \x06\xab\x02\0\xad\x02\x01\n\x0b\n\x03\x04%\x01\x12\x04\xab\x02\x08\x19\
    \n\x0c\n\x04\x04%\x02\0\x12\x04\xac\x02\x08\"\n\r\n\x05\x04%\x02\0\x04\
    \x12\x04\xac\x02\x08\x10\n\r\n\x05\x04%\x02\0\x06\x12\x04\xac\x02\x11\
    \x16\n\r\n\x05\x04%\x02\0\x01\x12\x04\xac\x02\x17\x1d\n\r\n\x05\x04%\x02\
    \0\x03\x12\x04\xac\x02\x20!\n\x0c\n\x02\x04&\x12\x06\xaf\x02\0\xb1\x02\
    \x01\n\x0b\n\x03\x04&\x01\x12\x04\xaf\x02\x08\x1c\n\x0c\n\x04\x04&\x02\0\
    \x12\x04\xb0\x02\x08\x1c\n\x0f\n\x05\x04&\x02\0\x04\x12\x06\xb0\x02\x08\
    \xaf\x02\x1e\n\r\n\x05\x04&\x02\0\x05\x12\x04\xb0\x02\x08\x0e\n\r\n\x05\
    \x04&\x02\0\x01\x12\x04\xb0\x02\x0f\x17\n\r\n\x05\x04&\x02\0\x03\x12\x04\
    \xb0\x02\x1a\x1b\n\x0c\n\x02\x04'\x12\x06\xb2\x02\0\xb4\x02\x01\n\x0b\n\
    \x03\x04'\x01\x12\x04\xb2\x02\x08\x1d\n\x0c\n\x04\x04'\x02\0\x12\x04\xb3\
    \x02\x08\"\n\r\n\x05\x04'\x02\0\x04\x12\x04\xb3\x02\x08\x10\n\r\n\x05\
    \x04'\x02\0\x06\x12\x04\xb3\x02\x11\x16\n\r\n\x05\x04'\x02\0\x01\x12\x04\
    \xb3\x02\x17\x1d\n\r\n\x05\x04'\x02\0\x03\x12\x04\xb3\x02\x20!\n\x0c\n\
    \x02\x04(\x12\x06\xb6\x02\0\xb8\x02\x01\n\x0b\n\x03\x04(\x01\x12\x04\xb6\
    \x02\x08\x1e\n\x0c\n\x04\x04(\x02\0\x12\x04\xb7\x02\x08\x1c\n\x0f\n\x05\
    \x04(\x02\0\x04\x12\x06\xb7\x02\x08\xb6\x02\x20\n\r\n\x05\x04(\x02\0\x05\
    \x12\x04\xb7\x02\x08\r\n\r\n\x05\x04(\x02\0\x01\x12\x04\xb7\x02\x0e\x17\
    \n\r\n\x05\x04(\x02\0\x03\x12\x04\xb7\x02\x1a\x1b\n\x0c\n\x02\x04)\x12\
    \x06\xb9\x02\0\xbb\x02\x01\n\x0b\n\x03\x04)\x01\x12\x04\xb9\x02\x08\x1f\
    \n\x0c\n\x04\x04)\x02\0\x12\x04\xba\x02\x08\"\n\r\n\x05\x04)\x02\0\x04\
    \x12\x04\xba\x02\x08\x10\n\r\n\x05\x04)\x02\0\x06\x12\x04\xba\x02\x11\
    \x16\n\r\n\x05\x04)\x02\0\x01\x12\x04\xba\x02\x17\x1d\n\r\n\x05\x04)\x02\
    \0\x03\x12\x04\xba\x02\x20!\n\x0c\n\x02\x04*\x12\x06\xbd\x02\0\xbf\x02\
    \x01\n\x0b\n\x03\x04*\x01\x12\x04\xbd\x02\x08\x17\n\x0c\n\x04\x04*\x02\0\
    \x12\x04\xbe\x02\x08\"\n\r\n\x05\x04*\x02\0\x04\x12\x04\xbe\x02\x08\x10\
    \n\r\n\x05\x04*\x02\0\x06\x12\x04\xbe\x02\x11\x16\n\r\n\x05\x04*\x02\0\
    \x01\x12\x04\xbe\x02\x17\x1d\n\r\n\x05\x04*\x02\0\x03\x12\x04\xbe\x02\
    \x20!\n\x0c\n\x02\x04+\x12\x06\xc0\x02\0\xc2\x02\x01\n\x0b\n\x03\x04+\
    \x01\x12\x04\xc0\x02\x08\x18\n\x0c\n\x04\x04+\x02\0\x12\x04\xc1\x02\x08\
    \x17\n\x0f\n\x05\x04+\x02\0\x04\x12\x06\xc1\x02\x08\xc0\x02\x1a\n\r\n\
    \x05\x04+\x02\0\x05\x12\x04\xc1\x02\x08\r\n\r\n\x05\x04+\x02\0\x01\x12\
    \x04\xc1\x02\x0e\x12\n\r\n\x05\x04+\x02\0\x03\x12\x04\xc1\x02\x15\x16\n\
    \x0c\n\x02\x04,\x12\x06\xc4\x02\0\xca\x02\x01\n\x0b\n\x03\x04,\x01\x12\
    \x04\xc4\x02\x08\x1b\n\x0c\n\x04\x04,\x02\0\x12\x04\xc5\x02\x08\"\n\r\n\
    \x05\x04,\x02\0\x04\x12\x04\xc5\x02\x08\x10\n\r\n\x05\x04,\x02\0\x06\x12\
    \x04\xc5\x02\x11\x16\n\r\n\x05\x04,\x02\0\x01\x12\x04\xc5\x02\x17\x1d\n\
    \r\n\x05\x04,\x02\0\x03\x12\x04\xc5\x02\x20!\n6\n\x04\x04,\x02\x01\x12\
    \x04\xc7\x02\x08\x1c\x1a(\x20destination\x20file\x20and\x20its\x20config\
    uration\n\n\x0f\n\x05\x04,\x02\x01\x04\x12\x06\xc7\x02\x08\xc5\x02\"\n\r\
    \n\x05\x04,\x02\x01\x05\x12\x04\xc7\x02\x08\x0e\n\r\n\x05\x04,\x02\x01\
    \x01\x12\x04\xc7\x02\x0f\x17\n\r\n\x05\x04,\x02\x01\x03\x12\x04\xc7\x02\
    \x1a\x1b\n\x0c\n\x04\x04,\x02\x02\x12\x04\xc8\x02\x08\x1e\n\x0f\n\x05\
    \x04,\x02\x02\x04\x12\x06\xc8\x02\x08\xc7\x02\x1c\n\r\n\x05\x04,\x02\x02\
    \x06\x12\x04\xc8\x02\x08\x10\n\r\n\x05\x04,\x02\x02\x01\x12\x04\xc8\x02\
    \x11\x19\n\r\n\x05\x04,\x02\x02\x03\x12\x04\xc8\x02\x1c\x1d\n-\n\x04\x04\
    ,\x02\x03\x12\x04\xc9\x02\x08\x1f\"\x1f\x20open\x20file\x20for\x20synchr\
    onous\x20I/O\n\n\x0f\n\x05\x04,\x02\x03\x04\x12\x06\xc9\x02\x08\xc8\x02\
    \x1e\n\r\n\x05\x04,\x02\x03\x05\x12\x04\xc9\x02\x08\x0c\n\r\n\x05\x04,\
    \x02\x03\x01\x12\x04\xc9\x02\r\x1a\n\r\n\x05\x04,\x02\x03\x03\x12\x04\
    \xc9\x02\x1d\x1e\n\x0c\n\x02\x04-\x12\x06\xcb\x02\0\xcc\x02\x01\n\x0b\n\
    \x03\x04-\x01\x12\x04\xcb\x02\x08\x1c\n\x0c\n\x02\x04.\x12\x06\xce\x02\0\
    \xd1\x02\x01\n\x0b\n\x03\x04.\x01\x12\x04\xce\x02\x08\x1d\n\x0c\n\x04\
    \x04.\x02\0\x12\x04\xcf\x02\x08\"\n\r\n\x05\x04.\x02\0\x04\x12\x04\xcf\
    \x02\x08\x10\n\r\n\x05\x04.\x02\0\x06\x12\x04\xcf\x02\x11\x16\n\r\n\x05\
    \x04.\x02\0\x01\x12\x04\xcf\x02\x17\x1d\n\r\n\x05\x04.\x02\0\x03\x12\x04\
    \xcf\x02\x20!\n\x0c\n\x04\x04.\x02\x01\x12\x04\xd0\x02\x08\x1c\n\x0f\n\
    \x05\x04.\x02\x01\x04\x12\x06\xd0\x02\x08\xcf\x02\"\n\r\n\x05\x04.\x02\
    \x01\x05\x12\x04\xd0\x02\x08\r\n\r\n\x05\x04.\x02\x01\x01\x12\x04\xd0\
    \x02\x0e\x17\n\r\n\x05\x04.\x02\x01\x03\x12\x04\xd0\x02\x1a\x1b\n\x0c\n\
    \x02\x04/\x12\x06\xd2\x02\0\xd4\x02\x01\n\x0b\n\x03\x04/\x01\x12\x04\xd2\
    \x02\x08\x1e\n\x0c\n\x04\x04/\x02\0\x12\x04\xd3\x02\x08\x1c\n\x0f\n\x05\
    \x04/\x02\0\x04\x12\x06\xd3\x02\x08\xd2\x02\x20\n\r\n\x05\x04/\x02\0\x05\
    \x12\x04\xd3\x02\x08\r\n\r\n\x05\x04/\x02\0\x01\x12\x04\xd3\x02\x0e\x17\
    \n\r\n\x05\x04/\x02\0\x03\x12\x04\xd3\x02\x1a\x1b\n\x0c\n\x02\x040\x12\
    \x06\xd6\x02\0\xd8\x02\x01\n\x0b\n\x03\x040\x01\x12\x04\xd6\x02\x08\x19\
    \n\x0c\n\x04\x040\x02\0\x12\x04\xd7\x02\x08\"\n\r\n\x05\x040\x02\0\x04\
    \x12\x04\xd7\x02\x08\x10\n\r\n\x05\x040\x02\0\x06\x12\x04\xd7\x02\x11\
    \x16\n\r\n\x05\x040\x02\0\x01\x12\x04\xd7\x02\x17\x1d\n\r\n\x05\x040\x02\
    \0\x03\x12\x04\xd7\x02\x20!\n\x0c\n\x02\x041\x12\x06\xd9\x02\0\xda\x02\
    \x01\n\x0b\n\x03\x041\x01\x12\x04\xd9\x02\x08\x1a\n\x0c\n\x02\x042\x12\
    \x06\xdc\x02\0\xdf\x02\x01\n\x0b\n\x03\x042\x01\x12\x04\xdc\x02\x08\x18\
    \n\x0c\n\x04\x042\x02\0\x12\x04\xdd\x02\x08\"\n\r\n\x05\x042\x02\0\x04\
    \x12\x04\xdd\x02\x08\x10\n\r\n\x05\x042\x02\0\x06\x12\x04\xdd\x02\x11\
    \x16\n\r\n\x05\x042\x02\0\x01\x12\x04\xdd\x02\x17\x1d\n\r\n\x05\x042\x02\
    \0\x03\x12\x04\xdd\x02\x20!\n\x0c\n\x04\x042\x02\x01\x12\x04\xde\x02\x08\
    \x16\n\x0f\n\x05\x042\x02\x01\x04\x12\x06\xde\x02\x08\xdd\x02\"\n\r\n\
    \x05\x042\x02\x01\x05\x12\x04\xde\x02\x08\x0c\n\r\n\x05\x042\x02\x01\x01\
    \x12\x04\xde\x02\r\x11\n\r\n\x05\x042\x02\x01\x03\x12\x04\xde\x02\x14\
    \x15\n\x0c\n\x02\x043\x12\x06\xe0\x02\0\xe2\x02\x01\n\x0b\n\x03\x043\x01\
    \x12\x04\xe0\x02\x08\x19\n\x0c\n\x04\x043\x02\0\x12\x04\xe1\x02\x08\x1f\
    \n\x0f\n\x05\x043\x02\0\x04\x12\x06\xe1\x02\x08\xe0\x02\x1b\n\r\n\x05\
    \x043\x02\0\x06\x12\x04\xe1\x02\x08\x13\n\r\n\x05\x043\x02\0\x01\x12\x04\
    \xe1\x02\x14\x1a\n\r\n\x05\x043\x02\0\x03\x12\x04\xe1\x02\x1d\x1e\n\x0c\
    \n\x02\x044\x12\x06\xe4\x02\0\xe6\x02\x01\n\x0b\n\x03\x044\x01\x12\x04\
    \xe4\x02\x08\x19\n\x0c\n\x04\x044\x02\0\x12\x04\xe5\x02\x08\"\n\r\n\x05\
    \x044\x02\0\x04\x12\x04\xe5\x02\x08\x10\n\r\n\x05\x044\x02\0\x06\x12\x04\
    \xe5\x02\x11\x16\n\r\n\x05\x044\x02\0\x01\x12\x04\xe5\x02\x17\x1d\n\r\n\
    \x05\x044\x02\0\x03\x12\x04\xe5\x02\x20!\n\x0c\n\x02\x045\x12\x06\xe7\
    \x02\0\xe9\x02\x01\n\x0b\n\x03\x045\x01\x12\x04\xe7\x02\x08\x1a\n\x0c\n\
    \x04\x045\x02\0\x12\x04\xe8\x02\x08\"\n\r\n\x05\x045\x02\0\x04\x12\x04\
    \xe8\x02\x08\x10\n\r\n\x05\x045\x02\0\x06\x12\x04\xe8\x02\x11\x16\n\r\n\
    \x05\x045\x02\0\x01\x12\x04\xe8\x02\x17\x1d\n\r\n\x05\x045\x02\0\x03\x12\
    \x04\xe8\x02\x20!b\x06proto3\
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
