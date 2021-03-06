use maat_graphics::DrawCall;

use crate::modules::scenes::Scene;
use crate::modules::scenes::SceneData;

use crate::modules::system_interface::{ClickUi, AutoUi};

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
  ui: ClickUi,
  auto_ui: AutoUi,
  cloud_offset: f32,
  stage: i32,
}

impl GameScreen {
  pub fn new(window_size: Vector2<f32>, model_sizes: Vec<(String, Vector3<f32>)>) -> GameScreen {
    let mut rng =  thread_rng();
    
    GameScreen {
      data: SceneData::new(window_size, model_sizes),
      rng,
      last_mouse_pos: Vector2::new(-1.0, -1.0),
      total_delta: 0.0,
      game_speed: 1.0,
      ui: ClickUi::new(window_size),
      auto_ui: AutoUi::new(window_size),
      cloud_offset: 0.0,
      stage: 1,
    }
  }
  
  pub fn new_with_data(window_size: Vector2<f32>, rng: rand::prelude::ThreadRng, model_sizes: Vec<(String, Vector3<f32>)>, game_speed: f32, ui: ClickUi, auto_ui: AutoUi, stage: i32) -> GameScreen {
    GameScreen {
      data: SceneData::new(window_size, model_sizes),
      rng,
      last_mouse_pos: Vector2::new(-1.0, -1.0),
      total_delta: 0.0,
      game_speed,
      ui,
      auto_ui,
      cloud_offset: 0.0,
      stage,
    }
  }
  
  pub fn _update_keypresses(&mut self, delta_time: f32) {
    
  }
  
  pub fn _update_controller_input(&mut self) {
    
  }
  
  pub fn update_ui(&mut self, delta_time: f32) {
    let mouse_pos = self.data.mouse_pos;
    let left_mouse = self.data.left_mouse;
    let scroll_delta = self.data.scroll_delta;
    let keys_pressed_this_frame = self.data.keys.get_pressed_this_frame();
    self.ui.update(delta_time, mouse_pos, left_mouse, &keys_pressed_this_frame, scroll_delta);
    self.auto_ui.update(delta_time, mouse_pos, left_mouse, &keys_pressed_this_frame, scroll_delta);
    
    let (seeds, fv, chopped, cooked, meal, small_buffet, regular_buffet, large_buffet) = self.auto_ui.gather_resources();
    
    self.ui.modify_seed_amount(seeds);
    self.ui.modify_fv_amount(fv);
    self.ui.modify_chopped_fv_amount(chopped);
    self.ui.modify_cooked_fv_amount(cooked);
    self.ui.modify_meal_amount(meal);
    self.ui.modify_small_buffet_amount(small_buffet);
    self.ui.modify_regular_buffet_amount(regular_buffet);
    self.ui.modify_large_buffet_amount(large_buffet);
    
    if self.auto_ui.land_lvl_pressed() {
      if self.ui.fv_count() >= 5 {
        self.ui.modify_fv_amount(-5);
        self.auto_ui.level_up_land();
      }
    }
    
    if self.auto_ui.farmer_lvl_pressed() {
      if self.ui.chopped_fv_count() >= 5 {
        self.ui.modify_chopped_fv_amount(-5);
        self.auto_ui.level_up_farmer();
      }
    }
    
    if self.auto_ui.chef_lvl_pressed() {
      if self.ui.cooked_fv_count() >= 5 {
        self.ui.modify_cooked_fv_amount(-5);
        self.auto_ui.level_up_chef();
      }
    }
    
    if self.auto_ui.cook_lvl_pressed() {
      if self.ui.meal_count() >= 5 {
        self.ui.modify_meal_amount(-5);
        self.auto_ui.level_up_cook();
      }
    }
    
    if self.auto_ui.waiter_lvl_pressed() {
      if self.ui.small_buffet_count() >= 5 {
        self.ui.modify_small_buffet_amount(-5);
        self.auto_ui.level_up_waiter();
      }
    }
    
    if self.auto_ui.serving_tray_lvl_pressed() {
      if self.ui.regular_buffet_count() >= 5 {
        self.ui.modify_regular_buffet_amount(-5);
        self.auto_ui.level_up_serving_tray();
      }
    }
    
    if self.auto_ui.food_servers_lvl_pressed() {
      if self.ui.large_buffet_count() >= 5 {
        self.ui.modify_large_buffet_amount(-5);
        self.auto_ui.level_up_food_servers();
      }
    }
    
    if self.auto_ui.food_servers_lvl_pressed() {
      if self.ui.large_buffet_count() >= 50 {
        self.ui.modify_large_buffet_amount(-50);
        self.auto_ui.level_up_catering_company();
      }
    }
    
    
    if self.ui.fv_count() >= 5 {
      self.auto_ui.show_land_option();
    }
    
    if self.ui.chopped_fv_count() >= 5 {
      self.auto_ui.show_farmer_option();
    }
    
    if self.ui.cooked_fv_count() >= 5 {
      self.auto_ui.show_chef_option();
    }
    
    if self.ui.meal_count() >= 5 {
      self.auto_ui.show_cook_option();
    }
    
    if self.ui.small_buffet_count() >= 5 {
      self.auto_ui.show_waiters_option();
    }
    
    if self.ui.regular_buffet_count() >= 5 {
      self.auto_ui.show_serving_trays_option();
    }
    
    if self.ui.large_buffet_count() >= 5 {
      self.auto_ui.show_food_servers_option();
    }
    
    if self.ui.large_buffet_count() >= 50 {
      self.auto_ui.show_catering_company_option();
    }
  }
  
