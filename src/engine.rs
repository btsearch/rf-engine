use crate::antenna::AntennaPattern;
use crate::coverage::CoverageCalculator;
use crate::link_budget::calculate_link_budget;
use crate::profile::ProfileBuilder;
use crate::propagation::{P1812Model, PropagationModel, PropagationParams};
use crate::raster::CoverageRaster;
use crate::source::{SourceResolver, TerrainSource};
use crate::types::{Coordinates, CoverageParams, LinkBudgetParams, LinkBudgetResult};

pub struct RfEngine {
    resolver: SourceResolver,
    model: Box<dyn PropagationModel>,
    default_spacing_m: f64,
}

impl RfEngine {
    pub fn new(sources: Vec<Box<dyn TerrainSource>>) -> Self {
        Self {
            resolver: SourceResolver::new(sources),
            model: Box::new(P1812Model),
            default_spacing_m: 30.0,
        }
    }

    pub fn with_model(mut self, model: Box<dyn PropagationModel>) -> Self {
        self.model = model;
        self
    }

    pub fn with_spacing(mut self, spacing_m: f64) -> Self {
        self.default_spacing_m = spacing_m;
        self
    }

    pub fn path(
        &self,
        tx: Coordinates,
        rx: Coordinates,
        params: &PropagationParams,
    ) -> Result<crate::propagation::PropagationResult, Box<dyn std::error::Error + Send + Sync>>
    {
        let builder = ProfileBuilder::new(&self.resolver, self.default_spacing_m);
        let profile = builder.build(tx, rx)?;
        self.model.calculate(&profile, params)
    }

    pub fn coverage(&self, params: &CoverageParams) -> CoverageRaster {
        let calculator = CoverageCalculator::new(self.model.as_ref(), &self.resolver);
        calculator.calculate(params)
    }

    pub fn coverage_with_antenna(
        &self,
        params: &CoverageParams,
        antenna: Box<dyn AntennaPattern>,
    ) -> CoverageRaster {
        let calculator = CoverageCalculator::new(self.model.as_ref(), &self.resolver)
            .with_antenna(antenna);
        calculator.calculate(params)
    }

    pub fn link_budget(&self, path_loss_db: f64, params: &LinkBudgetParams) -> LinkBudgetResult {
        calculate_link_budget(path_loss_db, params)
    }

    pub fn resolver(&self) -> &SourceResolver {
        &self.resolver
    }
}
