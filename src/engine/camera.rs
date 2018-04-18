use cgmath::prelude::*;
use cgmath::{Deg, perspective};
use glfw::{Action, Key, Window};
use lang::{Point3, Vector3, Matrix4, Direction};
use super::input::{InputControl, KeyEvent, MouseEvent};

pub struct Camera {
    // Camera Attributes
    pub position: Point3,
    pub front: Vector3,
    pub up: Vector3,
    pub right: Vector3,
    pub world_up: Vector3,
    pub near: f32,
    pub far: f32,

    // Euler Angles
    pub yaw: f32,
    pub pitch: f32,
    pub constrain_pitch: bool,

    // Camera options
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
    pub zoom: f32,
}

impl Default for Camera {
    fn default() -> Camera {
        let mut camera = Camera {
            position: Point3::new(0.0, 0.0, 3.0),
            front: Vector3::new(0.0, 0.0, -1.0),
            up: Vector3::zero(),
            right: Vector3::zero(),
            world_up: Vector3::unit_y(),
            near: 0.1,
            far: 100.0,
            yaw: -90.0,
            pitch: 0.0,
            constrain_pitch: true,
            movement_speed: 2.5,
            mouse_sensitivity: 0.1,
            zoom: 45.0,
        };
        camera.update_vectors();
        camera
    }
}

impl Camera {
    /// Returns the view matrix calculated using Eular Angles and the LookAt Matrix
    pub fn view_matrix(&self) -> Matrix4 {
        Matrix4::look_at(self.position, self.position + self.front, self.up)
    }

    pub fn projection_matrix(&self, width: i32, height: i32) -> Matrix4 {
        perspective(Deg(self.zoom), width as f32 / height as f32, self.near, self.far)
    }

    /// Calculates the front vector from the Camera's (updated) Eular Angles
    pub fn update_vectors(&mut self) {
        // Calculate the new Front vector
        let front = Vector3 {
            x: self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            y: self.pitch.to_radians().sin(),
            z: self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        };
        self.front = front.normalize();
        // Also re-calculate the Right and Up vector
        self.right = self.front.cross(self.world_up).normalize(); // Normalize the vectors, because their length gets closer to 0 the more you look up or down which results in slower movement.
        self.up = self.right.cross(self.front).normalize();
    }

    pub fn movement(&mut self, direction: Direction, delta_time: f32) {
        match direction {
            Direction::FORWARD => {
                self.position += self.front * self.movement_speed * delta_time;
            },
            Direction::BACKWARD => {
                self.position += -(self.front * self.movement_speed * delta_time);
            },
            Direction::LEFT => {
                self.position += -(self.right * self.movement_speed * delta_time);
            },
            Direction::RIGHT => {
                self.position += self.right * self.movement_speed * delta_time;
            }
        }
    }
}

impl InputControl for Camera {
    fn on_mouse(&mut self, mouse: MouseEvent, _delta_time: f32) {
        if mouse.is_scroll {
            // Processes input received from a mouse scroll-wheel event.
            // Only requires input on the vertical wheel-axis

            if self.zoom >= 1.0 && self.zoom <= 45.0 {
                self.zoom -= mouse.y_offset;
            }
            if self.zoom <= 1.0 {
                self.zoom = 1.0;
            }
            if self.zoom >= 45.0 {
                self.zoom = 45.0;
            }
        } else {
            // Mouse cursor pos event

            let x_offset = mouse.x_offset * self.mouse_sensitivity;
            let y_offset = mouse.y_offset * self.mouse_sensitivity;

            self.yaw += x_offset;
            self.pitch += y_offset;

            // Make sure that when pitch is out of bounds, screen doesn't get flipped
            if self.constrain_pitch {
                if self.pitch > 89.0 {
                    self.pitch = 89.0;
                }
                if self.pitch < -89.0 {
                    self.pitch = -89.0;
                }
            }

            // Update Front, Right and Up Vectors using the updated Eular angles
            self.update_vectors();
        }
    }

    fn on_keyboard(&mut self, _key: KeyEvent, _delta_time: f32) {
    }

    fn on_input(&mut self, window: &Window, delta_time: f32) {
        if window.get_key(Key::W) == Action::Press {
            self.movement(Direction::FORWARD, delta_time);
        }
        if window.get_key(Key::S) == Action::Press {
            self.movement(Direction::BACKWARD, delta_time);
        }
        if window.get_key(Key::A) == Action::Press {
            self.movement(Direction::LEFT, delta_time);
        }
        if window.get_key(Key::D) == Action::Press {
            self.movement(Direction::RIGHT, delta_time);
        }
    }
}