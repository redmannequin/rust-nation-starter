use cheats::positioning::Position;
use hs_hackathon::{
    drone::Frame,
    prelude::eyre::{self, ContextCompat},
    vision::{detect as led_detection, Color, LedDetectionConfig},
};
use imageproc::corners::{corners_fast9, Corner};

mod cheats;

#[derive(Debug, Clone)]
struct CornerDistance<'a> {
    corner: &'a Corner,
    distance: u32,
}

fn get_closest_corners<'a>(led_position: &'_ Position, corners: &'a [Corner]) -> Vec<&'a Corner> {
    // TODO: we are sorting the full array of corners. We can optimize this by using a min heap instead
    let mut closest_corners: Vec<CornerDistance<'_>> = corners
        .iter()
        .map(|corner| CornerDistance {
            corner,
            distance: led_position.distance(Position::new(corner.x, corner.y)),
        })
        .collect();
    closest_corners.sort_by(|a, b| a.distance.cmp(&b.distance));
    closest_corners.iter().map(|corner| corner.corner).collect()
}

pub fn car_detection(frame: &Frame, led_color: Color) -> eyre::Result<()> {
    let leds = led_detection(&frame.0, &LedDetectionConfig::default())?;

    let led_pos = leds
        .iter()
        .find(|&led| led.color == led_color)
        .map(|led| Position::from(led.bbox))
        .context(format!("car led not in camara view: {:?}", led_color))?;

    let _corners: Vec<_> = corners_fast9(&frame.0.to_luma8(), 150)
        .into_iter()
        .filter_map(|corner| {
            let pos = Position::new(corner.x, corner.y);
            if led_pos.distance(pos) < 100 {
                Some(pos)
            } else {
                None
            }
        })
        .collect();

    todo!()
}

trait ColorExt {
    #[allow(clippy::wrong_self_convention)]
    fn as_str(self) -> &'static str;
}

impl ColorExt for Color {
    fn as_str(self) -> &'static str {
        match self {
            Color::Blue => "blue",
            Color::Red => "red",
            Color::Green => "green",
            Color::White => "white",
            Color::Unknown => "unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use imageproc::corners::Corner;

    #[test]
    fn test_get_closest_corners() {
        let led_position = Position::new(0, 0);
        let corners = vec![
            Corner {
                x: 1,
                y: 1,
                score: 0_f32,
            },
            Corner {
                x: 3,
                y: 3,
                score: 0_f32,
            },
            Corner {
                x: 4,
                y: 4,
                score: 0_f32,
            },
            Corner {
                x: 2,
                y: 2,
                score: 0_f32,
            },
        ];

        let closest_corners = get_closest_corners(&led_position, &corners);
        assert_eq!(closest_corners.len(), 4);
        assert_eq!(closest_corners[0].x, 1);
        assert_eq!(closest_corners[0].y, 1);
        assert_eq!(closest_corners[1].x, 2);
        assert_eq!(closest_corners[1].y, 2);
    }
}
