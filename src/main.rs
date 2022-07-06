use physical_constants;

//List of pyshical constants
const BASE_TEMP: f64 = 288.15;         // Kelvin, at MSL
const BASE_PRESS: f64 = 101325.0;      // pascals, at MSL
const BASE_DENS: f64 = 1.225;          // kilograms per cubic meter, at MSL
const TROPO_TERM_GRAD: f64 = -6.5e-3;  // Kelvin/meter, for troposphere only
const GRAV: f64 = -1.0 * physical_constants::STANDARD_ACCELERATION_OF_GRAVITY; // -9.80665 m/s^2, assuming the negative direction of the force pointing to the earth center 
const AIR_IDEAL_CONS: f64 = 287.05287; // (Joule/kilogram)/Kelvin, for ideal gas Air 
const STRATO_TEMP: f64 = 216.66;       // Kelvin, isothermal temperature for stratosphere
const STRATO_PRESS:f64 = 22552.0;      // pascals, pressure at transition point between tropo and stratosphere =~ 11,000m
const STRATO_DENS:f64 = 0.3629;        // kilogram/meter, density at transition point between tropo and stratosphere =~ 11,000m
const STRATO_ALTITUDE:f64 = 11000.0;   // meters
const EARTH_RADIUS:f64 = 6356766.0;    // meters
const BASE_VISCOSITY:f64 = 1.7894e-5;  // kilogram/(meter*second), at MSL
const SUTHERLAND_TEMP:f64 = 110.0;     // Kelvin, Sutherland constant
const R_U:f64 = physical_constants::MOLAR_GAS_CONSTANT; // Joule/(mol*Kelvin) Ideal molar gas constant

// Geometric to geopotential calculation method
fn geo_alt(alt: f64) -> f64 {
    let result = alt * (EARTH_RADIUS / (EARTH_RADIUS + alt));
    result
}

// Troposphere temperature calculation method
fn temp(alt: f64) -> f64 {
    let result = BASE_TEMP + (TROPO_TERM_GRAD * alt);
    result
}

// Troposphere pressure calculation method
fn press(alt: f64) -> f64 {
    let result = BASE_PRESS * f64::powf((BASE_TEMP + (TROPO_TERM_GRAD * alt)) / BASE_TEMP, GRAV / (AIR_IDEAL_CONS * TROPO_TERM_GRAD));
    result
}

// Troposphere density calculation method
fn densy(alt: f64) -> f64 {
    let result = BASE_DENS * f64::powf((BASE_TEMP + (TROPO_TERM_GRAD * alt)) / BASE_TEMP, (GRAV / (AIR_IDEAL_CONS * TROPO_TERM_GRAD)) - 1.0);
    result
}

// Low stratosphere pressure calculation method
fn strato_press(alt: f64) -> f64 {
    let result = STRATO_PRESS * f64::exp((GRAV / (AIR_IDEAL_CONS * STRATO_TEMP)) * (alt - STRATO_ALTITUDE));
    result
}

// Low stratosphere density calculation method
fn strato_densy(alt: f64) -> f64 {
    let result = STRATO_DENS * f64::exp((GRAV / (AIR_IDEAL_CONS * STRATO_TEMP)) * (alt - STRATO_ALTITUDE));
    result
}

// Temperature based viscosity calculation method
fn visco(alt: f64) -> f64 {
    let alt_temp = temp(alt);
    let result = BASE_VISCOSITY * ((BASE_TEMP + SUTHERLAND_TEMP) / (alt_temp + SUTHERLAND_TEMP)) * f64::powf(alt_temp / BASE_TEMP, 1.5);
    result
}

// Constant pressure specific heat for Air calculation method, in polynomial form
fn ideal_cp(temp: f64) -> f64 {
    let cp = 28.11 + (0.1967e-2 * temp) + (0.4802e-5 * f64::powf(temp, 2.0)) + (-1.966e-9 * f64::powf(temp, 3.0));
    cp
}

// Constant volume specific heat for Air calculation method 
fn ideal_cv(temp: f64) -> f64 {
    let cp = ideal_cp(temp);
    let cv = cp - R_U;
    cv
}

// Temperature based local speed of sound
fn sound_speed(alt: f64) -> f64 {
    let alt_temp = temp(alt);
    let gamma = ideal_cp(alt_temp) / ideal_cv(alt_temp); // Specific heats ratio
    let c = f64::sqrt(gamma * (press(alt) / densy(alt)));
    c
}

fn main() {
    let mut command = std::env::args().skip(1);
    let alt_input = command.next().unwrap();
    let value: f64 = alt_input.parse().expect("The expected value is a number, not a string!");
    if value < 0.0 {
        panic!("You are under the ground!!!");
    } else if value > 20000.0 {
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
    println!("At {}m of altitude the air properties are the followings:\nTemperature = {:.2} K\nPressure = {:.2} Pa\nDensity = {:.3} kg/m3\nViscosity = {:.5}e-5 Pa*s\nLocal speed of sound = {:.2} m/s", value, properties[0], properties[1], properties[2], properties[3], properties[4]);
    }
