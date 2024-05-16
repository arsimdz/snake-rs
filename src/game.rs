use crate::snake::Snake;
use crate::point::Point;
use crate::direction::Direction;
use piston_window::types::Color;
use piston_window::*;
use rand::{Rng,thread_rng};
use crate::draw::{draw_block,draw_rectangle};

const FOOD_COLOR : Color = [0.8,0.00,0.00,1.0];
const BORDER_COLOR: Color = [0.00,0.00,0.00,1.00];
const GAMEOVER_COLOR: Color = [0.9,0.00,0.00,0.5];
const MOVIN_PERIOD: f64 = 0.1;
const RESTARTING_TIME: f64 = 1.0;
#[derive(Debug)]
pub struct Game {
    
    snake: Snake,
    food_exists: bool,
    food: Point,
    width: i32,
    height: i32,
    game_over: bool,
    waiting_time: f64
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
            Game { snake: Snake::new(2,2),
                 waiting_time:0.0,
                 food_exists:true,
                 food: Point::new(6,4),
                 width,
                 height,
                 game_over: false
                 }
        }
    
    pub fn key_pressed(&mut self,key:Key){
        if self.game_over{
            return;
        }
        let dir = match key{
            Key::Up=>Some(Direction::Up),
            Key::Down=>Some(Direction::Down),
            Key::Left=>Some(Direction::Left),
            Key::Right=>Some(Direction::Right),
            _=> Some(self.snake.get_direction()),
        };
        if let Some(dir) = dir{
            if dir==self.snake.get_direction().opposite(){
                return;
            }
        };
        self.update_snake(dir);
    }
    pub fn draw(&self,con:&Context,g:&mut G2d){
        self.snake.draw(con, g);
        if self.food_exists{
            draw_block(FOOD_COLOR, self.food.x, self.food.y, con, g);
        }
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height-1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width-1, 0, 1, self.height, con, g);
        if self.game_over{
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }

    }
    pub fn update(&mut self,delta_time:f64){
        self.waiting_time += delta_time;
        if self.game_over{
            if self.waiting_time > RESTARTING_TIME{
                self.restart();
            }
            return;
        }
        if !self.food_exists{
            self.add_food()
        }
        if self.waiting_time>MOVIN_PERIOD{
            self.update_snake(None);
        }

    }
    fn check_eating(&mut self){
        let (head_x,head_y) = self.snake.get_head();
        if self.food_exists && head_x==self.food.x && head_y==self.food.y{
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }
    fn check_if_snake_alive(&self,dir:Option<Direction>)->bool{
         let (head_x,head_y) = self.snake.next_head(dir);
         if self.snake.overlap_tail(head_x, head_y){
             return false;
         }
         head_x >0 && head_y>0 && head_x < self.width-1 && head_y< self.height-1
    }
    fn update_snake(&mut self, dir: Option<Direction>){
        if self.check_if_snake_alive(dir){
            self.snake.move_forward(dir);
            self.check_eating();
        }
        else{
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }
    
    fn restart(&mut self){
        self.waiting_time = 0.0;
        self.food_exists  = true;
        self.food = Point::new(6,4);
        self.snake = Snake::new(2,2);
        self.game_over = false;
    }
    fn add_food(&mut self) {
        let mut rng = thread_rng();
        let mut  random_x = rng.gen_range(1,self.width-1);
        let mut  random_y = rng.gen_range(1,self.height-1);
        while self.snake.overlap_tail(random_x, random_y) {
             random_x = rng.gen_range(1,self.width-1);
             random_y = rng.gen_range(1,self.height-1);
            
            
        }
        let point = Point::new(random_x, random_y);
        self.food = point;
        self.food_exists = true;
    }
 
}