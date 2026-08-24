use crate::types::Bounds;

pub struct CoverageRaster {
    pub width: usize,
    pub height: usize,
    pub bounds: Bounds,
    pub resolution_m: f64,
    pub data: Vec<f32>,
    pub source_ids: Vec<String>,
}

impl CoverageRaster {
    pub fn new(width: usize, height: usize, bounds: Bounds, resolution_m: f64) -> Self {
        Self {
            width,
            height,
            bounds,
            resolution_m,
            data: vec![f32::NAN; width * height],
            source_ids: Vec::new(),
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: f32) {
        if x < self.width && y < self.height {
            self.data[y * self.width + x] = value;
        }
    }

    pub fn get(&self, x: usize, y: usize) -> f32 {
        if x < self.width && y < self.height {
            self.data[y * self.width + x]
        } else {
            f32::NAN
        }
    }

    pub fn apply_power_offset(&mut self, offset_db: f32) {
        for val in &mut self.data {
            if !val.is_nan() {
                *val += offset_db;
            }
        }
    }

    pub fn coverage_mask(&self, threshold_db: f32) -> Vec<bool> {
        self.data
            .iter()
            .map(|&v| !v.is_nan() && v <= threshold_db)
            .collect()
    }

    pub fn to_f32_buffer(&self) -> &[f32] {
        &self.data
    }

    pub fn to_u8_buffer(&self, min_db: f32, max_db: f32) -> Vec<u8> {
        let range = max_db - min_db;
        self.data
            .iter()
            .map(|&v| {
                if v.is_nan() {
                    0u8
                } else {
                    let normalized = ((v - min_db) / range).clamp(0.0, 1.0);
                    (normalized * 255.0) as u8
                }
            })
            .collect()
    }
}
