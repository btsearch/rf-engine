use rayon::prelude::*;

use crate::antenna::{compute_azimuth, compute_elevation, AntennaPattern, IsotropicAntenna};
use crate::propagation::{PropagationModel, PropagationParams};
use crate::raster::CoverageRaster;
use crate::source::{haversine_km, SourceResolver};
use crate::types::{Coordinates, CoverageParams};

pub struct CoverageCalculator<'a> {
    model: &'a dyn PropagationModel,
    resolver: &'a SourceResolver,
    antenna: Box<dyn AntennaPattern>,
}

impl<'a> CoverageCalculator<'a> {
    pub fn new(model: &'a dyn PropagationModel, resolver: &'a SourceResolver) -> Self {
        Self {
            model,
            resolver,
            antenna: Box::new(IsotropicAntenna),
        }
    }

    pub fn with_antenna(mut self, antenna: Box<dyn AntennaPattern>) -> Self {
        self.antenna = antenna;
        self
    }

    pub fn calculate(&self, params: &CoverageParams) -> CoverageRaster {
        let deg_per_m_lat = 1.0 / 111320.0;
        let deg_per_m_lon = 1.0 / (111320.0 * params.center.lat.to_radians().cos());

        let radius_deg_lat = params.radius_km * 1000.0 * deg_per_m_lat;
        let radius_deg_lon = params.radius_km * 1000.0 * deg_per_m_lon;

        let res_deg_lat = params.resolution_m * deg_per_m_lat;
        let res_deg_lon = params.resolution_m * deg_per_m_lon;

        let width = (2.0 * radius_deg_lon / res_deg_lon).ceil() as usize + 1;
        let height = (2.0 * radius_deg_lat / res_deg_lat).ceil() as usize + 1;

        let bounds = crate::types::Bounds {
            min: Coordinates {
                lat: params.center.lat - radius_deg_lat,
                lon: params.center.lon - radius_deg_lon,
            },
            max: Coordinates {
                lat: params.center.lat + radius_deg_lat,
                lon: params.center.lon + radius_deg_lon,
            },
        };

        let pixels: Vec<(usize, usize, Coordinates)> = (0..height)
            .flat_map(|y| {
                (0..width).map(move |x| {
                    let lat = bounds.max.lat - y as f64 * res_deg_lat;
                    let lon = bounds.min.lon + x as f64 * res_deg_lon;
                    (x, y, Coordinates { lat, lon })
                })
            })
            .collect();

        let results: Vec<(usize, usize, f64, Option<String>)> = pixels
            .par_iter()
            .filter_map(|&(x, y, rx_coord)| {
                let dist_km = haversine_km(params.center, rx_coord);

                if dist_km > params.radius_km || dist_km < 0.001 {
                    return None;
                }

                let source = self.resolver.resolve(rx_coord)?;
                let profile = source
                    .get_profile(params.center, rx_coord, params.resolution_m)
                    .ok()?;

                if profile.distance_km.len() <= 4 {
                    return None;
                }

                let prop_params = PropagationParams {
                    frequency_ghz: params.frequency_ghz,
                    tx_height_m: params.tx_height_m,
                    rx_height_m: params.rx_height_m,
                    time_percent: params.time_percent,
                    location_percent: 50.0,
                    sigma_l: 0.0,
                    tx_lat: params.center.lat,
                    tx_lon: params.center.lon,
                    rx_lat: rx_coord.lat,
                    rx_lon: rx_coord.lon,
                    dn: params.dn,
                    n0: params.n0,
                    polarization: params.polarization,
                };

                let result = self.model.calculate(&profile, &prop_params).ok()?;

                let azimuth = compute_azimuth(params.center, rx_coord);
                let elevation = compute_elevation(
                    params.center,
                    rx_coord,
                    params.tx_height_m,
                    params.rx_height_m,
                    dist_km,
                );

                let relative_az = azimuth - params.antenna_azimuth_deg;
                let relative_el = elevation - params.antenna_downtilt_deg;
                let antenna_gain = self.antenna.gain_dbi(relative_az, relative_el);

                let effective_loss = result.basic_transmission_loss_db - antenna_gain;

                let source_id = result.source_info.map(|s| s.id);
                Some((x, y, effective_loss, source_id))
            })
            .collect();

        let mut raster = CoverageRaster::new(width, height, bounds, params.resolution_m);
        for (x, y, loss, source_id) in results {
            raster.set(x, y, loss as f32);
            if let Some(sid) = source_id {
                if !raster.source_ids.contains(&sid) {
                    raster.source_ids.push(sid);
                }
            }
        }

        raster
    }
}
