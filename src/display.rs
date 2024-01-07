use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

const RECT_HEIGHT: u32 = 10;
const RECT_WIDTH: u32 = 10;

pub struct Display {
    canvas: Canvas<Window>,
    pub num_rows: usize,
    pub num_cols: usize,
    buffer: Vec<Vec<u8>>,
    should_update: bool,
}

fn canvas_init(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
}

impl Display {
    pub fn new(num_rows: usize, num_cols: usize, mut canvas: Canvas<Window>) -> Display {
        canvas_init(&mut canvas);
        Display {
            canvas,
            num_rows,
            num_cols,
            buffer: vec![vec![0; num_cols]; num_rows],
            should_update: false,
        }
    }

    pub fn draw_grid(&mut self) {
        for (i, row) in self.buffer.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                let color = if cell == 0 {
                    Color::RGB(0, 0, 0)
                } else {
                    Color::RGB(255, 255, 255)
                };
                self.canvas.set_draw_color(color);
                let rect = Rect::new(j as i32 * 10, i as i32 * 10, RECT_WIDTH, RECT_HEIGHT);
                self.canvas.fill_rect(rect).unwrap();
            }
        }
        self.canvas.present();
    }

    pub fn pretty_print_display_grid(&self) {
        println!("-----------------------------------");
        println!("DEBUG: Printing DISPLAY GRID");
        for row in &self.buffer {
            for cell in row {
                let symbol = if cell.clone() == 1 { 'X' } else { ' ' };
                print!("{}", symbol);
            }
            println!();
        }
    }
    pub fn display_update(&mut self) {
        if self.should_update {
            // self.pretty_print_display_grid();
            self.draw_grid();
            self.should_update = false;
        }
    }

    pub fn clear_display(&mut self) {
        self.buffer = vec![vec![0; self.num_cols]; self.num_rows];
        canvas_init(&mut self.canvas);
        self.should_update = true;
    }

    pub fn set_display(&mut self, row: u8, col: u8, val: u8) {
        self.buffer[row as usize][col as usize] = val;
        self.should_update = true;
    }

    pub fn get_display_buffer(&mut self, row: u8, col: u8) -> u8 {
        self.buffer[row as usize][col as usize]
    }
}
