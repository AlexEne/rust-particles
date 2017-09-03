use super::cgmath;
use super::cgmath::perspective;
use super::cgmath::SquareMatrix;
use super::cgmath::Matrix4;

pub struct Camera {
    pub view_from_world: cgmath::Matrix4<f32>,
    pub proj_from_view: cgmath::Matrix4<f32>,
    pub position: cgmath::Point3<f32>,
    pub angle_yaw: f32,
    pub angle_pitch: f32
}


impl Camera {
    pub fn new() -> Camera {
        let mut cam = Camera {
            view_from_world: Matrix4::<f32>::identity(),
            proj_from_view: Matrix4::<f32>::identity(),
            position: cgmath::Point3::<f32>::new(0.0, 300.0, -1500.0),
            angle_yaw: 0.0,
            angle_pitch: 0.0
        };

        cam.update_matrices();    

        cam
    }

    pub fn update_matrices(&mut self) {
        self.view_from_world = super::cgmath::Matrix4::look_at(self.position, 
        cgmath::Point3::<f32>::new(0.0, 0.0, 0.0), 
        cgmath::Vector3::<f32>::new(0.0, 1.0, 0.0));
        let rot = super::cgmath::Matrix4::from_angle_x(super::cgmath::Deg::<f32>(self.angle_pitch));
        self.view_from_world = self.view_from_world * rot;
        
        let rot = super::cgmath::Matrix4::from_angle_y(super::cgmath::Deg::<f32>(self.angle_yaw));
        self.view_from_world = self.view_from_world * rot;
        self.proj_from_view = perspective(cgmath::Rad(0.785398), 4.0 / 3.0, 0.5, 10000.0);
    }
}