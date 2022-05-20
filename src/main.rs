//    Some modes to implement as flags:
//    Aproximate as defaul and not aproximate with a flag
//    A flag to print the atmosphere leyer of the altitude value entered
//    clarify that the tool aplication is for air only temperature range of usage [273 - 1800]K 
//    theory source: Yunus cengel for Thermodynamics, Intro to flight to speed of sounf, sutherland
//    for viscosity and ICAO ISA model for everything else
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
    let str_input = command.next().unwrap();
    let value: f64 = str_input.parse().expect("Some weird error!");
    let altitude = geo_alt(value);
    if altitude >= 0.0 && altitude <= 11000.0 {
        let temperature = temp(altitude);
        let pressure = press(altitude);
        let density = densy(altitude);
        println!("At {}m of altitude the air properties are the followings:\nTemperature = {} K\nPressure = {} Pa\nDensity = {} kg/m3", altitude, temperature, pressure, density);
        } else if altitude > 11000.0 && altitude <= 25000.0 {
            let temperature = STRATO_TEMP;
            let pressure = strato_press(altitude);
            let density = strato_densy(altitude);
            println!("At {}m of altitude the air properties are the followings:\nTemperature = {} K\nPressure = {} Pa\nDensity = {} kg/m3", altitude, temperature, pressure, density);
        };
    let testvisc = visco(5000.0);
    println!("{}", testvisc);
    let testcp = ideal_cp(700.0);
    println!("{}", testcp);
    let gamma = testcp / ideal_cv(700.0);
    println!("{}", gamma);
    let altc = sound_speed(7000.0);
    println!("{:.2}", altc);
}
