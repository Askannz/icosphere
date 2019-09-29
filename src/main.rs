mod camera;
mod icosphere;
use three::Object;
use icosphere::Icosphere;

const NB_ITERATIONS: usize = 6;

fn main() {

    let mut window = three::Window::new("Icosphere");

    let mut camera = camera::Camera::new(&mut window.factory);
    window.scene.add(&camera.three_camera);

    let mut meshes_list = Vec::new();
    for icosphere in make_icospheres().iter() {
        let mesh = make_visualization_mesh(&mut window.factory, icosphere);
        window.scene.add(&mesh);
        mesh.set_visible(false);
        meshes_list.push(mesh);
    }

    meshes_list[0].set_visible(true);

    let mut mesh_index = 0;
    let mut key_down = false;

    while window.update() && !window.input.hit(three::KEY_ESCAPE) {

        if window.input.hit(three::KEY_SPACE) {
            if !key_down {
                mesh_index = (mesh_index + 1) % NB_ITERATIONS;
                for (i, mesh) in meshes_list.iter().enumerate() {
                    mesh.set_visible(i == mesh_index);
                }
                key_down = true;
            }
        } else {
            key_down = false;
        }

        camera.update(window.input.mouse_pos_ndc(), window.input.mouse_wheel());
        window.render(&camera.three_camera);

    }

}

fn make_icospheres() -> Vec<Icosphere> {

    let mut icospheres_list = Vec::new();

    let mut current = Icosphere::make_base();

    for _i in 0..NB_ITERATIONS {
        let new = current.iterate_construction();
        icospheres_list.push(current);
        current = new;
    }

    icospheres_list

}

fn make_visualization_mesh(three_factory: &mut three::Factory, icosphere: &Icosphere) -> three::Mesh {

    let mut mesh_vertices = Vec::new();
    let mut mesh_faces_indices = Vec::new();

    for (i, face) in icosphere.faces.iter().enumerate() {
        mesh_vertices.extend(face.points.iter());
        let i = i as u32;
        mesh_faces_indices.push([3*i, 3*i+1, 3*i+2]);
    }

    let geometry = three::Geometry {
        faces: mesh_faces_indices,
        base: three::Shape {
            vertices: mesh_vertices,
            .. three::Shape::default()
        },
        .. three::Geometry::default()
    };
    
    let material = three::material::Wireframe { color: 0xFFFFFF };


    three_factory.mesh(geometry, material)
}