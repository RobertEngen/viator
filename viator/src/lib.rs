pub mod grid;
pub mod path;
pub mod primitive;

#[cfg(all(feature = "y_up", feature = "z_up"))]
compile_error!("Features `y_up` and `z_up` cannot be enabled at the same time.");

#[cfg(not(any(feature = "y_up", feature = "z_up")))]
compile_error!("At least one of the features `y_up` or `z_up` must be enabled.");

#[cfg(all(feature = "transform_32", feature = "transform_64"))]
compile_error!("Features `transform_32` and `transform_64` cannot be enabled at the same time.");

#[cfg(not(any(feature = "transform_32", feature = "transform_64")))]
compile_error!("At least one of the features `transform_32` or `transform_64` must be enabled.");

#[cfg(all(feature = "transform_right_handed", feature = "transform_left_handed"))]
compile_error!("Features `transform_right_handed` and `transform_left_handed` cannot be enabled at the same time.");

#[cfg(not(any(feature = "transform_right_handed", feature = "transform_left_handed")))]
compile_error!("At least one of the features `transform_right_handed` or `transform_left_handed` must be enabled.");

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
