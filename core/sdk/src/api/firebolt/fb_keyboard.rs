// If not stated otherwise in this file or this component's license file the
// following copyright and licenses apply:
//
// Copyright 2023 RDK Management
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use serde::{Deserialize, Serialize};

use crate::{
    api::gateway::rpc_gateway_api::CallContext,
    extn::extn_client_message::{ExtnPayload, ExtnPayloadProvider, ExtnRequest, ExtnResponse},
    framework::ripple_contract::RippleContract,
};

use super::provider::{ProviderResponse, ProviderResponsePayload};

pub const EMAIL_EVENT_PREFIX: &'static str = "keyboard.onRequestEmail";
pub const PASSWORD_EVENT_PREFIX: &'static str = "keyboard.onRequestPassword";
pub const STANDARD_EVENT_PREFIX: &'static str = "keyboard.onRequestStandard";

pub const KEYBOARD_PROVIDER_CAPABILITY: &'static str = "xrn:firebolt:capability:input:keyboard";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum KeyboardType {
    Email,
    Password,
    Standard,
}

impl KeyboardType {
    pub fn to_provider_method(&self) -> &str {
        match self {
            KeyboardType::Email => "email",
            KeyboardType::Password => "password",
            KeyboardType::Standard => "standard",
        }
    }
}

#[derive(Deserialize)]
pub struct KeyboardRequestPassword {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Deserialize)]
pub struct KeyboardRequestEmail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "type")]
    pub _type: EmailUsage,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EmailUsage {
    SignIn,
    SignUp,
}

#[derive(Deserialize)]
pub struct KeyboardRequest {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyboardProviderResponse {
    correlation_id: String,
    result: KeyboardSessionResponse,
}

impl KeyboardProviderResponse {
    pub fn to_provider_response(&self) -> ProviderResponse {
        ProviderResponse {
            correlation_id: self.correlation_id.clone(),
            result: ProviderResponsePayload::KeyboardResult(self.result.clone()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KeyboardSessionRequest {
    #[serde(rename = "type")]
    pub _type: KeyboardType,
    pub ctx: CallContext,
    pub message: String,
}

impl ExtnPayloadProvider for KeyboardSessionRequest {
    fn get_extn_payload(&self) -> ExtnPayload {
        ExtnPayload::Request(ExtnRequest::Keyboard(self.clone()))
    }

    fn get_from_payload(payload: ExtnPayload) -> Option<Self> {
        match payload {
            ExtnPayload::Request(request) => match request {
                ExtnRequest::Keyboard(r) => return Some(r),
                _ => {}
            },
            _ => {}
        }
        None
    }

    fn contract() -> RippleContract {
        RippleContract::Keyboard
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KeyboardSessionResponse {
    pub text: String,
    pub canceled: bool,
}

impl ExtnPayloadProvider for KeyboardSessionResponse {
    fn get_extn_payload(&self) -> ExtnPayload {
        ExtnPayload::Response(ExtnResponse::Keyboard(self.clone()))
    }

    fn get_from_payload(payload: ExtnPayload) -> Option<Self> {
        match payload {
            ExtnPayload::Response(r) => match r {
                ExtnResponse::Keyboard(r) => return Some(r),
                _ => {}
            },
            _ => {}
        }
        None
    }

    fn contract() -> RippleContract {
        RippleContract::Keyboard
    }
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PromptEmailRequest {
    pub prefill_type: PrefillType,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PrefillType {
    SignIn,
    SignUp,
}