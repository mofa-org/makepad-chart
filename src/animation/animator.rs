use crate::core::EasingType;
use super::easing::apply_easing;

/// Animation state for chart transitions
#[derive(Clone, Debug)]
pub struct ChartAnimator {
    /// Animation start time in seconds
    start_time: Option<f64>,
    /// Animation duration in seconds
    duration: f64,
    /// Easing function to use
    easing: EasingType,
    /// Current animation progress (0-1)
    progress: f64,
    /// Whether animation is complete
    completed: bool,
    /// Delay before animation starts (in seconds)
    delay: f64,
}

impl ChartAnimator {
    /// Create a new animator with the given duration in milliseconds
    pub fn new(duration_ms: f64) -> Self {
        Self {
            start_time: None,
            duration: duration_ms / 1000.0,
            easing: EasingType::EaseOutQuart,
            progress: 0.0,
            completed: false,
            delay: 0.0,
        }
    }

    /// Set the easing function
    pub fn with_easing(mut self, easing: EasingType) -> Self {
        self.easing = easing;
        self
    }

    /// Set the delay before animation starts (in milliseconds)
    pub fn with_delay(mut self, delay_ms: f64) -> Self {
        self.delay = delay_ms / 1000.0;
        self
    }

    /// Start the animation at the given time
    pub fn start(&mut self, current_time: f64) {
        self.start_time = Some(current_time);
        self.progress = 0.0;
        self.completed = false;
    }

    /// Reset the animation
    pub fn reset(&mut self) {
        self.start_time = None;
        self.progress = 0.0;
        self.completed = false;
    }

    /// Update the animation progress
    /// Returns true if animation is still running
    pub fn update(&mut self, current_time: f64) -> bool {
        if self.completed {
            return false;
        }

        let Some(start_time) = self.start_time else {
            return false;
        };

        let elapsed = current_time - start_time - self.delay;

        if elapsed < 0.0 {
            // Still in delay period
            self.progress = 0.0;
            return true;
        }

        if self.duration <= 0.0 {
            self.progress = 1.0;
            self.completed = true;
            return false;
        }

        let raw_progress = elapsed / self.duration;

        if raw_progress >= 1.0 {
            self.progress = 1.0;
            self.completed = true;
            return false;
        }

        self.progress = raw_progress;
        true
    }

    /// Get the current eased progress value (0-1)
    pub fn get_progress(&self) -> f64 {
        apply_easing(self.progress, self.easing)
    }

    /// Get the raw (uneased) progress value
    pub fn get_raw_progress(&self) -> f64 {
        self.progress
    }

    /// Check if animation is complete
    pub fn is_complete(&self) -> bool {
        self.completed
    }

    /// Check if animation is running
    pub fn is_running(&self) -> bool {
        self.start_time.is_some() && !self.completed
    }

    /// Interpolate between two values using current animation progress
    pub fn interpolate(&self, from: f64, to: f64) -> f64 {
        let t = self.get_progress();
        from + (to - from) * t
    }

    /// Skip to the end of the animation
    pub fn skip_to_end(&mut self) {
        self.progress = 1.0;
        self.completed = true;
    }
}

impl Default for ChartAnimator {
    fn default() -> Self {
        Self::new(400.0) // Default 400ms duration
    }
}

/// Manager for coordinating multiple animations
pub struct AnimationManager {
    animators: Vec<ChartAnimator>,
    stagger_delay: f64,
}

impl AnimationManager {
    /// Create a new animation manager
    pub fn new() -> Self {
        Self {
            animators: Vec::new(),
            stagger_delay: 0.0,
        }
    }

    /// Set the stagger delay between animations (in milliseconds)
    pub fn with_stagger(mut self, delay_ms: f64) -> Self {
        self.stagger_delay = delay_ms / 1000.0;
        self
    }

    /// Create animators for N items
    pub fn create_animators(&mut self, count: usize, base_duration_ms: f64, easing: EasingType) {
        self.animators.clear();
        for i in 0..count {
            let animator = ChartAnimator::new(base_duration_ms)
                .with_easing(easing)
                .with_delay(i as f64 * self.stagger_delay * 1000.0);
            self.animators.push(animator);
        }
    }

    /// Start all animations at the given time
    pub fn start_all(&mut self, current_time: f64) {
        for animator in &mut self.animators {
            animator.start(current_time);
        }
    }

    /// Update all animations
    /// Returns true if any animation is still running
    pub fn update_all(&mut self, current_time: f64) -> bool {
        let mut any_running = false;
        for animator in &mut self.animators {
            if animator.update(current_time) {
                any_running = true;
            }
        }
        any_running
    }

    /// Get the animator at index
    pub fn get(&self, index: usize) -> Option<&ChartAnimator> {
        self.animators.get(index)
    }

    /// Get the progress for an item
    pub fn get_progress(&self, index: usize) -> f64 {
        self.animators.get(index).map(|a| a.get_progress()).unwrap_or(1.0)
    }

    /// Check if all animations are complete
    pub fn is_complete(&self) -> bool {
        self.animators.iter().all(|a| a.is_complete())
    }

    /// Reset all animations
    pub fn reset(&mut self) {
        for animator in &mut self.animators {
            animator.reset();
        }
    }

    /// Skip all animations to end
    pub fn skip_to_end(&mut self) {
        for animator in &mut self.animators {
            animator.skip_to_end();
        }
    }
}

impl Default for AnimationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animator_basic() {
        let mut animator = ChartAnimator::new(1000.0); // 1 second
        animator.start(0.0);

        assert!(animator.is_running());
        assert!(!animator.is_complete());

        // At 500ms
        animator.update(0.5);
        assert!(animator.get_raw_progress() > 0.0);
        assert!(animator.get_raw_progress() < 1.0);

        // At 1500ms (past duration)
        animator.update(1.5);
        assert!(animator.is_complete());
        assert_eq!(animator.get_progress(), 1.0);
    }

    #[test]
    fn test_animator_with_delay() {
        let mut animator = ChartAnimator::new(1000.0).with_delay(500.0); // 500ms delay
        animator.start(0.0);

        // During delay period
        animator.update(0.3);
        assert_eq!(animator.get_raw_progress(), 0.0);

        // After delay
        animator.update(1.0);
        assert!(animator.get_raw_progress() > 0.0);
    }

    #[test]
    fn test_interpolate() {
        let mut animator = ChartAnimator::new(1000.0).with_easing(EasingType::Linear);
        animator.start(0.0);
        animator.update(0.5);

        let value = animator.interpolate(0.0, 100.0);
        assert!((value - 50.0).abs() < 0.1);
    }

    #[test]
    fn test_animation_manager() {
        let mut manager = AnimationManager::new().with_stagger(100.0);
        manager.create_animators(3, 500.0, EasingType::EaseOutQuart);
        manager.start_all(0.0);

        assert!(!manager.is_complete());

        // After all animations should be done
        manager.update_all(2.0);
        assert!(manager.is_complete());
    }
}
