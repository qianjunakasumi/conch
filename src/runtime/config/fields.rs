////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-2023 qianjunakasumi <i@qianjunakasumi.ren>               /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use {
    serde::Deserialize,
    std::fmt::{Display, Formatter, Result},
};

#[derive(Debug, Deserialize)]
pub enum License {
    #[serde(rename = "AGPL")] Agpl,
    #[serde(rename = "MPL")] Mpl,
    Apache,
}

impl Display for License {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            License::Agpl => write!(f, "AGPL"),
            License::Mpl => write!(f, "MPL"),
            License::Apache => write!(f, "Apache"),
        }
    }
}
