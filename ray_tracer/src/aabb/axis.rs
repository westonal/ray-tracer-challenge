#[derive(Debug, Copy, Clone)]
pub(crate) struct AABBAxis {
    pub(crate) min: f32,
    pub(crate) max: f32,
}

impl Default for AABBAxis {
    fn default() -> Self {
        Self {
            min: f32::MAX,
            max: f32::MIN,
        }
    }
}

impl AABBAxis {
    pub(crate) fn ensure_some_width(&mut self, width: f32) {
        if self.width() == 0. {
            self.min_width(width);
        }
    }

    pub(crate) fn min_width(&mut self, min_width: f32) {
        let width_diff = min_width - self.width();
        if width_diff > 0. {
            self.push(self.min - width_diff / 2.);
            self.push(self.max + width_diff / 2.);
        }
    }
}

impl AABBAxis {
    pub(crate) fn is_empty(&self) -> bool {
        self.min > self.max
    }

    pub(crate) fn width(&self) -> f32 {
        self.max - self.min
    }

    pub(crate) fn push(&mut self, value: f32) {
        self.min = self.min.min(value);
        self.max = self.max.max(value);
    }
}
