//! Utilities for working with bit masks and bitwise operations.

/// Keeps only the bits that are set in both `x` and `y`.
///
/// Example: `1100 & 1010 = 1000`
pub const fn and_u32(x: u32, y: u32) -> u32 {
    x & y
}

/// Sets every bit that is set in either `x` or `y`.
///
/// Example: `1100 | 1010 = 1110`
pub const fn or_u32(x: u32, y: u32) -> u32 {
    x | y
}

/// Sets the bits that are set in exactly one of `x` and `y`.
///
/// Example: `1100 ^ 1010 = 0110`
pub const fn xor_u32(x: u32, y: u32) -> u32 {
    x ^ y
}

/// Returns whether `x` contains every bit set in the `y` mask.
///
/// Example: `1110` contains all bits set in `0110`.
pub const fn has_u32(x: u32, y: u32) -> bool {
    (x & y) == y
}

/// Keeps only the bits that are set in both `x` and `y`.
pub const fn and_u64(x: u64, y: u64) -> u64 {
    x & y
}

/// Sets every bit that is set in either `x` or `y`.
pub const fn or_u64(x: u64, y: u64) -> u64 {
    x | y
}

/// Sets the bits that are set in exactly one of `x` and `y`.
pub const fn xor_u64(x: u64, y: u64) -> u64 {
    x ^ y
}

/// Returns whether `x` contains every bit set in the `y` mask.
pub const fn has_u64(x: u64, y: u64) -> bool {
    (x & y) == y
}

/// Rotates the bits of `x` to the left by `n` positions.
pub const fn rotate_left_u32(x: u32, n: u32) -> u32 {
    x.rotate_left(n)
}

/// Rotates the bits of `x` to the right by `n` positions.
pub const fn rotate_right_u32(x: u32, n: u32) -> u32 {
    x.rotate_right(n)
}

/// Rotates the bits of `x` to the left by `n` positions.
pub const fn rotate_left_u64(x: u64, n: u32) -> u64 {
    x.rotate_left(n)
}

/// Rotates the bits of `x` to the right by `n` positions.
pub const fn rotate_right_u64(x: u64, n: u32) -> u64 {
    x.rotate_right(n)
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combines_32_bit_values() {
        assert_eq!(and_u32(0b1100, 0b1010), 0b1000);
        assert_eq!(or_u32(0b1100, 0b1010), 0b1110);
        assert_eq!(xor_u32(0b1100, 0b1010), 0b0110);
    }

    #[test]
    fn detects_requested_bits() {
        assert!(has_u32(0b1110, 0b0110));
        assert!(!has_u32(0b1000, 0b0010));
        assert!(has_u32(0b1000, 0));
    }

    #[test]
    fn combines_64_bit_values() {
        let x = 0x8000_0000_0000_000C;
        let y = 0x8000_0000_0000_000A;

        assert_eq!(and_u64(x, y), 0x8000_0000_0000_0008);
        assert_eq!(or_u64(x, y), 0x8000_0000_0000_000E);
        assert_eq!(xor_u64(x, y), 0x0000_0000_0000_0006);
    }

    #[test]
    fn detects_requested_64_bit_mask() {
        assert!(has_u64(0xF000_0000_0000_000E, 0x3000_0000_0000_0006,));

        assert!(!has_u64(0x8000_0000_0000_0000, 0x4000_0000_0000_0000,));
    }

    #[test]
    fn rotates_32_bit_values() {
        let x = 0x8000_0001;

        assert_eq!(rotate_left_u32(x, 1), 0x0000_0003);
        assert_eq!(rotate_right_u32(x, 1), 0xC000_0000);
        assert_eq!(rotate_left_u32(x, 0), x);
        assert_eq!(rotate_left_u32(x, 32), x);
    }

    #[test]
    fn rotates_64_bit_values() {
        let x = 0x8000_0000_0000_0001;

        assert_eq!(rotate_left_u64(x, 1), 0x0000_0000_0000_0003);
        assert_eq!(rotate_right_u64(x, 1), 0xC000_0000_0000_0000);
        assert_eq!(rotate_right_u64(x, 0), x);
        assert_eq!(rotate_right_u64(x, 64), x);
    }
}
