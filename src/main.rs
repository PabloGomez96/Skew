//    Some modes to implement as flags:
//    Aproximate as defaul and not aproximate with a flag
//    A flag to print the atmosphere leyer of the altitude value entered
//    A flag for imperial unit system calculations end resuts
//    clarify that the tool aplication is for air only temperature range of usage [273 - 1800]K 
//    theory source: Yunus cengel for Thermodynamics, Intro to flight to speed of sounf, sutherland
//    for viscosity and ICAO ISA model for everything else
//    Implement a new method to convert the properties array from SI to Imperial
use physical_constants;

const BASE_TEMP: f64 = 288.15;
const BASE_PRESS: f64 = 101325.0;
const BASE_DENS: f64 = 1.225;
const TROPO_TERM_GRAD: f64 = -6.5e-3;
const GRAV: f64 = -1.0 * physical_constants::STANDARD_ACCELERATION_OF_GRAVITY;
const AIR_IDEAL_CONS: f64 = 287.0;
const STRATO_TEMP: f64 = 216.66;
const STRATO_PRESS:f64 = 22552.0;
const STRATO_DENS:f64 = 0.3629;
const STRATO_ALTITUDE:f64 = 11000.0;
const EARTH_RADIUS:f64 = 6356766.0;
const BASE_VISCOSITY:f64 = 1.7894e-5;
const SUTHERLAND_TEMP:f64 = 110.0;
const R_U:f64 = physical_constants::MOLAR_GAS_CONSTANT;

fn geo_alt(alt: f64) -> f64 {
    let result = alt * (EARTH_RADIUS / (EARTH_RADIUS + alt));
    result
}

fn temp(alt: f64) -> f64 {
    let result = BASE_TEMP + (TROPO_TERM_GRAD * alt);
    result
}

fn press(alt: f64) -> f64 {
    let result = BASE_PRESS * f64::powf((BASE_TEMP + (TROPO_TERM_GRAD * alt)) / BASE_TEMP, GRAV / (AIR_IDEAL_CONS * TROPO_TERM_GRAD));
    result
}

fn densy(alt: f64) -> f64 {
    let result = BASE_DENS * f64::powf((BASE_TEMP + (TROPO_TERM_GRAD * alt)) / BASE_TEMP, (GRAV / (AIR_IDEAL_CONS * TROPO_TERM_GRAD)) - 1.0);
    result
}

fn strato_press(alt: f64) -> f64 {
    let result = STRATO_PRESS * f64::exp((GRAV / (AIR_IDEAL_CONS * STRATO_TEMP)) * (alt - STRATO_ALTITUDE));
    result
}

fn strato_densy(alt: f64) -> f64 {
    let result = STRATO_DENS * f64::exp((GRAV / (AIR_IDEAL_CONS * STRATO_TEMP)) * (alt - STRATO_ALTITUDE));
    result
}

fn visco(alt: f64) -> f64 {
    let alt_temp = temp(alt);
    let result = BASE_VISCOSITY * ((BASE_TEMP + SUTHERLAND_TEMP) / (alt_temp + SUTHERLAND_TEMP)) * f64::powf(alt_temp / BASE_TEMP, 1.5);
    result
}

fn ideal_cp(temp: f64) -> f64 {
    let cp = 28.11 + (0.1967e-2 * temp) + (0.4802e-5 * f64::powf(temp, 2.0)) + (-1.966e-9 * f64::powf(temp, 3.0));
    cp
}

fn ideal_cv(temp: f64) -> f64 {
    let cp = ideal_cp(temp);
    let cv = cp - R_U;
    cv
}

fn sound_speed(alt: f64) -> f64 {
    let alt_temp = temp(alt);
    let gamma = ideal_cp(alt_temp) / ideal_cv(alt_temp);
    let c = f64::sqrt(gamma * (press(alt) / densy(alt)));
    c
}

fn main() {
    let mut command = std::env::args().skip(1);
    let mut flag = std::env::args().skip(2);
    let alt_input = command.next().unwrap();
    let flag_input = flag.next().unwrap();
    println!("{}", flag_input);
    let value: f64 = alt_input.parse().expect("The expected value is a number, not a string!");
    if value < 0.0 {
        panic!("You are under the ground!!!");
    } else if value > 25000.0 {
        panic!("You are too high for air properties calculation!");
    } else {
        true;
    };
    let altitude = geo_alt(value);
    let properties: [f64; 5] =
        if altitude >= 0.0 && altitude <= 11000.0 {
            [temp(altitude),
            press(altitude),
            densy(altitude),
            visco(altitude) * 100000.0,
            sound_speed(altitude)]
        } else {
            [STRATO_TEMP,
            strato_press(altitude),
            strato_densy(altitude),
            visco(altitude) * 100000.0,
            sound_speed(altitude)]
        };
    let message = "At {}m of altitude the air properties are the followings:\nTemperature = {:.2} K\nPressure = {:.2} Pa\nDensity = {:.3} kg/m3\nViscosity = {:.5}e-5 Pa*s\nLocal speed of sound = {:.2} m/s";
    println!(message, value, properties[0], properties[1], properties[2], properties[3], properties[4]);
    }
