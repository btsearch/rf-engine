pub trait AntennaPattern: Send + Sync {
    fn gain_dbi(&self, azimuth_deg: f64, elevation_deg: f64) -> f64;
}

pub struct IsotropicAntenna;

impl AntennaPattern for IsotropicAntenna {
    fn gain_dbi(&self, _azimuth_deg: f64, _elevation_deg: f64) -> f64 {
        0.0
    }
}

pub struct SectorAntenna {
    pub horizontal_beamwidth_deg: f64,
    pub vertical_beamwidth_deg: f64,
    pub max_gain_dbi: f64,
    pub front_to_back_db: f64,
}

impl Default for SectorAntenna {
    fn default() -> Self {
        Self {
            horizontal_beamwidth_deg: 65.0,
            vertical_beamwidth_deg: 7.0,
            max_gain_dbi: 17.0,
            front_to_back_db: 30.0,
        }
    }
}

impl AntennaPattern for SectorAntenna {
    fn gain_dbi(&self, azimuth_deg: f64, elevation_deg: f64) -> f64 {
        let mut az = azimuth_deg % 360.0;
        if az > 180.0 {
            az -= 360.0;
        }
        if az < -180.0 {
            az += 360.0;
        }

        let ah = -12.0 * (az / self.horizontal_beamwidth_deg).powi(2);
        let ah = ah.max(-self.front_to_back_db);

        let av = -12.0 * (elevation_deg / self.vertical_beamwidth_deg).powi(2);
        let av = av.max(-self.front_to_back_db);

        let rolloff = -((-ah).min(self.front_to_back_db) + (-av).min(self.front_to_back_db));
        let rolloff = rolloff.max(-self.front_to_back_db);

        self.max_gain_dbi + rolloff
    }
}

pub fn compute_azimuth(from: crate::types::Coordinates, to: crate::types::Coordinates) -> f64 {
    let dlon = (to.lon - from.lon).to_radians();
    let lat1 = from.lat.to_radians();
    let lat2 = to.lat.to_radians();

    let x = dlon.sin() * lat2.cos();
    let y = lat1.cos() * lat2.sin() - lat1.sin() * lat2.cos() * dlon.cos();

    let bearing = x.atan2(y).to_degrees();
    (bearing + 360.0) % 360.0
}

pub fn compute_elevation(
    _from: crate::types::Coordinates,
    _to: crate::types::Coordinates,
    tx_height_m: f64,
    rx_height_m: f64,
    distance_km: f64,
) -> f64 {
    let dh = rx_height_m - tx_height_m;
    (dh / (distance_km * 1000.0)).atan().to_degrees()
}
