use core::mem;

use sdl2::{
    pixels::Color,
    rect::Point,
    render::{Canvas, RenderTarget},
};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct OneColored {
    color: Color,
    points: Vec<Point>,
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum DrawError {
    #[error("{0}")]
    Draw(String),
    #[error("Line was empty.")]
    Empty,
}

impl OneColored {
    #[must_use]
    #[inline]
    pub fn new_45_deg(start: Point, end: Point, color: Color) -> Self {
        let distance_x = end.x - start.x;
        let distance_y = start.y - end.y;
        let mut decision = 2 * distance_y - distance_x;
        let mut points = Vec::new();
        let mut x = start.x;
        let mut y = start.y;

        for _ in 0..distance_x {
            points.push((x, y).into());

            if decision > 0 {
                y -= 1;
                decision += 2 * (distance_y - distance_x);
            } else {
                decision += 2 * distance_y;
            }

            x += 1;
        }

        Self { color, points }
    }

    #[must_use]
    #[inline]
    pub fn new_all_deg(start: Point, end: Point, color: Color) -> Self {
        let mut distance_x = (end.x - start.x).abs();
        let mut distance_y = (start.y - end.y).abs();
        let sign_x = (end.x - start.x).signum();
        let sign_y = (start.y - end.y).signum();
        let swapped = if distance_x < distance_y {
            mem::swap(&mut distance_x, &mut distance_y);
            true
        } else {
            false
        };
        let mut decision = 2 * distance_y - distance_x;
        let mut x = start.x;
        let mut y = start.y;
        let mut points = Vec::from([(x, y).into()]);

        while x != end.x || y != end.y {
            if decision > 0 {
                if swapped {
                    x += sign_x;
                } else {
                    y -= sign_y;
                }
                decision -= 2 * distance_x;
            }

            if swapped {
                y -= sign_y;
            } else {
                x += sign_x;
            }

            decision += 2 * distance_y;
            points.push((x, y).into());
        }

        Self { color, points }
    }

    #[inline]
    pub fn draw<T>(&self, canvas: &mut Canvas<T>) -> Result<(), DrawError>
    where
        T: RenderTarget,
    {
        let old_color = canvas.draw_color();

        if self.points.is_empty() {
            return Err(DrawError::Empty);
        }

        canvas.set_draw_color(self.color);
        canvas.draw_points(&*self.points).map_err(DrawError::Draw)?;

        canvas.set_draw_color(old_color);

        Ok(())
    }
}
