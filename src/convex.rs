use bevy::{
    prelude::{Mesh, Vec3},
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use bevy_rapier3d::prelude::Collider;

pub fn convex_mesh(points: Vec<Vec3>) -> Option<Mesh> {
    // 通过点集构造convex hull
    // create convex collider
    let collider = Collider::convex_hull(&points)?;
    // get convex polyhedron
    let convex = collider.as_convex_polyhedron()?;
    // 取convex hull的所有面
    // get faces
    let faces = convex.raw.faces();
    // 取点集
    // get points 
    let points = convex.raw.points();
    // 取映射关系
    // get face mapping
    let face_to_vertices = convex.raw.vertices_adj_to_face();

    let mut positions = Vec::new();
    // 法向量 用于处理光照
    let mut normals = Vec::new();
    let mut indices = Vec::new();
    // 遍历所有的面
    // iterate all faces
    for face in faces {
        let i1 = face.first_vertex_or_edge;
        let i2 = i1 + face.num_vertices_or_edges;

        for idx in i1..i2 {
            let point = points[face_to_vertices[idx as usize] as usize];
            // 重新构造点集 points是原始点集
            // collect points of face
            positions.push([point.x, point.y, point.z]);
            // 面上的所有点的朝向与面相同
            // collect normals of face
            normals.push([face.normal.x, face.normal.y, face.normal.z]);
        }

        for idx in i1 + 1..i2 - 1 {
            // 构造顶点索引
            // create indices
            indices.push([i1, idx as u32, idx + 1 as u32]);
        }
    }
    // 构造Mesh
    // create bevy mesh
    Some(
        Mesh::new(PrimitiveTopology::TriangleList)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
            .with_indices(Some(Indices::U32(indices.concat()))),
    )
}
