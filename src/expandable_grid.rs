use crate::util;
use nalgebra::{vector, Vector2};

/// Represents a 2d grid that can be expanded in any direction. It can be expanded to fit a point
/// or box with `expand_to_fit_point` and `expand_to_fit_box`, as well as set to a specific size
/// with `change_size`.
///
/// Values are accessed with signed 2d coordinates stored as a `nalgebra::Vector2<isize>`.
#[derive(Clone, Debug)]
pub struct ExpandableGrid<T> {
    pub size: Vector2<usize>,
    pub origin: Vector2<isize>,
    pub data: Box<[T]>,
}

impl<T> ExpandableGrid<T> {
    /// Creates a new, empty grid
    pub fn new() -> Self {
        Self {
            size: vector![0, 0],
            origin: vector![0, 0],
            data: Box::new([]),
        }
    }

    /// Creates a new grid filled with clones of `fill`
    pub fn with_size(size: Vector2<usize>, origin: Vector2<isize>, fill: &T) -> Self
    where
        T: Clone,
    {
        Self {
            size,
            origin,
            data: std::iter::repeat(fill.clone())
                .take(size.x * size.y)
                .collect(),
        }
    }

    /// Increases the size of the grid such that `point` is included within the bounds of the grid.
    /// The newly created space is filled with clones of `fill`.
    ///
    /// Note that this is not guarenteed to expand exactly as much as is needed, rather, this
    /// method will first expand by doubling the width or height of the grid in each direction as
    /// nececary, and will expand further if this is not enough.
    pub fn expand_to_fit_point(&mut self, point: Vector2<isize>, fill: &T)
    where
        T: Clone,
    {
        self.expand_to_fit_box(point, vector![1, 1], fill);
    }

    /// Increases the size of the grid such that all elements of a box with its bottom right corner at
    /// `box_origin`, with size `box_size` are within bounds of the grid. The newly created space
    /// is filled with clones of `fill`.
    ///
    /// Note that this is not guarenteed to expand exactly as much as is needed, rather, this
    /// method will first expand by doubling the width or height of the grid in each direction as
    /// nececary, and will expand further if this is not enough.
    pub fn expand_to_fit_box(
        &mut self,
        box_origin: Vector2<isize>,
        box_size: Vector2<usize>,
        fill: &T,
    ) where
        T: Clone,
    {
        if self.size == vector![0, 0] {
            self.size = box_size;
            self.origin = box_origin;
            self.data = std::iter::repeat(fill.clone())
                .take(box_size.x * box_size.y)
                .collect();
        } else {
            let area_corner = util::usize_vec_to_isize(self.size) + self.origin;
            let box_corner = util::usize_vec_to_isize(box_size) + box_origin;

            let mut new_size = self.size;
            let mut offset = vector![0, 0];
            let mut expanded = false;

            // Expand on x-axis
            if box_origin.x < self.origin.x {
                let distance = self.origin.x - box_origin.x;
                let distance = util::calculate_exponential_distance(distance, self.size.x);
                offset.x = -(distance as isize);
                new_size.x += distance;
                expanded = true;
            }
            if box_corner.x > area_corner.x {
                let distance = box_corner.x - area_corner.x;
                let distance = util::calculate_exponential_distance(distance, self.size.x);
                new_size.x += distance;
                expanded = true;
            }

            // Expand on y-axis
            if box_origin.y < self.origin.y {
                let distance = self.origin.y - box_origin.y;
                let distance = util::calculate_exponential_distance(distance, self.size.y);
                offset.y = -(distance as isize);
                new_size.y += distance;
                expanded = true;
            }
            if box_corner.y > area_corner.y {
                let distance = box_corner.y - area_corner.y;
                let distance = util::calculate_exponential_distance(distance, self.size.y);
                new_size.y += distance;
                expanded = true;
            }

            if expanded {
                self.change_size(new_size, offset, fill);
            }
        }
    }

    /// Changes the size of this grid, shifting the origin of the grid by `offset`. Any grid cells that
    /// become out of bounds due to this are removed, and any new cells are cloned values of fill.
    pub fn change_size(&mut self, new_size: Vector2<usize>, offset: Vector2<isize>, fill: &T)
    where
        T: Clone,
    {
        // Maintain consistant behavior if the grid is empty
        if self.data.len() == 0 {
            *self = ExpandableGrid::with_size(new_size, offset, fill);
        }

        // Calculate the offsets of size and size + origin
        let relative_size =
            util::usize_vec_to_isize(new_size) - util::usize_vec_to_isize(self.size);
        let corner_offset = offset + relative_size;

        // Allocate and fill array with `fill`
        let mut data: Box<_> = std::iter::repeat(fill.clone())
            .take(new_size.x * new_size.y)
            .collect();

        // Calculate bounds of the old size in the coordinate space of the new size
        let start = util::isize_vec_to_usize_saturating(-offset);
        let end = new_size - util::isize_vec_to_usize_saturating(corner_offset);

        // Copy the old data to the new array
        for y in start.y..end.y {
            let new_line_start = y * new_size.x;
            let old_line_start = (y.checked_add_signed(offset.y))
                .expect("offset.y should never be less than -y")
                * self.size.x;

            for x in start.x..end.x {
                let new_index = x + new_line_start;
                let old_index = (x.checked_add_signed(offset.x))
                    .expect("offset.x should never be less than -x")
                    + old_line_start;

                data[new_index] = self.data[old_index].clone();
            }
        }

        // Update `self` with new values
        self.data = data;
        self.size = new_size;
        self.origin += offset;
    }

    pub fn get(&self, index: Vector2<isize>) -> Option<&T> {
        Some(&self.data[self.index_of(index)?])
    }

    pub fn get_mut(&mut self, index: Vector2<isize>) -> Option<&mut T> {
        Some(&mut self.data[self.index_of(index)?])
    }

    /// Returns the index within self.data that a value is present within.
    pub fn index_of(&self, index: Vector2<isize>) -> Option<usize> {
        let absolute_index = index - self.origin;

        if absolute_index.x < 0 || absolute_index.y < 0 {
            None
        } else {
            let absolute_index = vector![absolute_index.x as usize, absolute_index.y as usize];

            if absolute_index.x >= self.size.x || absolute_index.y >= self.size.y {
                None
            } else {
                // Safety: absolute_index has been bounds checked
                let data_index = unsafe { self.vector_to_1d_index(absolute_index) };

                Some(data_index)
            }
        }
    }

    /// Returns the index within self.data that a value is present within.
    /// # Safety
    /// `index` is expected to fall within the bounds of the grid
    pub unsafe fn index_of_unchecked(&self, index: Vector2<isize>) -> usize {
        let absolute_index = index - self.origin;
        let absolute_index = vector![absolute_index.x as usize, absolute_index.y as usize];

        self.vector_to_1d_index(absolute_index)
    }

    unsafe fn vector_to_1d_index(&self, index: Vector2<usize>) -> usize {
        index.x + index.y * self.size.x
    }
}

impl<T> std::ops::Index<Vector2<isize>> for ExpandableGrid<T> {
    type Output = T;

    fn index(&self, index: Vector2<isize>) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T> std::ops::IndexMut<Vector2<isize>> for ExpandableGrid<T> {
    fn index_mut(&mut self, index: Vector2<isize>) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}
