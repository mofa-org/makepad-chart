use makepad_widgets::*;

/// Hit testing utility for chart interactions
#[derive(Clone, Debug, Default)]
pub struct HitTester {
    /// Regions registered for hit testing
    regions: Vec<HitRegion>,
}

/// A region that can be hit tested
#[derive(Clone, Debug)]
pub struct HitRegion {
    pub id: usize,
    pub rect: Rect,
    pub data: HitData,
}

/// Data associated with a hit region
#[derive(Clone, Debug)]
pub enum HitData {
    /// A bar in a bar chart
    Bar { dataset_index: usize, data_index: usize },
    /// A point in a line or scatter chart
    Point { dataset_index: usize, data_index: usize },
    /// A slice in a pie chart
    Slice { index: usize },
    /// Custom data
    Custom(String),
}

impl HitTester {
    pub fn new() -> Self {
        Self { regions: Vec::new() }
    }

    /// Clear all registered regions
    pub fn clear(&mut self) {
        self.regions.clear();
    }

    /// Register a hit region
    pub fn register(&mut self, rect: Rect, data: HitData) {
        let id = self.regions.len();
        self.regions.push(HitRegion { id, rect, data });
    }

    /// Test if a point hits any region
    pub fn hit_test(&self, pos: DVec2) -> Option<&HitRegion> {
        for region in &self.regions {
            if region.rect.contains(pos) {
                return Some(region);
            }
        }
        None
    }

    /// Get all regions that contain the point
    pub fn hit_test_all(&self, pos: DVec2) -> Vec<&HitRegion> {
        self.regions
            .iter()
            .filter(|r| r.rect.contains(pos))
            .collect()
    }

    /// Find the nearest region to a point within a max distance
    pub fn find_nearest(&self, pos: DVec2, max_distance: f64) -> Option<&HitRegion> {
        let mut nearest: Option<&HitRegion> = None;
        let mut min_dist = max_distance;

        for region in &self.regions {
            let center = dvec2(
                region.rect.pos.x + region.rect.size.x / 2.0,
                region.rect.pos.y + region.rect.size.y / 2.0,
            );
            let dx = pos.x - center.x;
            let dy = pos.y - center.y;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist < min_dist {
                min_dist = dist;
                nearest = Some(region);
            }
        }

        nearest
    }
}
