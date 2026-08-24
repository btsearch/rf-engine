use napi::bindgen_prelude::*;
use napi_derive::napi;

use crate::engine::RfEngine;
use crate::raster::CoverageRaster;
use crate::source::{FlatTerrainSource, GridTerrainSource, SourceMetadata};
use crate::types::{Bounds, Coordinates, CoverageParams, VerticalDatum};

#[napi(object)]
pub struct JsTerrainGrid {
    pub width: u32,
    pub height: u32,
    pub min_lat: f64,
    pub min_lon: f64,
    pub max_lat: f64,
    pub max_lon: f64,
    pub elevation: Vec<f64>,
    pub surface: Vec<f64>,
    pub zone: Vec<u8>,
    pub source_id: Option<String>,
    pub resolution_m: Option<f64>,
}

#[napi(object)]
pub struct JsPathProfile {
    pub distance_km: Vec<f64>,
    pub terrain_m: Vec<f64>,
    pub surface_m: Vec<f64>,
    pub zone: Vec<u8>,
}

#[napi(object)]
pub struct JsP1812Params {
    pub frequency_ghz: f64,
    pub tx_height_m: f64,
    pub rx_height_m: f64,
    pub time_percent: f64,
    pub location_percent: Option<f64>,
    pub sigma_l: Option<f64>,
    pub tx_lat: f64,
    pub tx_lon: f64,
    pub rx_lat: f64,
    pub rx_lon: f64,
    pub dn: f64,
    pub n0: f64,
    pub polarization: Option<u8>,
}

#[napi(object)]
pub struct JsP1812Result {
    pub basic_transmission_loss_db: f64,
    pub field_strength_dbuvm: f64,
}

#[napi(object)]
pub struct JsP1812DetailedResult {
    pub basic_transmission_loss_db: f64,
    pub field_strength_dbuvm: f64,
    pub path_type: String,
    pub free_space_loss_db: f64,
    pub diffraction_loss_db: f64,
    pub troposcatter_loss_db: f64,
    pub anomalous_loss_db: f64,
    pub tx_horizon_distance_km: f64,
    pub rx_horizon_distance_km: f64,
    pub effective_earth_radius_km: f64,
    pub beta0: f64,
    pub sea_fraction: f64,
    pub bullington_distance_km: Option<f64>,
}

#[napi(object)]
pub struct JsCoverageParams {
    pub center_lat: f64,
    pub center_lon: f64,
    pub radius_km: f64,
    pub resolution_m: f64,
    pub frequency_ghz: f64,
    pub tx_height_m: f64,
    pub rx_height_m: Option<f64>,
    pub time_percent: Option<f64>,
    pub dn: f64,
    pub n0: f64,
    pub polarization: Option<u8>,
}

#[napi(object)]
pub struct JsCoverageResult {
    pub width: u32,
    pub height: u32,
    pub min_lat: f64,
    pub min_lon: f64,
    pub max_lat: f64,
    pub max_lon: f64,
    pub resolution_m: f64,
    pub data: Vec<f64>,
    pub source_ids: Vec<String>,
}

#[napi(object)]
pub struct JsLinkBudgetParams {
    pub tx_power_dbm: f64,
    pub tx_gain_dbi: Option<f64>,
    pub tx_feeder_loss_db: Option<f64>,
    pub rx_gain_dbi: Option<f64>,
    pub rx_feeder_loss_db: Option<f64>,
    pub rx_sensitivity_dbm: f64,
    pub path_loss_db: f64,
}

#[napi(object)]
pub struct JsLinkBudgetResult {
    pub eirp_dbm: f64,
    pub path_loss_db: f64,
    pub received_power_dbm: f64,
    pub margin_db: f64,
}

fn to_polarization(val: Option<u8>) -> p1812::Polarization {
    match val {
        Some(2) => p1812::Polarization::Vertical,
        _ => p1812::Polarization::Horizontal,
    }
}

fn build_p1812_inputs(
    profile: &JsPathProfile,
    params: &JsP1812Params,
) -> (Vec<f64>, p1812::P1812Params) {
    let clutter: Vec<f64> = profile
        .surface_m
        .iter()
        .zip(profile.terrain_m.iter())
        .map(|(s, t)| (s - t).max(0.0))
        .collect();

    let p1812_params = p1812::P1812Params {
        frequency_ghz: params.frequency_ghz,
        time_percent: params.time_percent,
        tx_height_m: params.tx_height_m,
        rx_height_m: params.rx_height_m,
        polarization: to_polarization(params.polarization),
        tx_lat: params.tx_lat,
        rx_lat: params.rx_lat,
        tx_lon: params.tx_lon,
        rx_lon: params.rx_lon,
        dn: params.dn,
        n0: params.n0,
        location_percent: params.location_percent.unwrap_or(50.0),
        sigma_l: params.sigma_l.unwrap_or(0.0),
        ..Default::default()
    };

    (clutter, p1812_params)
}

fn build_coverage_params(params: &JsCoverageParams) -> CoverageParams {
    CoverageParams {
        center: Coordinates {
            lat: params.center_lat,
            lon: params.center_lon,
        },
        radius_km: params.radius_km,
        resolution_m: params.resolution_m,
        frequency_ghz: params.frequency_ghz,
        tx_height_m: params.tx_height_m,
        rx_height_m: params.rx_height_m.unwrap_or(1.5),
        time_percent: params.time_percent.unwrap_or(50.0),
        dn: params.dn,
        n0: params.n0,
        polarization: to_polarization(params.polarization),
        ..Default::default()
    }
}

