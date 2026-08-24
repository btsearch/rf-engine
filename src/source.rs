use crate::types::{Bounds, Coordinates, SourceInfo, TerrainProfile, VerticalDatum};

pub struct SourceMetadata {
    pub id: String,
    pub resolution_m: f64,
    pub has_terrain: bool,
    pub has_surface: bool,
    pub priority: u32,
    pub vertical_datum: VerticalDatum,
}

pub trait TerrainSource: Send + Sync {
    fn metadata(&self) -> &SourceMetadata;
    fn supports(&self, coord: Coordinates) -> bool;
    fn get_profile(
        &self,
        start: Coordinates,
        end: Coordinates,
        spacing_m: f64,
    ) -> Result<TerrainProfile, Box<dyn std::error::Error + Send + Sync>>;
}

pub struct SourceResolver {
    sources: Vec<Box<dyn TerrainSource>>,
}

impl SourceResolver {
    pub fn new(mut sources: Vec<Box<dyn TerrainSource>>) -> Self {
        sources.sort_by_key(|a| std::cmp::Reverse(a.metadata().priority));
        Self { sources }
    }

    pub fn resolve(&self, coord: Coordinates) -> Option<&dyn TerrainSource> {
        self.sources
            .iter()
            .find(|s| s.supports(coord))
            .map(|s| s.as_ref())
    }

    pub fn build_profile(
        &self,
        start: Coordinates,
        end: Coordinates,
        spacing_m: f64,
    ) -> Result<TerrainProfile, Box<dyn std::error::Error + Send + Sync>> {
        let dist_km = haversine_km(start, end);
        let spacing_km = spacing_m / 1000.0;
        let n = (dist_km / spacing_km).ceil().max(1.0) as usize + 1;

        let mut distance_km = Vec::with_capacity(n);
        let mut terrain_m = Vec::with_capacity(n);
        let mut surface_m = Vec::with_capacity(n);
        let mut zone = Vec::with_capacity(n);
        let mut source_ids: Vec<String> = Vec::new();

        for i in 0..n {
            let frac = i as f64 / (n - 1).max(1) as f64;
            let coord = Coordinates {
                lat: start.lat + frac * (end.lat - start.lat),
                lon: start.lon + frac * (end.lon - start.lon),
            };
            let d = (i as f64 * spacing_km).min(dist_km);

            let source = self
                .resolve(coord)
                .ok_or("no terrain source for profile point")?;

            let point_profile = source.get_profile(coord, coord, spacing_m)?;

            distance_km.push(d);
            terrain_m.push(point_profile.terrain_m[0]);
            surface_m.push(point_profile.surface_m[0]);
            zone.push(point_profile.zone[0]);

            let sid = &source.metadata().id;
            if !source_ids.contains(sid) {
                source_ids.push(sid.clone());
            }
        }

        let primary = self.resolve(Coordinates {
            lat: (start.lat + end.lat) / 2.0,
            lon: (start.lon + end.lon) / 2.0,
        });

        Ok(TerrainProfile {
            distance_km,
            terrain_m,
            surface_m,
            zone,
            source_info: primary.map(|s| SourceInfo {
                id: s.metadata().id.clone(),
                resolution_m: s.metadata().resolution_m,
            }),
        })
    }

    pub fn sources(&self) -> &[Box<dyn TerrainSource>] {
        &self.sources
    }
}

pub fn haversine_km(a: Coordinates, b: Coordinates) -> f64 {
    let r = 6371.0;
    let dlat = (b.lat - a.lat).to_radians();
    let dlon = (b.lon - a.lon).to_radians();
    let lat1 = a.lat.to_radians();
    let lat2 = b.lat.to_radians();

    let h = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
    2.0 * r * h.sqrt().asin()
}

pub struct FlatTerrainSource {
    pub meta: SourceMetadata,
    pub coverage: Bounds,
    pub height_m: f64,
}

impl TerrainSource for FlatTerrainSource {
    fn metadata(&self) -> &SourceMetadata {
        &self.meta
    }

    fn supports(&self, coord: Coordinates) -> bool {
        self.coverage.contains(coord)
    }

