//
//   This Source Code Form is subject to the terms of the Mozilla Public
//   License, v. 2.0. If a copy of the MPL was not distributed with this
//   file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

#![allow(dead_code)]

use type_description::{AsTypeDescription, TypeDescription};

#[derive(Debug, TypeDescription)]

struct Window {
    open: bool,
}

#[derive(Debug, TypeDescription)]
#[description(untagged)]
enum Door {
    Wooden,
    Steel,
    Plastic,
}

#[derive(Debug, TypeDescription)]
struct Ac {
    on: bool,
    temperature: f32,
}

#[derive(Debug, TypeDescription)]
struct House {
    windows: Vec<Window>,
    doors: Vec<Door>,
    ac: Ac,
    tropical: bool,
}

#[test]
fn check_type_description() {
    let desc = House::as_type_description();

    println!("{:#?}", desc);
}
