use rscenes::prelude::*;

use super::resources::Resources;

#[derive(Debug)]
pub struct Pause {
    time_lapse: f32,
    threshold: f32,
}

impl Default for Pause {
    fn default() -> Self {
        Self {
            time_lapse: 0.0,
            threshold: 1.0 / 4.0,
        }
    }
}

impl Scene<Resources> for Pause {
    fn update(
        &mut self,
        (handle, _): (&mut RaylibHandle, &RaylibThread),
        dt: f32,
        _: &mut Resources,
    ) -> anyhow::Result<State<Resources>> {
        self.time_lapse += dt;
        if self.time_lapse > self.threshold && handle.is_key_released(KeyboardKey::KEY_F3) {
            return Ok(State::Previous(1));
        }
        Ok(State::Keep)
    }

    fn draw(
        &mut self,
        handle: &mut RaylibDrawHandle,
        screen: Rectangle,
        resources: &Resources,
    ) -> anyhow::Result<()> {
        let font = resources.font.clone();
        let camera = Camera2D {
            zoom: 1.0,
            ..Default::default()
        };
        let mut draw = handle.begin_mode2D(camera);

        let background_color = colors::WHEAT;
        draw.clear_background(background_color);

        let text = "Nonogram";
        let size = measure_text_ex(font.as_ref(), text, 84.0, 2.0);
        let position = Vector2::new((screen.width - size.x) / 2.0, 0.0);
        let bottom = size.y + 64.0;
        draw.draw_text_ex(font.as_ref(), text, position, 84.0, 2.0, colors::DARKCYAN);

        let text = "PAUSED";
        let size = measure_text_ex(font.as_ref(), text, 84.0, 2.0);
        let position = Vector2::new((screen.width - size.x) / 2.0, bottom);
        let bottom = bottom + size.y + 64.0;
        draw.draw_text_ex(font.as_ref(), text, position, 84.0, 2.0, colors::BROWN);

        let text = "ESC resume game";
        let size = measure_text_ex(font.as_ref(), text, 32.0, 2.0);
        let position = Vector2::new((screen.width - size.x) / 2.0, bottom);
        draw.draw_text_ex(font.as_ref(), text, position, 32.0, 2.0, colors::BLACK);

        Ok(())
    }
}
