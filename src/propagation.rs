use crate::types::{SourceInfo, TerrainProfile};

pub struct PropagationParams {
    pub frequency_ghz: f64,
    pub tx_height_m: f64,
    pub rx_height_m: f64,
    pub time_percent: f64,
    pub location_percent: f64,
    pub sigma_l: f64,
    pub tx_lat: f64,
    pub tx_lon: f64,
    pub rx_lat: f64,
    pub rx_lon: f64,
    pub dn: f64,
    pub n0: f64,
    pub polarization: p1812::Polarization,
}

pub struct PropagationResult {
    pub basic_transmission_loss_db: f64,
    pub field_strength_dbuvm: f64,
    pub source_info: Option<SourceInfo>,
}

pub trait PropagationModel: Send + Sync {
    fn calculate(
        &self,
        profile: &TerrainProfile,
        params: &PropagationParams,
    ) -> Result<PropagationResult, Box<dyn std::error::Error + Send + Sync>>;
}

pub struct P1812Model;

impl PropagationModel for P1812Model {
    fn calculate(
        &self,
        profile: &TerrainProfile,
        params: &PropagationParams,
    ) -> Result<PropagationResult, Box<dyn std::error::Error + Send + Sync>> {
        let clutter: Vec<f64> = profile
            .surface_m
            .iter()
            .zip(profile.terrain_m.iter())
            .map(|(s, t)| (s - t).max(0.0))
            .collect();

        let path_profile = p1812::PathProfile {
            distance_km: &profile.distance_km,
            height_asl_m: &profile.terrain_m,
            clutter_height_m: &clutter,
            zone: &profile.zone,
        };

        let p1812_params = p1812::P1812Params {
            frequency_ghz: params.frequency_ghz,
            time_percent: params.time_percent,
            tx_height_m: params.tx_height_m,
            rx_height_m: params.rx_height_m,
            polarization: params.polarization,
            tx_lat: params.tx_lat,
            rx_lat: params.rx_lat,
            tx_lon: params.tx_lon,
            rx_lon: params.rx_lon,
            dn: params.dn,
            n0: params.n0,
            location_percent: params.location_percent,
            sigma_l: params.sigma_l,
            ..Default::default()
        };

        let result = p1812::calculate(&path_profile, &p1812_params)?;

        Ok(PropagationResult {
            basic_transmission_loss_db: result.basic_transmission_loss_db,
            field_strength_dbuvm: result.field_strength_dbuvm,
            source_info: profile.source_info.clone(),
        })
    }
}
