// @generated
/// Generated client implementations.
pub mod control_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ControlServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ControlServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ControlServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ControlServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            ControlServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn get_host_status(
            &mut self,
            request: impl tonic::IntoRequest<super::GetHostStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetHostStatusReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/GetHostStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protect.control.v1.ControlService", "GetHostStatus"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn snoop_idm(
            &mut self,
            request: impl tonic::IntoRequest<super::SnoopIdmRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::SnoopIdmReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/SnoopIdm",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protect.control.v1.ControlService", "SnoopIdm"),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn snoop_control(
            &mut self,
            request: impl tonic::IntoRequest<super::SnoopControlRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::SnoopControlReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/SnoopControl",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protect.control.v1.ControlService", "SnoopControl"),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn get_host_cpu_topology(
            &mut self,
            request: impl tonic::IntoRequest<super::GetHostCpuTopologyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetHostCpuTopologyReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/GetHostCpuTopology",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protect.control.v1.ControlService",
                        "GetHostCpuTopology",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_host_resources(
            &mut self,
            request: impl tonic::IntoRequest<super::GetHostResourcesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetHostResourcesReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/GetHostResources",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protect.control.v1.ControlService",
                        "GetHostResources",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_devices(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDevicesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDevicesReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/ListDevices",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protect.control.v1.ControlService", "ListDevices"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_network_reservation(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateNetworkReservationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateNetworkReservationReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/CreateNetworkReservation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protect.control.v1.ControlService",
                        "CreateNetworkReservation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn destroy_network_reservation(
            &mut self,
            request: impl tonic::IntoRequest<super::DestroyNetworkReservationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DestroyNetworkReservationReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/DestroyNetworkReservation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protect.control.v1.ControlService",
                        "DestroyNetworkReservation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_network_reservations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNetworkReservationsRequest>,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::ListNetworkReservationsReply>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/ListNetworkReservations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protect.control.v1.ControlService",
                        "ListNetworkReservations",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn pull_image(
            &mut self,
            request: impl tonic::IntoRequest<super::PullImageRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::PullImageReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/PullImage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protect.control.v1.ControlService", "PullImage"),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn import_image(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::ImportImageRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::ImportImageReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/ImportImage",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protect.control.v1.ControlService", "ImportImage"),
                );
            self.inner.streaming(req, path, codec).await
        }
        pub async fn import_packed_image(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::ImportPackedImageRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ImportPackedImageReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/ImportPackedImage",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protect.control.v1.ControlService",
                        "ImportPackedImage",
                    ),
                );
            self.inner.client_streaming(req, path, codec).await
        }
        pub async fn remove_image(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveImageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveImageReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/RemoveImage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protect.control.v1.ControlService", "RemoveImage"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn prune_images(
            &mut self,
            request: impl tonic::IntoRequest<super::PruneImagesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PruneImagesReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/PruneImages",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protect.control.v1.ControlService", "PruneImages"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_images(
            &mut self,
            request: impl tonic::IntoRequest<super::ListImagesRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::ListImagesReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/ListImages",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protect.control.v1.ControlService", "ListImages"),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn list_kernel_variants(
            &mut self,
            request: impl tonic::IntoRequest<super::ListKernelVariantsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListKernelVariantsReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/ListKernelVariants",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protect.control.v1.ControlService",
                        "ListKernelVariants",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_zone(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateZoneRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateZoneReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/CreateZone",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protect.control.v1.ControlService", "CreateZone"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn destroy_zone(
            &mut self,
            request: impl tonic::IntoRequest<super::DestroyZoneRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DestroyZoneReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/DestroyZone",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protect.control.v1.ControlService", "DestroyZone"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn suspend_zone(
            &mut self,
            request: impl tonic::IntoRequest<super::SuspendZoneRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SuspendZoneReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/SuspendZone",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protect.control.v1.ControlService", "SuspendZone"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn resume_zone(
            &mut self,
            request: impl tonic::IntoRequest<super::ResumeZoneRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResumeZoneReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/ResumeZone",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protect.control.v1.ControlService", "ResumeZone"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn fork_zone(
            &mut self,
            request: impl tonic::IntoRequest<super::ForkZoneRequest>,
        ) -> std::result::Result<tonic::Response<super::ForkZoneReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/ForkZone",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protect.control.v1.ControlService", "ForkZone"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn export_zone(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportZoneRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::ExportZoneReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/ExportZone",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protect.control.v1.ControlService", "ExportZone"),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn import_zone(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::ImportZoneRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ImportZoneReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/ImportZone",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protect.control.v1.ControlService", "ImportZone"),
                );
            self.inner.client_streaming(req, path, codec).await
        }
        pub async fn replicate_zone(
            &mut self,
            request: impl tonic::IntoRequest<super::ReplicateZoneRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReplicateZoneReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/ReplicateZone",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protect.control.v1.ControlService", "ReplicateZone"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn claim_restored_zone(
            &mut self,
            request: impl tonic::IntoRequest<super::ClaimRestoredZoneRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ClaimRestoredZoneReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/ClaimRestoredZone",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protect.control.v1.ControlService",
                        "ClaimRestoredZone",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn start_restored_zone(
            &mut self,
            request: impl tonic::IntoRequest<super::StartRestoredZoneRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StartRestoredZoneReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/StartRestoredZone",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protect.control.v1.ControlService",
                        "StartRestoredZone",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn adopt_workload(
            &mut self,
            request: impl tonic::IntoRequest<super::AdoptWorkloadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AdoptWorkloadReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/AdoptWorkload",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protect.control.v1.ControlService", "AdoptWorkload"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn resolve_zone_id(
            &mut self,
            request: impl tonic::IntoRequest<super::ResolveZoneIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResolveZoneIdReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/ResolveZoneId",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protect.control.v1.ControlService", "ResolveZoneId"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn resolve_zone_ids(
            &mut self,
            request: impl tonic::IntoRequest<super::ResolveZoneIdsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResolveZoneIdsReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/ResolveZoneIds",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protect.control.v1.ControlService",
                        "ResolveZoneIds",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_zone(
            &mut self,
            request: impl tonic::IntoRequest<super::GetZoneRequest>,
        ) -> std::result::Result<tonic::Response<super::GetZoneReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/GetZone",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("protect.control.v1.ControlService", "GetZone"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_zones(
            &mut self,
            request: impl tonic::IntoRequest<super::ListZonesRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::ListZonesReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/ListZones",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protect.control.v1.ControlService", "ListZones"),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn discover_services(
            &mut self,
            request: impl tonic::IntoRequest<super::DiscoverServicesRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::DiscoverServicesReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/DiscoverServices",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protect.control.v1.ControlService",
                        "DiscoverServices",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn get_zones_vcpu_infos(
            &mut self,
            request: impl tonic::IntoRequest<super::GetZonesVcpuInfosRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::GetZonesVcpuInfosReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/GetZonesVcpuInfos",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protect.control.v1.ControlService",
                        "GetZonesVcpuInfos",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn set_zone_vcpu_affinity(
            &mut self,
            request: impl tonic::IntoRequest<super::SetZoneVcpuAffinityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetZoneVcpuAffinityReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/SetZoneVcpuAffinity",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protect.control.v1.ControlService",
                        "SetZoneVcpuAffinity",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_zone_resources(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateZoneResourcesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateZoneResourcesReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/UpdateZoneResources",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protect.control.v1.ControlService",
                        "UpdateZoneResources",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn configure_zone_network(
            &mut self,
            request: impl tonic::IntoRequest<super::ConfigureZoneNetworkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConfigureZoneNetworkReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/ConfigureZoneNetwork",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protect.control.v1.ControlService",
                        "ConfigureZoneNetwork",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn advertise_zone_service(
            &mut self,
            request: impl tonic::IntoRequest<super::AdvertiseZoneServiceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AdvertiseZoneServiceReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/AdvertiseZoneService",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protect.control.v1.ControlService",
                        "AdvertiseZoneService",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn unadvertise_zone_service(
            &mut self,
            request: impl tonic::IntoRequest<super::UnadvertiseZoneServiceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnadvertiseZoneServiceReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/UnadvertiseZoneService",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protect.control.v1.ControlService",
                        "UnadvertiseZoneService",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn attach_zone_console(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::ZoneConsoleRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::ZoneConsoleReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/AttachZoneConsole",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protect.control.v1.ControlService",
                        "AttachZoneConsole",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
        pub async fn execute_zone_command(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::ExecuteZoneCommandRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::ExecuteZoneCommandReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/ExecuteZoneCommand",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protect.control.v1.ControlService",
                        "ExecuteZoneCommand",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
        pub async fn read_zone_metrics(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadZoneMetricsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReadZoneMetricsReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/ReadZoneMetrics",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protect.control.v1.ControlService",
                        "ReadZoneMetrics",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn monitor_zone_kernel_events(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::MonitorZoneKernelEventRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::MonitorZoneKernelEventReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/MonitorZoneKernelEvents",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protect.control.v1.ControlService",
                        "MonitorZoneKernelEvents",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
        pub async fn create_workload(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWorkloadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateWorkloadReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/CreateWorkload",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protect.control.v1.ControlService",
                        "CreateWorkload",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn start_workload(
            &mut self,
            request: impl tonic::IntoRequest<super::StartWorkloadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StartWorkloadReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/StartWorkload",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protect.control.v1.ControlService", "StartWorkload"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn stop_workload(
            &mut self,
            request: impl tonic::IntoRequest<super::StopWorkloadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StopWorkloadReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/StopWorkload",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protect.control.v1.ControlService", "StopWorkload"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn destroy_workload(
            &mut self,
            request: impl tonic::IntoRequest<super::DestroyWorkloadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DestroyWorkloadReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/DestroyWorkload",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protect.control.v1.ControlService",
                        "DestroyWorkload",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_workload_resources(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateWorkloadResourcesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateWorkloadResourcesReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/UpdateWorkloadResources",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protect.control.v1.ControlService",
                        "UpdateWorkloadResources",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn attach_workload_console(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::WorkloadConsoleRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::WorkloadConsoleReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/AttachWorkloadConsole",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protect.control.v1.ControlService",
                        "AttachWorkloadConsole",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
        pub async fn resolve_workload_id(
            &mut self,
            request: impl tonic::IntoRequest<super::ResolveWorkloadIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResolveWorkloadIdReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/ResolveWorkloadId",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protect.control.v1.ControlService",
                        "ResolveWorkloadId",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn resolve_workload_ids(
            &mut self,
            request: impl tonic::IntoRequest<super::ResolveWorkloadIdsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResolveWorkloadIdsReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/ResolveWorkloadIds",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protect.control.v1.ControlService",
                        "ResolveWorkloadIds",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_workloads(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWorkloadsRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::ListWorkloadsReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/ListWorkloads",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protect.control.v1.ControlService", "ListWorkloads"),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn get_workload(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWorkloadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetWorkloadReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/GetWorkload",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protect.control.v1.ControlService", "GetWorkload"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn watch_events(
            &mut self,
            request: impl tonic::IntoRequest<super::WatchEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::WatchEventsReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/WatchEvents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("protect.control.v1.ControlService", "WatchEvents"),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn read_hypervisor_console(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadHypervisorConsoleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReadHypervisorConsoleReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/ReadHypervisorConsole",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protect.control.v1.ControlService",
                        "ReadHypervisorConsole",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn hypervisor_debug_info(
            &mut self,
            request: impl tonic::IntoRequest<super::HypervisorDebugInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::HypervisorDebugInfoReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/HypervisorDebugInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protect.control.v1.ControlService",
                        "HypervisorDebugInfo",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_host_power_management_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::SetHostPowerManagementPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetHostPowerManagementPolicyReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/SetHostPowerManagementPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protect.control.v1.ControlService",
                        "SetHostPowerManagementPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn dial_network_socket(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::DialNetworkSocketRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::DialNetworkSocketReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/DialNetworkSocket",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protect.control.v1.ControlService",
                        "DialNetworkSocket",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
        pub async fn dial_object_capability(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::DialObjectCapabilityRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::DialObjectCapabilityReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/DialObjectCapability",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protect.control.v1.ControlService",
                        "DialObjectCapability",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
        pub async fn list_object_capabilities(
            &mut self,
            request: impl tonic::IntoRequest<super::ListObjectCapabilitiesRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::ListObjectCapabilitiesReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protect.control.v1.ControlService/ListObjectCapabilities",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "protect.control.v1.ControlService",
                        "ListObjectCapabilities",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod control_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ControlServiceServer.
    #[async_trait]
    pub trait ControlService: std::marker::Send + std::marker::Sync + 'static {
        async fn get_host_status(
            &self,
            request: tonic::Request<super::GetHostStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetHostStatusReply>,
            tonic::Status,
        >;
        /// Server streaming response type for the SnoopIdm method.
        type SnoopIdmStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::SnoopIdmReply, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn snoop_idm(
            &self,
            request: tonic::Request<super::SnoopIdmRequest>,
        ) -> std::result::Result<tonic::Response<Self::SnoopIdmStream>, tonic::Status>;
        /// Server streaming response type for the SnoopControl method.
        type SnoopControlStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::SnoopControlReply, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn snoop_control(
            &self,
            request: tonic::Request<super::SnoopControlRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::SnoopControlStream>,
            tonic::Status,
        >;
        async fn get_host_cpu_topology(
            &self,
            request: tonic::Request<super::GetHostCpuTopologyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetHostCpuTopologyReply>,
            tonic::Status,
        >;
        async fn get_host_resources(
            &self,
            request: tonic::Request<super::GetHostResourcesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetHostResourcesReply>,
            tonic::Status,
        >;
        async fn list_devices(
            &self,
            request: tonic::Request<super::ListDevicesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDevicesReply>,
            tonic::Status,
        >;
        async fn create_network_reservation(
            &self,
            request: tonic::Request<super::CreateNetworkReservationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateNetworkReservationReply>,
            tonic::Status,
        >;
        async fn destroy_network_reservation(
            &self,
            request: tonic::Request<super::DestroyNetworkReservationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DestroyNetworkReservationReply>,
            tonic::Status,
        >;
        /// Server streaming response type for the ListNetworkReservations method.
        type ListNetworkReservationsStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::ListNetworkReservationsReply,
                    tonic::Status,
                >,
            >
            + std::marker::Send
            + 'static;
        async fn list_network_reservations(
            &self,
            request: tonic::Request<super::ListNetworkReservationsRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::ListNetworkReservationsStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the PullImage method.
        type PullImageStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::PullImageReply, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn pull_image(
            &self,
            request: tonic::Request<super::PullImageRequest>,
        ) -> std::result::Result<tonic::Response<Self::PullImageStream>, tonic::Status>;
        /// Server streaming response type for the ImportImage method.
        type ImportImageStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::ImportImageReply, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn import_image(
            &self,
            request: tonic::Request<tonic::Streaming<super::ImportImageRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::ImportImageStream>,
            tonic::Status,
        >;
        async fn import_packed_image(
            &self,
            request: tonic::Request<tonic::Streaming<super::ImportPackedImageRequest>>,
        ) -> std::result::Result<
            tonic::Response<super::ImportPackedImageReply>,
            tonic::Status,
        >;
        async fn remove_image(
            &self,
            request: tonic::Request<super::RemoveImageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveImageReply>,
            tonic::Status,
        >;
        async fn prune_images(
            &self,
            request: tonic::Request<super::PruneImagesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PruneImagesReply>,
            tonic::Status,
        >;
        /// Server streaming response type for the ListImages method.
        type ListImagesStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::ListImagesReply, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn list_images(
            &self,
            request: tonic::Request<super::ListImagesRequest>,
        ) -> std::result::Result<tonic::Response<Self::ListImagesStream>, tonic::Status>;
        async fn list_kernel_variants(
            &self,
            request: tonic::Request<super::ListKernelVariantsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListKernelVariantsReply>,
            tonic::Status,
        >;
        async fn create_zone(
            &self,
            request: tonic::Request<super::CreateZoneRequest>,
        ) -> std::result::Result<tonic::Response<super::CreateZoneReply>, tonic::Status>;
        async fn destroy_zone(
            &self,
            request: tonic::Request<super::DestroyZoneRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DestroyZoneReply>,
            tonic::Status,
        >;
        async fn suspend_zone(
            &self,
            request: tonic::Request<super::SuspendZoneRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SuspendZoneReply>,
            tonic::Status,
        >;
        async fn resume_zone(
            &self,
            request: tonic::Request<super::ResumeZoneRequest>,
        ) -> std::result::Result<tonic::Response<super::ResumeZoneReply>, tonic::Status>;
        async fn fork_zone(
            &self,
            request: tonic::Request<super::ForkZoneRequest>,
        ) -> std::result::Result<tonic::Response<super::ForkZoneReply>, tonic::Status>;
        /// Server streaming response type for the ExportZone method.
        type ExportZoneStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::ExportZoneReply, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn export_zone(
            &self,
            request: tonic::Request<super::ExportZoneRequest>,
        ) -> std::result::Result<tonic::Response<Self::ExportZoneStream>, tonic::Status>;
        async fn import_zone(
            &self,
            request: tonic::Request<tonic::Streaming<super::ImportZoneRequest>>,
        ) -> std::result::Result<tonic::Response<super::ImportZoneReply>, tonic::Status>;
        async fn replicate_zone(
            &self,
            request: tonic::Request<super::ReplicateZoneRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReplicateZoneReply>,
            tonic::Status,
        >;
        async fn claim_restored_zone(
            &self,
            request: tonic::Request<super::ClaimRestoredZoneRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ClaimRestoredZoneReply>,
            tonic::Status,
        >;
        async fn start_restored_zone(
            &self,
            request: tonic::Request<super::StartRestoredZoneRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StartRestoredZoneReply>,
            tonic::Status,
        >;
        async fn adopt_workload(
            &self,
            request: tonic::Request<super::AdoptWorkloadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AdoptWorkloadReply>,
            tonic::Status,
        >;
        async fn resolve_zone_id(
            &self,
            request: tonic::Request<super::ResolveZoneIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResolveZoneIdReply>,
            tonic::Status,
        >;
        async fn resolve_zone_ids(
            &self,
            request: tonic::Request<super::ResolveZoneIdsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResolveZoneIdsReply>,
            tonic::Status,
        >;
        async fn get_zone(
            &self,
            request: tonic::Request<super::GetZoneRequest>,
        ) -> std::result::Result<tonic::Response<super::GetZoneReply>, tonic::Status>;
        /// Server streaming response type for the ListZones method.
        type ListZonesStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::ListZonesReply, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn list_zones(
            &self,
            request: tonic::Request<super::ListZonesRequest>,
        ) -> std::result::Result<tonic::Response<Self::ListZonesStream>, tonic::Status>;
        /// Server streaming response type for the DiscoverServices method.
        type DiscoverServicesStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::DiscoverServicesReply, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn discover_services(
            &self,
            request: tonic::Request<super::DiscoverServicesRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::DiscoverServicesStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the GetZonesVcpuInfos method.
        type GetZonesVcpuInfosStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::GetZonesVcpuInfosReply, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn get_zones_vcpu_infos(
            &self,
            request: tonic::Request<super::GetZonesVcpuInfosRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::GetZonesVcpuInfosStream>,
            tonic::Status,
        >;
        async fn set_zone_vcpu_affinity(
            &self,
            request: tonic::Request<super::SetZoneVcpuAffinityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetZoneVcpuAffinityReply>,
            tonic::Status,
        >;
        async fn update_zone_resources(
            &self,
            request: tonic::Request<super::UpdateZoneResourcesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateZoneResourcesReply>,
            tonic::Status,
        >;
        async fn configure_zone_network(
            &self,
            request: tonic::Request<super::ConfigureZoneNetworkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConfigureZoneNetworkReply>,
            tonic::Status,
        >;
        async fn advertise_zone_service(
            &self,
            request: tonic::Request<super::AdvertiseZoneServiceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AdvertiseZoneServiceReply>,
            tonic::Status,
        >;
        async fn unadvertise_zone_service(
            &self,
            request: tonic::Request<super::UnadvertiseZoneServiceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnadvertiseZoneServiceReply>,
            tonic::Status,
        >;
        /// Server streaming response type for the AttachZoneConsole method.
        type AttachZoneConsoleStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::ZoneConsoleReply, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn attach_zone_console(
            &self,
            request: tonic::Request<tonic::Streaming<super::ZoneConsoleRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::AttachZoneConsoleStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the ExecuteZoneCommand method.
        type ExecuteZoneCommandStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::ExecuteZoneCommandReply, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn execute_zone_command(
            &self,
            request: tonic::Request<tonic::Streaming<super::ExecuteZoneCommandRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::ExecuteZoneCommandStream>,
            tonic::Status,
        >;
        async fn read_zone_metrics(
            &self,
            request: tonic::Request<super::ReadZoneMetricsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReadZoneMetricsReply>,
            tonic::Status,
        >;
        /// Server streaming response type for the MonitorZoneKernelEvents method.
        type MonitorZoneKernelEventsStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::MonitorZoneKernelEventReply,
                    tonic::Status,
                >,
            >
            + std::marker::Send
            + 'static;
        async fn monitor_zone_kernel_events(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::MonitorZoneKernelEventRequest>,
            >,
        ) -> std::result::Result<
            tonic::Response<Self::MonitorZoneKernelEventsStream>,
            tonic::Status,
        >;
        async fn create_workload(
            &self,
            request: tonic::Request<super::CreateWorkloadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateWorkloadReply>,
            tonic::Status,
        >;
        async fn start_workload(
            &self,
            request: tonic::Request<super::StartWorkloadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StartWorkloadReply>,
            tonic::Status,
        >;
        async fn stop_workload(
            &self,
            request: tonic::Request<super::StopWorkloadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StopWorkloadReply>,
            tonic::Status,
        >;
        async fn destroy_workload(
            &self,
            request: tonic::Request<super::DestroyWorkloadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DestroyWorkloadReply>,
            tonic::Status,
        >;
        async fn update_workload_resources(
            &self,
            request: tonic::Request<super::UpdateWorkloadResourcesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateWorkloadResourcesReply>,
            tonic::Status,
        >;
        /// Server streaming response type for the AttachWorkloadConsole method.
        type AttachWorkloadConsoleStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::WorkloadConsoleReply, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn attach_workload_console(
            &self,
            request: tonic::Request<tonic::Streaming<super::WorkloadConsoleRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::AttachWorkloadConsoleStream>,
            tonic::Status,
        >;
        async fn resolve_workload_id(
            &self,
            request: tonic::Request<super::ResolveWorkloadIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResolveWorkloadIdReply>,
            tonic::Status,
        >;
        async fn resolve_workload_ids(
            &self,
            request: tonic::Request<super::ResolveWorkloadIdsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResolveWorkloadIdsReply>,
            tonic::Status,
        >;
        /// Server streaming response type for the ListWorkloads method.
        type ListWorkloadsStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::ListWorkloadsReply, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn list_workloads(
            &self,
            request: tonic::Request<super::ListWorkloadsRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::ListWorkloadsStream>,
            tonic::Status,
        >;
        async fn get_workload(
            &self,
            request: tonic::Request<super::GetWorkloadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetWorkloadReply>,
            tonic::Status,
        >;
        /// Server streaming response type for the WatchEvents method.
        type WatchEventsStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::WatchEventsReply, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn watch_events(
            &self,
            request: tonic::Request<super::WatchEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::WatchEventsStream>,
            tonic::Status,
        >;
        async fn read_hypervisor_console(
            &self,
            request: tonic::Request<super::ReadHypervisorConsoleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReadHypervisorConsoleReply>,
            tonic::Status,
        >;
        async fn hypervisor_debug_info(
            &self,
            request: tonic::Request<super::HypervisorDebugInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::HypervisorDebugInfoReply>,
            tonic::Status,
        >;
        async fn set_host_power_management_policy(
            &self,
            request: tonic::Request<super::SetHostPowerManagementPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetHostPowerManagementPolicyReply>,
            tonic::Status,
        >;
        /// Server streaming response type for the DialNetworkSocket method.
        type DialNetworkSocketStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::DialNetworkSocketReply, tonic::Status>,
            >
            + std::marker::Send
            + 'static;
        async fn dial_network_socket(
            &self,
            request: tonic::Request<tonic::Streaming<super::DialNetworkSocketRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::DialNetworkSocketStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the DialObjectCapability method.
        type DialObjectCapabilityStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::DialObjectCapabilityReply,
                    tonic::Status,
                >,
            >
            + std::marker::Send
            + 'static;
        async fn dial_object_capability(
            &self,
            request: tonic::Request<tonic::Streaming<super::DialObjectCapabilityRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::DialObjectCapabilityStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the ListObjectCapabilities method.
        type ListObjectCapabilitiesStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::ListObjectCapabilitiesReply,
                    tonic::Status,
                >,
            >
            + std::marker::Send
            + 'static;
        async fn list_object_capabilities(
            &self,
            request: tonic::Request<super::ListObjectCapabilitiesRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::ListObjectCapabilitiesStream>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct ControlServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> ControlServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ControlServiceServer<T>
    where
        T: ControlService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/protect.control.v1.ControlService/GetHostStatus" => {
                    #[allow(non_camel_case_types)]
                    struct GetHostStatusSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::GetHostStatusRequest>
                    for GetHostStatusSvc<T> {
                        type Response = super::GetHostStatusReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetHostStatusRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::get_host_status(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetHostStatusSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/SnoopIdm" => {
                    #[allow(non_camel_case_types)]
                    struct SnoopIdmSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::ServerStreamingService<super::SnoopIdmRequest>
                    for SnoopIdmSvc<T> {
                        type Response = super::SnoopIdmReply;
                        type ResponseStream = T::SnoopIdmStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SnoopIdmRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::snoop_idm(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = SnoopIdmSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/SnoopControl" => {
                    #[allow(non_camel_case_types)]
                    struct SnoopControlSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::ServerStreamingService<super::SnoopControlRequest>
                    for SnoopControlSvc<T> {
                        type Response = super::SnoopControlReply;
                        type ResponseStream = T::SnoopControlStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SnoopControlRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::snoop_control(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = SnoopControlSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/GetHostCpuTopology" => {
                    #[allow(non_camel_case_types)]
                    struct GetHostCpuTopologySvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::GetHostCpuTopologyRequest>
                    for GetHostCpuTopologySvc<T> {
                        type Response = super::GetHostCpuTopologyReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetHostCpuTopologyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::get_host_cpu_topology(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetHostCpuTopologySvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/GetHostResources" => {
                    #[allow(non_camel_case_types)]
                    struct GetHostResourcesSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::GetHostResourcesRequest>
                    for GetHostResourcesSvc<T> {
                        type Response = super::GetHostResourcesReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetHostResourcesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::get_host_resources(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetHostResourcesSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/ListDevices" => {
                    #[allow(non_camel_case_types)]
                    struct ListDevicesSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::ListDevicesRequest>
                    for ListDevicesSvc<T> {
                        type Response = super::ListDevicesReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListDevicesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::list_devices(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListDevicesSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/CreateNetworkReservation" => {
                    #[allow(non_camel_case_types)]
                    struct CreateNetworkReservationSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::CreateNetworkReservationRequest>
                    for CreateNetworkReservationSvc<T> {
                        type Response = super::CreateNetworkReservationReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateNetworkReservationRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::create_network_reservation(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateNetworkReservationSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/DestroyNetworkReservation" => {
                    #[allow(non_camel_case_types)]
                    struct DestroyNetworkReservationSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<
                        super::DestroyNetworkReservationRequest,
                    > for DestroyNetworkReservationSvc<T> {
                        type Response = super::DestroyNetworkReservationReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::DestroyNetworkReservationRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::destroy_network_reservation(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DestroyNetworkReservationSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/ListNetworkReservations" => {
                    #[allow(non_camel_case_types)]
                    struct ListNetworkReservationsSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::ServerStreamingService<
                        super::ListNetworkReservationsRequest,
                    > for ListNetworkReservationsSvc<T> {
                        type Response = super::ListNetworkReservationsReply;
                        type ResponseStream = T::ListNetworkReservationsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListNetworkReservationsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::list_network_reservations(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListNetworkReservationsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/PullImage" => {
                    #[allow(non_camel_case_types)]
                    struct PullImageSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::ServerStreamingService<super::PullImageRequest>
                    for PullImageSvc<T> {
                        type Response = super::PullImageReply;
                        type ResponseStream = T::PullImageStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PullImageRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::pull_image(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = PullImageSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/ImportImage" => {
                    #[allow(non_camel_case_types)]
                    struct ImportImageSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::StreamingService<super::ImportImageRequest>
                    for ImportImageSvc<T> {
                        type Response = super::ImportImageReply;
                        type ResponseStream = T::ImportImageStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::ImportImageRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::import_image(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ImportImageSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/ImportPackedImage" => {
                    #[allow(non_camel_case_types)]
                    struct ImportPackedImageSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::ClientStreamingService<
                        super::ImportPackedImageRequest,
                    > for ImportPackedImageSvc<T> {
                        type Response = super::ImportPackedImageReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::ImportPackedImageRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::import_packed_image(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ImportPackedImageSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/RemoveImage" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveImageSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::RemoveImageRequest>
                    for RemoveImageSvc<T> {
                        type Response = super::RemoveImageReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveImageRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::remove_image(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RemoveImageSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/PruneImages" => {
                    #[allow(non_camel_case_types)]
                    struct PruneImagesSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::PruneImagesRequest>
                    for PruneImagesSvc<T> {
                        type Response = super::PruneImagesReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PruneImagesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::prune_images(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = PruneImagesSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/ListImages" => {
                    #[allow(non_camel_case_types)]
                    struct ListImagesSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::ServerStreamingService<super::ListImagesRequest>
                    for ListImagesSvc<T> {
                        type Response = super::ListImagesReply;
                        type ResponseStream = T::ListImagesStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListImagesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::list_images(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListImagesSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/ListKernelVariants" => {
                    #[allow(non_camel_case_types)]
                    struct ListKernelVariantsSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::ListKernelVariantsRequest>
                    for ListKernelVariantsSvc<T> {
                        type Response = super::ListKernelVariantsReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListKernelVariantsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::list_kernel_variants(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListKernelVariantsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/CreateZone" => {
                    #[allow(non_camel_case_types)]
                    struct CreateZoneSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::CreateZoneRequest>
                    for CreateZoneSvc<T> {
                        type Response = super::CreateZoneReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateZoneRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::create_zone(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateZoneSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/DestroyZone" => {
                    #[allow(non_camel_case_types)]
                    struct DestroyZoneSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::DestroyZoneRequest>
                    for DestroyZoneSvc<T> {
                        type Response = super::DestroyZoneReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DestroyZoneRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::destroy_zone(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DestroyZoneSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/SuspendZone" => {
                    #[allow(non_camel_case_types)]
                    struct SuspendZoneSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::SuspendZoneRequest>
                    for SuspendZoneSvc<T> {
                        type Response = super::SuspendZoneReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SuspendZoneRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::suspend_zone(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = SuspendZoneSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/ResumeZone" => {
                    #[allow(non_camel_case_types)]
                    struct ResumeZoneSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::ResumeZoneRequest>
                    for ResumeZoneSvc<T> {
                        type Response = super::ResumeZoneReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ResumeZoneRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::resume_zone(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ResumeZoneSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/ForkZone" => {
                    #[allow(non_camel_case_types)]
                    struct ForkZoneSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::ForkZoneRequest>
                    for ForkZoneSvc<T> {
                        type Response = super::ForkZoneReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ForkZoneRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::fork_zone(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ForkZoneSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/ExportZone" => {
                    #[allow(non_camel_case_types)]
                    struct ExportZoneSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::ServerStreamingService<super::ExportZoneRequest>
                    for ExportZoneSvc<T> {
                        type Response = super::ExportZoneReply;
                        type ResponseStream = T::ExportZoneStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExportZoneRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::export_zone(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ExportZoneSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/ImportZone" => {
                    #[allow(non_camel_case_types)]
                    struct ImportZoneSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::ClientStreamingService<super::ImportZoneRequest>
                    for ImportZoneSvc<T> {
                        type Response = super::ImportZoneReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::ImportZoneRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::import_zone(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ImportZoneSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/ReplicateZone" => {
                    #[allow(non_camel_case_types)]
                    struct ReplicateZoneSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::ReplicateZoneRequest>
                    for ReplicateZoneSvc<T> {
                        type Response = super::ReplicateZoneReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReplicateZoneRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::replicate_zone(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ReplicateZoneSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/ClaimRestoredZone" => {
                    #[allow(non_camel_case_types)]
                    struct ClaimRestoredZoneSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::ClaimRestoredZoneRequest>
                    for ClaimRestoredZoneSvc<T> {
                        type Response = super::ClaimRestoredZoneReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ClaimRestoredZoneRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::claim_restored_zone(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ClaimRestoredZoneSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/StartRestoredZone" => {
                    #[allow(non_camel_case_types)]
                    struct StartRestoredZoneSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::StartRestoredZoneRequest>
                    for StartRestoredZoneSvc<T> {
                        type Response = super::StartRestoredZoneReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StartRestoredZoneRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::start_restored_zone(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = StartRestoredZoneSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/AdoptWorkload" => {
                    #[allow(non_camel_case_types)]
                    struct AdoptWorkloadSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::AdoptWorkloadRequest>
                    for AdoptWorkloadSvc<T> {
                        type Response = super::AdoptWorkloadReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AdoptWorkloadRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::adopt_workload(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = AdoptWorkloadSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/ResolveZoneId" => {
                    #[allow(non_camel_case_types)]
                    struct ResolveZoneIdSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::ResolveZoneIdRequest>
                    for ResolveZoneIdSvc<T> {
                        type Response = super::ResolveZoneIdReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ResolveZoneIdRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::resolve_zone_id(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ResolveZoneIdSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/ResolveZoneIds" => {
                    #[allow(non_camel_case_types)]
                    struct ResolveZoneIdsSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::ResolveZoneIdsRequest>
                    for ResolveZoneIdsSvc<T> {
                        type Response = super::ResolveZoneIdsReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ResolveZoneIdsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::resolve_zone_ids(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ResolveZoneIdsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/GetZone" => {
                    #[allow(non_camel_case_types)]
                    struct GetZoneSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::GetZoneRequest>
                    for GetZoneSvc<T> {
                        type Response = super::GetZoneReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetZoneRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::get_zone(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetZoneSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/ListZones" => {
                    #[allow(non_camel_case_types)]
                    struct ListZonesSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::ServerStreamingService<super::ListZonesRequest>
                    for ListZonesSvc<T> {
                        type Response = super::ListZonesReply;
                        type ResponseStream = T::ListZonesStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListZonesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::list_zones(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListZonesSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/DiscoverServices" => {
                    #[allow(non_camel_case_types)]
                    struct DiscoverServicesSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::ServerStreamingService<
                        super::DiscoverServicesRequest,
                    > for DiscoverServicesSvc<T> {
                        type Response = super::DiscoverServicesReply;
                        type ResponseStream = T::DiscoverServicesStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DiscoverServicesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::discover_services(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DiscoverServicesSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/GetZonesVcpuInfos" => {
                    #[allow(non_camel_case_types)]
                    struct GetZonesVcpuInfosSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::ServerStreamingService<
                        super::GetZonesVcpuInfosRequest,
                    > for GetZonesVcpuInfosSvc<T> {
                        type Response = super::GetZonesVcpuInfosReply;
                        type ResponseStream = T::GetZonesVcpuInfosStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetZonesVcpuInfosRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::get_zones_vcpu_infos(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetZonesVcpuInfosSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/SetZoneVcpuAffinity" => {
                    #[allow(non_camel_case_types)]
                    struct SetZoneVcpuAffinitySvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::SetZoneVcpuAffinityRequest>
                    for SetZoneVcpuAffinitySvc<T> {
                        type Response = super::SetZoneVcpuAffinityReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetZoneVcpuAffinityRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::set_zone_vcpu_affinity(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = SetZoneVcpuAffinitySvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/UpdateZoneResources" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateZoneResourcesSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::UpdateZoneResourcesRequest>
                    for UpdateZoneResourcesSvc<T> {
                        type Response = super::UpdateZoneResourcesReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateZoneResourcesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::update_zone_resources(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateZoneResourcesSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/ConfigureZoneNetwork" => {
                    #[allow(non_camel_case_types)]
                    struct ConfigureZoneNetworkSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::ConfigureZoneNetworkRequest>
                    for ConfigureZoneNetworkSvc<T> {
                        type Response = super::ConfigureZoneNetworkReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConfigureZoneNetworkRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::configure_zone_network(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ConfigureZoneNetworkSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/AdvertiseZoneService" => {
                    #[allow(non_camel_case_types)]
                    struct AdvertiseZoneServiceSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::AdvertiseZoneServiceRequest>
                    for AdvertiseZoneServiceSvc<T> {
                        type Response = super::AdvertiseZoneServiceReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AdvertiseZoneServiceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::advertise_zone_service(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = AdvertiseZoneServiceSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/UnadvertiseZoneService" => {
                    #[allow(non_camel_case_types)]
                    struct UnadvertiseZoneServiceSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::UnadvertiseZoneServiceRequest>
                    for UnadvertiseZoneServiceSvc<T> {
                        type Response = super::UnadvertiseZoneServiceReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnadvertiseZoneServiceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::unadvertise_zone_service(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UnadvertiseZoneServiceSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/AttachZoneConsole" => {
                    #[allow(non_camel_case_types)]
                    struct AttachZoneConsoleSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::StreamingService<super::ZoneConsoleRequest>
                    for AttachZoneConsoleSvc<T> {
                        type Response = super::ZoneConsoleReply;
                        type ResponseStream = T::AttachZoneConsoleStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::ZoneConsoleRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::attach_zone_console(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = AttachZoneConsoleSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/ExecuteZoneCommand" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteZoneCommandSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::StreamingService<super::ExecuteZoneCommandRequest>
                    for ExecuteZoneCommandSvc<T> {
                        type Response = super::ExecuteZoneCommandReply;
                        type ResponseStream = T::ExecuteZoneCommandStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::ExecuteZoneCommandRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::execute_zone_command(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ExecuteZoneCommandSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/ReadZoneMetrics" => {
                    #[allow(non_camel_case_types)]
                    struct ReadZoneMetricsSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::ReadZoneMetricsRequest>
                    for ReadZoneMetricsSvc<T> {
                        type Response = super::ReadZoneMetricsReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadZoneMetricsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::read_zone_metrics(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ReadZoneMetricsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/MonitorZoneKernelEvents" => {
                    #[allow(non_camel_case_types)]
                    struct MonitorZoneKernelEventsSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::StreamingService<
                        super::MonitorZoneKernelEventRequest,
                    > for MonitorZoneKernelEventsSvc<T> {
                        type Response = super::MonitorZoneKernelEventReply;
                        type ResponseStream = T::MonitorZoneKernelEventsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::MonitorZoneKernelEventRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::monitor_zone_kernel_events(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = MonitorZoneKernelEventsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/CreateWorkload" => {
                    #[allow(non_camel_case_types)]
                    struct CreateWorkloadSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::CreateWorkloadRequest>
                    for CreateWorkloadSvc<T> {
                        type Response = super::CreateWorkloadReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateWorkloadRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::create_workload(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateWorkloadSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/StartWorkload" => {
                    #[allow(non_camel_case_types)]
                    struct StartWorkloadSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::StartWorkloadRequest>
                    for StartWorkloadSvc<T> {
                        type Response = super::StartWorkloadReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StartWorkloadRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::start_workload(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = StartWorkloadSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/StopWorkload" => {
                    #[allow(non_camel_case_types)]
                    struct StopWorkloadSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::StopWorkloadRequest>
                    for StopWorkloadSvc<T> {
                        type Response = super::StopWorkloadReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StopWorkloadRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::stop_workload(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = StopWorkloadSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/DestroyWorkload" => {
                    #[allow(non_camel_case_types)]
                    struct DestroyWorkloadSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::DestroyWorkloadRequest>
                    for DestroyWorkloadSvc<T> {
                        type Response = super::DestroyWorkloadReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DestroyWorkloadRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::destroy_workload(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DestroyWorkloadSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/UpdateWorkloadResources" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateWorkloadResourcesSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::UpdateWorkloadResourcesRequest>
                    for UpdateWorkloadResourcesSvc<T> {
                        type Response = super::UpdateWorkloadResourcesReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdateWorkloadResourcesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::update_workload_resources(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateWorkloadResourcesSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/AttachWorkloadConsole" => {
                    #[allow(non_camel_case_types)]
                    struct AttachWorkloadConsoleSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::StreamingService<super::WorkloadConsoleRequest>
                    for AttachWorkloadConsoleSvc<T> {
                        type Response = super::WorkloadConsoleReply;
                        type ResponseStream = T::AttachWorkloadConsoleStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::WorkloadConsoleRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::attach_workload_console(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = AttachWorkloadConsoleSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/ResolveWorkloadId" => {
                    #[allow(non_camel_case_types)]
                    struct ResolveWorkloadIdSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::ResolveWorkloadIdRequest>
                    for ResolveWorkloadIdSvc<T> {
                        type Response = super::ResolveWorkloadIdReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ResolveWorkloadIdRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::resolve_workload_id(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ResolveWorkloadIdSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/ResolveWorkloadIds" => {
                    #[allow(non_camel_case_types)]
                    struct ResolveWorkloadIdsSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::ResolveWorkloadIdsRequest>
                    for ResolveWorkloadIdsSvc<T> {
                        type Response = super::ResolveWorkloadIdsReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ResolveWorkloadIdsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::resolve_workload_ids(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ResolveWorkloadIdsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/ListWorkloads" => {
                    #[allow(non_camel_case_types)]
                    struct ListWorkloadsSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::ServerStreamingService<super::ListWorkloadsRequest>
                    for ListWorkloadsSvc<T> {
                        type Response = super::ListWorkloadsReply;
                        type ResponseStream = T::ListWorkloadsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListWorkloadsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::list_workloads(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListWorkloadsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/GetWorkload" => {
                    #[allow(non_camel_case_types)]
                    struct GetWorkloadSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::GetWorkloadRequest>
                    for GetWorkloadSvc<T> {
                        type Response = super::GetWorkloadReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetWorkloadRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::get_workload(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetWorkloadSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/WatchEvents" => {
                    #[allow(non_camel_case_types)]
                    struct WatchEventsSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::ServerStreamingService<super::WatchEventsRequest>
                    for WatchEventsSvc<T> {
                        type Response = super::WatchEventsReply;
                        type ResponseStream = T::WatchEventsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WatchEventsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::watch_events(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = WatchEventsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/ReadHypervisorConsole" => {
                    #[allow(non_camel_case_types)]
                    struct ReadHypervisorConsoleSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::ReadHypervisorConsoleRequest>
                    for ReadHypervisorConsoleSvc<T> {
                        type Response = super::ReadHypervisorConsoleReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadHypervisorConsoleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::read_hypervisor_console(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ReadHypervisorConsoleSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/HypervisorDebugInfo" => {
                    #[allow(non_camel_case_types)]
                    struct HypervisorDebugInfoSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<super::HypervisorDebugInfoRequest>
                    for HypervisorDebugInfoSvc<T> {
                        type Response = super::HypervisorDebugInfoReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HypervisorDebugInfoRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::hypervisor_debug_info(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = HypervisorDebugInfoSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/SetHostPowerManagementPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct SetHostPowerManagementPolicySvc<T: ControlService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ControlService,
                    > tonic::server::UnaryService<
                        super::SetHostPowerManagementPolicyRequest,
                    > for SetHostPowerManagementPolicySvc<T> {
                        type Response = super::SetHostPowerManagementPolicyReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SetHostPowerManagementPolicyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::set_host_power_management_policy(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = SetHostPowerManagementPolicySvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/DialNetworkSocket" => {
                    #[allow(non_camel_case_types)]
                    struct DialNetworkSocketSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::StreamingService<super::DialNetworkSocketRequest>
                    for DialNetworkSocketSvc<T> {
                        type Response = super::DialNetworkSocketReply;
                        type ResponseStream = T::DialNetworkSocketStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::DialNetworkSocketRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::dial_network_socket(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DialNetworkSocketSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/DialObjectCapability" => {
                    #[allow(non_camel_case_types)]
                    struct DialObjectCapabilitySvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::StreamingService<super::DialObjectCapabilityRequest>
                    for DialObjectCapabilitySvc<T> {
                        type Response = super::DialObjectCapabilityReply;
                        type ResponseStream = T::DialObjectCapabilityStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::DialObjectCapabilityRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::dial_object_capability(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DialObjectCapabilitySvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/protect.control.v1.ControlService/ListObjectCapabilities" => {
                    #[allow(non_camel_case_types)]
                    struct ListObjectCapabilitiesSvc<T: ControlService>(pub Arc<T>);
                    impl<
                        T: ControlService,
                    > tonic::server::ServerStreamingService<
                        super::ListObjectCapabilitiesRequest,
                    > for ListObjectCapabilitiesSvc<T> {
                        type Response = super::ListObjectCapabilitiesReply;
                        type ResponseStream = T::ListObjectCapabilitiesStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListObjectCapabilitiesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControlService>::list_object_capabilities(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListObjectCapabilitiesSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for ControlServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "protect.control.v1.ControlService";
    impl<T> tonic::server::NamedService for ControlServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
