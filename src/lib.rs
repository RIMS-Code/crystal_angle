#![warn(clippy::all, rust_2018_idioms)]

mod app;

pub use app::XtalCalculatorApp;

use enterpolation::linear::Linear;
use enterpolation::Generator;

#[derive(PartialEq)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum Harmonic { Second, Third }

struct CrystalData {
    wavelengths: Vec<f32>,
    angles: Vec<f32>,
    walkoffs: Vec<f32>,
}




pub fn calculate_angle_walkoff(wavelength: f32, harmonic: &Harmonic) -> (f32, f32) {

    let second_harmonic_lbo: CrystalData = CrystalData {
        wavelengths: vec![
            700.0, 710.0, 720.0, 730.0, 740.0, 750.0, 760.0, 770.0, 780.0, 790.0, 800.0, 810.0, 820.0, 830.0, 840.0, 850.0, 860.0, 870.0, 880.0, 890.0, 900.0, 910.0, 920.0, 930.0, 940.0, 950.0, 960.0, 970.0, 980.0, 990.0, 1000.0,
        ],
        angles: vec![
            43.4, 42.0, 40.7, 39.4, 38.2, 37.0, 35.9, 34.7, 33.7, 32.6, 31.6, 30.7, 29.7, 28.8, 27.9, 27.0, 26.1, 25.3, 24.4, 23.6, 22.8, 22.0, 21.3, 20.5, 19.8, 19.0, 18.3, 17.6, 16.9, 16.2, 15.6,
        ],
        walkoffs: vec![
            18.83, 18.71, 18.55, 18.36, 18.15, 17.91, 17.66, 17.38, 17.1, 16.79, 16.48, 16.16, 15.83, 15.49, 15.15, 14.8, 14.44, 14.08, 13.72, 13.36, 12.99, 12.62, 12.25, 11.88, 11.51, 11.14, 10.77, 10.39, 10.02, 9.65, 9.27,
        ],
    };

    let third_harmonic_bbo: CrystalData = CrystalData {
        wavelengths: vec![
            700.0, 710.0, 720.0, 730.0, 740.0, 750.0, 760.0, 770.0, 780.0, 790.0, 800.0, 810.0, 820.0, 830.0, 840.0, 850.0, 860.0, 870.0, 880.0, 890.0, 900.0, 910.0, 920.0, 930.0, 940.0, 950.0, 960.0, 970.0, 980.0, 990.0, 1000.0,
        ],
        angles: vec![
            53.3, 53.0, 51.8, 50.7, 49.6, 48.6, 47.7, 46.8, 45.9, 45.1, 44.3, 43.5, 42.8, 42.1, 41.5, 40.8, 40.2, 39.6, 39.1, 38.5, 38.0, 37.5, 37.0, 36.5, 36.0, 35.6, 35.2, 34.7, 34.3, 33.9, 33.5
        ],
        walkoffs: vec![
            85.81, 86.52, 87.01, 87.32, 87.48, 87.52, 87.45, 87.3, 87.06, 86.77, 86.42, 86.03, 85.6, 85.14, 84.66, 84.15, 83.62, 83.09, 82.54, 81.98, 81.41, 80.84, 80.27, 79.7, 79.12, 78.55, 77.98, 77.41, 76.85, 76.29, 75.73
        ],
    };
    
    let (wl_dat, ang_dat, wo_dat) = match harmonic {
        Harmonic::Second => (second_harmonic_lbo.wavelengths, second_harmonic_lbo.angles, second_harmonic_lbo.walkoffs),
        Harmonic::Third => (third_harmonic_bbo.wavelengths, third_harmonic_bbo.angles, third_harmonic_bbo.walkoffs),
    };


    let angle = interpolate(wl_dat.clone(), ang_dat, vec![wavelength])[0];
    let walkoff = interpolate(wl_dat, wo_dat, vec![wavelength])[0];
    
    (angle, walkoff)
}

fn interpolate(xs: Vec<f32>, ys: Vec<f32>, out_xs: Vec<f32>) -> Vec<f32> {
    let linear = Linear::builder().elements(ys).knots(xs).build().unwrap();
    linear.sample(out_xs).collect()
}