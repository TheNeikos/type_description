//
//   This Source Code Form is subject to the terms of the Mozilla Public
//   License, v. 2.0. If a copy of the MPL was not distributed with this
//   file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

use type_description::{AsTypeDescription, TypeDescription};

/// How to configure a kubernetes cluster
///
/// # Example
///
/// ```toml
/// [[cluster]]
/// api_version = "25"
/// ```
///
/// As you can see, this is rendered __with__ _full_ support.
#[derive(Debug, TypeDescription)]
pub struct KubeConfig {
    /// The API version of the kubernetes cluster
    pub api_version: String,
    pub clusters: Vec<Cluster>,
    pub contexts: Vec<Context>,
    pub current_context: String,
    pub kind: String,
    pub users: Vec<User>,
}

#[derive(Debug, TypeDescription)]
pub struct Cluster {
    pub name: String,
    pub cluster: ClusterDetail,
}

#[derive(Debug, TypeDescription)]
pub struct ClusterDetail {
    pub insecure_skip_tls_verify: Option<bool>,
    pub certificate_authority: Option<String>,
    pub certificate_authority_data: Option<String>,
    pub server: String,
}

#[derive(Debug, TypeDescription)]
pub struct User {
    pub name: String,
    pub user: UserDetail,
}

#[derive(Debug, TypeDescription)]
pub struct UserDetail {
    pub auth_provider: Option<AuthProviderDetail>,
    pub client_certificate: Option<String>,
    pub client_key: Option<String>,
    pub client_certificate_data: Option<String>,
    pub client_key_data: Option<String>,
    pub exec: Option<Exec>,
    pub token: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, TypeDescription)]
pub struct Exec {
    pub api_version: String,
    pub args: Vec<String>,
    pub command: String,
}

#[derive(Debug, TypeDescription)]
#[description(tag = "name")]
pub enum AuthProviderDetail {
    Gcp(GcpAuthProviderConfig),

    Other,
}

#[derive(Debug, TypeDescription)]
pub struct GcpAuthProviderConfig {
    pub access_token: Option<String>,
    pub cmd_args: String,
    pub cmd_path: String,
    pub expiry: Option<String>,
    pub expiry_key: String,
    pub token_key: String,
}

#[derive(Debug, TypeDescription)]
pub struct Context {
    pub name: String,
    pub context: ContextDetail,
}

#[derive(Debug, TypeDescription)]
pub struct ContextDetail {
    pub cluster: String,
    pub user: String,
    pub namespace: Option<String>,
}

#[test]
fn check_complex_type_desc() {
    let desc = KubeConfig::as_type_description();

    println!(
        "{}",
        serde_json::to_string(&desc).expect("SHould be able to serialize to json")
    );
}
