use std::f64::consts::PI;
use crate::core::EasingType;

/// Apply an easing function to a progress value (0-1)
pub fn apply_easing(progress: f64, easing: EasingType) -> f64 {
    let t = progress.clamp(0.0, 1.0);

    match easing {
        EasingType::Linear => t,
        EasingType::EaseInQuad => t * t,
        EasingType::EaseOutQuad => 1.0 - (1.0 - t) * (1.0 - t),
        EasingType::EaseInOutQuad => {
            if t < 0.5 {
                2.0 * t * t
            } else {
                1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
            }
        }
        EasingType::EaseInCubic => t * t * t,
        EasingType::EaseOutCubic => 1.0 - (1.0 - t).powi(3),
        EasingType::EaseInOutCubic => {
            if t < 0.5 {
                4.0 * t * t * t
            } else {
                1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
            }
        }
        EasingType::EaseInQuart => t * t * t * t,
        EasingType::EaseOutQuart => 1.0 - (1.0 - t).powi(4),
        EasingType::EaseInOutQuart => {
            if t < 0.5 {
                8.0 * t * t * t * t
            } else {
                1.0 - (-2.0 * t + 2.0).powi(4) / 2.0
            }
        }
        EasingType::EaseInQuint => t * t * t * t * t,
        EasingType::EaseOutQuint => 1.0 - (1.0 - t).powi(5),
        EasingType::EaseInOutQuint => {
            if t < 0.5 {
                16.0 * t * t * t * t * t
            } else {
                1.0 - (-2.0 * t + 2.0).powi(5) / 2.0
            }
        }
        EasingType::EaseInSine => 1.0 - (t * PI / 2.0).cos(),
        EasingType::EaseOutSine => (t * PI / 2.0).sin(),
        EasingType::EaseInOutSine => -((PI * t).cos() - 1.0) / 2.0,
        EasingType::EaseInExpo => {
            if t == 0.0 {
                0.0
            } else {
                2.0_f64.powf(10.0 * t - 10.0)
            }
        }
        EasingType::EaseOutExpo => {
            if t == 1.0 {
                1.0
            } else {
                1.0 - 2.0_f64.powf(-10.0 * t)
            }
        }
        EasingType::EaseInOutExpo => {
            if t == 0.0 {
                0.0
            } else if t == 1.0 {
                1.0
            } else if t < 0.5 {
                2.0_f64.powf(20.0 * t - 10.0) / 2.0
            } else {
                (2.0 - 2.0_f64.powf(-20.0 * t + 10.0)) / 2.0
            }
        }
        EasingType::EaseInCirc => 1.0 - (1.0 - t * t).sqrt(),
        EasingType::EaseOutCirc => (1.0 - (t - 1.0).powi(2)).sqrt(),
        EasingType::EaseInOutCirc => {
            if t < 0.5 {
                (1.0 - (1.0 - (2.0 * t).powi(2)).sqrt()) / 2.0
            } else {
                ((1.0 - (-2.0 * t + 2.0).powi(2)).sqrt() + 1.0) / 2.0
            }
        }
        EasingType::EaseInBack => {
            let c1 = 1.70158;
            let c3 = c1 + 1.0;
            c3 * t * t * t - c1 * t * t
        }
        EasingType::EaseOutBack => {
            let c1 = 1.70158;
            let c3 = c1 + 1.0;
            1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
        }
        EasingType::EaseInOutBack => {
            let c1 = 1.70158;
            let c2 = c1 * 1.525;
            if t < 0.5 {
                ((2.0 * t).powi(2) * ((c2 + 1.0) * 2.0 * t - c2)) / 2.0
            } else {
                ((2.0 * t - 2.0).powi(2) * ((c2 + 1.0) * (t * 2.0 - 2.0) + c2) + 2.0) / 2.0
            }
        }
        EasingType::EaseInElastic => {
            let c4 = (2.0 * PI) / 3.0;
            if t == 0.0 {
                0.0
            } else if t == 1.0 {
                1.0
            } else {
                -2.0_f64.powf(10.0 * t - 10.0) * ((t * 10.0 - 10.75) * c4).sin()
            }
        }
        EasingType::EaseOutElastic => {
            let c4 = (2.0 * PI) / 3.0;
            if t == 0.0 {
                0.0
            } else if t == 1.0 {
                1.0
            } else {
                2.0_f64.powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0
            }
        }
        EasingType::EaseInOutElastic => {
            let c5 = (2.0 * PI) / 4.5;
            if t == 0.0 {
                0.0
            } else if t == 1.0 {
                1.0
            } else if t < 0.5 {
                -(2.0_f64.powf(20.0 * t - 10.0) * ((20.0 * t - 11.125) * c5).sin()) / 2.0
            } else {
                (2.0_f64.powf(-20.0 * t + 10.0) * ((20.0 * t - 11.125) * c5).sin()) / 2.0 + 1.0
            }
        }
        EasingType::EaseInBounce => 1.0 - ease_out_bounce(1.0 - t),
        EasingType::EaseOutBounce => ease_out_bounce(t),
        EasingType::EaseInOutBounce => {
            if t < 0.5 {
                (1.0 - ease_out_bounce(1.0 - 2.0 * t)) / 2.0
            } else {
                (1.0 + ease_out_bounce(2.0 * t - 1.0)) / 2.0
            }
        }
    }
}

/// Helper for bounce easing
fn ease_out_bounce(t: f64) -> f64 {
    let n1 = 7.5625;
    let d1 = 2.75;

    if t < 1.0 / d1 {
        n1 * t * t
    } else if t < 2.0 / d1 {
        let t = t - 1.5 / d1;
        n1 * t * t + 0.75
    } else if t < 2.5 / d1 {
        let t = t - 2.25 / d1;
        n1 * t * t + 0.9375
    } else {
        let t = t - 2.625 / d1;
        n1 * t * t + 0.984375
    }
}

/// Interpolate between two values with easing
pub fn interpolate(from: f64, to: f64, progress: f64, easing: EasingType) -> f64 {
    let t = apply_easing(progress, easing);
    from + (to - from) * t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_easing() {
        assert_eq!(apply_easing(0.0, EasingType::Linear), 0.0);
        assert_eq!(apply_easing(0.5, EasingType::Linear), 0.5);
        assert_eq!(apply_easing(1.0, EasingType::Linear), 1.0);
    }

    #[test]
    fn test_ease_out_quad() {
        assert_eq!(apply_easing(0.0, EasingType::EaseOutQuad), 0.0);
        assert_eq!(apply_easing(1.0, EasingType::EaseOutQuad), 1.0);
        // EaseOut should be faster at start
        assert!(apply_easing(0.5, EasingType::EaseOutQuad) > 0.5);
    }

    #[test]
    fn test_ease_in_quad() {
        assert_eq!(apply_easing(0.0, EasingType::EaseInQuad), 0.0);
        assert_eq!(apply_easing(1.0, EasingType::EaseInQuad), 1.0);
        // EaseIn should be slower at start
        assert!(apply_easing(0.5, EasingType::EaseInQuad) < 0.5);
    }

    #[test]
    fn test_interpolate() {
        assert_eq!(interpolate(0.0, 100.0, 0.0, EasingType::Linear), 0.0);
        assert_eq!(interpolate(0.0, 100.0, 0.5, EasingType::Linear), 50.0);
        assert_eq!(interpolate(0.0, 100.0, 1.0, EasingType::Linear), 100.0);
    }
}
