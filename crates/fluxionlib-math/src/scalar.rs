//! Scalar mathematical operations.

/// Restricts `x` to the inclusive range between `lower` and `upper`.
///
/// Assumes that `lower` is not greater than `upper`.
pub const fn clamp(x: f64, lower: f64, upper: f64) -> f64 {
    if x < lower {
        lower
    } else if x > upper {
        upper
    } else {
        x
    }
}

/// Restricts `x` to the inclusive range from `0.0` to `1.0`.
pub const fn clamp01(x: f64) -> f64 {
    clamp(x, 0.0, 1.0)
}

/// Restricts `x` to the inclusive range from `-1.0` to `1.0`.
pub const fn clamp_unit(x: f64) -> f64 {
    clamp(x, -1.0, 1.0)
}

/// Restricts `x` to the range defined by `a` and `b`, regardless of their order.
pub const fn clamp_safe(x: f64, a: f64, b: f64) -> f64 {
    let (lower, upper) = if a <= b { (a, b) } else { (b, a) };
    clamp(x, lower, upper)
}
/// Maps the magnitude of `x` into the range from `0.0` toward `1.0`.
pub fn exp_map(x: f64, p: f64) -> f64 {
    let magnitude = x.abs().powf(p);
    1.0 - (-magnitude).exp()
}
/// Linearly interpolates between `a` and `b` using the parameter `t`.
///
/// A value of `0.0` returns `a`, while `1.0` returns `b`.
pub const fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + t * (b - a)
}

/// Evaluates a quadratic Bezier curve with three control values.
pub const fn bezier3(t: f64, y1: f64, y2: f64, y3: f64) -> f64 {
    let y12 = lerp(y1, y2, t);
    let y23 = lerp(y2, y3, t);

    lerp(y12, y23, t)
}

/// Evaluates a cubic Bezier curve with four control values.
pub const fn bezier4(t: f64, y1: f64, y2: f64, y3: f64, y4: f64) -> f64 {
    let y12 = lerp(y1, y2, t);
    let y23 = lerp(y2, y3, t);
    let y34 = lerp(y3, y4, t);

    let y123 = lerp(y12, y23, t);
    let y234 = lerp(y23, y34, t);

    lerp(y123, y234, t)
}

/// Returns `1.0` for positive values, `-1.0` for negative values, and `0.0` for zero.
pub const fn sign(x: f64) -> f64 {
    if x > 0.0 {
        1.0
    } else if x < 0.0 {
        -1.0
    } else {
        0.0
    }
}

/// Rounds `x` to the nearest integer, with halfway values rounded toward positive infinity.
pub fn round(x: f64) -> f64 {
    (x + 0.5).floor()
}

/// Returns `1.0` for positive values and `-1.0` otherwise.
const fn sign_nonzero(x: f64) -> f64 {
    if x > 0.0 { 1.0 } else { -1.0 }
}

/// Maps the magnitude of `x` exponentially while preserving its sign.
pub fn exp_map_signed(x: f64, p: f64) -> f64 {
    sign_nonzero(x) * exp_map(x, p)
}

/// Maps the magnitude of `x` exponentially using a fixed exponent of `1.0`.
pub fn exp_map1(x: f64) -> f64 {
    1.0 - (-x.abs()).exp()
}

/// Maps the magnitude of `x` exponentially using a fixed exponent of `1.0`.
pub fn exp_map1_signed(x: f64) -> f64 {
    sign_nonzero(x) * exp_map1(x)
}