  pub fn update_neutral(&mut self, real_delta: f32, delta_time: f32) {
    if self.data.window_resized {
      self.data.next_scene = true;
    }
    
    self.cloud_offset += 50.0*delta_time;
    if self.cloud_offset >= 6400.0 {
      self.cloud_offset = 0.0;
    }
    
   match self.stage {
     7 => {
       if self.ui.large_buffet_count() >= 5 {
         self.stage += 1;
       }
     },
     6 => {
       if self.ui.regular_buffet_count() >= 5 {
         self.stage += 1;
       }
     },
     5 => {
       if self.ui.small_buffet_count() >= 5 {
         self.stage += 1;
       }
     },
     4 => {
       if self.ui.meal_count() >= 5 {
         self.stage += 1;
       }
     },
     3 => {
       if self.ui.cooked_fv_count() >= 5 {
         self.stage += 1;
       }
     },
     2 => {
       if self.ui.chopped_fv_count() >= 5 {
         self.stage += 1;
       }
     },
     1 => {
       if self.ui.fv_count() >= 5 {
         self.stage += 1;
       }
     },
     _ => {}
    }
  }
  
  pub fn cloud_overlay(&self, draw_calls: &mut Vec<DrawCall>) {
    let width = self.data.window_dim.x;
    let height = self.data.window_dim.y;
    /*
    draw_calls.push(
      DrawCall::draw_textured(Vector2::new(width-width*0.25-3200.0*0.5 + 320.0 + self.cloud_offset, height-height*0.5*1.0/3.0), 
                              Vector2::new(3200.0, height*1.0/3.0),
                              90.0,
                              String::from("CloudOverlay"))
    );
    draw_calls.push(
      DrawCall::draw_textured(Vector2::new(width-width*0.25-3200.0*1.5 + 320.0 + self.cloud_offset, height-height*0.5*1.0/3.0), 
                              Vector2::new(3200.0, height*1.0/3.0),
                              90.0,
                              String::from("CloudOverlay"))
    );*/
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
      Box::new(GameScreen::new_with_data(window_size, self.rng.clone(), self.data.model_sizes.clone(), self.game_speed, self.ui.clone(), self.auto_ui.clone(), self.stage))
    } else {
      Box::new(GameScreen::new(window_size, self.data.model_sizes.clone()))
    }
  }
  
  fn update(&mut self, delta_time: f32) {
    let real_delta = delta_time;
    let delta_time = delta_time * self.game_speed as f32;
    self.mut_data().controller.update();
    self.total_delta += delta_time;
    
    self.update_ui(delta_time);
    
    self.update_neutral(real_delta, delta_time);
    
    self._update_keypresses(real_delta);
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    
    let width = self.data().window_dim.x;
    let height = self.data().window_dim.y;
    
    let mut texture = String::from("Stage1");
    match self.stage {
      8 => {
        texture = String::from("Stage8");
      },
      7 => {
        texture = String::from("Stage7");
      },
      6 => {
       texture = String::from("Stage6");
      },
      5 => {
        texture = String::from("Stage5");
      },
      4 => {
        texture = String::from("Stage4");
      },
      3 => {
        texture = String::from("Stage3");
      },
      2 => {
        texture = String::from("Stage2");
      },
      _ => {},
    }
    
    draw_calls.push(
      DrawCall::draw_textured(Vector2::new(width-width*0.25, height-height*0.5*1.0/3.0), 
                              Vector2::new(width*0.5, height*1.0/3.0),
                              90.0,
                              texture)
    );
    
    self.cloud_overlay(draw_calls);
    
    self.ui.draw(draw_calls);
    self.auto_ui.draw(draw_calls);
    
    draw_calls.push(DrawCall::draw_coloured(Vector2::new(width*0.75, height*2.0/3.0), Vector2::new(width*0.5, 10.0), Vector4::new(0.0, 0.0, 0.0, 1.0), 90.0));
    draw_calls.push(DrawCall::draw_coloured(Vector2::new(width*0.5, height*0.5), Vector2::new(10.0, height), Vector4::new(0.0, 0.0, 0.0, 1.0), 90.0));
  }
}
