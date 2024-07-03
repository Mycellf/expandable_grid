use crate::{util, ExpandableGrid};
use nalgebra::{vector, Vector2};

pub trait Subchunk
where
    Self: std::ops::Index<Vector2<usize>> + std::ops::IndexMut<Vector2<usize>>,
    Self::Output: Sized,
{
    const SUBCHUNK_SIZE: Vector2<usize>;
}

impl<T: Subchunk> ExpandableGrid<T>
where
    T::Output: Sized,
{
    pub fn get_from_subchunk(&self, index: Vector2<isize>) -> Option<&T::Output> {
        let (chunk, subchunk) = Self::subchunk_index_of(index);

        Some(&self.get(chunk)?[subchunk])
    }

    pub fn get_mut_from_subchunk(&mut self, index: Vector2<isize>) -> Option<&mut T::Output> {
        let (chunk, subchunk) = Self::subchunk_index_of(index);

        Some(&mut self.get_mut(chunk)?[subchunk])
    }

    pub fn subchunk_index_of(index: Vector2<isize>) -> (Vector2<isize>, Vector2<usize>) {
        (
            vector![
                index.x.div_euclid(T::SUBCHUNK_SIZE.x as isize),
                index.y.div_euclid(T::SUBCHUNK_SIZE.y as isize),
            ],
            vector![
                index.x.rem_euclid(T::SUBCHUNK_SIZE.x as isize) as usize,
                index.y.rem_euclid(T::SUBCHUNK_SIZE.y as isize) as usize,
            ],
        )
    }

    pub fn subchunk_index_size(&self) -> Vector2<usize> {
        self.size.component_mul(&T::SUBCHUNK_SIZE)
    }

    pub fn subchunk_index_origin(&self) -> Vector2<isize> {
        self.origin
            .component_mul(&util::usize_vec_to_isize(T::SUBCHUNK_SIZE))
    }
}
