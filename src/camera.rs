use three::object::Object;

const R_INIT: f32 = 3.0;
const ZOOM_RATE: f32 = 0.1;

pub struct Camera {
    pub three_camera: three::camera::Camera,
    r: f32,
    smooth_r : f32
}

impl Camera {

    pub fn new(three_factory: &mut three::Factory) -> Camera {

        let three_camera = three_factory.perspective_camera(60.0, 1.0..1000.0);

        Camera { three_camera, r: R_INIT, smooth_r: R_INIT }
    }

    pub fn update(&mut self, mouse_pos_ndc: mint::Point2<f32>, mouse_wheel: f32) {

        self.r =  (1.0 / (  (1.0/self.r) *(1.0 + 0.01* mouse_wheel ))).max(2.13);
        self.smooth_r = self.smooth_r*0.9 + self.r*0.1;

        let mint::Point2 {x: mx, y: my} = mouse_pos_ndc;
        let ax = mx * std::f32::consts::PI;
        let ay = my * std::f32::consts::PI/2.0;

        let x = self.smooth_r * f32::cos(ay) * f32::cos(ax);
        let y = self.smooth_r * f32::cos(ay) * f32::sin(ax);
        let z = self.smooth_r * f32::sin(ay);

        self.three_camera.look_at([x, y, z],
                                  [0.0, 0.0, 0.0],
                                  Some(mint::Vector3 { x: 0.0, y: 0.0, z : 1.0 } ));        
    }

}