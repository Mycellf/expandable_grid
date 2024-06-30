#![cfg(test)]

use crate::expandable_grid::ExpandableGrid;
use nalgebra::{vector, Vector2};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

#[test]
fn grid_expands_to_fit_points() {
    let radius = 1000;

    let range_size = radius * 2 + 1;
    let range = {
        let radius = radius as isize;
        -radius..=radius
    };

    let mut seed_rng = ChaCha8Rng::seed_from_u64(10);
    for _ in 0..10 {
        println!("\tstart of iteration");
        let seed = seed_rng.gen();
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut grid = ExpandableGrid::new();
        for _ in 0..100 {
            let new_point = vector![rng.gen_range(range.clone()), rng.gen_range(range.clone())];

            grid.expand_to_fit_point(new_point, &0);

            println!("{new_point:?}, {:?}", grid.size);
            assert!(
                grid.index_of(new_point).is_some(),
                "test seed: {seed}, point {new_point:?} shoud be within size",
            );
        }
        assert!(
            grid.size.max() <= range_size * 4,
            "test seed: {seed}, size {:?} is too big",
            grid.size,
        );
    }
}

#[test]
fn grid_expands_to_fit_boxes() {
    // Targeted test
    {
        let mut grid = ExpandableGrid::with_size(vector![1, 1], vector![0, 0], &0);

        let box_origin = vector![-10, -10];
        let box_size = vector![20, 20];
        grid.expand_to_fit_box(box_origin, box_size, &0);

        for corner in corners_of_box(box_origin, box_size) {
            assert!(
                grid.index_of(corner).is_some(),
                "point {box_origin:?} of \
                    box with origin: {box_origin:?}, size: {box_size:?} shoud be within size",
            );
        }
    }

    // Random tests
    let placement_radius = 1000;
    let max_box_size = 1000;

    let placement = {
        let radius = placement_radius as isize;
        -radius..=radius
    };
    let box_size = 1..=max_box_size;

    let placement_range = (placement_radius + max_box_size) * 2 + 1;

    let mut seed_rng = ChaCha8Rng::seed_from_u64(10);
    for _ in 0..10 {
        println!("\tstart of iteration");
        let seed = seed_rng.gen();
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut grid = ExpandableGrid::new();

        for _ in 0..100 {
            let box_origin = vector![
                rng.gen_range(placement.clone()),
                rng.gen_range(placement.clone()),
            ];
            let box_size = vector![
                rng.gen_range(box_size.clone()),
                rng.gen_range(box_size.clone()),
            ];

            grid.expand_to_fit_box(box_origin, box_size, &0);

            println!("{box_origin:?}, {:?}", grid.size);

            println!("size: {:?}, {:?}, {:?}", grid.size, grid.origin, box_size);
            for corner in corners_of_box(box_origin, box_size) {
                assert!(
                    grid.index_of(corner).is_some(),
                    "test seed: {seed}, point {box_origin:?} of \
                        box with origin: {box_origin:?}, size: {box_size:?} shoud be within size",
                );
            }
        }

        assert!(
            grid.size.max() <= placement_range * 4,
            "test seed: {seed}, size {:?} is too big",
            grid.size,
        );
    }
}

fn corners_of_box(origin: Vector2<isize>, size: Vector2<usize>) -> [Vector2<isize>; 4] {
    let corner = origin + vector![size.x as isize, size.y as isize] - vector![1, 1];
    [
        origin,
        vector![origin.x, corner.y],
        vector![corner.x, origin.y],
        corner,
    ]
}
