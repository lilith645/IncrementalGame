 use maat_graphics::DrawCall;

use crate::modules::system_interface::Widget;

use cgmath::Vector2;
use cgmath::Vector4;

const MAIN_WINDOW_INDEX: usize = 0;

const MAIN_WINDOW_NAME: &str = "MainWindow"; 

const SEED_BUTTON_NAME: &str = "SeedB";
const SEED_TEXTFIELD_NAME: &str = "SeedTF";
const FV_BUTTON_NAME: &str = "FVB";
const FV_TEXTFIELD_NAME: &str = "FVTF";
const CHOPPED_BUTTON_NAME: &str = "ChoppedB";
const CHOPPED_TEXTFIELD_NAME: &str = "ChoppedTF";
const COOKED_BUTTON_NAME: &str = "CookedVB";
const COOKED_TEXTFIELD_NAME: &str = "CookedTF";
const MEAL_BUTTON_NAME: &str = "MealB";
const MEAL_TEXTFIELD_NAME: &str = "mealTF";
const SMALL_B_BUTTON_NAME: &str = "SmallBVB";
const SMALL_B_TEXTFIELD_NAME: &str = "SmallBTF";
const REGULAR_B_BUTTON_NAME: &str = "RegularB";
const REGULAR_B_TEXTFIELD_NAME: &str = "RegularBTF";
const LARGE_B_BUTTON_NAME: &str = "LargeBVB";
const LARGE_B_TEXTFIELD_NAME: &str = "LargeBTF";

const SEED_TIMER: f32 = 0.1;
const FV_TIMER: f32 = 0.2;
const CHOPPED_TIMER: f32 = 0.4;
const COOKED_TIMER: f32 = 0.8;
const MEAL_TIMER: f32 = 1.6;
const SMALL_B_TIMER: f32 = 3.2;
const REGULAR_B_TIMER: f32 = 6.4;
const LARGE_B_TIMER: f32 = 1.28;

#[derive(Clone)]
pub struct ClickUi {
  hidden: bool,
  widgets: Vec<Widget>,
  seeds: i32,
  seed_cooldown: f32,
  fv: i32,
  fv_cooldown: f32,
  chopped_fv: i32,
  chopped_fv_cooldown: f32,
  cooked_fv: i32,
  cooked_fv_cooldown: f32,
  meals: i32,
  meal_cooldown: f32,
  small_buffet: i32,
  small_buffet_cooldown: f32,
  regular_buffet: i32,
  regular_buffet_cooldown: f32,
  large_buffet: i32,
  large_buffet_cooldown: f32,
}

