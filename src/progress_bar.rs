/*
* Rust-music-player - Copyright (c) 2014 Gomez Guillaume.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
*
* 3. This notice may not be removed or altered from any source distribution.
*/

#![allow(dead_code)]
#![allow(unused_variables)]

use graphic_element::GraphicElement;
use sfml::graphics::{Color, Font, RectangleShape, RenderTarget, RenderWindow};
use sfml::graphics::{Shape, Transformable};
use sfml::system::Vector2f;
pub struct ProgressBar<'b> {
    line: RectangleShape<'b>,
    pub maximum: usize,
    value: usize,
    real_value: usize,
    cleaner: RectangleShape<'b>,
    need_to_draw: bool,
    name: String,
}

impl<'b> ProgressBar<'b> {
    fn init(mut self, color: &Color, position: &Vector2f) -> ProgressBar<'b> {
        self.set_position(position);
        self.line.set_fill_color(color);
        self.cleaner.set_fill_color(&Color::rgb(0, 0, 0));
        self.cleaner.set_outline_color(&Color::rgb(255, 255, 255));
        self.cleaner.set_outline_thickness(1f32);
        self
    }

    pub fn set_progress(&mut self, position: usize) {
        let tmp = if position > self.maximum {
            self.maximum
        } else {
            position
        };
        if self.maximum > 0usize {
            let new_value = tmp * (self.cleaner.size().x as usize - 2usize) / self.maximum;

            if new_value != self.value {
                self.need_to_draw = true;
                self.value = new_value;
                self.real_value = position;
                self.line.set_size(Vector2f {
                    x: self.value as f32,
                    y: self.cleaner.size().y as f32 - 2f32,
                });
            }
        }
    }

    pub fn get_real_value(&self) -> usize {
        self.real_value
    }

    pub fn set_maximum(&mut self, maximum: usize) {
        self.maximum = maximum;
    }
}

impl<'b> GraphicElement<'b> for ProgressBar<'b> {
    fn new_init(
        size: &Vector2f,
        position: &Vector2f,
        color: &Color,
        unused: Option<&Font>,
    ) -> ProgressBar<'b> {
        ProgressBar {
            line: RectangleShape::with_size(Vector2f { x: 0f32, y: size.y }),
            maximum: 0usize,
            value: 0usize,
            real_value: 0usize,
            name: String::new(),
            cleaner: RectangleShape::with_size(Vector2f {
                x: size.x as f32 + 1f32,
                y: size.y as f32 + 1f32,
            }),
            need_to_draw: true,
        }.init(color, position)
    }

    fn is_inside(&self, position: &Vector2f) -> bool {
        position.y >= self.line.position().y
            && position.y <= self.line.position().y + self.cleaner.size().y
            && position.x >= self.line.position().x
            && position.x <= self.line.position().x + self.cleaner.size().x
    }

    fn clicked(&mut self, position: &Vector2f) {
        let in_order =
            (position.x - self.line.position().x) / (self.cleaner.size().x - 1f32) * 100f32;
        let tmp_maximum = self.maximum;

        self.set_progress((in_order * tmp_maximum as f32 / 100f32) as usize);
    }

    fn draw(&mut self, window: &mut RenderWindow) {
        window.draw(&self.cleaner);
        window.draw(&self.line);
        self.need_to_draw = false;
    }

    fn get_size(&self) -> Vector2f {
        let tmp = self.cleaner.size();

        Vector2f {
            x: tmp.x + 2f32,
            y: tmp.y + 2f32,
        }
    }

    fn set_size(&mut self, size: &Vector2f) {
        self.need_to_draw = true;
        self.cleaner.set_size(Vector2f {
            x: size.x - 2f32,
            y: size.y - 2f32,
        });
        let tmp_real_value = self.real_value;
        self.set_progress(tmp_real_value);
    }

    fn get_position(&self) -> Vector2f {
        let tmp = self.cleaner.position();

        Vector2f {
            x: tmp.x - 1f32,
            y: tmp.y - 1f32,
        }
    }

    fn set_position(&mut self, position: &Vector2f) {
        self.need_to_draw = true;
        self.line.set_position(Vector2f {
            x: position.x + 2f32,
            y: position.y + 2f32,
        });
        self.cleaner.set_position(Vector2f {
            x: position.x + 1f32,
            y: position.y + 1f32,
        });
    }

    fn cursor_moved(&mut self, position: &Vector2f) {}

    fn get_min_size(&self) -> Vector2f {
        Vector2f { x: 3f32, y: 3f32 }
    }

    fn get_max_size(&self) -> Option<Vector2f> {
        None
    }

    fn set_element_name(&mut self, name: &String) {
        self.name = name.clone();
    }

    fn get_element_name<'a>(&'a self) -> &'a String {
        &self.name
    }

    fn mouse_leave(&mut self) {}
}
