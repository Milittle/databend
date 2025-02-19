// Copyright 2022 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use common_exception::ErrorCode;
use common_meta_stoerr::MetaStorageError;
use tonic::Status;

use crate::AppError;
use crate::InvalidReply;
use crate::MetaAPIError;
use crate::MetaClientError;
use crate::MetaError;
use crate::MetaNetworkError;

/// Errors for a kvapi::KVApi based application, such SchemaApi, ShareApi.
///
/// There are three subset of errors in it:
///
/// - (1) AppError: the errors that relate to the application of meta but not about meta itself.
///
/// - (2) Meta data errors raised by a embedded meta-store(not the remote meta-service): StorageError.
///
/// - (3) Meta data errors returned when accessing the remote meta-store service:
///   - ClientError: errors returned when creating a client.
///   - NetworkError: errors returned when sending/receiving RPC to/from a remote meta-store service.
///   - APIError: errors returned by the remote meta-store service.
///
/// Either a local or remote meta-store will returns (1) AppError.
/// An embedded meta-store only returns (1) and (2), while a remote meta-store service only returns (1) and (3)
#[derive(thiserror::Error, serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum KVAppError {
    /// An error that indicates something wrong for the application of kvapi::KVApi, but nothing wrong about meta.
    #[error(transparent)]
    AppError(#[from] AppError),

    #[error("fail to access meta-store: {0}")]
    MetaError(#[from] MetaError),
}

impl From<KVAppError> for ErrorCode {
    fn from(e: KVAppError) -> Self {
        match e {
            KVAppError::AppError(app_err) => app_err.into(),
            KVAppError::MetaError(meta_err) => meta_err.into(),
        }
    }
}

impl From<Status> for KVAppError {
    fn from(s: Status) -> Self {
        let meta_err = MetaError::from(s);
        Self::MetaError(meta_err)
    }
}

impl From<MetaStorageError> for KVAppError {
    fn from(e: MetaStorageError) -> Self {
        let meta_err = MetaError::from(e);
        Self::MetaError(meta_err)
    }
}

impl From<MetaClientError> for KVAppError {
    fn from(e: MetaClientError) -> Self {
        let meta_err = MetaError::from(e);
        Self::MetaError(meta_err)
    }
}

impl From<MetaNetworkError> for KVAppError {
    fn from(e: MetaNetworkError) -> Self {
        let meta_err = MetaError::from(e);
        Self::MetaError(meta_err)
    }
}

impl From<MetaAPIError> for KVAppError {
    fn from(e: MetaAPIError) -> Self {
        let meta_err = MetaError::from(e);
        Self::MetaError(meta_err)
    }
}

impl From<InvalidReply> for KVAppError {
    fn from(e: InvalidReply) -> Self {
        let meta_err = MetaError::from(e);
        Self::MetaError(meta_err)
    }
}
