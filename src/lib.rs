pub mod math;

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