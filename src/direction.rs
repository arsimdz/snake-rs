#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub enum Direction{
   Up,
   Right,
   Down,
   Left
}
impl Direction{
    pub fn opposite(&self)->Self{
         match self {
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
         }
    }
}