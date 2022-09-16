//
//   This Source Code Form is subject to the terms of the Mozilla Public
//   License, v. 2.0. If a copy of the MPL was not distributed with this
//   file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
extern crate type_description;
extern crate serde_json;

use type_description::{AsTypeDescription, TypeDescription};

fn main() {


// ANCHOR: definition
#[derive(TypeDescription)]
struct MyConfiguration {
    /// The name of this configuration
    name: String,

    /// List of actions this config can take
    action: Vec<String>,
}
// ANCHOR_END: definition

assert_eq!(serde_json::to_string_pretty(&MyConfiguration::as_type_description()).unwrap(), r#"
// ANCHOR: test
{
  "name": "MyConfiguration",
  "kind": {
    "Struct": [
      {
        "name": "name",
        "doc": "The name of this configuration",
        "kind": {
          "name": "String",
          "kind": "String",
          "doc": "An UTF-8 string"
        }
      },
      {
        "name": "action",
        "doc": "List of actions this config can take",
        "kind": {
          "name": "Array of 'String's",
          "kind": {
            "Array": {
              "name": "String",
              "kind": "String",
              "doc": "An UTF-8 string"
            }
          },
          "doc": null
        }
      }
    ]
  },
  "doc": null
}
// ANCHOR_END: test
"#.trim())

}
