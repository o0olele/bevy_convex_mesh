use std::ops::{MulAssign, Sub};

use bevy::{prelude::{Vec3, Mesh}, render::{render_resource::PrimitiveTopology, mesh::Indices}};

pub fn triangle_mesh(mut points: Vec<Vec3>, mut indices: Vec<u32>) -> Option<Mesh> {
    let detail = 1;
    let cols: i32 = detail + 1;
    let mut list: Vec<Vec3> = Vec::new();
    let mut list_indices: Vec<u32> = Vec::new();

    list.resize(((cols + 1) * (cols + 1)) as usize, Vec3::ZERO);

    // divide triangles
    indices.chunks(3).for_each(|v| {
        let a = points.get(v[0] as usize).unwrap();
        let b = points.get(v[1] as usize).unwrap();
        let c = points.get(v[2] as usize).unwrap();

        for i in 0..=cols {
            let aj = a.clone().lerp(c.clone(), i as f32 / cols as f32);
            let bj = b.clone().lerp(c.clone(), i as f32 / cols as f32);

            for j in 0..=(cols - 1) {
                if j == 0 && i == cols {
                    list[(i * (cols - 1)) as usize] = aj;
                } else {
                    list[(i * (cols - 1)) as usize] = aj.lerp(bj, j as f32 / (cols - 1) as f32);
                }
            }
        }

        for i in 0..cols {
            for j in 0..(2 * (cols - i) - 1) {
                let k = (j as f32 / 2.).floor() as i32;

                list_indices.push(points.len() as u32);
                list_indices.push(points.len() as u32);
                list_indices.push(points.len() as u32);

                if j % 2 == 0 {
                    points.push(list[(i * (cols - 1) + k + 1) as usize]);
                    points.push(list[((i + 1) * (cols - 1) + k) as usize]);
                    points.push(list[(i * (cols - 1) + k) as usize]);
                } else {
                    points.push(list[(i * (cols - 1) + k + 1) as usize]);
                    points.push(list[((i + 1) * (cols - 1) + k + 1) as usize]);
                    points.push(list[((i + 1) * (cols - 1) + k) as usize]);
                }
            }
        }
    });

    points.iter_mut().for_each(|v| {
        v.normalize().mul_assign(1.);
    });

    let mut normals = points.clone();
    if detail == 0 {
        let mut cnt: usize = 0;
        points.array_chunks::<3>().for_each(|v| {
            let cb = v[2].sub(v[1]);
            let ab = v[0].sub(v[1]);
            let cross = cb.cross(ab).normalize();

            normals[cnt] = cross;
            normals[cnt + 1] = cross;
            normals[cnt + 2] = cross;

            cnt += 3;
        });
    } else {
        normals
            .iter_mut()
            .for_each(|v| v.clone_from(&v.normalize()));
    }

    list_indices.append(&mut indices);
    Some(
        Mesh::new(PrimitiveTopology::TriangleList)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, points)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
            .with_indices(Some(Indices::U32(list_indices))),
    )
}
