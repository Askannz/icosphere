use std::f32;
use std::f32::consts::PI;
use mint::{Point3, Vector3};

pub struct Icosphere {
    pub faces: Vec<Face>,
    iteration: usize
} 

pub struct Face {
    pub points: [Point3<f32>; 3]
} 

impl Icosphere {

    pub fn make_base() -> Icosphere {

        let mut faces = Vec::new();

        let top = Point3::from_slice(&[0.0, 0.0, 1.0]);
        let bottom = Point3::from_slice(&[0.0, 0.0, -1.0]);

        let points_ring_top = make_points_ring(true);
        let points_ring_bottom = make_points_ring(false);
    
        // Top 5 faces
        let it1 = points_ring_top.iter();
        let it2 = points_ring_top.iter().cycle().skip(1).take(5);
        for (p1, p2) in it1.zip(it2) {
            faces.push(Face { points: [top, p1.clone(), p2.clone()] });
        }

        // Middle-top 5 faces
        let it1 = points_ring_top.iter();
        let it2 = points_ring_bottom.iter();
        let it3 = points_ring_top.iter().cycle().skip(1).take(5);
        for ((p1, p2), p3) in it1.zip(it2).zip(it3) {
            faces.push(Face { points: [p1.clone(), p2.clone(), p3.clone()] });
        }

        // Middle-bottom 5 faces
        let it1 = points_ring_top.iter();
        let it2 = points_ring_bottom.iter().cycle().skip(4).take(5);
        let it3 = points_ring_bottom.iter();
        for ((p1, p2), p3) in it1.zip(it2).zip(it3) {
            faces.push(Face { points: [p1.clone(), p2.clone(), p3.clone()] });
        }

        // Bottom 5 faces
        let it1 = points_ring_bottom.iter();
        let it2 = points_ring_bottom.iter().cycle().skip(1).take(5);
        for (p1, p2) in it1.zip(it2) {
            faces.push(Face { points: [p1.clone(), bottom, p2.clone()] });
        }

        Icosphere { faces, iteration: 0 }

    }

    pub fn iterate_construction(&self) -> Icosphere {

        let mut new_faces: Vec<Face> = Vec::new();
        for face in self.faces.iter() {
            new_faces.append(&mut subdivide_face(face));
        }

        Icosphere { faces: new_faces, iteration: self.iteration + 1 }

    }

}

fn make_points_ring(top: bool) -> Vec<Point3<f32>> {

    let a = f32::atan(0.5);
    let delta_b = 2.0 * PI / 5.0;

    let mut points = Vec::new();

    let (z, b0) = if top {
                    (f32::sin(a), 0.0)
                    } else {
                    (-f32::sin(a), delta_b / 2.0)
                    };

    for i in 0..5 {

        let b = (i as f32) * delta_b + b0;

        let x = f32::cos(a) * f32::cos(b);
        let y = f32::cos(a) * f32::sin(b);

        points.push([x, y, z].into());

    }
    points
}

fn subdivide_face(face: &Face) -> Vec<Face> {

    let [p1, p2, p3] = &face.points;

    let pm1 = subdivide_edge(*p1, *p2);
    let pm2 = subdivide_edge(*p2, *p3);
    let pm3 = subdivide_edge(*p3, *p1);

    let face1 = Face { points: [*p1, pm1, pm3] };
    let face2 = Face { points: [pm1, *p2, pm2] };
    let face3 = Face { points: [pm3, pm2, *p3] };
    let face4 = Face { points: [pm1, pm2, pm3] };

    vec![face1, face2, face3, face4]

}

fn subdivide_edge(pa: Point3<f32>, pb: Point3<f32>) -> Point3<f32> {

    let v_m_ab = Vector3 {
        x: 0.5 * (pa.x + pb.x),
        y: 0.5 * (pa.y + pb.y),
        z: 0.5 * (pa.z + pb.z),
    };

    let norm = f32::sqrt(f32::powi(v_m_ab.x, 2) + 
                         f32::powi(v_m_ab.y, 2) +
                         f32::powi(v_m_ab.z, 2));

    Point3 {
        x: v_m_ab.x / norm,
        y: v_m_ab.y / norm,
        z: v_m_ab.z / norm
    }
}