/// Maps the magnitude of `x` exponentially using a fixed exponent of `2.0`.
pub fn exp_map2(x: f64) -> f64 {
    1.0 - (-(x * x)).exp()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamps_to_ordered_range() {
        assert_eq!(clamp(5.0, 0.0, 10.0), 5.0);
        assert_eq!(clamp(-5.0, 0.0, 10.0), 0.0);
        assert_eq!(clamp(15.0, 0.0, 10.0), 10.0);
    }

    #[test]
    fn clamps_to_zero_one_range() {
        assert_eq!(clamp01(-0.5), 0.0);
        assert_eq!(clamp01(0.25), 0.25);
        assert_eq!(clamp01(1.5), 1.0);
    }

    #[test]
    fn clamps_to_unit_range() {
        assert_eq!(clamp_unit(-2.0), -1.0);
        assert_eq!(clamp_unit(0.5), 0.5);
        assert_eq!(clamp_unit(2.0), 1.0);
    }

    #[test]
    fn accepts_reversed_bounds() {
        assert_eq!(clamp_safe(5.0, 10.0, 0.0), 5.0);
        assert_eq!(clamp_safe(-5.0, 10.0, 0.0), 0.0);
        assert_eq!(clamp_safe(15.0, 10.0, 0.0), 10.0);
    }
    #[test]
    fn interpolates_linearly() {
        assert_eq!(lerp(0.0, 10.0, 0.0), 0.0);
        assert_eq!(lerp(0.0, 10.0, 0.5), 5.0);
        assert_eq!(lerp(0.0, 10.0, 1.0), 10.0);
        assert_eq!(lerp(0.0, 10.0, 1.5), 15.0);
    }

    #[test]
    fn evaluates_quadratic_bezier() {
        assert_eq!(bezier3(0.0, 0.0, 10.0, 0.0), 0.0);
        assert_eq!(bezier3(0.5, 0.0, 10.0, 0.0), 5.0);
        assert_eq!(bezier3(1.0, 0.0, 10.0, 0.0), 0.0);
    }

    #[test]
    fn evaluates_cubic_bezier() {
        assert_eq!(bezier4(0.0, 0.0, 0.0, 1.0, 1.0), 0.0);
        assert_eq!(bezier4(0.5, 0.0, 0.0, 1.0, 1.0), 0.5);
        assert_eq!(bezier4(1.0, 0.0, 0.0, 1.0, 1.0), 1.0);
    }
    #[test]
    fn returns_numeric_sign() {
        assert_eq!(sign(5.0), 1.0);
        assert_eq!(sign(-5.0), -1.0);
        assert_eq!(sign(0.0), 0.0);
        assert_eq!(sign(f64::NAN), 0.0);
    }

    #[test]
    fn rounds_halfway_values_toward_positive_infinity() {
        assert_eq!(round(1.4), 1.0);
        assert_eq!(round(1.5), 2.0);
        assert_eq!(round(-1.4), -1.0);
        assert_eq!(round(-1.5), -1.0);
    }

    #[test]
    fn maps_magnitude_exponentially() {
        assert_eq!(exp_map(0.0, 2.0), 0.0);

        let expected = 1.0 - 1.0 / std::f64::consts::E;
        assert!((exp_map(1.0, 2.0) - expected).abs() < 1e-12);

        assert_eq!(exp_map(-1.0, 2.0), exp_map(1.0, 2.0));
    }

    #[test]
    fn maps_magnitude_exponentially_with_sign() {
        assert_eq!(exp_map_signed(0.0, 2.0), 0.0);
        assert!(exp_map_signed(1.0, 2.0) > 0.0);
        assert!(exp_map_signed(-1.0, 2.0) < 0.0);
        assert_eq!(exp_map_signed(-1.0, 2.0), -exp_map_signed(1.0, 2.0));
    }

    #[test]
    fn maps_magnitude_with_linear_exponent() {
        assert_eq!(exp_map1(0.0), 0.0);
        assert_eq!(exp_map1(-1.0), exp_map1(1.0));
        assert!((exp_map1(1.0) - exp_map(1.0, 1.0)).abs() < 1e-12);
    }

    #[test]
    fn maps_with_linear_exponent_and_preserves_sign() {
        assert_eq!(exp_map1_signed(0.0), 0.0);
        assert!(exp_map1_signed(1.0) > 0.0);
        assert!(exp_map1_signed(-1.0) < 0.0);
        assert_eq!(exp_map1_signed(-1.0), -exp_map1_signed(1.0));
    }

    #[test]
    fn maps_magnitude_with_squared_exponent() {
        assert_eq!(exp_map2(0.0), 0.0);
        assert_eq!(exp_map2(-1.0), exp_map2(1.0));
        assert!((exp_map2(1.0) - exp_map(1.0, 2.0)).abs() < 1e-12);
    }
}
