//! C-compatible scalar mathematics API.

use fluxionlib::math::scalar;

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

/// Rounds `x` to the nearest integer, with halfway values rounded toward positive infinity.
#[unsafe(no_mangle)]
pub extern "C" fn Math_Round(x: f64) -> f64 {
    scalar::round(x)
}

/// Maps the magnitude of `x` into the range from `0.0` toward `1.0`.
#[unsafe(no_mangle)]
pub extern "C" fn Math_ExpMap(x: f64, p: f64) -> f64 {
    scalar::exp_map(x, p)
}

/// Maps the magnitude of `x` exponentially while preserving its sign.
#[unsafe(no_mangle)]
pub extern "C" fn Math_ExpMapSigned(x: f64, p: f64) -> f64 {
    scalar::exp_map_signed(x, p)
}

/// Maps the magnitude of `x` exponentially using a fixed exponent of `1.0`.
#[unsafe(no_mangle)]
pub extern "C" fn Math_ExpMap1(x: f64) -> f64 {
    scalar::exp_map1(x)
}

/// Maps `x` exponentially using a fixed exponent of `1.0` while preserving its sign.
#[unsafe(no_mangle)]
pub extern "C" fn Math_ExpMap1Signed(x: f64) -> f64 {
    scalar::exp_map1_signed(x)
}

/// Maps the magnitude of `x` exponentially using a fixed exponent of `2.0`.
#[unsafe(no_mangle)]
pub extern "C" fn Math_ExpMap2(x: f64) -> f64 {
    scalar::exp_map2(x)
}

/// Maps `x` exponentially using a fixed exponent of `2.0` while preserving its sign.
#[unsafe(no_mangle)]
pub extern "C" fn Math_ExpMap2Signed(x: f64) -> f64 {
    scalar::exp_map2_signed(x)
}

/// Raises the magnitude of `x` to `p` while preserving the sign of `x`.
#[unsafe(no_mangle)]
pub extern "C" fn Math_PowSigned(x: f64, p: f64) -> f64 {
    scalar::pow_signed(x, p)
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

    #[test]
    fn forwards_general_exponential_mapping_operations() {
        assert_eq!(Math_ExpMap(1.0, 2.0), scalar::exp_map(1.0, 2.0));
        assert_eq!(
            Math_ExpMapSigned(-1.0, 2.0),
            scalar::exp_map_signed(-1.0, 2.0)
        );
    }

    #[test]
    fn forwards_specialized_exponential_mapping_operations() {
        assert_eq!(Math_ExpMap1(1.0), scalar::exp_map1(1.0));
        assert_eq!(Math_ExpMap1Signed(-1.0), scalar::exp_map1_signed(-1.0));

        assert_eq!(Math_ExpMap2(1.0), scalar::exp_map2(1.0));
        assert_eq!(Math_ExpMap2Signed(-1.0), scalar::exp_map2_signed(-1.0));

        assert_eq!(Math_PowSigned(-4.0, 0.5), -2.0);
    }
}
