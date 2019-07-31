use ggez::*;

struct MainState {
    map: Map,
}

impl ggez::event::EventHandler for MainState {
  fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
      self.map.update(ctx)?;
      Ok(())
  }
  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
      self.map.draw(ctx)?;
      Ok(())
  }
}

fn main() {
    let state = &mut MainState { map: Map::new() };
    let cb = ggez::ContextBuilder::new("Game of Life", "Sammy");
    let (ref mut ctx, ref mut event_loop) = &mut cb.build().unwrap();
    event::run(ctx, event_loop, state).unwrap();
}

///////////////////////////////////////////////////////////////////////////
#[derive(PartialEq)]
enum State {
    Alive,
    Dead,
}
struct Cell {
    x: f32,
    y: f32,
    state: State,
}

impl ggez::event::EventHandler for Cell {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let rect = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(self.x, self.y, Map::CELL_STEP, Map::CELL_STEP),
                    match self.state {
                        State::Alive => 
                            graphics::BLACK,
                        State::Dead =>
                            graphics::WHITE,
                    },
        )?;

        graphics::draw(ctx, &rect, graphics::DrawParam::default())?;
        Ok(())
    }
} 
///////////////////////////////////////////////////////////////////////////
struct Map {
    rows: Vec<Vec<Cell>>,
}

impl Map {
    const ROW_COUNT: u32 = 24;
    const COLUMN_COUNT: u32 = 32;
    const CELL_STEP: f32 = 25.0;
    fn new() -> Self{
        let mut rows: Vec<Vec<Cell>> = Vec::with_capacity(Map::ROW_COUNT as usize);
        let mut y = 0.0;

        for _ in 0..Map::ROW_COUNT {
            let mut x = 0.0;
            let mut cells: Vec<Cell> = Vec::with_capacity(Map::COLUMN_COUNT as usize);
            for _ in 0..Map::COLUMN_COUNT {
                cells.push(Cell{x: x, y: y, state: State::Dead});
                x += Map::CELL_STEP;
            }
            rows.push(cells);
            y += Map::CELL_STEP;
        }
        Map{rows}
    }
    fn check_cells(&self) -> Vec<(u32, u32, State)> {
        let mut cells_to_update: Vec<(u32,u32, State)> = Vec::new();
        for x in 1..Map::ROW_COUNT-1{
            for y in 1..Map::COLUMN_COUNT-1{
                let mut alive_count = 0;
                if self.rows[(x-1) as usize][(y-1) as usize].state == State::Alive { alive_count += 1; }
                if self.rows[x as usize][(y-1) as usize].state == State::Alive { alive_count += 1; }
                if self.rows[(x+1) as usize][(y-1) as usize].state == State::Alive { alive_count += 1; }
                if self.rows[(x-1) as usize][y as usize].state == State::Alive { alive_count += 1; }
                if self.rows[(x+1) as usize][y as usize].state == State::Alive { alive_count += 1; }
                if self.rows[(x-1) as usize][(y+1) as usize].state == State::Alive { alive_count += 1; }
                if self.rows[x as usize][(y+1) as usize].state == State::Alive { alive_count += 1; }
                if self.rows[(x+1) as usize][(y+1) as usize].state == State::Alive { alive_count += 1; }

                if (self.rows[x as usize][y as usize].state == State::Alive) && ((alive_count < 2) || (alive_count > 3)) {
                    cells_to_update.push((x, y, State::Dead));
                }
                else if (self.rows[x as usize][y as usize].state == State::Dead) && (alive_count == 3) {
                    cells_to_update.push((x, y, State::Alive));
                }
            }
        }
        cells_to_update
    }
    fn update_cells(&mut self, cells_to_update: &Vec<(u32, u32, State)>) {
        for (x, y, state) in cells_to_update.iter() {
            match *state {
                State::Alive =>
                    self.rows[*x as usize][*y as usize].state = State::Alive,
                State::Dead => 
                    self.rows[*x as usize][*y as usize].state = State::Dead,
            }
        }
    }
}

impl ggez::event::EventHandler for Map {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut start = false;
        if ggez::input::mouse::button_pressed(ctx, ggez::input::mouse::MouseButton::Left) {
            let (x, y) = cell_pos(ctx);
            println!("({}, {})", x, y);
            self.rows[y as usize][x as usize].state = State::Alive;
        }
        if ggez::input::keyboard::is_key_pressed(ctx, ggez::input::keyboard::KeyCode::Space) {
            start = true;
        }
        if start {
            self.update_cells(&self.check_cells());
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        for row in self.rows.iter_mut() {
            for cell in row.iter_mut() {
                cell.draw(ctx)?;
            }
        }
        graphics::present(ctx)?;
        Ok(())
    }
}

fn cell_pos(ctx: &mut Context) -> (u32, u32) {
    let mouse_pos = ggez::input::mouse::position(ctx);
    let mouse_x = mouse_pos.x;
    let mouse_y = mouse_pos.y;
    ((mouse_x / Map::CELL_STEP) as u32, (mouse_y / Map::CELL_STEP) as u32)
}

/////////////////////////////////////////////////////////////////////////////