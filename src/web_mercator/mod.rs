use wgs84;
use math::clamp;

pub mod consts
{
  pub const PIXELS_IN_TILE_ARRIS: u32 = 256;
  pub const CRITICAL_LATITUDE: f64 = 85.051_128_78;
  // Calculated with Vincenty method with WGS84, WebMercator parameters
  pub const CRITICAL_LATITUDE_IN_METERS: f64 = 9_417_539.062_5;
}

pub fn meters_per_pixel( latitude: f64, level_of_detail: u8 )
  -> f64
{
  latitude.to_radians().cos() * 2.0 * std::f64::consts::PI * wgs84::consts::MAJOR_RADIUS_IN_METER / pixels_in_arris( level_of_detail ) as f64
}

pub fn pixels_in_arris( level_of_detail: u8 )
  -> u32
{
  consts::PIXELS_IN_TILE_ARRIS << level_of_detail
}

pub fn angle_to_pixel( longitude: f64, latitude: f64, level_of_detail: u8 )
  -> ( u32, u32 )
{
  let x = ( longitude + 180.0 ) / 360.0;
  let sin_lat = latitude.to_radians().sin();
  let y = 0.5 - ( ( 1.0 + sin_lat ) / ( 1.0 - sin_lat ) ).log( std::f64::consts::E ) / ( 4.0 * std::f64::consts::PI );
  let s = pixels_in_arris( level_of_detail ) as f64;
  ( ( x * s + 0.5 ) as u32, ( y * s + 0.5 ) as u32 )
}

pub fn pixel_to_tile( x: u32, y: u32 )
  -> ( u32, u32 )
{
  ( x / consts::PIXELS_IN_TILE_ARRIS, y / consts::PIXELS_IN_TILE_ARRIS )
}

pub fn angle_to_tile( longitude: f64, latitude: f64, level_of_detail: u8 )
  -> ( u32, u32 )
{
  let ( px, py ) = angle_to_pixel( longitude, latitude, level_of_detail );
  pixel_to_tile( px, py )
}

pub fn pixel_to_angle( x: u32, y: u32, level_of_detail: u8 )
  -> ( f64, f64 )
{
  let s = pixels_in_arris( level_of_detail ) as f64;
  
  let nx = clamp( x as f64, 0.0, s ) / s - 0.5;
  let ny = 0.5 - clamp( y as f64, 0.0, s ) / s;
  
  ( 360.0 * nx
  , 90.0 - 360.0 * ( -ny * 2.0 * std::f64::consts::PI ).exp().atan() / std::f64::consts::PI
  )
}