impl ClickUi {
  pub fn new(window_size: Vector2<f32>) -> ClickUi {
    let mut widgets = Vec::new();
    
    let button_width = 128.0;
    let button_height = 32.0;
    
    let widget_bezel = 25.0;
    
    let widget_size = Vector2::new(window_size.x*0.5, window_size.y);
    
    let button_start_y = window_size.y;
    
    let text_colour = Vector4::new(0.0, 0.0, 0.0, 1.0);
    let button_colour = Vector4::new(0.211372549, 0.385490196, 0.684313725, 1.0);
    let pressed_colour = Vector4::new(0.2, 0.2, 0.2, 1.0);
    let button_size = Vector2::new(button_width, button_height);
    let text_size = Vector2::new(96.0, 96.0);
    let text_offset = Vector2::new(button_width*0.5, button_height*0.2);
    
    let resource_x = widget_size.x - button_width*1.5;
    let resource_start_y = button_start_y - 15.0;
    
    let button_y_offset = button_height*2.0;
    
    widgets.push(Widget::new(MAIN_WINDOW_NAME.to_string(), 
                             widget_size*0.5, 
                             widget_size, 
                             Vector4::new(0.337254902, 0.662745098, 0.788235294, 1.0))
                 //.start_hidden()
                 
                 // SEEDS
                 .with_button(Vector2::new(button_width*1.0, button_start_y - button_y_offset),
                              button_size,
                              text_offset, 
                              text_size,
                              SEED_BUTTON_NAME.to_string(), 
                              text_colour, 
                              pressed_colour,
                              button_colour,
                              true, "Get Seed".to_string(), "Arial".to_string())
                 .with_text_field_centered(SEED_TEXTFIELD_NAME.to_string(), 
                                  Vector2::new(resource_x, resource_start_y - button_y_offset), 
                                  text_size, 
                                  text_colour, 
                                  "Seeds: 0".to_string(), "Arial".to_string())
                 
                 // Fruit and Veg
                 .with_button(Vector2::new(button_width*1.0, button_start_y - button_y_offset*2.0),
                              button_size,
                              text_offset, 
                              text_size,
                              FV_BUTTON_NAME.to_string(), 
                              text_colour, 
                              pressed_colour,
                              button_colour,
                              true, "Grow Fruit and Veg".to_string(), "Arial".to_string())
                 .with_text_field_centered(FV_TEXTFIELD_NAME.to_string(), 
                                  Vector2::new(resource_x, resource_start_y - button_y_offset*2.0), 
                                  text_size, 
                                  text_colour, 
                                  "Fruit & Veg (FV): 0".to_string(), "Arial".to_string())
                
                // Chopped FV
                 .with_button(Vector2::new(button_width*1.0, button_start_y - button_y_offset*3.0),
                              button_size,
                              text_offset, 
                              text_size,
                              CHOPPED_BUTTON_NAME.to_string(), 
                              text_colour, 
                              pressed_colour,
                              button_colour,
                              true, "Chop FV".to_string(), "Arial".to_string())
                 .with_text_field_centered(CHOPPED_TEXTFIELD_NAME.to_string(), 
                                  Vector2::new(resource_x, resource_start_y - button_y_offset*3.0), 
                                  text_size, 
                                  text_colour, 
                                  "Chopped FV: 0".to_string(), "Arial".to_string())
                                  
                // Cooked FV
                 .with_button(Vector2::new(button_width*1.0, button_start_y - button_y_offset*4.0),
                              button_size,
                              text_offset, 
                              text_size,
                              COOKED_BUTTON_NAME.to_string(), 
                              text_colour, 
                              pressed_colour,
                              button_colour,
                              true, "Cook FV".to_string(), "Arial".to_string())
                 .with_text_field_centered(COOKED_TEXTFIELD_NAME.to_string(), 
                                  Vector2::new(resource_x, resource_start_y - button_y_offset*4.0), 
                                  text_size, 
                                  text_colour, 
                                  "Cooked FV: 0".to_string(), "Arial".to_string())
                
                // Meal
                 .with_button(Vector2::new(button_width*1.0, button_start_y - button_y_offset*5.0),
                              button_size,
                              text_offset, 
                              text_size,
                              MEAL_BUTTON_NAME.to_string(), 
                              text_colour, 
                              pressed_colour,
                              button_colour,
                              true, "Make Meal".to_string(), "Arial".to_string())
                 .with_text_field_centered(MEAL_TEXTFIELD_NAME.to_string(), 
                                  Vector2::new(resource_x, resource_start_y - button_y_offset*5.0), 
                                  text_size, 
                                  text_colour, 
                                  "Meals: 0".to_string(), "Arial".to_string())
                // Small buffet
                .with_button(Vector2::new(button_width*1.0, button_start_y - button_y_offset*6.0),
                              button_size,
                              text_offset, 
                              text_size,
                              SMALL_B_BUTTON_NAME.to_string(), 
                              text_colour, 
                              pressed_colour,
                              button_colour,
                              true, "Create Small Buffet".to_string(), "Arial".to_string())
                .with_text_field_centered(SMALL_B_TEXTFIELD_NAME.to_string(), 
                                  Vector2::new(resource_x, resource_start_y - button_y_offset*6.0), 
                                  text_size, 
                                  text_colour, 
                                  "Small buffets: 0".to_string(), "Arial".to_string())
                // Regular buffet
                .with_button(Vector2::new(button_width*1.0, button_start_y - button_y_offset*7.0),
                              button_size,
                              text_offset, 
                              text_size,
                              REGULAR_B_BUTTON_NAME.to_string(), 
                              text_colour, 
                              pressed_colour,
                              button_colour,
                              true, "Create Regular Buffet".to_string(), "Arial".to_string())
                .with_text_field_centered(REGULAR_B_TEXTFIELD_NAME.to_string(), 
                                  Vector2::new(resource_x, resource_start_y - button_y_offset*7.0), 
                                  text_size, 
                                  text_colour, 
                                  "Regular buffets: 0".to_string(), "Arial".to_string())
                
                // Large buffet
                .with_button(Vector2::new(button_width*1.0, button_start_y - button_y_offset*8.0),
                              button_size,
                              text_offset, 
                              text_size,
                              LARGE_B_BUTTON_NAME.to_string(), 
                              text_colour, 
                              pressed_colour,
                              button_colour,
                              true, "Create Large Buffet".to_string(), "Arial".to_string())
                .with_text_field_centered(LARGE_B_TEXTFIELD_NAME.to_string(), 
                                  Vector2::new(resource_x, resource_start_y - button_y_offset*8.0), 
                                  text_size, 
                                  text_colour, 
                                  "Large buffets: 0".to_string(), "Arial".to_string())
                );
    
    widgets[MAIN_WINDOW_INDEX].set_button_hidden(&FV_BUTTON_NAME.to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&FV_TEXTFIELD_NAME.to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_button_hidden(&CHOPPED_BUTTON_NAME.to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&CHOPPED_TEXTFIELD_NAME.to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_button_hidden(&COOKED_BUTTON_NAME.to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&COOKED_TEXTFIELD_NAME.to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_button_hidden(&MEAL_BUTTON_NAME.to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&MEAL_TEXTFIELD_NAME.to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_button_hidden(&SMALL_B_BUTTON_NAME.to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&SMALL_B_TEXTFIELD_NAME.to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_button_hidden(&REGULAR_B_BUTTON_NAME.to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&REGULAR_B_TEXTFIELD_NAME.to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_button_hidden(&LARGE_B_BUTTON_NAME.to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&LARGE_B_TEXTFIELD_NAME.to_string(), true);
    
    ClickUi {
      hidden: true,
      widgets: widgets,
      seeds: 0,
      seed_cooldown: SEED_TIMER,
      fv: 0,
      fv_cooldown: FV_TIMER,
      chopped_fv: 0,
      chopped_fv_cooldown: SEED_TIMER,
      cooked_fv: 0,
      cooked_fv_cooldown: FV_TIMER,
      meals: 0,
      meal_cooldown: SEED_TIMER,
      small_buffet: 0,
      small_buffet_cooldown: FV_TIMER,
      regular_buffet: 0,
      regular_buffet_cooldown: SEED_TIMER,
      large_buffet: 0,
      large_buffet_cooldown: FV_TIMER,
    }
  }
  
  pub fn show(&mut self) {
    for widget in &mut self.widgets {
      widget.show();
    }
    self.hidden = false;
  }
  
  pub fn hide(&mut self) {
    for widget in &mut self.widgets {
      widget.hide();
    }
    self.hidden = true;
  }
  
  pub fn is_hidden(&self) -> bool {
    self.hidden
  }
  
  pub fn seed_count(&self) -> i32 {
    self.seeds
  }
  
  pub fn fv_count(&self) -> i32 {
    self.fv
  }
  
  pub fn chopped_fv_count(&self) -> i32 {
    self.chopped_fv
  }
  
  pub fn cooked_fv_count(&self) -> i32 {
    self.cooked_fv
  }
  
  pub fn meal_count(&self) -> i32 {
    self.meals
  }
  
  pub fn small_buffet_count(&self) -> i32 {
    self.small_buffet
  }
  
  pub fn regular_buffet_count(&self) -> i32 {
    self.regular_buffet
  }
  
  pub fn large_buffet_count(&self) -> i32 {
    self.large_buffet
  }
  
  pub fn modify_seed_amount(&mut self, amount: i32) {
    self.seeds += amount;
  }
  
  pub fn modify_fv_amount(&mut self, amount: i32) {
    self.fv += amount;
  }
  
  pub fn modify_chopped_fv_amount(&mut self, amount: i32) {
    self.chopped_fv += amount;
  }
  
  pub fn modify_cooked_fv_amount(&mut self, amount: i32) {
    self.cooked_fv += amount;
  }
  
  pub fn modify_meal_amount(&mut self, amount: i32) {
    self.meals += amount;
  }
  
  pub fn modify_small_buffet_amount(&mut self, amount: i32) {
    self.small_buffet += amount;
  }
  
  pub fn modify_regular_buffet_amount(&mut self, amount: i32) {
    self.regular_buffet += amount;
  }
  
  pub fn modify_large_buffet_amount(&mut self, amount: i32) {
    self.large_buffet += amount;
  }
  
  pub fn update(&mut self, delta_time: f32, mouse_pos: Vector2<f32>, left_mouse: bool, keys_pressed_this_frame: &Vec<String>, scroll_delta: f32) {
    for widget in &mut self.widgets {
      widget.update(delta_time, mouse_pos, left_mouse, keys_pressed_this_frame, scroll_delta);
    }
    
    if self.seed_cooldown >= SEED_TIMER {
      if self.widgets[MAIN_WINDOW_INDEX].get_button_state(&SEED_BUTTON_NAME.to_string()) {
        self.seeds += 1;
        self.seed_cooldown = 0.0;
      }
    } else {
      self.seed_cooldown += delta_time;
    }
    
    if self.fv_cooldown >= FV_TIMER {
      if self.widgets[MAIN_WINDOW_INDEX].get_button_state(&FV_BUTTON_NAME.to_string()) {
        if self.seeds >= 10 {
          self.seeds -= 10;
          self.fv += 1;
          self.fv_cooldown = 0.0;
        }
      }
    } else {
      self.fv_cooldown += delta_time;
    }
    
    if self.chopped_fv_cooldown >= CHOPPED_TIMER {
      if self.widgets[MAIN_WINDOW_INDEX].get_button_state(&CHOPPED_BUTTON_NAME.to_string()) {
        if self.fv >= 10 {
          self.fv -= 10;
          self.chopped_fv += 1;
          self.chopped_fv_cooldown = 0.0;
        }
      }
    } else {
      self.chopped_fv_cooldown += delta_time;
    }
    
    if self.cooked_fv_cooldown >= COOKED_TIMER {
      if self.widgets[MAIN_WINDOW_INDEX].get_button_state(&COOKED_BUTTON_NAME.to_string()) {
        if self.chopped_fv >= 10 {
          self.chopped_fv -= 10;
          self.cooked_fv += 1;
          self.cooked_fv_cooldown = 0.0;
        }
      }
    } else {
      self.cooked_fv_cooldown += delta_time;
    }
    
    if self.meal_cooldown >= MEAL_TIMER {
      if self.widgets[MAIN_WINDOW_INDEX].get_button_state(&MEAL_BUTTON_NAME.to_string()) {
        if self.cooked_fv >= 10 {
          self.cooked_fv -= 10;
          self.meals += 1;
          self.meal_cooldown = 0.0;
        }
      }
    } else {
      self.meal_cooldown += delta_time;
    }
    
    if self.small_buffet_cooldown >= SMALL_B_TIMER {
      if self.widgets[MAIN_WINDOW_INDEX].get_button_state(&SMALL_B_BUTTON_NAME.to_string()) {
        if self.meals >= 10 {
          self.meals -= 10;
          self.small_buffet += 1;
          self.small_buffet_cooldown = 0.0;
        }
      }
    } else {
      self.small_buffet_cooldown += delta_time;
    }
    
    if self.regular_buffet_cooldown >= REGULAR_B_TIMER {
      if self.widgets[MAIN_WINDOW_INDEX].get_button_state(&REGULAR_B_BUTTON_NAME.to_string()) {
        if self.small_buffet >= 10 {
          self.small_buffet -= 10;
          self.regular_buffet += 1;
          self.regular_buffet_cooldown = 0.0;
        }
      }
    } else {
      self.regular_buffet_cooldown += delta_time;
    }
    
    if self.large_buffet_cooldown >= LARGE_B_TIMER {
      if self.widgets[MAIN_WINDOW_INDEX].get_button_state(&LARGE_B_BUTTON_NAME.to_string()) {
        if self.regular_buffet >= 10 {
          self.regular_buffet -= 10;
          self.large_buffet += 1;
          self.large_buffet_cooldown = 0.0;
        }
      }
    } else {
      self.large_buffet_cooldown += delta_time;
    }
    
    if self.seeds >= 10 {
      self.widgets[MAIN_WINDOW_INDEX].set_button_hidden(&FV_BUTTON_NAME.to_string(), false);
      self.widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&FV_TEXTFIELD_NAME.to_string(), false);
    }
    if self.fv >= 10 {
      self.widgets[MAIN_WINDOW_INDEX].set_button_hidden(&CHOPPED_BUTTON_NAME.to_string(), false);
      self.widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&CHOPPED_TEXTFIELD_NAME.to_string(), false);
    }
    if self.chopped_fv >= 10 {
      self.widgets[MAIN_WINDOW_INDEX].set_button_hidden(&COOKED_BUTTON_NAME.to_string(), false);
      self.widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&COOKED_TEXTFIELD_NAME.to_string(), false);
    }
    if self.cooked_fv >= 10 {
      self.widgets[MAIN_WINDOW_INDEX].set_button_hidden(&MEAL_BUTTON_NAME.to_string(), false);
      self.widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&MEAL_TEXTFIELD_NAME.to_string(), false);
    }
    if self.meals >= 10 {
      self.widgets[MAIN_WINDOW_INDEX].set_button_hidden(&SMALL_B_BUTTON_NAME.to_string(), false);
      self.widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&SMALL_B_TEXTFIELD_NAME.to_string(), false);
    }
    if self.small_buffet >= 10 {
      self.widgets[MAIN_WINDOW_INDEX].set_button_hidden(&REGULAR_B_BUTTON_NAME.to_string(), false);
      self.widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&REGULAR_B_TEXTFIELD_NAME.to_string(), false);
    }
    if self.regular_buffet >= 10 {
      self.widgets[MAIN_WINDOW_INDEX].set_button_hidden(&LARGE_B_BUTTON_NAME.to_string(), false);
      self.widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&LARGE_B_TEXTFIELD_NAME.to_string(), false);
    }
    
