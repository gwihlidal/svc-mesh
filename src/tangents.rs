pub fn calculate_tangents(
    positions: &[[f32; 3]],
    normals: &[[f32; 3]],
    tex_coords: &[[f32; 2]],
) -> Vec<[f32; 3]> {
    generate_tangents(positions, normals, tex_coords)
        .iter()
        .map(|(_, t)| [t[0], t[1], t[2]])
        .collect()
}

fn generate_tangents(
    positions: &[[f32; 3]],
    normals: &[[f32; 3]],
    tex_coords: &[[f32; 2]],
) -> Vec<(usize, [f32; 4])> {
    let vertices_per_face = || 3;
    let face_count = || positions.len() / 3;
    let position = |face, vert| &positions[face * 3 + vert];
    let normal = |face, vert| &normals[face * 3 + vert];
    let tx = |face, vert| &tex_coords[face * 3 + vert];
    let mut tangents: Vec<(usize, [f32; 4])> = Vec::with_capacity(positions.len());

    {
        let mut set_tangent = |face, vert, tangent| {
            let index = face * 3 + vert;
            if let Err(pos) = tangents.binary_search_by(|probe| probe.0.cmp(&index)) {
                tangents.insert(pos, (index, tangent));
            }
        };
        mikktspace::generate_tangents(
            &vertices_per_face,
            &face_count,
            &position,
            &normal,
            &tx,
            &mut set_tangent,
        );
    }

    tangents
}