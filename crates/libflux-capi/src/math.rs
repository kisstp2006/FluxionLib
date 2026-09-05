//! C-compatible scalar mathematics API.

use libflux::math::scalar;

/// Restricts `x` to the inclusive range between `a` and `b`.
#[unsafe(no_mangle)]
pub extern "C" fn Math_Clamp(x: f64, a: f64, b: f64) -> f64 {
    scalar::clamp(x, a, b)
}

/// Restricts `x` to the inclusive range from `0.0` to `1.0`.
#[unsafe(no_mangle)]
pub extern "C" fn Math_Clamp01(x: f64) -> f64 {
    scalar::clamp01(x)
}

/// Restricts `x` to the range defined by `a` and `b`, regardless of their order.
#[unsafe(no_mangle)]
pub extern "C" fn Math_ClampSafe(x: f64, a: f64, b: f64) -> f64 {
    scalar::clamp_safe(x, a, b)
}

/// Restricts `x` to the inclusive range from `-1.0` to `1.0`.
#[unsafe(no_mangle)]
pub extern "C" fn Math_ClampUnit(x: f64) -> f64 {
    scalar::clamp_unit(x)
}

/// Evaluates a quadratic Bezier curve with three control values.
#[unsafe(no_mangle)]
pub extern "C" fn Math_Bezier3(x: f64, y1: f64, y2: f64, y3: f64) -> f64 {
    scalar::bezier3(x, y1, y2, y3)
}

/// Evaluates a cubic Bezier curve with four control values.
#[unsafe(no_mangle)]
pub extern "C" fn Math_Bezier4(x: f64, y1: f64, y2: f64, y3: f64, y4: f64) -> f64 {
    scalar::bezier4(x, y1, y2, y3, y4)
}

///Returns the numeric sign of `x`
#[unsafe(no_mangle)]
pub extern "C" fn Math_Sign(x: f64) -> f64 {
    scalar::sign(x)
}

/// Rounds `x` using the original LibPHX rounding behavior.
#[unsafe(no_mangle)]
pub extern "C" fn Math_Round(x: f64) -> f64 {
    scalar::round(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwards_clamp_operations() {
        assert_eq!(Math_Clamp(5.0, 0.0, 10.0), 5.0);
        assert_eq!(Math_Clamp01(2.0), 1.0);
        assert_eq!(Math_ClampSafe(5.0, 10.0, 0.0), 5.0);
        assert_eq!(Math_ClampUnit(-2.0), -1.0);
    }
    #[test]
    fn forwards_bezier_operations() {
        assert_eq!(Math_Bezier3(0.5, 0.0, 10.0, 0.0), 5.0);
        assert_eq!(Math_Bezier4(0.5, 0.0, 0.0, 1.0, 1.0), 0.5);
    }

    #[test]
    fn forwards_sign_and_round_operations() {
        assert_eq!(Math_Sign(-5.0), -1.0);
        assert_eq!(Math_Sign(0.0), 0.0);
        assert_eq!(Math_Sign(5.0), 1.0);

        assert_eq!(Math_Round(1.5), 2.0);
        assert_eq!(Math_Round(-1.5), -1.0);
    }
}
