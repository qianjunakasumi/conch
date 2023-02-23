////////////////////////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-present qianjunakasumi <i@qianjunakasumi.ren>                                /
//                            project-senjuko/conch Contributors                                   /
//                                                                                                 /
//           https://github.com/qianjunakasumi                                                     /
//           https://github.com/project-senjuko/conch/graphs/contributors                          /
//                                                                                                 /
//   This Source Code Form is subject to the terms of the Mozilla Public                           /
//   License, v. 2.0. If a copy of the MPL was not distributed with this                           /
//   file, You can obtain one at http://mozilla.org/MPL/2.0/.                                      /
////////////////////////////////////////////////////////////////////////////////////////////////////


// 0x144
//    128  16e
//  0x147, 0x154, 0x141
// 0x187 ,0x188, 0x191, 0x177
// 0x516, 0x521, 0x525, 0x544,0x545, 0x548, 0x542


use bytes::{BufMut, BytesMut};

pub mod t1;
pub mod t8;
pub mod t18;
pub mod t100;
pub mod t106;
pub mod t107;
pub mod t109;
pub mod t116;
pub mod t124;
pub mod t142;
pub mod t145;
pub mod t187;
pub mod t188;
pub mod t511;
pub mod t52d;

trait TlvField: Default {
    fn tag() -> u16;

    fn to_payload(&self, b: &mut BytesMut);

    fn to_bytes(&self) -> BytesMut {
        let mut b = BytesMut::with_capacity(4);

        b.put_u16(Self::tag());
        b.put_u16(0); // payload length
        self.to_payload(&mut b);

        let l = b.len() - 4;
        b[2..4].swap_with_slice(&mut l.to_be_bytes()); // set payload length

        b
    }
}
