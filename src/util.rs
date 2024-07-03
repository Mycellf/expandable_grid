use nalgebra::{vector, Vector2};

pub fn calculate_exponential_distance(distance: isize, current_size: usize) -> usize {
    let minimum_size = current_size + distance as usize;
    let new_size = minimum_size.max(current_size * 2);
    new_size - current_size
}

pub fn usize_vec_to_isize(vector: Vector2<usize>) -> Vector2<isize> {
    vector![vector.x as isize, vector.y as isize]
}

pub fn isize_vec_to_usize_saturating(vector: Vector2<isize>) -> Vector2<usize> {
    vector![
        if vector.x < 0 { 0 } else { vector.x as usize },
        if vector.y < 0 { 0 } else { vector.y as usize }
    ]
}
