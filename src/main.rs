
/* implement conway's game of life using a state struct with a grid and a cell struct with a function
 * to get the next state, and a function to get number of alive neighbours.
 * use enum to describe a cell, whether it is alive or dead.
 *
 */
#[derive(Debug, Clone, PartialEq, Copy)]
enum   CellState { 
    Alive = 0, Dead = 1,
}

#[derive(Debug)]
struct Grid {
   grid: Vec<Vec<CellState>>,
   width: usize,
   height: usize,
}

impl Grid {
   pub fn new(width: usize, height: usize) -> Grid {  
      Grid { 
         grid: vec![vec![CellState::Dead; height]; width], 
         height,
         width,
      }
   }
   pub fn count_neighbors(&self, x: usize, y: usize) -> usize {
      let mut count = 0;
      for i in 0..3 {
         for j in 0..3 {
            if (!(i==1 && j==1)) && self.get_cell((x as i32)+i-1, (y as i32)+j-1) == CellState::Alive {
               count += 1;
            }
         }
      }
      count
   }
   fn get_cell(&self, x: i32, y: i32) -> CellState {
      if x < 0 || x as usize >= self.width || y < 0 || y as usize >= self.height {
         return CellState::Dead;
      }
      self.grid[x as usize][y as usize]
   }
   fn get_future_cell(&self, x: usize, y: usize) -> CellState {
      match self.grid[x][y]{
         CellState::Dead => {
            if self.count_neighbors(x, y) == 3 {
               return CellState::Alive;
            }
            else {
               return CellState::Dead;
            }
         },
         CellState::Alive =>{
            if self.count_neighbors(x, y) == 2 || self.count_neighbors(x, y) == 3 {
               return CellState::Alive;
            }
            else {
               return CellState::Dead;
            }
         }
      }
   }
   pub fn next_grid(&mut self){
      let mut new_grid =  Grid::new(self.width, self.height);
      for i in 0..self.width{
         for j in 0..self.height{
            new_grid.grid[i][j] = self.get_future_cell(i, j);
         }
      }
      self.grid = new_grid.grid;
   }
   pub fn print_grid(&self) {
      for j in 0i32..(self.width as i32){
         for i in 0i32..(self.height as i32){
            match self.get_cell(i, j){
               CellState::Alive => print!("⬜"),
               CellState::Dead => print!("⬛"),
            }
         }
         println!("");
      }
   }
}

fn main() {
   let mut grid = Grid::new(10,12);
   grid.grid[0+3][1+3] = CellState::Alive;
   grid.grid[1+3][0+3] = CellState::Alive;
   grid.grid[1+3][1+3] = CellState::Alive;
   grid.grid[2+3][0+3] = CellState::Alive;
   grid.grid[1+3][2+3] = CellState::Alive;
   let mut gen = 0;
   loop {
      println!("Generation {} grid is", gen);
      grid.print_grid();
      grid.next_grid();
      gen += 1;
      if gen > 10{
         break;
      }
   }
}