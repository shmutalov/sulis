//  This file is part of Sulis, a turn based RPG written in Rust.
//  Copyright 2018 Jared Stephen
//
//  Sulis is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  Sulis is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with Sulis.  If not, see <http://www.gnu.org/licenses/>

use std::collections::HashMap;
use std::io::Error;
use std::rc::Rc;

use crate::image::Image;
use crate::io::{DrawList, GraphicsRenderer};
use crate::ui::AnimationState;
use crate::util::{invalid_data_error, Size};

#[derive(Debug)]
pub struct TimerImage {
    id: String,
    frames: Vec<Rc<dyn Image>>,
    frame_time_millis: u32,
    size: Size,
}

impl TimerImage {
    pub fn new(
        builder: TimerImageBuilder,
        images: &HashMap<String, Rc<dyn Image>>,
    ) -> Result<Rc<dyn Image>, Error> {
        let mut frames: Vec<Rc<dyn Image>> = Vec::new();

        if builder.frames.is_empty() {
            return invalid_data_error("Timer image must have 1 or more frames.");
        }

        let mut size: Option<Size> = None;
        for id in builder.frames {
            let image = match images.get(&id) {
                None => {
                    return invalid_data_error(&format!("Unable to locate image for frame {}", id));
                }
                Some(image) => image,
            };

            match size {
                None => size = Some(*image.get_size()),
                Some(size) => {
                    if size != *image.get_size() {
                        return invalid_data_error(&format!(
                            "All frames in a timer image must have the\
                             same size."
                        ));
                    }
                }
            }

            frames.push(Rc::clone(&image));
        }

        Ok(Rc::new(TimerImage {
            frames,
            size: size.unwrap(),
            frame_time_millis: builder.frame_time_millis,
            id: builder.id,
        }))
    }

    fn get_cur_frame(&self, millis: u32) -> &Rc<dyn Image> {
        let total_frame_time = self.frame_time_millis * self.frames.len() as u32;
        let offset = millis % total_frame_time;
        let index = (offset / self.frame_time_millis) as usize;

        &self.frames[index]
    }
}

impl Image for TimerImage {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn draw(
        &self,
        renderer: &mut dyn GraphicsRenderer,
        state: &AnimationState,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        millis: u32,
    ) {
        self.get_cur_frame(millis)
            .draw(renderer, state, x, y, w, h, millis);
    }

    fn append_to_draw_list(
        &self,
        draw_list: &mut DrawList,
        state: &AnimationState,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        millis: u32,
    ) {
        self.get_cur_frame(millis)
            .append_to_draw_list(draw_list, state, x, y, w, h, millis);
    }

    fn get_width_f32(&self) -> f32 {
        self.size.width as f32
    }

    fn get_height_f32(&self) -> f32 {
        self.size.height as f32
    }

    fn get_size(&self) -> &Size {
        &self.size
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct TimerImageBuilder {
    id: String,
    frames: Vec<String>,
    frame_time_millis: u32,
}
