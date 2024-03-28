mod cheats;

use std::time::Duration;

use hs_hackathon::prelude::*;

use cheats::angles::Vector;
use cheats::approaching::Hint;
use cheats::positioning::Position;
use cheats::TeamColors;
use rust_nation_starter::car_detection;

const CAR: Color = Color::Red;
const TARGET: Color = Color::Blue;

#[allow(unused)]
struct MapState {
    car: Position,
    target: Position,
}

#[allow(unused)]
impl MapState {
    pub async fn infer(drone: &mut Camera) -> eyre::Result<Self> {
        unimplemented!()
    }

    async fn car_orientation(
        current: Position,
        drone: &mut Camera,
        motor: &mut MotorSocket,
        wheels: &mut WheelOrientation,
    ) -> eyre::Result<Vector> {
        unimplemented!()
    }
}

#[derive(Debug)]
#[allow(unused)]
enum State {
    /// Turn the cars direction by doing consecutive front and back movements
    /// until the angle between the cars orientation and the target converges to be under
    /// a specified threshold
    Turning,
    /// Approach the car by doing incremental actions of approaching and measuring interleaved.
    /// So we approach the target a bit, measure if we decreased the distance, if yes repeat, if no
    /// then calibrate. We do this until we hit the target.
    Approaching,
    /// Simply idling on the target and identifying when the target moves away from our current
    /// position.
    Idle,
}

static ANGLE_THRESHOLD: f64 = 0.1;
static STEP_DURATION: Duration = Duration::from_millis(500);

impl State {
    async fn execute(
        &mut self,
        drone: &mut Camera,
        motor: &mut MotorSocket,
        wheels: &mut WheelOrientation,
    ) -> eyre::Result<()> {
        println!("state: {:?}", self);
        match self {
            State::Turning => {
                let mut wheels_orientation = Angle::straight();
                loop {
                    // TODO: replace this with getting the current angle between our direction and the target
                    let frame = drone.snapshot().await?;
                    let angle = car_detection(&frame, Color::Blue, Color::Green)?;
                    if angle < ANGLE_THRESHOLD {
                        break;
                    }
                    // TODO: validate direction is correct
                    if angle > 0.0 && wheels_orientation != Angle::right() {
                        wheels_orientation = Angle::right();
                        wheels.set(wheels_orientation).await?;
                    };

                    // TODO: do we ever need to go backward?
                    motor.move_for(Velocity::forward(), STEP_DURATION).await?;
                    println!("moving forward");
                }
                // test
                *self = Self::Approaching;
            }
            State::Approaching => {
                let hint = cheats::approaching::auto(
                    &TeamColors {
                        car: CAR,
                        target: TARGET,
                    },
                    drone,
                    motor,
                    wheels,
                )
                .await?;

                *self = match hint {
                    Hint::TargetWasHit => Self::Idle,
                    Hint::OrientationIsOff => Self::Turning,
                };
            }
            State::Idle => {
                cheats::idling::auto(
                    &TeamColors {
                        car: CAR,
                        target: TARGET,
                    },
                    drone,
                    motor,
                    wheels,
                )
                .await?;

                *self = Self::Turning;
            }
        }
        Ok(())
    }
}

#[hs_hackathon::main]
async fn main() -> eyre::Result<()> {
    let mut wheels = WheelOrientation::new().await?;
    let mut motor = MotorSocket::open().await?;
    let mut drone = Camera::connect().await?;

    let mut machine = State::Turning;

    loop {
        machine.execute(&mut drone, &mut motor, &mut wheels).await?;
        tracing::debug!("{:?}", machine);
    }
}