    fn get_profile(
        &self,
        start: Coordinates,
        end: Coordinates,
        spacing_m: f64,
    ) -> Result<TerrainProfile, Box<dyn std::error::Error + Send + Sync>> {
        let dist_km = haversine_km(start, end);
        let spacing_km = spacing_m / 1000.0;

        let n = if dist_km < 0.001 {
            1
        } else {
            (dist_km / spacing_km).ceil() as usize + 1
        };

        let distance_km: Vec<f64> = (0..n)
            .map(|i| (i as f64 * spacing_km).min(dist_km))
            .collect();
        let terrain_m = vec![self.height_m; n];
        let surface_m = vec![self.height_m; n];
        let zone = vec![p1812::ZONE_INLAND; n];

        Ok(TerrainProfile {
            distance_km,
            terrain_m,
            surface_m,
            zone,
            source_info: Some(SourceInfo {
                id: self.meta.id.clone(),
                resolution_m: self.meta.resolution_m,
            }),
        })
    }
}

pub struct GridTerrainSource {
    pub meta: SourceMetadata,
    pub bounds: Bounds,
    pub width: usize,
    pub height: usize,
    pub elevation: Vec<f64>,
    pub surface: Vec<f64>,
    pub zone: Vec<u8>,
}

impl GridTerrainSource {
    fn sample_at(&self, lat: f64, lon: f64) -> (f64, f64, u8) {
        let lat_range = self.bounds.max.lat - self.bounds.min.lat;
        let lon_range = self.bounds.max.lon - self.bounds.min.lon;

        let fy = (self.bounds.max.lat - lat) / lat_range * (self.height - 1) as f64;
        let fx = (lon - self.bounds.min.lon) / lon_range * (self.width - 1) as f64;

        let fy = fy.clamp(0.0, (self.height - 1) as f64);
        let fx = fx.clamp(0.0, (self.width - 1) as f64);

        let x0 = fx.floor() as usize;
        let y0 = fy.floor() as usize;
        let x1 = (x0 + 1).min(self.width - 1);
        let y1 = (y0 + 1).min(self.height - 1);

        let xf = fx - x0 as f64;
        let yf = fy - y0 as f64;

        let elev = bilerp(
            self.elevation[y0 * self.width + x0],
            self.elevation[y0 * self.width + x1],
            self.elevation[y1 * self.width + x0],
            self.elevation[y1 * self.width + x1],
            xf,
            yf,
        );

        let surf = bilerp(
            self.surface[y0 * self.width + x0],
            self.surface[y0 * self.width + x1],
            self.surface[y1 * self.width + x0],
            self.surface[y1 * self.width + x1],
            xf,
            yf,
        );

        let nearest_x = if xf < 0.5 { x0 } else { x1 };
        let nearest_y = if yf < 0.5 { y0 } else { y1 };
        let z = self.zone[nearest_y * self.width + nearest_x];

        (elev, surf, z)
    }
}

fn bilerp(ul: f64, ur: f64, ll: f64, lr: f64, xf: f64, yf: f64) -> f64 {
    let top = ul + xf * (ur - ul);
    let bottom = ll + xf * (lr - ll);
    top + yf * (bottom - top)
}

impl TerrainSource for GridTerrainSource {
    fn metadata(&self) -> &SourceMetadata {
        &self.meta
    }

    fn supports(&self, coord: Coordinates) -> bool {
        self.bounds.contains(coord)
    }

    fn get_profile(
        &self,
        start: Coordinates,
        end: Coordinates,
        spacing_m: f64,
    ) -> Result<TerrainProfile, Box<dyn std::error::Error + Send + Sync>> {
        let dist_km = haversine_km(start, end);
        let spacing_km = spacing_m / 1000.0;

        let n = if dist_km < 0.001 {
            1
        } else {
            (dist_km / spacing_km).ceil() as usize + 1
        };

        let mut distance_km = Vec::with_capacity(n);
        let mut terrain_m = Vec::with_capacity(n);
        let mut surface_m = Vec::with_capacity(n);
        let mut zone_v = Vec::with_capacity(n);

        for i in 0..n {
            let frac = if n <= 1 { 0.0 } else { i as f64 / (n - 1) as f64 };
            let lat = start.lat + frac * (end.lat - start.lat);
            let lon = start.lon + frac * (end.lon - start.lon);
            let d = (i as f64 * spacing_km).min(dist_km);

            let (elev, surf, z) = self.sample_at(lat, lon);

            distance_km.push(d);
            terrain_m.push(elev);
            surface_m.push(surf);
            zone_v.push(z);
        }

        Ok(TerrainProfile {
            distance_km,
            terrain_m,
            surface_m,
            zone: zone_v,
            source_info: Some(SourceInfo {
                id: self.meta.id.clone(),
                resolution_m: self.meta.resolution_m,
            }),
        })
    }
}
