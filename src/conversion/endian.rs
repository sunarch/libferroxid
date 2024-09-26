// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub const fn u16_low_byte(value: u16) -> u8 {
    value as u8
}

pub const fn u16_high_byte(value: u16) -> u8 {
    (value >> 8) as u8
}

pub fn u16_to_little_endian_array(value: u16) -> [u8; 2] {
    [u16_low_byte(value), u16_high_byte(value)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn little_endian_is_correct() {
        let value: u16 = 0x1234;
        let result: [u8; 2] = u16_to_little_endian_array(value);
        assert_eq!(result, [0x34, 0x12]);
    }

    #[test]
    fn u16_low_byte_is_correct() {
        let value: u16 = 0x1234;
        let result: u8 = u16_low_byte(value);
        assert_eq!(result, 0x34);
    }

    #[test]
    fn u16_low_byte_is_const_fn() {
        const VALUE: u16 = 0x1234;
        const RESULT: u8 = u16_low_byte(VALUE);
        assert_eq!(RESULT, 0x34);
    }

    #[test]
    fn u16_high_byte_is_correct() {
        let value: u16 = 0x1234;
        let result: u8 = u16_high_byte(value);
        assert_eq!(result, 0x12);
    }

    #[test]
    fn u16_high_byte_is_const_fn() {
        const VALUE: u16 = 0x1234;
        const RESULT: u8 = u16_high_byte(VALUE);
        assert_eq!(RESULT, 0x12);
    }
}
