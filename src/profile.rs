use crate::source::SourceResolver;
use crate::types::{Coordinates, TerrainProfile};

pub struct ProfileBuilder<'a> {
    resolver: &'a SourceResolver,
    spacing_m: f64,
}

impl<'a> ProfileBuilder<'a> {
    pub fn new(resolver: &'a SourceResolver, spacing_m: f64) -> Self {
        Self { resolver, spacing_m }
    }

    pub fn with_spacing(mut self, spacing_m: f64) -> Self {
        self.spacing_m = spacing_m;
        self
    }

    pub fn build(
        &self,
        start: Coordinates,
        end: Coordinates,
    ) -> Result<TerrainProfile, Box<dyn std::error::Error + Send + Sync>> {
        self.resolver.build_profile(start, end, self.spacing_m)
    }

    pub fn spacing_m(&self) -> f64 {
        self.spacing_m
    }
}
