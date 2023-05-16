//
//   This Source Code Form is subject to the terms of the Mozilla Public
//   License, v. 2.0. If a copy of the MPL was not distributed with this
//   file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

use std::path::PathBuf;

#[derive(serde::Deserialize, Debug, type_description::TypeDescription)]
#[serde(tag = "type")]
#[description(tag = "type")]
pub enum ConnectConfig {
    UnsecuredHttp {
        addr: String,
        timeout: String,
    },

    Local {
        addr: String,
        timeout: String,
    },

    Socket {
        path: PathBuf,
        timeout: String,
    },

    Ssl {
        addr: String,
        ssl_key: PathBuf,
        ssl_cert: PathBuf,
        ssl_ca: PathBuf,
        timeout: String,
    },

    Unix {
        path: PathBuf,
        timeout: String,
    },
}
