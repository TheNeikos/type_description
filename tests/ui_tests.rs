//
//   This Source Code Form is subject to the terms of the Mozilla Public
//   License, v. 2.0. If a copy of the MPL was not distributed with this
//   file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

#![allow(dead_code)]

#[test]
fn check_type_description() {
    let tests = trybuild::TestCases::new();

    tests.compile_fail("tests/ui/fail/*.rs");
}
