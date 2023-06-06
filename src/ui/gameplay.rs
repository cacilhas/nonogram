use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use rscenes::prelude::*;

use crate::audio::Sfx;
use crate::audio::SfxType;
use crate::game::Board;
use crate::game::Cell;

use super::pause::Pause;
use super::resources::Resources;

pub struct GameplayScene {
    sfx: Sfx,
    board: Rc<RefCell<dyn Board>>,
    hhints: Vec<String>,
    vhints: Vec<String>,
    size: Vector2,
    board_rect: Rectangle,
    hhints_rect: Rectangle,
    vhints_rect: Rectangle,
    cell_size: Vector2,
    time_lapse: f32,
    vic_index: f32,
    mute: bool,
    done: bool,
}

impl GameplayScene {
    pub fn new(board: Rc<RefCell<dyn Board>>) -> Self {
        let (w, h) = board.borrow().size();
        let size = Vector2::new(w as f32, h as f32);
        let hhints = (0..w)
            .map(|x| {
                board
                    .borrow()
                    .get_hhint(x)
                    .unwrap()
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>();
        let vhints = (0..h)
            .map(|y| {
                board
                    .borrow()
                    .get_vhint(y)
                    .unwrap()
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>();
        Self {
            board,
            size,
            hhints,
            vhints,
            sfx: Sfx::default(),
            board_rect: Rectangle::default(),
            hhints_rect: Rectangle::default(),
            vhints_rect: Rectangle::default(),
            cell_size: Vector2::default(),
            time_lapse: 0.0,
            vic_index: 0.0,
            mute: false,
            done: false,
        }
    }

    fn play(&self, audio: Rc<RefCell<RaylibAudio>>, tpe: SfxType) {
        if !self.mute {
            self.sfx.play(&mut audio.borrow_mut(), tpe);
        }
    }

    fn draw_lines(&self, draw: &mut RaylibMode2D<'_, RaylibDrawHandle>, font: Rc<Font>) {
        let font_size = if self.cell_size.x < self.cell_size.y {
            self.cell_size.x
        } else {
            self.cell_size.y
        } * 0.75
            - 2.0;

        self.draw_vertical_lines(draw, font.clone(), font_size);
        self.draw_horizontal_lines(draw, font.clone(), font_size);
    }

    fn draw_vertical_lines(
        &self,
        draw: &mut RaylibMode2D<'_, RaylibDrawHandle>,
        font: Rc<Font>,
        font_size: f32,
    ) {
        for i in 0..(self.size.x as usize) {
            let x = self.hhints_rect.x + (i as f32 * self.cell_size.x);
            if i % 2 == 0 {
                draw.draw_rectangle(
                    (x - self.cell_size.x / 2.0) as i32,
                    0,
                    self.cell_size.x as i32,
                    self.hhints_rect.height as i32,
                    colors::LIGHTGRAY,
                );
            }
            let mut y = 0.0;
            for text in self.hhints[i].split(' ') {
                draw.draw_text_ex(
                    font.as_ref(),
                    text,
                    Vector2::new(x, y),
                    font_size,
                    1.0,
                    colors::BLACK,
                );
                y += font_size;
            }
            draw.draw_line_ex(
                Vector2::new(
                    self.board_rect.x + (i as f32 * self.cell_size.x),
                    self.board_rect.y,
                ),
                Vector2::new(
                    self.board_rect.x + (i as f32 * self.cell_size.x),
                    self.board_rect.y + (self.size.y * self.cell_size.y),
                ),
                2.0,
                if i % 5 == 0 {
                    colors::BLACK
                } else {
                    colors::DARKGRAY
                },
            );
        }
        draw.draw_line_ex(
            Vector2::new(
                self.board_rect.x + (self.size.y * self.cell_size.x),
                self.board_rect.y,
            ),
            Vector2::new(
                self.board_rect.x + (self.size.x * self.cell_size.x),
                self.board_rect.y + (self.size.y * self.cell_size.y),
            ),
            2.0,
            colors::BLACK,
        );
    }

    fn draw_horizontal_lines(
        &self,
        draw: &mut RaylibMode2D<'_, RaylibDrawHandle>,
        font: Rc<Font>,
        font_size: f32,
    ) {
        for i in 0..(self.size.y as usize) {
            let y = self.vhints_rect.y + (i as f32 * self.cell_size.y) + 4.0;
            if i % 2 == 0 {
                draw.draw_rectangle(
                    self.board_rect.width as i32 + 2,
                    y as i32 - 4,
                    self.vhints_rect.width as i32,
                    self.cell_size.y as i32,
                    colors::LIGHTGRAY,
                );
            }
            let text = &self.vhints[i];
            draw.draw_text_ex(
                font.as_ref(),
                text,
                Vector2::new(self.vhints_rect.x, y),
                font_size,
                1.0,
                colors::BLACK,
            );
            draw.draw_line_ex(
                Vector2::new(
                    self.board_rect.x,
                    self.board_rect.y + (i as f32 * self.cell_size.y),
                ),
                Vector2::new(
                    self.board_rect.x + (self.size.y * self.cell_size.x),
                    self.board_rect.y + (i as f32 * self.cell_size.y),
                ),
                2.0,
                if i % 5 == 0 {
                    colors::BLACK
                } else {
                    colors::DARKGRAY
                },
            );
        }
        draw.draw_line_ex(
            Vector2::new(
                self.board_rect.x,
                self.board_rect.y + (self.size.y * self.cell_size.y),
            ),
            Vector2::new(
                self.board_rect.x + (self.size.x * self.cell_size.x),
                self.board_rect.y + (self.size.y * self.cell_size.y),
            ),
            2.0,
            colors::BLACK,
        );
    }

    fn draw_info(
        &self,
        draw: &mut RaylibMode2D<'_, RaylibDrawHandle>,
        screen: Rectangle,
        font: Rc<Font>,
    ) {
        let size = measure_text_ex(font.as_ref(), "F2 mute/unmute", 12.0, 1.0);
        let x = screen.width - size.x - 4.0;
        let mut y = 28.0;
        draw.draw_text_ex(
            font.as_ref(),
            "F2 mute/unmute",
            Vector2::new(x, y),
            12.0,
            1.0,
            colors::GRAY,
        );
        y += 14.0;
        draw.draw_text_ex(
            font.as_ref(),
            "F3 pause",
            Vector2::new(x, y),
            12.0,
            1.0,
            colors::GRAY,
        );
        y += 14.0;
        draw.draw_text_ex(
            font.as_ref(),
            "ESC abort",
            Vector2::new(x, y),
            12.0,
            1.0,
            colors::GRAY,
        );
    }
}

impl Scene<Resources> for GameplayScene {
    fn update(
        &mut self,
        (handle, _): (&mut RaylibHandle, &RaylibThread),
        dt: f32,
        _: &mut Resources,
    ) -> anyhow::Result<State<Resources>> {
        if handle.is_key_released(KeyboardKey::KEY_F2) {
            self.mute = !self.mute;
        }
        if handle.is_key_released(KeyboardKey::KEY_F3) && !self.board.borrow().is_done() {
            return Ok(State::New(Box::new(Pause::default())));
        }
        if self.done {
            self.vic_index += dt * 5.0;
        } else {
            self.time_lapse += dt;
        }
        Ok(State::Keep)
    }
    fn draw(
        &mut self,
        handle: &mut RaylibDrawHandle,
        screen: Rectangle,
        resources: &Resources,
    ) -> anyhow::Result<()> {
        let audio = resources.audio.clone();
        let font = resources.font.clone();

        self.board_rect = Rectangle {
            x: screen.x,
            y: screen.y + screen.height / 3.0,
            width: screen.width / 1.5,
            height: screen.height / 1.5,
        };
        self.cell_size = Vector2::new(
            self.board_rect.width / self.size.x,
            self.board_rect.height / self.size.y,
        );
        self.hhints_rect = Rectangle {
            x: screen.x + self.cell_size.x / 2.0,
            y: screen.y,
            width: screen.width / 1.5,
            height: screen.height / 3.0,
        };
        self.vhints_rect = Rectangle {
            x: self.board_rect.x + self.board_rect.width + self.cell_size.x / 2.0,
            y: self.board_rect.y,
            width: screen.width / 3.0,
            height: screen.height / 3.0,
        };

        let left_click = handle.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON);
        let right_click = handle.is_mouse_button_released(MouseButton::MOUSE_RIGHT_BUTTON);
        let ctrl = handle.is_key_down(KeyboardKey::KEY_LEFT_CONTROL)
            || handle.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL);
        let right_click = right_click || (left_click && ctrl);
        let left_click = left_click && !ctrl;
        let x = handle.get_mouse_x();
        let y = handle.get_mouse_y();
        let mouse = Vector2::new(x as f32, y as f32);

        let camera = Camera2D {
            zoom: 1.0,
            ..Default::default()
        };
        let mut draw = handle.begin_mode2D(camera);

        let background_color = colors::WHEAT;
        draw.clear_background(background_color);

        for y in 0..(self.size.y as usize) {
            for x in 0..(self.size.x as usize) {
                let current_rect = Rectangle {
                    x: self.board_rect.x + (x as f32) * self.cell_size.x,
                    y: self.board_rect.y + (y as f32) * self.cell_size.y,
                    width: self.cell_size.x,
                    height: self.cell_size.y,
                };

                let board = self.board.clone();
                let mut board = board.borrow_mut();
                if !board.is_done() {
                    if left_click && current_rect.check_collision_point_rec(mouse) {
                        match board.get(x, y)? {
                            Cell::Yes => {
                                board.set(x, y, Cell::Closed)?;
                                self.play(audio.clone(), SfxType::UNSET);
                            }
                            Cell::Closed => {
                                board.set(x, y, Cell::Yes)?;
                                self.play(audio.clone(), SfxType::SET);
                            }
                            Cell::No => self.play(audio.clone(), SfxType::ERROR),
                        }
                    }
                    if right_click && current_rect.check_collision_point_rec(mouse) {
                        match board.get(x, y)? {
                            Cell::No => {
                                board.set(x, y, Cell::Closed)?;
                                self.play(audio.clone(), SfxType::UNSET);
                            }
                            Cell::Closed => {
                                board.set(x, y, Cell::No)?;
                                self.play(audio.clone(), SfxType::LOCK);
                            }
                            Cell::Yes => self.play(audio.clone(), SfxType::ERROR),
                        }
                    }
                }

                match board.get(x, y).unwrap() {
                    Cell::No => {
                        draw.draw_line(
                            current_rect.x as i32,
                            current_rect.y as i32,
                            (current_rect.x + current_rect.width) as i32,
                            (current_rect.y + current_rect.height) as i32,
                            colors::DARKGRAY,
                        );
                        draw.draw_line(
                            current_rect.x as i32,
                            (current_rect.y + current_rect.height) as i32,
                            (current_rect.x + current_rect.width) as i32,
                            current_rect.y as i32,
                            colors::DARKGRAY,
                        );
                    }
                    Cell::Yes => draw.draw_rectangle(
                        current_rect.x as i32,
                        current_rect.y as i32,
                        current_rect.width as i32,
                        current_rect.height as i32,
                        colors::DARKBLUE,
                    ),
                    Cell::Closed => draw.draw_rectangle(
                        current_rect.x as i32,
                        current_rect.y as i32,
                        current_rect.width as i32,
                        current_rect.height as i32,
                        colors::LIGHTPINK,
                    ),
                }
            }
        }

        self.draw_lines(&mut draw, font.clone());

        if self.board.borrow().is_done() {
            let size = measure_text("V", 240) as f32;
            let rect = Rectangle {
                x: self.vhints_rect.x,
                y: screen.y,
                width: size,
                height: 240.0,
            };
            let text = VICTORY[self.vic_index as usize % VICTORY.len()];
            let size = measure_text(text, 240) as f32;
            draw.draw_text_ex(
                font.as_ref(),
                text,
                Vector2::new(rect.x + (rect.width - size) / 2.0, rect.y),
                240.0,
                0.0,
                colors::GREEN,
            );
            for y in 0..(self.size.y as usize) {
                for x in 0..(self.size.x as usize) {
                    if self.board.borrow().get(x, y).unwrap() == Cell::Closed {
                        self.board.borrow_mut().set(x, y, Cell::No).unwrap();
                    }
                }
            }

            if !self.done {
                self.done = true;
                self.play(audio.clone(), SfxType::CLAPPING);
            }
        } else {
        }

        let time = format!(
            "{:02.0}:{:02.0}:{:02.0}",
            self.time_lapse / 3600.0,
            (self.time_lapse / 60.0) % 60.0,
            self.time_lapse % 60.0,
        );
        //let size = measure_text_ex(font.as_ref(), &time, 12.0, 2.0);
        monospace(
            &mut draw,
            font.clone(),
            &time,
            Vector2::new(screen.width - 96.0, 4.0),
            12.0,
            colors::DARKGRAY,
        );
        if self.mute {
            draw.draw_text_ex(
                font.as_ref(),
                "M",
                Vector2::new(screen.width - 112.0, 4.0),
                12.0,
                0.0,
                colors::BROWN,
            );
            draw.draw_text_ex(
                font.as_ref(),
                "\\",
                Vector2::new(screen.width - 112.0, 4.0),
                12.0,
                0.0,
                colors::RED,
            );
        }
        self.draw_info(&mut draw, screen, font.clone());

        Ok(())
    }
}

impl fmt::Debug for GameplayScene {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "gameplay scene")
    }
}

fn monospace(
    draw: &mut RaylibMode2D<'_, RaylibDrawHandle>,
    font: Rc<Font>,
    text: &str,
    position: Vector2,
    font_size: f32,
    tint: Color,
) {
    for (i, c) in text.as_bytes().iter().enumerate() {
        let x = position.x + (i as f32) * font_size;
        let y = position.y;
        draw.draw_text_ex(
            font.as_ref(),
            &char::from(*c).to_string(),
            Vector2::new(x, y),
            font_size,
            0.0,
            tint,
        );
    }
}

static VICTORY: [&'static str; 6] = ["W", "w", "v", ".", "v", "w"];
