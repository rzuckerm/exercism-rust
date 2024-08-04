#[derive(Debug)]
pub struct Duration(f64);

const SECS_PER_EARTH_YEARS: f64 = 31557600.0;

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s as f64 / SECS_PER_EARTH_YEARS)
    }
}

pub trait Planet {
    const ORBITAL_PERIOD: f64;

    fn years_during(d: &Duration) -> f64 {
        d.0 / Self::ORBITAL_PERIOD
    }
}

macro_rules! planets {
    ($($planet:tt : $period:expr), *) => {
        $(
            pub struct $planet;
            impl Planet for $planet {
                const ORBITAL_PERIOD: f64 = $period;
            }
        )*
    }
}

planets!(
    Mercury: 0.2408467,
    Venus: 0.61519726,
    Earth: 1.0,
    Mars: 1.8808158,
    Jupiter: 11.862615,
    Saturn: 29.447498,
    Uranus: 84.016846,
    Neptune: 164.79132
);