fn raster_to_js_result(raster: CoverageRaster) -> JsCoverageResult {
    JsCoverageResult {
        width: raster.width as u32,
        height: raster.height as u32,
        min_lat: raster.bounds.min.lat,
        min_lon: raster.bounds.min.lon,
        max_lat: raster.bounds.max.lat,
        max_lon: raster.bounds.max.lon,
        resolution_m: raster.resolution_m,
        data: raster.data.iter().map(|&v| v as f64).collect(),
        source_ids: raster.source_ids,
    }
}

#[napi]
pub fn calculate_p1812(profile: JsPathProfile, params: JsP1812Params) -> Result<JsP1812Result> {
    let (clutter, p1812_params) = build_p1812_inputs(&profile, &params);

    let path_profile = p1812::PathProfile {
        distance_km: &profile.distance_km,
        height_asl_m: &profile.terrain_m,
        clutter_height_m: &clutter,
        zone: &profile.zone,
    };

    let result = p1812::calculate(&path_profile, &p1812_params)
        .map_err(|e| Error::from_reason(e.to_string()))?;

    Ok(JsP1812Result {
        basic_transmission_loss_db: result.basic_transmission_loss_db,
        field_strength_dbuvm: result.field_strength_dbuvm,
    })
}

#[napi]
pub fn calculate_p1812_detailed(
    profile: JsPathProfile,
    params: JsP1812Params,
) -> Result<JsP1812DetailedResult> {
    let (clutter, p1812_params) = build_p1812_inputs(&profile, &params);

    let path_profile = p1812::PathProfile {
        distance_km: &profile.distance_km,
        height_asl_m: &profile.terrain_m,
        clutter_height_m: &clutter,
        zone: &profile.zone,
    };

    let detailed = p1812::calculate_detailed(&path_profile, &p1812_params)
        .map_err(|e| Error::from_reason(e.to_string()))?;

    let path_type_str = match detailed.diagnostics.path_type {
        p1812::PathType::LineOfSight => "los",
        p1812::PathType::TransHorizon => "transhorizon",
    };

    Ok(JsP1812DetailedResult {
        basic_transmission_loss_db: detailed.result.basic_transmission_loss_db,
        field_strength_dbuvm: detailed.result.field_strength_dbuvm,
        path_type: path_type_str.to_string(),
        free_space_loss_db: detailed.diagnostics.free_space_loss_db,
        diffraction_loss_db: detailed.diagnostics.diffraction_loss_db,
        troposcatter_loss_db: detailed.diagnostics.troposcatter_loss_db,
        anomalous_loss_db: detailed.diagnostics.anomalous_loss_db,
        tx_horizon_distance_km: detailed.diagnostics.tx_horizon_distance_km,
        rx_horizon_distance_km: detailed.diagnostics.rx_horizon_distance_km,
        effective_earth_radius_km: detailed.diagnostics.effective_earth_radius_km,
        beta0: detailed.diagnostics.beta0,
        sea_fraction: detailed.diagnostics.sea_fraction,
        bullington_distance_km: detailed.diagnostics.bullington_distance_km,
    })
}

#[napi]
pub fn calculate_link_budget(params: JsLinkBudgetParams) -> JsLinkBudgetResult {
    let lb_params = crate::types::LinkBudgetParams {
        tx_power_dbm: params.tx_power_dbm,
        tx_gain_dbi: params.tx_gain_dbi.unwrap_or(0.0),
        tx_feeder_loss_db: params.tx_feeder_loss_db.unwrap_or(0.0),
        rx_gain_dbi: params.rx_gain_dbi.unwrap_or(0.0),
        rx_feeder_loss_db: params.rx_feeder_loss_db.unwrap_or(0.0),
        rx_sensitivity_dbm: params.rx_sensitivity_dbm,
    };

    let result = crate::link_budget::calculate_link_budget(params.path_loss_db, &lb_params);

    JsLinkBudgetResult {
        eirp_dbm: result.eirp_dbm,
        path_loss_db: result.path_loss_db,
        received_power_dbm: result.received_power_dbm,
        margin_db: result.margin_db,
    }
}

#[napi]
pub fn calculate_coverage(params: JsCoverageParams) -> JsCoverageResult {
    let source = FlatTerrainSource {
        meta: SourceMetadata {
            id: "flat-fallback".to_string(),
            resolution_m: params.resolution_m,
            has_terrain: true,
            has_surface: true,
            priority: 0,
            vertical_datum: VerticalDatum::default(),
        },
        coverage: Bounds {
            min: Coordinates { lat: -90.0, lon: -180.0 },
            max: Coordinates { lat: 90.0, lon: 180.0 },
        },
        height_m: 0.0,
    };

    let engine = RfEngine::new(vec![Box::new(source)]);
    raster_to_js_result(engine.coverage(&build_coverage_params(&params)))
}

#[napi]
pub fn calculate_coverage_with_terrain(
    params: JsCoverageParams,
    terrain: JsTerrainGrid,
) -> JsCoverageResult {
    let source_id = terrain.source_id.unwrap_or_else(|| "terrain-grid".to_string());
    let resolution = terrain.resolution_m.unwrap_or(params.resolution_m);

    let source = GridTerrainSource {
        meta: SourceMetadata {
            id: source_id,
            resolution_m: resolution,
            has_terrain: true,
            has_surface: true,
            priority: 100,
            vertical_datum: VerticalDatum::default(),
        },
        bounds: Bounds {
            min: Coordinates {
                lat: terrain.min_lat,
                lon: terrain.min_lon,
            },
            max: Coordinates {
                lat: terrain.max_lat,
                lon: terrain.max_lon,
            },
        },
        width: terrain.width as usize,
        height: terrain.height as usize,
        elevation: terrain.elevation,
        surface: terrain.surface,
        zone: terrain.zone,
    };

    let engine = RfEngine::new(vec![Box::new(source)]);
    raster_to_js_result(engine.coverage(&build_coverage_params(&params)))
}
