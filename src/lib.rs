#![feature(extern_prelude)]

pub mod math;
pub mod wgs84;
pub mod web_mercator;

#[cfg(test)]
#[test]
fn clamp()
{
  const MIN: i8 = -8;
  const MAX: i8 = 8;
  
  for value in 2 * MIN .. MIN
  { assert_eq!( math::clamp( value, MIN, MAX ), MIN ); }
  
  for value in MIN .. MAX
  { assert_eq!( math::clamp( value, MIN, MAX ), value ); }
  
  for value in MAX .. 2 * MAX
  { assert_eq!( math::clamp( value, MIN, MAX ), MAX ); }
}

#[cfg(test)]
#[test]
fn web_mercator()
{
  let ( lon0, lat0 ) = ( 43.062_083f64, 141.354_389f64 );
  for lod in 0 .. 20
  {
    let angle_error_tolerance = 360.0f64 / ( 1u64 << lod ) as f64 / 256.0;
    let ( px, py ) = web_mercator::angle_to_pixel( lon0, lat0, lod );
    let ( lon1, lat1 ) = web_mercator::pixel_to_angle( px, py, lod );
    assert!( lon1 - lon0 < angle_error_tolerance );
    assert!( lat1 - lat0 < angle_error_tolerance );
  }
}