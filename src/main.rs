//    let mut command = std::env::args().skip(1);
//    let altitudeS = command.next().unwrap();
//    println!("The altitude entered is {}", altitudeS);
//    let altitudeF: f32 = altitudeS.parse().expect("Some weird error");
//    if altitudeF > 0.0 && altitudeF < 11000.0 {
//        println!{"The altitude is located in the troposphere"};
//    } else {
//        println!{"The altitude is located beyond the troposphere"}
//    }
//    Some modes to implement as flags:
//    Not aproximate as defaul and aproximate with a flag
//    Geometric to geopotential altitude conversion need to be added to increase the accuracy
//    clarify that the tool aplication is for air only
//    evalute run cp and sound_speed methods with temperature as arguments directly instead of calculating those values in the method
//    for realising the geopetential correction will be done after catching the altitude entered by
//    the user and this corrected altitude will be passed to other functions as argument for further
//    calculations, that means in any method will be correction altitude related
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

fn geoalt(alt: f64) -> f64 {
    let result = alt * (EARTH_RADIUS / (EARTH_RADIUS + alt));
    result
}

fn temp(alt: f64) -> f64 {
    let altG = geoalt(alt);
    let result = BASE_TEMP + (TROPO_TERM_GRAD * altG);
    result
}

fn press(alt: f64) -> f64 {
    let altG = geoalt(alt);
    let result = BASE_PRESS * f64::powf((BASE_TEMP + (TROPO_TERM_GRAD * altG)) / BASE_TEMP, GRAV / (AIR_IDEAL_CONS * TROPO_TERM_GRAD));
    result
}

fn densy(alt: f64) -> f64 {
    let altG = geoalt(alt);
    let result = BASE_DENS * f64::powf((BASE_TEMP + (TROPO_TERM_GRAD * altG)) / BASE_TEMP, (GRAV / (AIR_IDEAL_CONS * TROPO_TERM_GRAD)) - 1.0);
    result
}

fn strato_press(alt: f64) -> f64 {
    let altG = geoalt(alt);
    let result = STRATO_PRESS * f64::exp((GRAV / (AIR_IDEAL_CONS * STRATO_TEMP)) * (altG - STRATO_ALTITUDE));
    result
}

fn strato_densy(alt: f64) -> f64 {
    let altG = geoalt(alt);
    let result = STRATO_DENS * f64::exp((GRAV / (AIR_IDEAL_CONS * STRATO_TEMP)) * (altG - STRATO_ALTITUDE));
    result
}

fn visco(alt: f64) -> f64 {
    let altG = geoalt(alt);
    let alt_temp = temp(altG);
    let result = BASE_VISCOSITY * ((BASE_TEMP + SUTHERLAND_TEMP) / (alt_temp + SUTHERLAND_TEMP)) * f64::powf(alt_temp / BASE_TEMP, 1.5);
    result
}

//fn ideal_cp() -> f64 {}

//fn ideal_cv() -> f64 {}

//fn sound_speed() -> f64 {}

fn main() {
    let altitude = 7900.0;
    let altitudeE = 17900.0;
    let temperature = temp(altitude);
    let pressure = press(altitude);
    let density = densy(altitude);
    println!("At {}m of altitude the air properties are the followings:\nTemperature = {} K\nPressure = {} Pa\nDensity = {} kg/m3", altitude, temperature, pressure, density);
    let testpress = strato_press(altitudeE);
    println!("{}", testpress);
    let testdensy = strato_densy(altitudeE);
    println!("{}", testdensy);
    let testgeoalt = geoalt(altitudeE);
    println!("{}", testgeoalt);
    let testvisc = visco(5000.0);
    println!("{}", testvisc);
}
