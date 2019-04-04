 use maat_graphics::DrawCall;

use crate::modules::system_interface::Widget;

use cgmath::Vector2;
use cgmath::Vector4;

const MAIN_WINDOW_INDEX: usize = 0;

const MAIN_WINDOW_NAME: &str = "MainWindow"; 

const SEED_LVL_TEXTFIELD_NAME: &str = "SeedLevelTF";
const SEED_LVL_BUTTON_NAME: &str = "SeedLevelB";
const FV_LVL_TEXTFIELD_NAME: &str = "FVLevelTF";
const FV_LVL_BUTTON_NAME: &str = "FVLevelB";
const CHOPPED_FV_LVL_TEXTFIELD_NAME: &str = "ChoppedFVLevelTF";
const CHOPPED_FV_LVL_BUTTON_NAME: &str = "ChoppedFVLevelB";
const COOKED_FV_LVL_TEXTFIELD_NAME: &str = "CookedFVLevelTF";
const COOKED_FV_LVL_BUTTON_NAME: &str = "CookedFVLevelB";
const MEAL_LVL_TEXTFIELD_NAME: &str = "MealLevelTF";
const MEAL_LVL_BUTTON_NAME: &str = "MealLevelB";
const SMALL_B_LVL_TEXTFIELD_NAME: &str = "SmallBLevelTF";
const SMALL_B_LVL_BUTTON_NAME: &str = "SmallBLevelB";
const REGULAR_B_LVL_TEXTFIELD_NAME: &str = "RegularBLevelTF";
const REGULAR_B_LVL_BUTTON_NAME: &str = "RegularBLevelB";
const LARGE_B_LVL_TEXTFIELD_NAME: &str = "LargeBLevelTF";
const LARGE_B_LVL_BUTTON_NAME: &str = "LargeBLevelB";

const LAND_GROWTH: f32 = 0.2;
const FARMER_GROWTH: f32 = 0.2;
const CHEF_GROWTH: f32 = 0.2;
const COOK_GROWTH: f32 = 0.2;
const MEAL_GROWTH: f32 = 0.2;
const SMALL_B_GROWTH: f32 = 0.2;
const REGULAR_B_GROWTH: f32 = 0.2;
const LARGE_B_GROWTH: f32 = 0.2;

#[derive(Clone)]
pub struct AutoUi {
  hidden: bool,
  widgets: Vec<Widget>,
  seeds: f32,
  seed_gen: f32,
  seed_lvl: i32,
  fv: f32,
  fv_gen: f32,
  fv_lvl: i32,
  chopped: f32,
  chopped_gen: f32,
  chopped_lvl: i32,
  cooked: f32,
  cooked_gen: f32,
  cooked_lvl: i32,
  meals: f32,
  meal_gen: f32,
  meal_lvl: i32,
  small_buffet: f32,
  small_buffet_gen: f32, 
  small_buffet_lvl: i32,
  regular_buffet: f32,
  regular_buffet_gen: f32,
  regular_buffet_lvl: i32,
  large_buffet: f32,
  large_buffet_gen: f32,
  large_buffet_lvl: i32,
}

