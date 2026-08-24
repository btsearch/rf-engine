#[derive(Debug, Clone, Copy)]
pub struct Coordinates {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Bounds {
    pub min: Coordinates,
    pub max: Coordinates,
}

impl Bounds {
    pub fn contains(&self, coord: Coordinates) -> bool {
        coord.lat >= self.min.lat
            && coord.lat <= self.max.lat
            && coord.lon >= self.min.lon
            && coord.lon <= self.max.lon
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum VerticalDatum {
    #[default]
    Egm96,
    Egm2008,
    Wgs84Ellipsoid,
    Local,
}

#[derive(Debug, Clone)]
pub struct SourceInfo {
    pub id: String,
    pub resolution_m: f64,
}

#[derive(Debug, Clone)]
pub struct TerrainProfile {
    pub distance_km: Vec<f64>,
    pub terrain_m: Vec<f64>,
    pub surface_m: Vec<f64>,
    pub zone: Vec<u8>,
    pub source_info: Option<SourceInfo>,
}

#[derive(Debug, Clone, Copy)]
pub struct CoverageParams {
    pub center: Coordinates,
    pub radius_km: f64,
    pub resolution_m: f64,
    pub frequency_ghz: f64,
    pub tx_height_m: f64,
    pub rx_height_m: f64,
    pub tx_power_kw: f64,
    pub tx_gain_dbi: f64,
    pub rx_gain_dbi: f64,
    pub time_percent: f64,
    pub dn: f64,
    pub n0: f64,
    pub polarization: p1812::Polarization,
    pub antenna_azimuth_deg: f64,
    pub antenna_downtilt_deg: f64,
}

impl Default for CoverageParams {
    fn default() -> Self {
        Self {
            center: Coordinates { lat: 0.0, lon: 0.0 },
            radius_km: 10.0,
            resolution_m: 30.0,
            frequency_ghz: 0.0,
            tx_height_m: 30.0,
            rx_height_m: 1.5,
            tx_power_kw: 1.0,
            tx_gain_dbi: 0.0,
            rx_gain_dbi: 0.0,
            time_percent: 50.0,
            dn: 45.0,
            n0: 325.0,
            polarization: p1812::Polarization::Horizontal,
            antenna_azimuth_deg: 0.0,
            antenna_downtilt_deg: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LinkBudgetParams {
    pub tx_power_dbm: f64,
    pub tx_gain_dbi: f64,
    pub tx_feeder_loss_db: f64,
    pub rx_gain_dbi: f64,
    pub rx_feeder_loss_db: f64,
    pub rx_sensitivity_dbm: f64,
}

impl Default for LinkBudgetParams {
    fn default() -> Self {
        Self {
            tx_power_dbm: 43.0,
            tx_gain_dbi: 0.0,
            tx_feeder_loss_db: 0.0,
            rx_gain_dbi: 0.0,
            rx_feeder_loss_db: 0.0,
            rx_sensitivity_dbm: -100.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LinkBudgetResult {
    pub eirp_dbm: f64,
    pub path_loss_db: f64,
    pub received_power_dbm: f64,
    pub margin_db: f64,
}
