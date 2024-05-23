#[cfg(feature = "transform_32")]
pub type TransformFloat = f32;
#[cfg(feature = "transform_64")]
pub type TransformFloat = f64;

#[cfg(feature = "transform_32")]
pub type TransformInt = i32;
#[cfg(feature = "transform_64")]
pub type TransformInt = i64;
