use crate::structs::*;
use std::cmp;

fn mut_two<T>(
    first_index: usize,
    second_index: usize,
    items: &mut [Option<T>],
) -> Option<(&mut T, &mut T)> {
    assert!(first_index != second_index);
    let split_at_index = cmp::max(first_index, second_index);
    let (first_slice, second_slice) = items.split_at_mut(split_at_index);

    if first_index < second_index {
        if let Some(item1) = &mut first_slice[first_index] {
            if let Some(item2) = &mut second_slice[0] {
                Some((item1, item2))
            } else {
                None
            }
        } else {
            None
        }
    } else {
        if let Some(item1) = &mut second_slice[0] {
            if let Some(item2) = &mut first_slice[second_index] {
                Some((item1, item2))
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub fn update(
    springs: &Vec<Option<Spring>>,
    positions: &Vec<Option<Position>>,
    velocities: &Vec<Option<Velocity>>,
    forces: &mut Vec<Option<Force>>,
) {
    for (index, s) in springs.iter().enumerate() {
        if let Some(spring) = s {
            if let ID::Num(n1_id) = &spring.to {
                if let ID::Num(n2_id) = &spring.from {
                    if let Some(pos1) = &positions[*n1_id as usize] {
                        if let Some(vel1) = &velocities[*n1_id as usize] {
                            if let Some(pos2) = &positions[*n2_id as usize] {
                                if let Some(vel2) = &velocities[*n2_id as usize] {
                                    if let Some((force1, force2)) =
                                        mut_two(*n1_id as usize, *n2_id as usize, &mut forces[..])
                                    {
                                        let pos1vec: Vector2d = pos1.to_vec2d();
                                        let pos2vec: Vector2d = pos2.to_vec2d();

                                        let vel1vec: Vector2d = vel1.to_vec2d();
                                        let vel2vec: Vector2d = vel2.to_vec2d();

                                        let mut distance = pos1vec.distance_to_vec(&pos2vec);

                                        if distance < 1.0 {
                                            distance = 1.0;
                                        }

                                        let norm1: Vector2d = pos2vec.sub_vec(&pos1vec).normalize();
                                        let norm2: Vector2d = pos1vec.sub_vec(&pos2vec).normalize();

                                        let v1: Vector2d = vel1vec.sub_vec(&vel2vec);
                                        let v2: Vector2d = vel2vec.sub_vec(&vel1vec);

                                        let stiffness_x_d = 0.08 * (distance - 150.0);

                                        let fx1 =
                                            stiffness_x_d * (norm1.x / distance) - 0.003 * v1.x;
                                        let fy1 =
                                            stiffness_x_d * (norm1.y / distance) - 0.003 * v1.y;

                                        let fx2 =
                                            stiffness_x_d * (norm2.x / distance) - 0.003 * v2.x;
                                        let fy2 =
                                            stiffness_x_d * (norm2.y / distance) - 0.003 * v2.y;

                                        force1.x += fx1;
                                        force1.y += fy1;

                                        force2.x += fx2;
                                        force2.y += fy2;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