impl AutoUi {
  pub fn new(window_size: Vector2<f32>) -> AutoUi {
    let mut widgets = Vec::new();
    
    let button_width = 26.0;
    let button_height = 26.0;
    
    let widget_bezel = 25.0;
    
    let widget_size = Vector2::new(window_size.x*0.5, window_size.y*2.0/3.0);
    
    let button_x = widget_size.x -64.0;
    let button_start_y = widget_size.y*0.9;
    
    let can_buy_colour = Vector4::new(0.0,0.0, 1.0, 1.0);
    let text_colour = Vector4::new(0.0, 0.0, 0.0, 1.0);
    let button_colour = Vector4::new(0.211372549, 0.385490196, 0.684313725, 1.0);
    let pressed_colour = Vector4::new(0.2, 0.2, 0.2, 1.0);
    let button_size = Vector2::new(button_width, button_height);
    let text_size = Vector2::new(80.0, 80.0);
    let text_offset = Vector2::new(button_width*0.5, 6.0);
    
    let resource_x = widget_size.x * 0.52;
    
    let text_y_offset = 10.0;
    let button_y_offset = button_height*2.0;
    
    widgets.push(Widget::new(MAIN_WINDOW_NAME.to_string(), 
                             Vector2::new(window_size.x-widget_size.x*0.5, widget_size.y*0.5),
                             widget_size, 
                             Vector4::new(0.555686275, 0.459607843, 0.950980392, 1.0))
                 //.start_hidden()
                
                // Land level
                .with_text_field_centered(SEED_LVL_TEXTFIELD_NAME.to_string(), 
                                  Vector2::new(resource_x, button_start_y), 
                                  text_size, 
                                  text_colour, 
                                  "Land: 0.0 seeds/sec lvl 1".to_string(), "Arial".to_string())
                .with_button(Vector2::new(button_x, button_start_y+text_y_offset),
                              button_size,
                              text_offset, 
                              text_size,
                              SEED_LVL_BUTTON_NAME.to_string(), 
                              text_colour, 
                              pressed_colour,
                              button_colour,
                              true, "+".to_string(), "Arial".to_string())
                .with_text_field("a".to_string(), 
                                  Vector2::new(button_width*0.5, button_start_y), 
                                  text_size*0.95, 
                                  can_buy_colour, 
                                  "5 FV".to_string(), "Arial".to_string())
                //Farmer level
                .with_text_field_centered(FV_LVL_TEXTFIELD_NAME.to_string(), 
                                  Vector2::new(resource_x, button_start_y - button_y_offset), 
                                  text_size, 
                                  text_colour, 
                                  "Farmers: 0 - 0.0 FV/sec".to_string(), "Arial".to_string())
                .with_button(Vector2::new(button_x, button_start_y - button_y_offset+text_y_offset),
                              button_size,
                              text_offset, 
                              text_size,
                              FV_LVL_BUTTON_NAME.to_string(), 
                              text_colour, 
                              pressed_colour,
                              button_colour,
                              true, "+".to_string(), "Arial".to_string())
                .with_text_field("b".to_string(), 
                                  Vector2::new(button_width*0.5, button_start_y- button_y_offset), 
                                  text_size*0.95, 
                                  can_buy_colour, 
                                  "5 Chopped FV".to_string(), "Arial".to_string())
                // Chef
                .with_text_field_centered(CHOPPED_FV_LVL_TEXTFIELD_NAME.to_string(), 
                                  Vector2::new(resource_x, button_start_y - button_y_offset*2.0), 
                                  text_size, 
                                  text_colour, 
                                  "Chefs: 0 - 0.0 Chopped FV/sec".to_string(), "Arial".to_string())
                .with_button(Vector2::new(button_x, button_start_y - button_y_offset*2.0+text_y_offset),
                              button_size,
                              text_offset, 
                              text_size,
                              CHOPPED_FV_LVL_BUTTON_NAME.to_string(), 
                              text_colour, 
                              pressed_colour,
                              button_colour,
                              true, "+".to_string(), "Arial".to_string())
                .with_text_field("c".to_string(), 
                                  Vector2::new(button_width*0.5, button_start_y- button_y_offset*2.0), 
                                  text_size*0.95, 
                                  can_buy_colour, 
                                  "5 Cooked FV".to_string(), "Arial".to_string())
                // cook
                 .with_text_field_centered(COOKED_FV_LVL_TEXTFIELD_NAME.to_string(), 
                                  Vector2::new(resource_x, button_start_y - button_y_offset*3.0), 
                                  text_size, 
                                  text_colour, 
                                  "Cooks: 0 - 0.0 Cooked FV/sec".to_string(), "Arial".to_string())
                .with_button(Vector2::new(button_x, button_start_y - button_y_offset*3.0+text_y_offset),
                              button_size,
                              text_offset, 
                              text_size,
                              COOKED_FV_LVL_BUTTON_NAME.to_string(), 
                              text_colour, 
                              pressed_colour,
                              button_colour,
                              true, "+".to_string(), "Arial".to_string())
                .with_text_field("d".to_string(), 
                                  Vector2::new(button_width*0.5, button_start_y- button_y_offset*3.0), 
                                  text_size*0.95, 
                                  can_buy_colour, 
                                  "5 Meals".to_string(), "Arial".to_string())
                //Waiters
                .with_text_field_centered(MEAL_LVL_TEXTFIELD_NAME.to_string(), 
                                  Vector2::new(resource_x, button_start_y - button_y_offset*4.0), 
                                  text_size, 
                                  text_colour, 
                                  "Waiters: 0 - 0.0 Meals/sec".to_string(), "Arial".to_string())
                .with_button(Vector2::new(button_x, button_start_y - button_y_offset*4.0+text_y_offset),
                            button_size,
                            text_offset, 
                            text_size,
                            MEAL_LVL_BUTTON_NAME.to_string(), 
                            text_colour, 
                            pressed_colour,
                            button_colour,
                            true, "+".to_string(), "Arial".to_string())
                .with_text_field("e".to_string(), 
                                  Vector2::new(button_width*0.5, button_start_y- button_y_offset*4.0), 
                                  text_size*0.95, 
                                  can_buy_colour, 
                                  "5 Small Buffets".to_string(), "Arial".to_string())
                // serving trays
                .with_text_field_centered(SMALL_B_LVL_TEXTFIELD_NAME.to_string(), 
                                  Vector2::new(resource_x, button_start_y -  button_y_offset*5.0), 
                                  text_size, 
                                  text_colour, 
                                  "Serving Trays: 0 - 0.0 Small Buffets/sec".to_string(), "Arial".to_string())
                .with_button(Vector2::new(button_x, button_start_y - button_y_offset*5.0+text_y_offset),
                              button_size,
                              text_offset, 
                              text_size,
                              SMALL_B_LVL_BUTTON_NAME.to_string(), 
                              text_colour, 
                              pressed_colour,
                              button_colour,
                              true, "+".to_string(), "Arial".to_string())
                .with_text_field("f".to_string(), 
                                  Vector2::new(button_width*0.5, button_start_y- button_y_offset*5.0), 
                                  text_size*0.95, 
                                  can_buy_colour, 
                                  "5 R Buffets".to_string(), "Arial".to_string())
                // food servers
                .with_text_field_centered(REGULAR_B_LVL_TEXTFIELD_NAME.to_string(), 
                                  Vector2::new(resource_x, button_start_y - button_y_offset*6.0), 
                                  text_size, 
                                  text_colour, 
                                  "Food servers: 0 - 0.0 Regular Buffets/sec".to_string(), "Arial".to_string())
                .with_button(Vector2::new(button_x, button_start_y - button_y_offset*6.0+text_y_offset),
                              button_size,
                              text_offset, 
                              text_size,
                              REGULAR_B_LVL_BUTTON_NAME.to_string(), 
                              text_colour, 
                              pressed_colour,
                              button_colour,
                              true, "+".to_string(), "Arial".to_string())
                 .with_text_field("g".to_string(), 
                                  Vector2::new(button_width*0.5, button_start_y- button_y_offset*6.0), 
                                  text_size*0.95, 
                                  can_buy_colour, 
                                  "5 L Buffets".to_string(), "Arial".to_string())
                
                // Catering company
                .with_text_field_centered(LARGE_B_LVL_TEXTFIELD_NAME.to_string(), 
                                  Vector2::new(resource_x, button_start_y - button_y_offset*7.0), 
                                  text_size, 
                                  text_colour, 
                                  "Catering Companies: 0 - 0.0 Large Buffets/sec".to_string(), "Arial".to_string())
                .with_button(Vector2::new(button_x, button_start_y - button_y_offset*7.0+text_y_offset),
                              button_size,
                              text_offset, 
                              text_size,
                              LARGE_B_LVL_BUTTON_NAME.to_string(), 
                              text_colour, 
                              pressed_colour,
                              button_colour,
                              true, "+".to_string(), "Arial".to_string())
                
                );
    
    widgets[MAIN_WINDOW_INDEX].set_button_hidden(&SEED_LVL_BUTTON_NAME.to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&SEED_LVL_TEXTFIELD_NAME.to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_button_hidden(&FV_LVL_BUTTON_NAME.to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&FV_LVL_TEXTFIELD_NAME.to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_button_hidden(&CHOPPED_FV_LVL_BUTTON_NAME.to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&CHOPPED_FV_LVL_TEXTFIELD_NAME.to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_button_hidden(&COOKED_FV_LVL_BUTTON_NAME.to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&COOKED_FV_LVL_TEXTFIELD_NAME.to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_button_hidden(&MEAL_LVL_BUTTON_NAME.to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&MEAL_LVL_TEXTFIELD_NAME.to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_button_hidden(&SMALL_B_LVL_BUTTON_NAME.to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&SMALL_B_LVL_TEXTFIELD_NAME.to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_button_hidden(&REGULAR_B_LVL_BUTTON_NAME.to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&REGULAR_B_LVL_TEXTFIELD_NAME.to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_button_hidden(&LARGE_B_LVL_BUTTON_NAME.to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&LARGE_B_LVL_TEXTFIELD_NAME.to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&"a".to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&"b".to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&"c".to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&"d".to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&"e".to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&"f".to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&"g".to_string(), true);
    widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&"h".to_string(), true);
    
    AutoUi {
      hidden: true,
      widgets: widgets,
      seeds: 0.0,
      seed_gen: 0.0,
      seed_lvl: 0,
      fv: 0.0,
      fv_gen: 0.0,
      fv_lvl: 0,
      chopped: 0.0,
      chopped_gen: 0.0,
      chopped_lvl: 0,
      cooked: 0.0,
      cooked_gen: 0.0,
      cooked_lvl: 0,
      meals: 0.0,
      meal_gen: 0.0,
      meal_lvl: 0,
      small_buffet: 0.0,
      small_buffet_gen: 0.0, 
      small_buffet_lvl: 0,
      regular_buffet: 0.0,
      regular_buffet_gen: 0.0,
      regular_buffet_lvl: 0,
      large_buffet: 0.0,
      large_buffet_gen: 0.0,
      large_buffet_lvl: 0,
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
  
  pub fn gather_resources(&mut self) -> (i32, i32, i32, i32, i32, i32, i32, i32) {
    let r1 = self.seeds.floor() as i32;
    let r2 = self.fv.floor() as i32;
    let r3 = self.chopped.floor() as i32;
    let r4 = self.cooked.floor() as i32;
    let r5 = self.meals.floor() as i32;
    let r6 = self.small_buffet.floor() as i32;
    let r7 = self.regular_buffet.floor() as i32;
    let r8 = self.large_buffet.floor() as i32;
    
    self.seeds -= r1 as f32;
    self.fv -= r2 as f32;
    self.chopped -= r3 as f32;
    self.cooked -= r4 as f32;
    self.meals -= r5 as f32;
    self.small_buffet -= r6 as f32;
    self.regular_buffet -= r7 as f32;
    self.large_buffet -= r8 as f32;
    
    (r1, r2, r3, r4, r5, r6, r7, r8)
  }
  
  pub fn show_land_option(&mut self) {
    self.widgets[MAIN_WINDOW_INDEX].set_button_hidden(&SEED_LVL_BUTTON_NAME.to_string(), false);
    self.widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&SEED_LVL_TEXTFIELD_NAME.to_string(), false);
    self.widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&"a".to_string(), false);
  }
  
  pub fn land_lvl_pressed(&self) -> bool {
    self.widgets[MAIN_WINDOW_INDEX].get_button_state(&SEED_LVL_BUTTON_NAME.to_string())
  }
  
  pub fn level_up_land(&mut  self) {
    self.seed_lvl += 1;
    self.seed_gen += LAND_GROWTH;
  }
  
  pub fn show_farmer_option(&mut self) {
    self.widgets[MAIN_WINDOW_INDEX].set_button_hidden(&FV_LVL_BUTTON_NAME.to_string(), false);
    self.widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&FV_LVL_TEXTFIELD_NAME.to_string(), false);
    self.widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&"b".to_string(), false);
  }
  
  pub fn farmer_lvl_pressed(&self) -> bool {
    self.widgets[MAIN_WINDOW_INDEX].get_button_state(&FV_LVL_BUTTON_NAME.to_string())
  }
  
  pub fn level_up_farmer(&mut  self) {
    self.fv_lvl += 1;
    self.fv_gen += FARMER_GROWTH;
  }
  
  pub fn show_chef_option(&mut self) {
    self.widgets[MAIN_WINDOW_INDEX].set_button_hidden(&CHOPPED_FV_LVL_BUTTON_NAME.to_string(), false);
    self.widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&CHOPPED_FV_LVL_TEXTFIELD_NAME.to_string(), false);
    self.widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&"c".to_string(), false);
  }
  
  pub fn chef_lvl_pressed(&self) -> bool {
    self.widgets[MAIN_WINDOW_INDEX].get_button_state(&CHOPPED_FV_LVL_BUTTON_NAME.to_string())
  }
  
  pub fn level_up_chef(&mut  self) {
    self.chopped_lvl += 1;
    self.chopped_gen += CHEF_GROWTH;
  }
  
  pub fn show_cook_option(&mut self) {
    self.widgets[MAIN_WINDOW_INDEX].set_button_hidden(&COOKED_FV_LVL_BUTTON_NAME.to_string(), false);
    self.widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&COOKED_FV_LVL_TEXTFIELD_NAME.to_string(), false);
    self.widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&"d".to_string(), false);
  }
  
