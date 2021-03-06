// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// http://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

use crate::common::{self, Core, Uid};
use crate::main::bootstrap::Cache as BootstrapCache;
use crate::main::Config;
use mio::Token;
use safe_crypto::PublicEncryptKey;
use std::net::SocketAddr;

// ========================================================================================
//                                     ConnectionId
// ========================================================================================
#[derive(Debug, Clone, Copy)]
pub struct ConnectionId {
    pub active_connection: Option<Token>,
    pub currently_handshaking: usize,
}

// ========================================================================================
//                                   ConnectionInfoResult
// ========================================================================================
/// The result of a `Service::prepare_contact_info` call.
#[derive(Debug)]
pub struct ConnectionInfoResult<UID> {
    /// The token that was passed to `prepare_connection_info`.
    pub result_token: u32,
    /// The new contact info, if successful.
    pub result: crate::Res<PrivConnectionInfo<UID>>,
}

// ========================================================================================
//                                     PrivConnectionInfo
// ========================================================================================
/// Contact info generated by a call to `Service::prepare_contact_info`.
#[derive(Debug)]
pub struct PrivConnectionInfo<UID> {
    #[doc(hidden)]
    pub id: UID,
    #[doc(hidden)]
    pub for_direct: Vec<SocketAddr>,
    #[doc(hidden)]
    pub our_pk: PublicEncryptKey,
}

impl<UID: Uid> PrivConnectionInfo<UID> {
    /// Use private connection info to create public connection info that can be shared with the
    /// peer.
    pub fn to_pub_connection_info(&self) -> PubConnectionInfo<UID> {
        PubConnectionInfo {
            for_direct: self.for_direct.clone(),
            id: self.id,
            our_pk: self.our_pk,
        }
    }
}

// ========================================================================================
//                                     PubConnectionInfo
// ========================================================================================
/// Contact info used to connect to another peer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubConnectionInfo<UID> {
    #[doc(hidden)]
    pub id: UID,
    #[doc(hidden)]
    pub for_direct: Vec<SocketAddr>,
    #[doc(hidden)]
    pub our_pk: PublicEncryptKey,
}

impl<UID: Uid> PubConnectionInfo<UID> {
    /// Returns the `UID` of the node that created this connection info.
    pub fn id(&self) -> UID {
        self.id
    }
}

// ========================================================================================
//                                     ConfigWrapper
// ========================================================================================
#[derive(Default)]
pub struct ConfigWrapper {
    pub cfg: Config,
    pub is_modified_for_next_refresh: bool,
}
impl ConfigWrapper {
    pub fn new(cfg: Config) -> Self {
        Self {
            cfg,
            is_modified_for_next_refresh: false,
        }
    }

    pub fn check_for_update_and_mark_modified(&mut self, new_cfg: Config) {
        if self.cfg != new_cfg {
            self.cfg = new_cfg;
            self.is_modified_for_next_refresh = true;
        }
    }

    /// Checks if `ActiveConnection` refresh is needed.
    pub fn check_for_refresh_and_reset_modified(&mut self, new_cfg: Config) -> bool {
        let should_refresh = if self.cfg != new_cfg {
            self.cfg = new_cfg;
            true
        } else {
            self.is_modified_for_next_refresh
        };

        self.is_modified_for_next_refresh = false;
        should_refresh
    }
}

/// Crust event loop state object. It is owned by the same thread event loop is running on,
/// it holds bootstrap cache and manages Crust states like `Connect`, `ConnectionCandidate`, etc.
pub type EventLoopCore = Core<BootstrapCache>;

/// Handle to Crust event loop that owns `EventLoopCore`.
pub type EventLoop = common::EventLoop<BootstrapCache>;
