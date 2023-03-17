enum TrafficLight {
    Red,
    Yellow,
    Green,
}

trait TimeDuration {
    fn duration(&self) -> u32;
}

impl TimeDuration for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Green => 45,
            TrafficLight::Yellow => 5,
        }
    }
}

fn main() {
    let red_light = TrafficLight::Red;
    let green_light = TrafficLight::Green;
    let yellow_light = TrafficLight::Yellow;

    println!("Red light duration: {} seconds", red_light.duration());
    println!("Green light duration: {} seconds", green_light.duration());
    println!("Yellow light duration: {} seconds", yellow_light.duration());
}
