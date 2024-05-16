use std::collections::LinkedList;
use piston_window::{Context,G2d};
use piston_window::types::Color;
use crate::draw::draw_block;
use crate::direction::Direction;
use crate::point::Point;
const SNAKE_COLOR: Color = [0.00,0.80,0.00,1.0];
#[derive(Debug)]
pub struct Snake{
    direction: Direction,
    body:LinkedList<Point>,
    tail: Option<Point>
}
impl Snake {
    pub fn new(x:i32,y:i32)->Snake{
        let mut body: LinkedList<Point> = LinkedList::new();
        body.push_back(Point::new(x+2,y));
        body.push_back(Point::new(x+1,y));
        body.push_back(Point::new(x,y));
        Snake{direction:Direction::Right,body,tail:None}
    }
    pub fn draw(&self,con:&Context,g:&mut G2d){
        for point in &self.body{
            draw_block(SNAKE_COLOR, point.x, point.y, con, g);
        }
    }
    pub fn move_forward(&mut self,dir:Option<Direction>){
        match dir{
            Some(d)=>self.direction = d,
            None => ()
        }
        let (last_x,last_y) = self.get_head();
        let new_point = match self.direction{
            Direction::Up => (last_x,last_y-1),
            Direction::Down => (last_x,last_y+1),
            Direction::Left=> (last_x-1, last_y),
            Direction::Right=> (last_x+1,last_y)
        };
        let new_head = Point::new(new_point.0,new_point.1);
        self.body.push_front(new_head);
        let removed_point = self.body.pop_back().unwrap();
        self.tail = Some(removed_point);

    }
    pub fn get_head(&self)->(i32,i32){
        let head_point = self.body.front().unwrap();
        (head_point.x,head_point.y)
    }
    pub fn next_head(&self,dir:Option<Direction>)->(i32,i32){
        let mut moving_dir = self.direction;
        let (last_x,last_y) = self.get_head();
        match dir{
            Some(d)=> moving_dir = d,
            None => ()
        }
        
         match moving_dir{
            Direction::Up => (last_x,last_y-1),
            Direction::Down => (last_x,last_y+1),
            Direction::Left=> (last_x-1, last_y),
            Direction::Right=> (last_x+1,last_y)
        }
    }
    pub fn restore_tail(&mut self){
        let tail_point = self.tail.clone().unwrap();
        self.body.push_back(tail_point);
    }
    pub fn overlap_tail(&self,x:i32,y:i32)->bool{
        let mut ch = 0;
        for point in &self.body{
             if point.x ==x && point.y==y{
                return true
             }
             ch+=1;
             if ch == self.body.len()-1{
                break;
             }
        }
        return false
    }
   
    pub fn get_direction(&self)->Direction{
        self.direction
    }
    
    
   
    
}