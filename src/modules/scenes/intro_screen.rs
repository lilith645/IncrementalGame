use maat_graphics::DrawCall;

use crate::modules::scenes::Scene;
use crate::modules::scenes::SceneData;
use crate::modules::scenes::GameScreen;

use crate::modules::system_interface::Button;

use cgmath::{Vector2, Vector3, Vector4};

pub struct IntroScreen {
  data: SceneData,
  start_button: Button
}

impl IntroScreen {
  pub fn new(window_size: Vector2<f32>, model_sizes: Vec<(String, Vector3<f32>)>) -> IntroScreen {
    IntroScreen {
      data: SceneData::new(window_size, model_sizes),
      start_button: Button::new_button("Godlike".to_string(), 
                               Vector2::new(window_size.x*0.5, window_size.y*0.25), 
                               Vector2::new(window_size.x*0.6, window_size.y*0.3), 
                               Vector2::new(window_size.x*0.3,  window_size.y*0.075),
                               Vector2::new(512.0, 512.0), 
                               Vector4::new(1.0, 0.0, 1.0, 1.0), 
                               Vector4::new(0.4, 0.0, 0.0, 1.0),
                               Vector4::new(0.8, 0.2, 0.2, 1.0), 
                               false,
                               true,
                               "Play God!".to_string(),
                               "Arial".to_string()) ,
    }
  }
}

impl Scene for IntroScreen {
  fn data(&self) -> &SceneData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut SceneData {
    &mut self.data
  }
  
  fn future_scene(&mut self, window_size: Vector2<f32>) -> Box<Scene> {
    Box::new(GameScreen::new(window_size, self.data.model_sizes.clone()))
  }
  
  fn update(&mut self, delta_time: f32) {
    let mouse_pos = self.data().mouse_pos;
    let left_mouse = self.data().left_mouse;
    
    self.start_button.update(delta_time, mouse_pos, left_mouse);
    
    if self.start_button.is_pressed() {
      self.data.next_scene = true;
    } 
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    let dim = self.data().window_dim;
    let (width, height) = (dim.x as f32, dim.y as f32);
    let mut text_spacing = 32.0;
    draw_calls.push(
      DrawCall::draw_text_basic_centered(Vector2::new(width*0.5, height-100.0), 
                                Vector2::new(128.0, 128.0),
                                Vector4::new(0.0, 0.0, 0.0, 1.0),
                                String::from("Welcome to incremental kitchen escape!"),
                                String::from("Arial"))
    );
    draw_calls.push(
      DrawCall::draw_text_basic_centered(Vector2::new(width*0.5, height-100.0-text_spacing), 
                                Vector2::new(96.0, 96.0),
                                Vector4::new(0.0, 0.0, 0.0, 1.0),
                                String::from("You have been summoned by the iseaki gods to serve the scarce people of earth."),
                                String::from("Arial"))
    );
    draw_calls.push(
      DrawCall::draw_text_basic_centered(Vector2::new(width*0.5, height-100.0-text_spacing*2.0), 
                                Vector2::new(96.0, 96.0),
                                Vector4::new(0.0, 0.0, 0.0, 1.0),
                                String::from("They are all working very hard to create a rocket ship to leave this polluted planet for good."),
                                String::from("Arial"))
    );
    draw_calls.push(
      DrawCall::draw_text_basic_centered(Vector2::new(width*0.5, height-100.0-text_spacing*3.0), 
                                Vector2::new(92.0, 92.0),
                                Vector4::new(0.0, 0.0, 0.0, 1.0),
                                String::from("Alas they face a dire problem, they kneel before the human need of hunger, and must be fed in order work."),
                                String::from("Arial"))
    );
    draw_calls.push(
      DrawCall::draw_text_basic_centered(Vector2::new(width*0.5, height-100.0-text_spacing*4.0), 
                                Vector2::new(96.0, 96.0),
                                Vector4::new(0.0, 0.0, 0.0, 1.0),
                                String::from("They have desprately summoned a god from another world in order to feed them."),
                                String::from("Arial"))
    );
    
    self.start_button.draw(draw_calls);
  }
}
