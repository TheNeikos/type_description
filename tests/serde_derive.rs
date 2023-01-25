//
//   This Source Code Form is subject to the terms of the Mozilla Public
//   License, v. 2.0. If a copy of the MPL was not distributed with this
//   file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use type_description::{AsTypeDescription, TypeDescription};

#[derive(Debug, TypeDescription, Serialize, Deserialize)]
struct Window {
    open: bool,
}

#[derive(Debug, TypeDescription, Serialize, Deserialize)]
#[description(untagged)]
enum Door {
    Wooden,
    Steel,
    Plastic,
}

#[derive(Debug, TypeDescription, Serialize, Deserialize)]
struct Ac {
    on: bool,
    temperature: f32,
}

#[derive(Debug, TypeDescription, Serialize, Deserialize)]
struct Cellar {
    cellar_temp: f32,
    cellar_humidity: f32,
    #[serde(skip)]
    cellar_light: bool,
}

#[derive(Debug, TypeDescription, Serialize, Deserialize)]
#[description(use_serde)]
struct House {
    #[serde(default)]
    windows: Vec<Window>,
    #[serde(rename = "doorios", default)]
    doors: Vec<Door>,
    #[serde(rename = "climate_control")]
    ac: Ac,
    #[serde(alias = "hawaii", alias = "trop")]
    tropical: bool,
    #[serde(skip)]
    unneeded: bool,
    #[serde(skip_deserializing)]
    unneeded_2: bool,
    #[serde(flatten)]
    cellar: Cellar,
}

#[test]
fn check_type_description() {
    let desc = House::as_type_description();

    let kind = desc.kind();
    match kind {
        type_description::TypeKind::Struct(fields) => {
            assert_eq!(fields[2].name(), "climate_control");

            assert_eq!(fields[0].optional(), true);
            assert_eq!(fields[1].optional(), true);
            assert_eq!(fields[2].optional(), false);

            assert_eq!(7, fields.len());
        }
        _ => panic!("Should be a struct"),
    }

    println!("{:#?}", desc);
}
