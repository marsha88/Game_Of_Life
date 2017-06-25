/* Grid definition that holds both grids; one used as a buffer while other is updated */

pub struct Grid{
    currentBuffer: u8,
    display: u8,
    one: [[bool; 8]; 8],
    two: [[bool; 8]; 8]
}

impl Grid{
    pub fn new() -> Grid{
        return Grid{ currentBuffer: 2, display: 1,
            one: [[false; 8], [false; 8], [false; 8], [false, false, false, true, true, false, false, false],
                [false, false, false, true, false, false, false, false],
                [false; 8], [false; 8], [false; 8]],
            two: [[false; 8], [false; 8], [false; 8], [false, false, false, true, true, false, false, false],
                [false, false, false, true, false, false, false, false],
                [false; 8], [false; 8], [false; 8]]
        };
    }

    pub fn display(&self) -> &[[bool; 8]; 8]{
        /* should I return a reference to the display grid or the grid itself? */
        if self.display == 1{
            return &self.one;
        }
        return &self.two;
    }

    fn switch_grid(&mut self){
        self.currentBuffer = if self.currentBuffer == 1{ 2 } else{ 1 };
        self.display = if self.display == 1{ 2 } else{ 1 };
    }

    fn valid_cell(x:i32, y:i32) -> bool{
     (x < 8 && x >= 0 && y < 8 && y >= 0)
    }

    fn num_neighbors(grid:&[[bool;8];8], x:u32, y:u32) -> u8{
        let mut count:u8 = 0;
        let indexX:i32 = x as i32;
        let indexY:i32 = y as i32;
        /* check each square touching current square */
        for i in (indexX-1)...(indexX+1){
            for j in (indexY-1)...(indexY+1){
                if i != indexX || j != indexY{
                    if Grid::valid_cell(i, j) && grid[i as usize][j as usize] == true{
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn neighbors(&self, x:u32, y:u32) -> u8{
        if self.currentBuffer == 1{
            return Grid::num_neighbors(&self.one, x, y);
        }
        return Grid::num_neighbors(&self.two, x, y);
    }

    pub fn next(&mut self){
        /* apply game of life logic here */
        for i in 0..8{
            for j in 0..8{
                let neighbors = self.neighbors(i,j);
                if neighbors >= 4 || neighbors <= 1{
                    /* kill cell */
                    if self.currentBuffer == 1{
                        self.one[i as usize][j as usize] = false;
                    }
                    else{
                        self.two[i as usize][j as usize] = false;
                    }
                }
                else if neighbors == 3{
                    /* revive cell */
                    if self.currentBuffer == 1{
                        self.one[i as usize][j as usize] = true;
                    }
                    else{
                        self.two[i as usize][j as usize] = true;
                    }
                }
            }
        }
        /* update the buffere grid as the display grid */
        self.switch_grid();
    }
}