    self.widgets[MAIN_WINDOW_INDEX].update_text_field(&FV_TEXTFIELD_NAME.to_string(), "Fruit & Veg (FV): ".to_owned() + &self.fv.to_string());
    self.widgets[MAIN_WINDOW_INDEX].update_text_field(&SEED_TEXTFIELD_NAME.to_string(), "Seeds: ".to_owned() + &self.seeds.to_string());
        self.widgets[MAIN_WINDOW_INDEX].update_text_field(&CHOPPED_TEXTFIELD_NAME.to_string(), "Chopped FV: ".to_owned() + &self.chopped_fv.to_string());
    self.widgets[MAIN_WINDOW_INDEX].update_text_field(&COOKED_TEXTFIELD_NAME.to_string(), "Cooked FV: ".to_owned() + &self.cooked_fv.to_string());
        self.widgets[MAIN_WINDOW_INDEX].update_text_field(&MEAL_TEXTFIELD_NAME.to_string(), "Meals: ".to_owned() + &self.meals.to_string());
    self.widgets[MAIN_WINDOW_INDEX].update_text_field(&SMALL_B_TEXTFIELD_NAME.to_string(), "Small Buffet: ".to_owned() + &self.small_buffet.to_string());
        self.widgets[MAIN_WINDOW_INDEX].update_text_field(&REGULAR_B_TEXTFIELD_NAME.to_string(), "Regular Buffet: ".to_owned() + &self.regular_buffet.to_string());
    self.widgets[MAIN_WINDOW_INDEX].update_text_field(&LARGE_B_TEXTFIELD_NAME.to_string(), "Large Buffet: ".to_owned() + &self.large_buffet.to_string());
  }
  
  pub fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    for widget in &self.widgets {
      widget.draw(draw_calls);
    }
  }
}