  pub fn cook_lvl_pressed(&self) -> bool {
    self.widgets[MAIN_WINDOW_INDEX].get_button_state(&COOKED_FV_LVL_BUTTON_NAME.to_string())
  }
  
  pub fn level_up_cook(&mut  self) {
    self.cooked_lvl += 1;
    self.cooked_gen += COOK_GROWTH;
  }
  
  pub fn show_waiters_option(&mut self) {
    self.widgets[MAIN_WINDOW_INDEX].set_button_hidden(&MEAL_LVL_BUTTON_NAME.to_string(), false);
    self.widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&MEAL_LVL_TEXTFIELD_NAME.to_string(), false);
    self.widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&"e".to_string(), false);
  }
  
  pub fn waiter_lvl_pressed(&self) -> bool {
    self.widgets[MAIN_WINDOW_INDEX].get_button_state(&MEAL_LVL_BUTTON_NAME.to_string())
  }
  
  pub fn level_up_waiter(&mut  self) {
    self.meal_lvl += 1;
    self.meal_gen += MEAL_GROWTH;
  }
  
  pub fn show_serving_trays_option(&mut self) {
    self.widgets[MAIN_WINDOW_INDEX].set_button_hidden(&SMALL_B_LVL_BUTTON_NAME.to_string(), false);
    self.widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&SMALL_B_LVL_TEXTFIELD_NAME.to_string(), false);
    self.widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&"f".to_string(), false);
  }
  
  pub fn serving_tray_lvl_pressed(&self) -> bool {
    self.widgets[MAIN_WINDOW_INDEX].get_button_state(&SMALL_B_LVL_BUTTON_NAME.to_string())
  }
  
  pub fn level_up_serving_tray(&mut  self) {
    self.small_buffet_lvl += 1;
    self.small_buffet_gen += SMALL_B_GROWTH;
  }
  
  pub fn show_food_servers_option(&mut self) {
    self.widgets[MAIN_WINDOW_INDEX].set_button_hidden(&REGULAR_B_LVL_BUTTON_NAME.to_string(), false);
    self.widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&REGULAR_B_LVL_TEXTFIELD_NAME.to_string(), false);
    self.widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&"g".to_string(), false);
  }
  
  pub fn food_servers_lvl_pressed(&self) -> bool {
    self.widgets[MAIN_WINDOW_INDEX].get_button_state(&REGULAR_B_LVL_BUTTON_NAME.to_string())
  }
  
  pub fn level_up_food_servers(&mut  self) {
    self.regular_buffet_lvl += 1;
    self.regular_buffet_gen += REGULAR_B_GROWTH;
  }
  
  pub fn show_catering_company_option(&mut self) {
    self.widgets[MAIN_WINDOW_INDEX].set_button_hidden(&LARGE_B_LVL_BUTTON_NAME.to_string(), false);
    self.widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&LARGE_B_LVL_TEXTFIELD_NAME.to_string(), false);
    self.widgets[MAIN_WINDOW_INDEX].set_textfield_hidden(&"h".to_string(), false);
  }
  
  pub fn catering_company_lvl_pressed(&self) -> bool {
    self.widgets[MAIN_WINDOW_INDEX].get_button_state(&LARGE_B_LVL_BUTTON_NAME.to_string())
  }
  
  pub fn level_up_catering_company(&mut  self) {
    self.large_buffet_lvl += 1;
    self.large_buffet_gen += LARGE_B_GROWTH;
  }
  
  pub fn update(&mut self, delta_time: f32, mouse_pos: Vector2<f32>, left_mouse: bool, keys_pressed_this_frame: &Vec<String>, scroll_delta: f32) {
    for widget in &mut self.widgets {
      widget.update(delta_time, mouse_pos, left_mouse, keys_pressed_this_frame, scroll_delta);
    }
    
    self.seeds += self.seed_gen*delta_time;
    self.fv += self.fv_gen*delta_time;
    self.chopped += self.chopped_gen*delta_time;
    self.cooked += self.cooked_gen*delta_time;
    self.meals += self.meal_gen*delta_time;
    self.small_buffet += self.small_buffet_gen*delta_time;
    self.regular_buffet += self.regular_buffet_gen*delta_time;
    self.large_buffet += self.large_buffet_gen*delta_time;
    
    let mut land_per_sec = self.seed_gen.to_string();
    let mut farmer_per_sec = self.fv_gen.to_string();
    let mut chef_per_sec = self.chopped_gen.to_string();
    let mut cook_per_sec = self.cooked_gen.to_string();
    let mut waiters_per_sec = self.meal_gen.to_string();
    let mut serving_trays_per_sec = self.small_buffet_gen.to_string();
    let mut food_servers_per_sec = self.regular_buffet_gen.to_string();
    let mut catering_company_per_sec = self.large_buffet_gen.to_string();
    land_per_sec.truncate(6);
    farmer_per_sec.truncate(6);
    chef_per_sec.truncate(6);
    cook_per_sec.truncate(6);
    waiters_per_sec.truncate(6);
    serving_trays_per_sec.truncate(6);
    food_servers_per_sec.truncate(6);
    catering_company_per_sec.truncate(6);
    
    self.widgets[MAIN_WINDOW_INDEX].update_text_field(&SEED_LVL_TEXTFIELD_NAME.to_string(), "Land: ".to_owned() + &land_per_sec + " seeds/sec lvl " + &self.seed_lvl.to_string());
    
    self.widgets[MAIN_WINDOW_INDEX].update_text_field(&FV_LVL_TEXTFIELD_NAME.to_string(), "Farmers: ".to_owned() + &self.fv_lvl.to_string() + " - " + &farmer_per_sec + " FV/sec");
    
    self.widgets[MAIN_WINDOW_INDEX].update_text_field(&CHOPPED_FV_LVL_TEXTFIELD_NAME.to_string(), "Chefs: ".to_owned() + &self.chopped_lvl.to_string() + " - " + &chef_per_sec + " Chopped FV/sec");
    
    self.widgets[MAIN_WINDOW_INDEX].update_text_field(&COOKED_FV_LVL_TEXTFIELD_NAME.to_string(), "Cooks: ".to_owned() + &self.cooked_lvl.to_string() + " - " + &cook_per_sec + " Cooked FV/sec");
    
    self.widgets[MAIN_WINDOW_INDEX].update_text_field(&MEAL_LVL_TEXTFIELD_NAME.to_string(), "Waiters: ".to_owned() + &self.meal_lvl.to_string() + " - " + &waiters_per_sec + " Meals/sec");
     
    self.widgets[MAIN_WINDOW_INDEX].update_text_field(&SMALL_B_LVL_TEXTFIELD_NAME.to_string(), "Serving trays: ".to_owned() + &self.small_buffet_lvl.to_string() + " - " + &serving_trays_per_sec + " Small Buffet/sec");
    
    self.widgets[MAIN_WINDOW_INDEX].update_text_field(&REGULAR_B_LVL_TEXTFIELD_NAME.to_string(), "Food servers: ".to_owned() + &self.regular_buffet_lvl.to_string() + " - " + &food_servers_per_sec + " Regular Buffet/sec");
    
    self.widgets[MAIN_WINDOW_INDEX].update_text_field(&LARGE_B_LVL_TEXTFIELD_NAME.to_string(), "Catering Company Lvl: ".to_owned() + &self.large_buffet_lvl.to_string() + " - " + &catering_company_per_sec + " Large Buffet/sec");
  }
  
  pub fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    for widget in &self.widgets {
      widget.draw(draw_calls);
    }
  }
}
