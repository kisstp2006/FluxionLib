//! C-compatible interface for FluxionLib.

#![allow(non_snake_case)]

mod math;

use fluxionlib::math::bit;

/// Performs a bitwise AND operation on two 32-bit unsigned integers.
#[unsafe(no_mangle)]
pub extern "C" fn Bit_And32(x: u32, y: u32) -> u32 {
    bit::and_u32(x, y)
}

/// Performs a bitwise OR operation on two 32-bit unsigned integers.
#[unsafe(no_mangle)]
pub extern "C" fn Bit_Or32(x: u32, y: u32) -> u32 {
    bit::or_u32(x, y)
}

/// Performs a bitwise XOR operation on two 32-bit unsigned integers.
#[unsafe(no_mangle)]
pub extern "C" fn Bit_Xor32(x: u32, y: u32) -> u32 {
    bit::xor_u32(x, y)
}

/// Returns whether `x` contains every bit set in the `y` mask.
#[unsafe(no_mangle)]
pub extern "C" fn Bit_Has32(x: u32, y: u32) -> bool {
    bit::has_u32(x, y)
}

/// Performs a bitwise AND operation on two 64-bit unsigned integers.
#[unsafe(no_mangle)]
pub extern "C" fn Bit_And64(x: u64, y: u64) -> u64 {
    bit::and_u64(x, y)
}

/// Performs a bitwise OR operation on two 64-bit unsigned integers.
#[unsafe(no_mangle)]
pub extern "C" fn Bit_Or64(x: u64, y: u64) -> u64 {
    bit::or_u64(x, y)
}

/// Performs a bitwise XOR operation on two 64-bit unsigned integers.
#[unsafe(no_mangle)]
pub extern "C" fn Bit_Xor64(x: u64, y: u64) -> u64 {
    bit::xor_u64(x, y)
}

/// Returns whether `x` contains every bit set in the `y` mask.
#[unsafe(no_mangle)]
pub extern "C" fn Bit_Has64(x: u64, y: u64) -> bool {
    bit::has_u64(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwards_all_bit_operations() {
        assert_eq!(Bit_And32(0b1100, 0b1010), 0b1000);
        assert_eq!(Bit_Or32(0b1100, 0b1010), 0b1110);
        assert_eq!(Bit_Xor32(0b1100, 0b1010), 0b0110);
        assert!(Bit_Has32(0b1110, 0b0110));

        assert_eq!(
            Bit_And64(0x8000_0000_0000_000C, 0x8000_0000_0000_000A),
            0x8000_0000_0000_0008,
        );
        assert_eq!(
            Bit_Or64(0x8000_0000_0000_000C, 0x8000_0000_0000_000A),
            0x8000_0000_0000_000E,
        );
        assert_eq!(
            Bit_Xor64(0x8000_0000_0000_000C, 0x8000_0000_0000_000A),
            0x0000_0000_0000_0006,
        );
        assert!(Bit_Has64(0xF000_0000_0000_000E, 0x3000_0000_0000_0006,));
    }
}
