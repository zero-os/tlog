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

const METHOD_NAMESPACE_SERVICE_CREATE_NAMESPACE: ::grpcio::Method<super::daemon::CreateNamespaceRequest, super::daemon::CreateNamespaceResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/schema.NamespaceService/CreateNamespace",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_NAMESPACE_SERVICE_DELETE_NAMESPACE: ::grpcio::Method<super::daemon::DeleteNamespaceRequest, super::daemon::DeleteNamespaceResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/schema.NamespaceService/DeleteNamespace",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_NAMESPACE_SERVICE_SET_PERMISSION: ::grpcio::Method<super::daemon::SetPermissionRequest, super::daemon::SetPermissionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/schema.NamespaceService/SetPermission",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_NAMESPACE_SERVICE_GET_PERMISSION: ::grpcio::Method<super::daemon::GetPermissionRequest, super::daemon::GetPermissionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/schema.NamespaceService/GetPermission",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct NamespaceServiceClient {
    client: ::grpcio::Client,
}

impl NamespaceServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        NamespaceServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn create_namespace_opt(&self, req: &super::daemon::CreateNamespaceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::daemon::CreateNamespaceResponse> {
        self.client.unary_call(&METHOD_NAMESPACE_SERVICE_CREATE_NAMESPACE, req, opt)
    }

    pub fn create_namespace(&self, req: &super::daemon::CreateNamespaceRequest) -> ::grpcio::Result<super::daemon::CreateNamespaceResponse> {
        self.create_namespace_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_namespace_async_opt(&self, req: &super::daemon::CreateNamespaceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::CreateNamespaceResponse>> {
        self.client.unary_call_async(&METHOD_NAMESPACE_SERVICE_CREATE_NAMESPACE, req, opt)
    }

    pub fn create_namespace_async(&self, req: &super::daemon::CreateNamespaceRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::CreateNamespaceResponse>> {
        self.create_namespace_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_namespace_opt(&self, req: &super::daemon::DeleteNamespaceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::daemon::DeleteNamespaceResponse> {
        self.client.unary_call(&METHOD_NAMESPACE_SERVICE_DELETE_NAMESPACE, req, opt)
    }

    pub fn delete_namespace(&self, req: &super::daemon::DeleteNamespaceRequest) -> ::grpcio::Result<super::daemon::DeleteNamespaceResponse> {
        self.delete_namespace_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_namespace_async_opt(&self, req: &super::daemon::DeleteNamespaceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::DeleteNamespaceResponse>> {
        self.client.unary_call_async(&METHOD_NAMESPACE_SERVICE_DELETE_NAMESPACE, req, opt)
    }

    pub fn delete_namespace_async(&self, req: &super::daemon::DeleteNamespaceRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::DeleteNamespaceResponse>> {
        self.delete_namespace_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_permission_opt(&self, req: &super::daemon::SetPermissionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::daemon::SetPermissionResponse> {
        self.client.unary_call(&METHOD_NAMESPACE_SERVICE_SET_PERMISSION, req, opt)
    }

    pub fn set_permission(&self, req: &super::daemon::SetPermissionRequest) -> ::grpcio::Result<super::daemon::SetPermissionResponse> {
        self.set_permission_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_permission_async_opt(&self, req: &super::daemon::SetPermissionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::SetPermissionResponse>> {
        self.client.unary_call_async(&METHOD_NAMESPACE_SERVICE_SET_PERMISSION, req, opt)
    }

    pub fn set_permission_async(&self, req: &super::daemon::SetPermissionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::SetPermissionResponse>> {
        self.set_permission_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_permission_opt(&self, req: &super::daemon::GetPermissionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::daemon::GetPermissionResponse> {
        self.client.unary_call(&METHOD_NAMESPACE_SERVICE_GET_PERMISSION, req, opt)
    }

    pub fn get_permission(&self, req: &super::daemon::GetPermissionRequest) -> ::grpcio::Result<super::daemon::GetPermissionResponse> {
        self.get_permission_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_permission_async_opt(&self, req: &super::daemon::GetPermissionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::GetPermissionResponse>> {
        self.client.unary_call_async(&METHOD_NAMESPACE_SERVICE_GET_PERMISSION, req, opt)
    }

    pub fn get_permission_async(&self, req: &super::daemon::GetPermissionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::GetPermissionResponse>> {
        self.get_permission_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait NamespaceService {
    fn create_namespace(&self, ctx: ::grpcio::RpcContext, req: super::daemon::CreateNamespaceRequest, sink: ::grpcio::UnarySink<super::daemon::CreateNamespaceResponse>);
    fn delete_namespace(&self, ctx: ::grpcio::RpcContext, req: super::daemon::DeleteNamespaceRequest, sink: ::grpcio::UnarySink<super::daemon::DeleteNamespaceResponse>);
    fn set_permission(&self, ctx: ::grpcio::RpcContext, req: super::daemon::SetPermissionRequest, sink: ::grpcio::UnarySink<super::daemon::SetPermissionResponse>);
    fn get_permission(&self, ctx: ::grpcio::RpcContext, req: super::daemon::GetPermissionRequest, sink: ::grpcio::UnarySink<super::daemon::GetPermissionResponse>);
}

pub fn create_namespace_service<S: NamespaceService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_NAMESPACE_SERVICE_CREATE_NAMESPACE, move |ctx, req, resp| {
        instance.create_namespace(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_NAMESPACE_SERVICE_DELETE_NAMESPACE, move |ctx, req, resp| {
        instance.delete_namespace(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_NAMESPACE_SERVICE_SET_PERMISSION, move |ctx, req, resp| {
        instance.set_permission(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_NAMESPACE_SERVICE_GET_PERMISSION, move |ctx, req, resp| {
        instance.get_permission(ctx, req, resp)
    });
    builder.build()
}

const METHOD_FILE_SERVICE_WRITE: ::grpcio::Method<super::daemon::WriteRequest, super::daemon::WriteResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/schema.FileService/Write",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_FILE_SERVICE_WRITE_FILE: ::grpcio::Method<super::daemon::WriteFileRequest, super::daemon::WriteFileResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/schema.FileService/WriteFile",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_FILE_SERVICE_WRITE_STREAM: ::grpcio::Method<super::daemon::WriteStreamRequest, super::daemon::WriteStreamResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/schema.FileService/WriteStream",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_FILE_SERVICE_READ: ::grpcio::Method<super::daemon::ReadRequest, super::daemon::ReadResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/schema.FileService/Read",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_FILE_SERVICE_READ_FILE: ::grpcio::Method<super::daemon::ReadFileRequest, super::daemon::ReadFileResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/schema.FileService/ReadFile",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_FILE_SERVICE_READ_STREAM: ::grpcio::Method<super::daemon::ReadStreamRequest, super::daemon::ReadStreamResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/schema.FileService/ReadStream",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_FILE_SERVICE_DELETE: ::grpcio::Method<super::daemon::DeleteRequest, super::daemon::DeleteResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/schema.FileService/Delete",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_FILE_SERVICE_CHECK: ::grpcio::Method<super::daemon::CheckRequest, super::daemon::CheckResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/schema.FileService/Check",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_FILE_SERVICE_REPAIR: ::grpcio::Method<super::daemon::RepairRequest, super::daemon::RepairResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/schema.FileService/Repair",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct FileServiceClient {
    client: ::grpcio::Client,
}

impl FileServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        FileServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn write_opt(&self, req: &super::daemon::WriteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::daemon::WriteResponse> {
        self.client.unary_call(&METHOD_FILE_SERVICE_WRITE, req, opt)
    }

    pub fn write(&self, req: &super::daemon::WriteRequest) -> ::grpcio::Result<super::daemon::WriteResponse> {
        self.write_opt(req, ::grpcio::CallOption::default())
    }

    pub fn write_async_opt(&self, req: &super::daemon::WriteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::WriteResponse>> {
        self.client.unary_call_async(&METHOD_FILE_SERVICE_WRITE, req, opt)
    }

    pub fn write_async(&self, req: &super::daemon::WriteRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::WriteResponse>> {
        self.write_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn write_file_opt(&self, req: &super::daemon::WriteFileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::daemon::WriteFileResponse> {
        self.client.unary_call(&METHOD_FILE_SERVICE_WRITE_FILE, req, opt)
    }

    pub fn write_file(&self, req: &super::daemon::WriteFileRequest) -> ::grpcio::Result<super::daemon::WriteFileResponse> {
        self.write_file_opt(req, ::grpcio::CallOption::default())
    }

    pub fn write_file_async_opt(&self, req: &super::daemon::WriteFileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::WriteFileResponse>> {
        self.client.unary_call_async(&METHOD_FILE_SERVICE_WRITE_FILE, req, opt)
    }

    pub fn write_file_async(&self, req: &super::daemon::WriteFileRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::WriteFileResponse>> {
        self.write_file_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn write_stream_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::daemon::WriteStreamRequest>, ::grpcio::ClientCStreamReceiver<super::daemon::WriteStreamResponse>)> {
        self.client.client_streaming(&METHOD_FILE_SERVICE_WRITE_STREAM, opt)
    }

    pub fn write_stream(&self) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::daemon::WriteStreamRequest>, ::grpcio::ClientCStreamReceiver<super::daemon::WriteStreamResponse>)> {
        self.write_stream_opt(::grpcio::CallOption::default())
    }

    pub fn read_opt(&self, req: &super::daemon::ReadRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::daemon::ReadResponse> {
        self.client.unary_call(&METHOD_FILE_SERVICE_READ, req, opt)
    }

    pub fn read(&self, req: &super::daemon::ReadRequest) -> ::grpcio::Result<super::daemon::ReadResponse> {
        self.read_opt(req, ::grpcio::CallOption::default())
    }

    pub fn read_async_opt(&self, req: &super::daemon::ReadRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::ReadResponse>> {
        self.client.unary_call_async(&METHOD_FILE_SERVICE_READ, req, opt)
    }

    pub fn read_async(&self, req: &super::daemon::ReadRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::ReadResponse>> {
        self.read_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn read_file_opt(&self, req: &super::daemon::ReadFileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::daemon::ReadFileResponse> {
        self.client.unary_call(&METHOD_FILE_SERVICE_READ_FILE, req, opt)
    }

    pub fn read_file(&self, req: &super::daemon::ReadFileRequest) -> ::grpcio::Result<super::daemon::ReadFileResponse> {
        self.read_file_opt(req, ::grpcio::CallOption::default())
    }

    pub fn read_file_async_opt(&self, req: &super::daemon::ReadFileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::ReadFileResponse>> {
        self.client.unary_call_async(&METHOD_FILE_SERVICE_READ_FILE, req, opt)
    }

    pub fn read_file_async(&self, req: &super::daemon::ReadFileRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::ReadFileResponse>> {
        self.read_file_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn read_stream_opt(&self, req: &super::daemon::ReadStreamRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::daemon::ReadStreamResponse>> {
        self.client.server_streaming(&METHOD_FILE_SERVICE_READ_STREAM, req, opt)
    }

    pub fn read_stream(&self, req: &super::daemon::ReadStreamRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::daemon::ReadStreamResponse>> {
        self.read_stream_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_opt(&self, req: &super::daemon::DeleteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::daemon::DeleteResponse> {
        self.client.unary_call(&METHOD_FILE_SERVICE_DELETE, req, opt)
    }

    pub fn delete(&self, req: &super::daemon::DeleteRequest) -> ::grpcio::Result<super::daemon::DeleteResponse> {
        self.delete_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_async_opt(&self, req: &super::daemon::DeleteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::DeleteResponse>> {
        self.client.unary_call_async(&METHOD_FILE_SERVICE_DELETE, req, opt)
    }

    pub fn delete_async(&self, req: &super::daemon::DeleteRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::DeleteResponse>> {
        self.delete_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn check_opt(&self, req: &super::daemon::CheckRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::daemon::CheckResponse> {
        self.client.unary_call(&METHOD_FILE_SERVICE_CHECK, req, opt)
    }

    pub fn check(&self, req: &super::daemon::CheckRequest) -> ::grpcio::Result<super::daemon::CheckResponse> {
        self.check_opt(req, ::grpcio::CallOption::default())
    }

    pub fn check_async_opt(&self, req: &super::daemon::CheckRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::CheckResponse>> {
        self.client.unary_call_async(&METHOD_FILE_SERVICE_CHECK, req, opt)
    }

    pub fn check_async(&self, req: &super::daemon::CheckRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::CheckResponse>> {
        self.check_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn repair_opt(&self, req: &super::daemon::RepairRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::daemon::RepairResponse> {
        self.client.unary_call(&METHOD_FILE_SERVICE_REPAIR, req, opt)
    }

    pub fn repair(&self, req: &super::daemon::RepairRequest) -> ::grpcio::Result<super::daemon::RepairResponse> {
        self.repair_opt(req, ::grpcio::CallOption::default())
    }

    pub fn repair_async_opt(&self, req: &super::daemon::RepairRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::RepairResponse>> {
        self.client.unary_call_async(&METHOD_FILE_SERVICE_REPAIR, req, opt)
    }

    pub fn repair_async(&self, req: &super::daemon::RepairRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::RepairResponse>> {
        self.repair_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait FileService {
    fn write(&self, ctx: ::grpcio::RpcContext, req: super::daemon::WriteRequest, sink: ::grpcio::UnarySink<super::daemon::WriteResponse>);
    fn write_file(&self, ctx: ::grpcio::RpcContext, req: super::daemon::WriteFileRequest, sink: ::grpcio::UnarySink<super::daemon::WriteFileResponse>);
    fn write_stream(&self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::daemon::WriteStreamRequest>, sink: ::grpcio::ClientStreamingSink<super::daemon::WriteStreamResponse>);
    fn read(&self, ctx: ::grpcio::RpcContext, req: super::daemon::ReadRequest, sink: ::grpcio::UnarySink<super::daemon::ReadResponse>);
    fn read_file(&self, ctx: ::grpcio::RpcContext, req: super::daemon::ReadFileRequest, sink: ::grpcio::UnarySink<super::daemon::ReadFileResponse>);
    fn read_stream(&self, ctx: ::grpcio::RpcContext, req: super::daemon::ReadStreamRequest, sink: ::grpcio::ServerStreamingSink<super::daemon::ReadStreamResponse>);
    fn delete(&self, ctx: ::grpcio::RpcContext, req: super::daemon::DeleteRequest, sink: ::grpcio::UnarySink<super::daemon::DeleteResponse>);
    fn check(&self, ctx: ::grpcio::RpcContext, req: super::daemon::CheckRequest, sink: ::grpcio::UnarySink<super::daemon::CheckResponse>);
    fn repair(&self, ctx: ::grpcio::RpcContext, req: super::daemon::RepairRequest, sink: ::grpcio::UnarySink<super::daemon::RepairResponse>);
}

pub fn create_file_service<S: FileService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FILE_SERVICE_WRITE, move |ctx, req, resp| {
        instance.write(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FILE_SERVICE_WRITE_FILE, move |ctx, req, resp| {
        instance.write_file(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_FILE_SERVICE_WRITE_STREAM, move |ctx, req, resp| {
        instance.write_stream(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FILE_SERVICE_READ, move |ctx, req, resp| {
        instance.read(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FILE_SERVICE_READ_FILE, move |ctx, req, resp| {
        instance.read_file(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_FILE_SERVICE_READ_STREAM, move |ctx, req, resp| {
        instance.read_stream(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FILE_SERVICE_DELETE, move |ctx, req, resp| {
        instance.delete(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FILE_SERVICE_CHECK, move |ctx, req, resp| {
        instance.check(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FILE_SERVICE_REPAIR, move |ctx, req, resp| {
        instance.repair(ctx, req, resp)
    });
    builder.build()
}

const METHOD_METADATA_SERVICE_SET_METADATA: ::grpcio::Method<super::daemon::SetMetadataRequest, super::daemon::SetMetadataResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/schema.MetadataService/SetMetadata",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_METADATA_SERVICE_GET_METADATA: ::grpcio::Method<super::daemon::GetMetadataRequest, super::daemon::GetMetadataResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/schema.MetadataService/GetMetadata",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_METADATA_SERVICE_DELETE_METADATA: ::grpcio::Method<super::daemon::DeleteMetadataRequest, super::daemon::DeleteMetadataResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/schema.MetadataService/DeleteMetadata",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct MetadataServiceClient {
    client: ::grpcio::Client,
}

impl MetadataServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        MetadataServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn set_metadata_opt(&self, req: &super::daemon::SetMetadataRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::daemon::SetMetadataResponse> {
        self.client.unary_call(&METHOD_METADATA_SERVICE_SET_METADATA, req, opt)
    }

    pub fn set_metadata(&self, req: &super::daemon::SetMetadataRequest) -> ::grpcio::Result<super::daemon::SetMetadataResponse> {
        self.set_metadata_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_metadata_async_opt(&self, req: &super::daemon::SetMetadataRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::SetMetadataResponse>> {
        self.client.unary_call_async(&METHOD_METADATA_SERVICE_SET_METADATA, req, opt)
    }

    pub fn set_metadata_async(&self, req: &super::daemon::SetMetadataRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::SetMetadataResponse>> {
        self.set_metadata_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_metadata_opt(&self, req: &super::daemon::GetMetadataRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::daemon::GetMetadataResponse> {
        self.client.unary_call(&METHOD_METADATA_SERVICE_GET_METADATA, req, opt)
    }

    pub fn get_metadata(&self, req: &super::daemon::GetMetadataRequest) -> ::grpcio::Result<super::daemon::GetMetadataResponse> {
        self.get_metadata_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_metadata_async_opt(&self, req: &super::daemon::GetMetadataRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::GetMetadataResponse>> {
        self.client.unary_call_async(&METHOD_METADATA_SERVICE_GET_METADATA, req, opt)
    }

    pub fn get_metadata_async(&self, req: &super::daemon::GetMetadataRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::GetMetadataResponse>> {
        self.get_metadata_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_metadata_opt(&self, req: &super::daemon::DeleteMetadataRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::daemon::DeleteMetadataResponse> {
        self.client.unary_call(&METHOD_METADATA_SERVICE_DELETE_METADATA, req, opt)
    }

    pub fn delete_metadata(&self, req: &super::daemon::DeleteMetadataRequest) -> ::grpcio::Result<super::daemon::DeleteMetadataResponse> {
        self.delete_metadata_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_metadata_async_opt(&self, req: &super::daemon::DeleteMetadataRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::DeleteMetadataResponse>> {
        self.client.unary_call_async(&METHOD_METADATA_SERVICE_DELETE_METADATA, req, opt)
    }

    pub fn delete_metadata_async(&self, req: &super::daemon::DeleteMetadataRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::DeleteMetadataResponse>> {
        self.delete_metadata_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait MetadataService {
    fn set_metadata(&self, ctx: ::grpcio::RpcContext, req: super::daemon::SetMetadataRequest, sink: ::grpcio::UnarySink<super::daemon::SetMetadataResponse>);
    fn get_metadata(&self, ctx: ::grpcio::RpcContext, req: super::daemon::GetMetadataRequest, sink: ::grpcio::UnarySink<super::daemon::GetMetadataResponse>);
    fn delete_metadata(&self, ctx: ::grpcio::RpcContext, req: super::daemon::DeleteMetadataRequest, sink: ::grpcio::UnarySink<super::daemon::DeleteMetadataResponse>);
}

pub fn create_metadata_service<S: MetadataService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_METADATA_SERVICE_SET_METADATA, move |ctx, req, resp| {
        instance.set_metadata(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_METADATA_SERVICE_GET_METADATA, move |ctx, req, resp| {
        instance.get_metadata(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_METADATA_SERVICE_DELETE_METADATA, move |ctx, req, resp| {
        instance.delete_metadata(ctx, req, resp)
    });
    builder.build()
}

const METHOD_DATA_SERVICE_WRITE: ::grpcio::Method<super::daemon::DataWriteRequest, super::daemon::DataWriteResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/schema.DataService/Write",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DATA_SERVICE_WRITE_FILE: ::grpcio::Method<super::daemon::DataWriteFileRequest, super::daemon::DataWriteFileResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/schema.DataService/WriteFile",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DATA_SERVICE_WRITE_STREAM: ::grpcio::Method<super::daemon::DataWriteStreamRequest, super::daemon::DataWriteStreamResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/schema.DataService/WriteStream",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DATA_SERVICE_READ: ::grpcio::Method<super::daemon::DataReadRequest, super::daemon::DataReadResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/schema.DataService/Read",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DATA_SERVICE_READ_FILE: ::grpcio::Method<super::daemon::DataReadFileRequest, super::daemon::DataReadFileResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/schema.DataService/ReadFile",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DATA_SERVICE_READ_STREAM: ::grpcio::Method<super::daemon::DataReadStreamRequest, super::daemon::DataReadStreamResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/schema.DataService/ReadStream",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DATA_SERVICE_DELETE: ::grpcio::Method<super::daemon::DataDeleteRequest, super::daemon::DataDeleteResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/schema.DataService/Delete",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DATA_SERVICE_CHECK: ::grpcio::Method<super::daemon::DataCheckRequest, super::daemon::DataCheckResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/schema.DataService/Check",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DATA_SERVICE_REPAIR: ::grpcio::Method<super::daemon::DataRepairRequest, super::daemon::DataRepairResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/schema.DataService/Repair",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct DataServiceClient {
    client: ::grpcio::Client,
}

impl DataServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        DataServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn write_opt(&self, req: &super::daemon::DataWriteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::daemon::DataWriteResponse> {
        self.client.unary_call(&METHOD_DATA_SERVICE_WRITE, req, opt)
    }

    pub fn write(&self, req: &super::daemon::DataWriteRequest) -> ::grpcio::Result<super::daemon::DataWriteResponse> {
        self.write_opt(req, ::grpcio::CallOption::default())
    }

    pub fn write_async_opt(&self, req: &super::daemon::DataWriteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::DataWriteResponse>> {
        self.client.unary_call_async(&METHOD_DATA_SERVICE_WRITE, req, opt)
    }

    pub fn write_async(&self, req: &super::daemon::DataWriteRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::DataWriteResponse>> {
        self.write_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn write_file_opt(&self, req: &super::daemon::DataWriteFileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::daemon::DataWriteFileResponse> {
        self.client.unary_call(&METHOD_DATA_SERVICE_WRITE_FILE, req, opt)
    }

    pub fn write_file(&self, req: &super::daemon::DataWriteFileRequest) -> ::grpcio::Result<super::daemon::DataWriteFileResponse> {
        self.write_file_opt(req, ::grpcio::CallOption::default())
    }

    pub fn write_file_async_opt(&self, req: &super::daemon::DataWriteFileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::DataWriteFileResponse>> {
        self.client.unary_call_async(&METHOD_DATA_SERVICE_WRITE_FILE, req, opt)
    }

    pub fn write_file_async(&self, req: &super::daemon::DataWriteFileRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::DataWriteFileResponse>> {
        self.write_file_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn write_stream_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::daemon::DataWriteStreamRequest>, ::grpcio::ClientCStreamReceiver<super::daemon::DataWriteStreamResponse>)> {
        self.client.client_streaming(&METHOD_DATA_SERVICE_WRITE_STREAM, opt)
    }

    pub fn write_stream(&self) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::daemon::DataWriteStreamRequest>, ::grpcio::ClientCStreamReceiver<super::daemon::DataWriteStreamResponse>)> {
        self.write_stream_opt(::grpcio::CallOption::default())
    }

    pub fn read_opt(&self, req: &super::daemon::DataReadRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::daemon::DataReadResponse> {
        self.client.unary_call(&METHOD_DATA_SERVICE_READ, req, opt)
    }

    pub fn read(&self, req: &super::daemon::DataReadRequest) -> ::grpcio::Result<super::daemon::DataReadResponse> {
        self.read_opt(req, ::grpcio::CallOption::default())
    }

    pub fn read_async_opt(&self, req: &super::daemon::DataReadRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::DataReadResponse>> {
        self.client.unary_call_async(&METHOD_DATA_SERVICE_READ, req, opt)
    }

    pub fn read_async(&self, req: &super::daemon::DataReadRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::DataReadResponse>> {
        self.read_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn read_file_opt(&self, req: &super::daemon::DataReadFileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::daemon::DataReadFileResponse> {
        self.client.unary_call(&METHOD_DATA_SERVICE_READ_FILE, req, opt)
    }

    pub fn read_file(&self, req: &super::daemon::DataReadFileRequest) -> ::grpcio::Result<super::daemon::DataReadFileResponse> {
        self.read_file_opt(req, ::grpcio::CallOption::default())
    }

    pub fn read_file_async_opt(&self, req: &super::daemon::DataReadFileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::DataReadFileResponse>> {
        self.client.unary_call_async(&METHOD_DATA_SERVICE_READ_FILE, req, opt)
    }

    pub fn read_file_async(&self, req: &super::daemon::DataReadFileRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::DataReadFileResponse>> {
        self.read_file_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn read_stream_opt(&self, req: &super::daemon::DataReadStreamRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::daemon::DataReadStreamResponse>> {
        self.client.server_streaming(&METHOD_DATA_SERVICE_READ_STREAM, req, opt)
    }

    pub fn read_stream(&self, req: &super::daemon::DataReadStreamRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::daemon::DataReadStreamResponse>> {
        self.read_stream_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_opt(&self, req: &super::daemon::DataDeleteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::daemon::DataDeleteResponse> {
        self.client.unary_call(&METHOD_DATA_SERVICE_DELETE, req, opt)
    }

    pub fn delete(&self, req: &super::daemon::DataDeleteRequest) -> ::grpcio::Result<super::daemon::DataDeleteResponse> {
        self.delete_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_async_opt(&self, req: &super::daemon::DataDeleteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::DataDeleteResponse>> {
        self.client.unary_call_async(&METHOD_DATA_SERVICE_DELETE, req, opt)
    }

    pub fn delete_async(&self, req: &super::daemon::DataDeleteRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::DataDeleteResponse>> {
        self.delete_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn check_opt(&self, req: &super::daemon::DataCheckRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::daemon::DataCheckResponse> {
        self.client.unary_call(&METHOD_DATA_SERVICE_CHECK, req, opt)
    }

    pub fn check(&self, req: &super::daemon::DataCheckRequest) -> ::grpcio::Result<super::daemon::DataCheckResponse> {
        self.check_opt(req, ::grpcio::CallOption::default())
    }

    pub fn check_async_opt(&self, req: &super::daemon::DataCheckRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::DataCheckResponse>> {
        self.client.unary_call_async(&METHOD_DATA_SERVICE_CHECK, req, opt)
    }

    pub fn check_async(&self, req: &super::daemon::DataCheckRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::DataCheckResponse>> {
        self.check_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn repair_opt(&self, req: &super::daemon::DataRepairRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::daemon::DataRepairResponse> {
        self.client.unary_call(&METHOD_DATA_SERVICE_REPAIR, req, opt)
    }

    pub fn repair(&self, req: &super::daemon::DataRepairRequest) -> ::grpcio::Result<super::daemon::DataRepairResponse> {
        self.repair_opt(req, ::grpcio::CallOption::default())
    }

    pub fn repair_async_opt(&self, req: &super::daemon::DataRepairRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::DataRepairResponse>> {
        self.client.unary_call_async(&METHOD_DATA_SERVICE_REPAIR, req, opt)
    }

    pub fn repair_async(&self, req: &super::daemon::DataRepairRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::daemon::DataRepairResponse>> {
        self.repair_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait DataService {
    fn write(&self, ctx: ::grpcio::RpcContext, req: super::daemon::DataWriteRequest, sink: ::grpcio::UnarySink<super::daemon::DataWriteResponse>);
    fn write_file(&self, ctx: ::grpcio::RpcContext, req: super::daemon::DataWriteFileRequest, sink: ::grpcio::UnarySink<super::daemon::DataWriteFileResponse>);
    fn write_stream(&self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::daemon::DataWriteStreamRequest>, sink: ::grpcio::ClientStreamingSink<super::daemon::DataWriteStreamResponse>);
    fn read(&self, ctx: ::grpcio::RpcContext, req: super::daemon::DataReadRequest, sink: ::grpcio::UnarySink<super::daemon::DataReadResponse>);
    fn read_file(&self, ctx: ::grpcio::RpcContext, req: super::daemon::DataReadFileRequest, sink: ::grpcio::UnarySink<super::daemon::DataReadFileResponse>);
    fn read_stream(&self, ctx: ::grpcio::RpcContext, req: super::daemon::DataReadStreamRequest, sink: ::grpcio::ServerStreamingSink<super::daemon::DataReadStreamResponse>);
    fn delete(&self, ctx: ::grpcio::RpcContext, req: super::daemon::DataDeleteRequest, sink: ::grpcio::UnarySink<super::daemon::DataDeleteResponse>);
    fn check(&self, ctx: ::grpcio::RpcContext, req: super::daemon::DataCheckRequest, sink: ::grpcio::UnarySink<super::daemon::DataCheckResponse>);
    fn repair(&self, ctx: ::grpcio::RpcContext, req: super::daemon::DataRepairRequest, sink: ::grpcio::UnarySink<super::daemon::DataRepairResponse>);
}

pub fn create_data_service<S: DataService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DATA_SERVICE_WRITE, move |ctx, req, resp| {
        instance.write(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DATA_SERVICE_WRITE_FILE, move |ctx, req, resp| {
        instance.write_file(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_DATA_SERVICE_WRITE_STREAM, move |ctx, req, resp| {
        instance.write_stream(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DATA_SERVICE_READ, move |ctx, req, resp| {
        instance.read(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DATA_SERVICE_READ_FILE, move |ctx, req, resp| {
        instance.read_file(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_DATA_SERVICE_READ_STREAM, move |ctx, req, resp| {
        instance.read_stream(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DATA_SERVICE_DELETE, move |ctx, req, resp| {
        instance.delete(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DATA_SERVICE_CHECK, move |ctx, req, resp| {
        instance.check(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DATA_SERVICE_REPAIR, move |ctx, req, resp| {
        instance.repair(ctx, req, resp)
    });
    builder.build()
}
