use crate::player::Player;
use crate::position::Position;
use crate::square::Square;
use crate::BOARD_SIZE;
use colored::Colorize;
use three_d::*;

const SQUARE_SIZE: u32 = 50;

pub struct Renderer {
    matrix: Vec<Vec<usize>>,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            matrix: vec![vec![0; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    // TODO
    pub fn paint_frame(&self, squares: &Vec<Square>) {
        let window = Window::new(WindowSettings {
            title: "Shapes 2D!".to_string(),
            max_size: Some((
                SQUARE_SIZE * 2 * BOARD_SIZE as u32,
                SQUARE_SIZE * 2 * BOARD_SIZE as u32,
            )),
            ..Default::default()
        })
        .unwrap();

        let context = window.gl();

        let mut arr: [Gm<Rectangle, ColorMaterial>; 1] = [];

        let mut rectangle = Gm::new(
            Rectangle::new(
                &context,
                vec2(SQUARE_SIZE as f32, SQUARE_SIZE as f32),
                degrees(0.0),
                SQUARE_SIZE as f32,
                SQUARE_SIZE as f32,
            ),
            ColorMaterial {
                color: Color::RED,
                ..Default::default()
            },
        );

        arr.push(rectangle);

        /*
        let mut circle = Gm::new(
            Circle::new(&context, vec2(500.0, 500.0), 200.0),
            ColorMaterial {
                color: Color::BLUE,
                ..Default::default()
            },
        );

        let mut line = Gm::new(
            Line::new(
                &context,
                vec2(0.0, 0.0),
                vec2(
                    window.viewport().width as f32,
                    window.viewport().height as f32,
                ),
                5.0,
            ),
            ColorMaterial {
                color: Color::GREEN,
                ..Default::default()
            },
        );
        */

        window.render_loop(move |frame_input| {
            for event in frame_input.events.iter() {
                match event {
                    Event::MousePress {
                        button,
                        position,
                        modifiers,
                        ..
                    } => {
                        let pos = vec2(
                            (frame_input.device_pixel_ratio * position.0) as f32,
                            (frame_input.device_pixel_ratio * position.1) as f32,
                        );
                        if *button == MouseButton::Left {
                            // TODO: DRAW X INSIDE RECTANGLE
                            // rectangle.set_center(pos);
                            arr[0].set_center(pos);
                        }

                        /*
                        if *button == MouseButton::Right && !modifiers.ctrl {
                            circle.set_center(pos);
                        }
                        if *button == MouseButton::Left && modifiers.ctrl {
                            let ep = line.end_point1();
                            line.set_endpoints(pos, ep);
                        }
                        if *button == MouseButton::Right && modifiers.ctrl {
                            let ep = line.end_point0();
                            line.set_endpoints(ep, pos);
                        }
                        */
                    }
                    _ => {}
                }
            }

            frame_input
                .screen()
                .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
                .render(
                    &camera2d(frame_input.viewport),
                    arr.into_iter(),
                    //line.into_iter().chain(&rectangle).chain(&circle),
                    &[],
                );

            FrameOutput::default()
        });
    }

    pub fn clear(&self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    pub fn draw(&self, squares: &Vec<Square>) {
        let mut border = String::new();
        let mut legend = String::new();

        for i in 0..BOARD_SIZE {
            border.push_str("---");
            legend.push_str(&format!(" {} ", i));
        }

        let border = format!(" -{border}-");
        let legend = format!("  {legend} ");
        let mut result = String::new();

        for (row_idx, row) in self.matrix.iter().enumerate() {
            let mut top = " |".to_string();
            let mut mid = format!("{}|", row_idx);
            let mut bot = " |".to_string();

            for (col_idx, _) in row.iter().enumerate() {
                let pos = Position::new(col_idx, row_idx);
                let s = squares.iter().find(|x| x.at(&pos));
                let (t, m, b) = &self.render_cell(s, row_idx, col_idx);

                top = format!("{}{}", top, t);
                mid = format!("{}{}", mid, m);
                bot = format!("{}{}", bot, b);
            }

            result = format!("{result}\n{top}|\n{mid}|\n{bot}|")
        }

        println!("{legend}\n{border}{result}\n{border}")
    }

    fn render_cell(
        &self,
        square: Option<&Square>,
        row: usize,
        col: usize,
    ) -> (String, String, String) {
        let spacer = if (row + col) % 2 == 0 { "·" } else { " " };
        let buffer = format!("{}{}{}", spacer, spacer, spacer);

        (
            String::from(&buffer),
            match square {
                Some(mov) => {
                    format!(
                        "{}{}{}",
                        spacer,
                        if mov.player == Player::X {
                            mov.player.to_string().red()
                        } else {
                            mov.player.to_string().green()
                        },
                        spacer
                    )
                }
                None => String::from(&buffer),
            },
            String::from(&buffer),
        )
    }
}
