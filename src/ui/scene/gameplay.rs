use std::cell::RefCell;
use std::rc::Rc;

use raylib::prelude::*;

use crate::audio::Sfx;
use crate::audio::SfxType;
use crate::game::Board;
use crate::game::Cell;

use super::pause::Pause;
use super::{Scene, State};

pub struct GameplayScene {
    audio: Option<Rc<RefCell<RaylibAudio>>>,
    sfx: Sfx,
    board: Box<dyn Board>,
    hhints: Vec<String>,
    vhints: Vec<String>,
    font: Rc<Font>,
    size: Vector2,
    window: Rectangle,
    board_rect: Rectangle,
    hhints_rect: Rectangle,
    vhints_rect: Rectangle,
    cell_size: Vector2,
    time_lapse: chrono::Duration,
    vic_index: f32,
    mute: bool,
    done: bool,
}

impl GameplayScene {
    pub fn new(board: Box<dyn Board>) -> Self {
        let (w, h) = board.size();
        let size = Vector2::new(w as f32, h as f32);
        let hhints = (0..w)
            .map(|x| {
                board
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
                    .get_vhint(y)
                    .unwrap()
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>();
        let font = unsafe { ffi::GetFontDefault() };
        let font = unsafe { Font::from_raw(font) };
        Self {
            board,
            size,
            hhints,
            vhints,
            audio: None,
            sfx: Sfx::default(),
            font: font.into(),
            window: Rectangle::default(),
            board_rect: Rectangle::default(),
            hhints_rect: Rectangle::default(),
            vhints_rect: Rectangle::default(),
            cell_size: Vector2::default(),
            time_lapse: chrono::Duration::zero(),
            vic_index: 0.0,
            mute: false,
            done: false,
        }
    }

    fn play(&self, tpe: &SfxType) {
        if !self.mute {
            if let Some(audio) = &self.audio {
                self.sfx.play(&mut audio.borrow_mut(), tpe);
            }
        }
    }

    fn draw_lines(&self, draw: &mut RaylibMode2D<'_, RaylibDrawHandle>) {
        let font_size = if self.cell_size.x < self.cell_size.y {
            self.cell_size.x
        } else {
            self.cell_size.y
        } * 0.75
            - 2.0;

        self.draw_vertical_lines(draw, font_size);
        self.draw_horizontal_lines(draw, font_size);
    }

    fn draw_vertical_lines(&self, draw: &mut RaylibMode2D<'_, RaylibDrawHandle>, font_size: f32) {
        for i in 0..(self.size.x as usize) {
            let x = self.hhints_rect.x + (i as f32 * self.cell_size.x);
            if i % 2 == 0 {
                draw.draw_rectangle(
                    (x - self.cell_size.x / 2.0) as i32,
                    0,
                    self.cell_size.x as i32,
                    self.hhints_rect.height as i32,
                    Color::LIGHTGRAY,
                );
            }
            let mut y = 0.0;
            for text in self.hhints[i].split(' ') {
                draw.draw_text_ex(
                    self.font.as_ref(),
                    text,
                    Vector2::new(x, y),
                    font_size,
                    1.0,
                    Color::BLACK,
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
                    Color::BLACK
                } else {
                    Color::DARKGRAY
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
            Color::BLACK,
        );
    }

    fn draw_horizontal_lines(&self, draw: &mut RaylibMode2D<'_, RaylibDrawHandle>, font_size: f32) {
        for i in 0..(self.size.y as usize) {
            let y = self.vhints_rect.y + (i as f32 * self.cell_size.y) + 4.0;
            if i % 2 == 0 {
                draw.draw_rectangle(
                    self.board_rect.width as i32 + 2,
                    y as i32 - 4,
                    self.vhints_rect.width as i32,
                    self.cell_size.y as i32,
                    Color::LIGHTGRAY,
                );
            }
            let text = &self.vhints[i];
            draw.draw_text_ex(
                self.font.as_ref(),
                text,
                Vector2::new(self.vhints_rect.x, y),
                font_size,
                1.0,
                Color::BLACK,
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
                    Color::BLACK
                } else {
                    Color::DARKGRAY
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
            Color::BLACK,
        );
    }

    fn draw_info(&self, draw: &mut RaylibMode2D<'_, RaylibDrawHandle>) {
        let x = self.window.width - 148.0;
        let mut y = 28.0;
        draw.draw_text_ex(
            self.font.as_ref(),
            "F2 mute/unmute",
            Vector2::new(x, y),
            12.0,
            1.0,
            Color::GRAY,
        );
        y += 14.0;
        draw.draw_text_ex(
            self.font.as_ref(),
            "F3 to pause",
            Vector2::new(x, y),
            12.0,
            1.0,
            Color::GRAY,
        );
        y += 14.0;
        draw.draw_text_ex(
            self.font.as_ref(),
            "ESC abort back to menu",
            Vector2::new(x, y),
            12.0,
            1.0,
            Color::GRAY,
        );
    }
}

impl Scene for GameplayScene {
    fn init(
        &mut self,
        handle: &mut raylib::RaylibHandle,
        _: &raylib::RaylibThread,
        rect: raylib::prelude::Rectangle,
        font: Rc<Font>,
        audio: Rc<RefCell<RaylibAudio>>,
    ) {
        handle.set_exit_key(None);
        self.font = font;
        self.audio = Some(audio);
        self.window = rect;
        self.board_rect = Rectangle {
            x: rect.x,
            y: rect.y + rect.height / 3.0,
            width: rect.width / 1.5,
            height: rect.height / 1.5,
        };
        self.board_rect.height = rect.height - self.board_rect.y;
        self.cell_size = Vector2::new(
            self.board_rect.width / self.size.x,
            self.board_rect.height / (self.size.y + 1.0),
        );
        self.hhints_rect = Rectangle {
            x: rect.x + self.cell_size.x / 2.0,
            y: rect.y,
            width: rect.width / 1.5,
            height: rect.height / 3.0,
        };
        self.vhints_rect = Rectangle {
            x: self.board_rect.x + self.board_rect.width + self.cell_size.x / 2.0,
            y: self.board_rect.y,
            width: rect.width / 3.0,
            height: rect.height / 3.0,
        };
    }

    fn update(
        &mut self,
        dt: chrono::Duration,
        handle: &mut raylib::RaylibHandle,
        thr: &raylib::RaylibThread,
    ) -> State {
        let left_click = handle.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT);
        let right_click = handle.is_mouse_button_released(MouseButton::MOUSE_BUTTON_RIGHT);
        let ctrl = handle.is_key_down(KeyboardKey::KEY_LEFT_CONTROL)
            || handle.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL);
        let right_click = right_click || (left_click && ctrl);
        let left_click = left_click && !ctrl;
        let x = handle.get_mouse_x();
        let y = handle.get_mouse_y();
        let mouse = Vector2::new(x as f32, y as f32);

        if handle.is_key_released(KeyboardKey::KEY_F2) {
            self.mute = !self.mute;
        }

        if handle.is_key_released(KeyboardKey::KEY_F3) && !self.board.is_done() {
            return State::New(Rc::new(RefCell::new(Pause::default())));
        }

        if handle.is_key_released(KeyboardKey::KEY_ESCAPE) {
            return State::Previous(1);
        }

        let camera = Camera2D {
            zoom: 1.0,
            ..Default::default()
        };
        let mut base_draw = handle.begin_drawing(thr);
        let mut draw = base_draw.begin_mode2D(camera);

        let background_color = Color::WHEAT;
        draw.clear_background(background_color);

        for y in 0..(self.size.y as usize) {
            for x in 0..(self.size.x as usize) {
                let current_rect = Rectangle {
                    x: self.board_rect.x + (x as f32) * self.cell_size.x,
                    y: self.board_rect.y + (y as f32) * self.cell_size.y,
                    width: self.cell_size.x,
                    height: self.cell_size.y,
                };

                if !self.board.is_done() {
                    if left_click && current_rect.check_collision_point_rec(mouse) {
                        match self.board.get(x, y).unwrap() {
                            Cell::Yes => {
                                self.board.set(x, y, Cell::Closed).unwrap();
                                self.play(&SfxType::UNSET);
                            }
                            Cell::Closed => {
                                self.board.set(x, y, Cell::Yes).unwrap();
                                self.play(&SfxType::SET);
                            }
                            Cell::No => self.play(&SfxType::ERROR),
                        }
                    }
                    if right_click && current_rect.check_collision_point_rec(mouse) {
                        match self.board.get(x, y).unwrap() {
                            Cell::No => {
                                self.board.set(x, y, Cell::Closed).unwrap();
                                self.play(&SfxType::UNSET);
                            }
                            Cell::Closed => {
                                self.board.set(x, y, Cell::No).unwrap();
                                self.play(&SfxType::LOCK);
                            }
                            Cell::Yes => self.play(&SfxType::ERROR),
                        }
                    }
                }

                match self.board.get(x, y).unwrap() {
                    Cell::No => {
                        draw.draw_line(
                            current_rect.x as i32,
                            current_rect.y as i32,
                            (current_rect.x + current_rect.width) as i32,
                            (current_rect.y + current_rect.height) as i32,
                            Color::DARKGRAY,
                        );
                        draw.draw_line(
                            current_rect.x as i32,
                            (current_rect.y + current_rect.height) as i32,
                            (current_rect.x + current_rect.width) as i32,
                            current_rect.y as i32,
                            Color::DARKGRAY,
                        );
                    }
                    Cell::Yes => draw.draw_rectangle(
                        current_rect.x as i32,
                        current_rect.y as i32,
                        current_rect.width as i32,
                        current_rect.height as i32,
                        Color::DARKBLUE,
                    ),
                    Cell::Closed => draw.draw_rectangle(
                        current_rect.x as i32,
                        current_rect.y as i32,
                        current_rect.width as i32,
                        current_rect.height as i32,
                        Color::LIGHTPINK,
                    ),
                }
            }
        }

        self.draw_lines(&mut draw);

        if self.board.is_done() {
            let size = measure_text("V", 240) as f32;
            let rect = Rectangle {
                x: self.vhints_rect.x,
                y: self.window.y,
                width: size,
                height: 240.0,
            };
            let text = VICTORY[self.vic_index as usize % VICTORY.len()];
            let size = measure_text(text, 240) as f32;
            draw.draw_text_ex(
                self.font.as_ref(),
                text,
                Vector2::new(rect.x + (rect.width - size) / 2.0, rect.y),
                240.0,
                0.0,
                Color::GREEN,
            );
            for y in 0..(self.size.y as usize) {
                for x in 0..(self.size.x as usize) {
                    if self.board.get(x, y).unwrap() == Cell::Closed {
                        self.board.set(x, y, Cell::No).unwrap();
                    }
                }
            }
            self.vic_index +=
                ((dt.num_seconds() as f32) + (dt.num_milliseconds() as f32 / 1_000.0)) * 5.0;

            if !self.done {
                self.done = true;
                self.play(&SfxType::CLAPPING);
            }
        } else {
            self.time_lapse = self.time_lapse.checked_add(&dt).unwrap();
        }

        let time = format!(
            "{:02}:{:02}:{:02}",
            self.time_lapse.num_hours(),
            self.time_lapse.num_minutes() % 60,
            self.time_lapse.num_seconds() % 60,
        );
        //let size = measure_text_ex(self.font.as_ref(), &time, 12.0, 2.0);
        monospace(
            &mut draw,
            self.font.clone(),
            &time,
            Vector2::new(self.window.width - 96.0, 4.0),
            12.0,
            Color::DARKGRAY,
        );
        if self.mute {
            draw.draw_text_ex(
                self.font.as_ref(),
                "M",
                Vector2::new(self.window.width - 112.0, 4.0),
                12.0,
                0.0,
                Color::BROWN,
            );
            draw.draw_text_ex(
                self.font.as_ref(),
                "\\",
                Vector2::new(self.window.width - 112.0, 4.0),
                12.0,
                0.0,
                Color::RED,
            );
        }
        self.draw_info(&mut draw);

        State::Keep
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
