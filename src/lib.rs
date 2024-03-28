use cheats::positioning::Position;
use hs_hackathon::{
    drone::Frame,
    prelude::eyre::{self, ContextCompat},
    vision::{detect as led_detection, Color, LedDetectionConfig},
};
use imageproc::corners::corners_fast9;

mod cheats;

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
