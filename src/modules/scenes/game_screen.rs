use maat_graphics::DrawCall;

use crate::modules::scenes::Scene;
use crate::modules::scenes::SceneData;

use crate::modules::update::update_game;

use rand;
use rand::{thread_rng};

use cgmath::{InnerSpace, SquareMatrix, Matrix4, Point3, Deg, Vector2, Vector3, Vector4, PerspectiveFov};

pub struct GameScreen {
  data: SceneData,
  rng: rand::prelude::ThreadRng,
  last_mouse_pos: Vector2<f32>,
  total_delta: f32,
  game_speed: f32,
}

impl GameScreen {
  pub fn new(window_size: Vector2<f32>, model_sizes: Vec<(String, Vector3<f32>)>, map_name: String) -> GameScreen {
    println!("Game Screen");
    let mut rng =  thread_rng();
    
    GameScreen {
      data: SceneData::new(window_size, model_sizes),
      rng,
      last_mouse_pos: Vector2::new(-1.0, -1.0),
      total_delta: 0.0,
      game_speed: 1.0,
    }
  }
  
  pub fn new_with_data(window_size: Vector2<f32>, rng: rand::prelude::ThreadRng, game_speed: f32) -> GameScreen {
    GameScreen {
      data: SceneData::new(window_size, model_sizes),
      rng,
      last_mouse_pos: Vector2::new(-1.0, -1.0),
      total_delta: 0.0,
      game_speed,
    }
  }
  
  pub fn _update_keypresses(&mut self, delta_time: f32) {
    
  }
  
  pub fn _update_controller_input(&mut self) {
    
  }
  
  pub fn _update_ui(&mut self, delta_time: f32) {
    
  }
  
  pub fn update_neutral(&mut self, real_delta: f32, delta_time: f32) {
    if self.data.window_resized || self.bin >= 100 {
      self.data.next_scene = true;
    }
  }
}

impl Scene for GameScreen {
  fn data(&self) -> &SceneData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut SceneData {
    &mut self.data
  }
  
  fn future_scene(&mut self, window_size: Vector2<f32>) -> Box<Scene> {
    if self.data().window_resized {
      Box::new(GameScreen::new_with_data(window_size, self.rng.clone(), self.game_speed))
    } else {
      Box::new(GameScreen::new(window_size, self.data.model_sizes.clone()))
    }
  }
  
  fn update(&mut self, delta_time: f32) {
    let real_delta = delta_time;
    let delta_time = delta_time * self.game_speed as f32;
    self.mut_data().controller.update();
    self.total_delta += delta_time;
    
    self._update_ui(delta_time);
    
    self.update_keypresses(real_delta);
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    
  }
}
