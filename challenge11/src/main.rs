#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

// Constant for number of seconds in one Earth year
const EARTH_YEAR_SECONDS: f64 = 31_557_600.0;

// Orbital periods in Earth years for each planet
const MERCURY_YEAR: f64 = 0.2408467;
const VENUS_YEAR: f64 = 0.61519726;
const EARTH_YEAR: f64 = 1.0;
const MARS_YEAR: f64 = 1.8808158;
const JUPITER_YEAR: f64 = 11.862615;
const SATURN_YEAR: f64 = 29.447498;
const URANUS_YEAR: f64 = 84.016846;
const NEPTUNE_YEAR: f64 = 164.79132;

// Implementations per planet
pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / (EARTH_YEAR_SECONDS * MERCURY_YEAR)
    }
}

impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / (EARTH_YEAR_SECONDS * VENUS_YEAR)
    }
}

impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / (EARTH_YEAR_SECONDS * EARTH_YEAR)
    }
}

impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / (EARTH_YEAR_SECONDS * MARS_YEAR)
    }
}

impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / (EARTH_YEAR_SECONDS * JUPITER_YEAR)
    }
}

impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / (EARTH_YEAR_SECONDS * SATURN_YEAR)
    }
}

impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / (EARTH_YEAR_SECONDS * URANUS_YEAR)
    }
}

impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / (EARTH_YEAR_SECONDS * NEPTUNE_YEAR)
    }
}

fn main() {
    let seconds = 1_000_000_000;
    let duration = Duration::from(seconds);

    println!("Given age in seconds: {}", seconds);
    println!("Age on Mercury: {:.2}", Mercury::years_during(&duration));
    println!("Age on Venus: {:.2}", Venus::years_during(&duration));
    println!("Age on Earth: {:.2}", Earth::years_during(&duration));
    println!("Age on Mars: {:.2}", Mars::years_during(&duration));
    println!("Age on Jupiter: {:.2}", Jupiter::years_during(&duration));
    println!("Age on Saturn: {:.2}", Saturn::years_during(&duration));
    println!("Age on Uranus: {:.2}", Uranus::years_during(&duration));
    println!("Age on Neptune: {:.2}", Neptune::years_during(&duration));
}