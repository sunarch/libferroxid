// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub const BIT_MASK_0: u8 = 0b00000001;
pub const BIT_MASK_1: u8 = 0b00000010;
pub const BIT_MASK_2: u8 = 0b00000100;
pub const BIT_MASK_3: u8 = 0b00001000;
pub const BIT_MASK_4: u8 = 0b00010000;
pub const BIT_MASK_5: u8 = 0b00100000;
pub const BIT_MASK_6: u8 = 0b01000000;
pub const BIT_MASK_7: u8 = 0b10000000;

const SHIFT_FOR_U16: u8 = 8;
pub const BIT_MASK_8: u16 = (BIT_MASK_0 as u16) << SHIFT_FOR_U16;
pub const BIT_MASK_9: u16 = (BIT_MASK_1 as u16) << SHIFT_FOR_U16;
pub const BIT_MASK_10: u16 = (BIT_MASK_2 as u16) << SHIFT_FOR_U16;
pub const BIT_MASK_11: u16 = (BIT_MASK_3 as u16) << SHIFT_FOR_U16;
pub const BIT_MASK_12: u16 = (BIT_MASK_4 as u16) << SHIFT_FOR_U16;
pub const BIT_MASK_13: u16 = (BIT_MASK_5 as u16) << SHIFT_FOR_U16;
pub const BIT_MASK_14: u16 = (BIT_MASK_6 as u16) << SHIFT_FOR_U16;
pub const BIT_MASK_15: u16 = (BIT_MASK_7 as u16) << SHIFT_FOR_U16;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u16_is_correct() {
        assert_eq!(BIT_MASK_15, 0b1000000000000000);
    }
}
