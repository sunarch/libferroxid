// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// dependencies
use byteorder::{ByteOrder, LittleEndian};


pub fn u16_as_little_endian_array(value: u16) -> [u8; 2] {
    let mut buffer: [u8; 2] = [0; 2];
    LittleEndian::write_u16(&mut buffer, value);

    return buffer;
}
