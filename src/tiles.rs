use crate::raster::CoverageRaster;

#[derive(Debug, Clone, Copy)]
pub struct TileCoord {
    pub z: u8,
    pub x: u32,
    pub y: u32,
}

pub fn lat_to_tile_y(lat: f64, zoom: u8) -> f64 {
    let n = 2_u32.pow(zoom as u32) as f64;
    let lat_rad = lat.to_radians();
    n * (1.0 - lat_rad.tan().asinh() / std::f64::consts::PI) / 2.0
}

pub fn lon_to_tile_x(lon: f64, zoom: u8) -> f64 {
    let n = 2_u32.pow(zoom as u32) as f64;
    (lon + 180.0) / 360.0 * n
}

pub fn tile_y_to_lat(y: f64, zoom: u8) -> f64 {
    let n = 2_u32.pow(zoom as u32) as f64;
    let lat_rad = (std::f64::consts::PI * (1.0 - 2.0 * y / n)).sinh().atan();
    lat_rad.to_degrees()
}

pub fn tile_x_to_lon(x: f64, zoom: u8) -> f64 {
    let n = 2_u32.pow(zoom as u32) as f64;
    x / n * 360.0 - 180.0
}

pub fn raster_to_tiles(
    raster: &CoverageRaster,
    min_zoom: u8,
    max_zoom: u8,
) -> Vec<(TileCoord, Vec<f32>)> {
    let mut tiles = Vec::new();

    for z in min_zoom..=max_zoom {
        let min_tx = lon_to_tile_x(raster.bounds.min.lon, z).floor() as u32;
        let max_tx = lon_to_tile_x(raster.bounds.max.lon, z).ceil() as u32;
        let min_ty = lat_to_tile_y(raster.bounds.max.lat, z).floor() as u32;
        let max_ty = lat_to_tile_y(raster.bounds.min.lat, z).ceil() as u32;

        for ty in min_ty..max_ty {
            for tx in min_tx..max_tx {
                let tile_data = sample_tile(raster, z, tx, ty);
                tiles.push((TileCoord { z, x: tx, y: ty }, tile_data));
            }
        }
    }

    tiles
}

fn sample_tile(raster: &CoverageRaster, z: u8, tx: u32, ty: u32) -> Vec<f32> {
    const TILE_SIZE: usize = 256;
    let mut data = vec![f32::NAN; TILE_SIZE * TILE_SIZE];

    let tile_min_lon = tile_x_to_lon(tx as f64, z);
    let tile_max_lon = tile_x_to_lon((tx + 1) as f64, z);
    let tile_min_lat = tile_y_to_lat((ty + 1) as f64, z);
    let tile_max_lat = tile_y_to_lat(ty as f64, z);

    let raster_lat_range = raster.bounds.max.lat - raster.bounds.min.lat;
    let raster_lon_range = raster.bounds.max.lon - raster.bounds.min.lon;

    if raster_lat_range <= 0.0 || raster_lon_range <= 0.0 {
        return data;
    }

    for py in 0..TILE_SIZE {
        let lat = tile_max_lat - (py as f64 / TILE_SIZE as f64) * (tile_max_lat - tile_min_lat);

        for px in 0..TILE_SIZE {
            let lon =
                tile_min_lon + (px as f64 / TILE_SIZE as f64) * (tile_max_lon - tile_min_lon);

            let rx = ((lon - raster.bounds.min.lon) / raster_lon_range * raster.width as f64)
                as usize;
            let ry = ((raster.bounds.max.lat - lat) / raster_lat_range * raster.height as f64)
                as usize;

            if rx < raster.width && ry < raster.height {
                data[py * TILE_SIZE + px] = raster.get(rx, ry);
            }
        }
    }

    data
}
