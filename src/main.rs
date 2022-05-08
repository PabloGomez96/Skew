//    let mut command = std::env::args().skip(1);
//    let altitudeS = command.next().unwrap();
//    println!("The altitude entered is {}", altitudeS);
//    let altitudeF: f32 = altitudeS.parse().expect("Some weird error");
//    if altitudeF > 0.0 && altitudeF < 11000.0 {
//        println!{"The altitude is located in the troposphere"};
//    } else {
//        println!{"The altitude is located beyond the troposphere"}
//    }
const BASE_TEMP: f64 = 288.15;
const BASE_PRESS: f64 = 101325.0;
const BASE_DENS: f64 = 1.225;
const TROPO_TERM_GRAD: f64 = -0.0065;
const GRAV: f64 = -9.81;
const AIR_IDEAL_CONS: f64 = 287.0;

fn temp(alt: f64) -> f64 {
    let result = BASE_TEMP + (TROPO_TERM_GRAD * alt);
    result
}

fn press(alt: f64) -> f64 {
    let result = f64::powf((BASE_TEMP + (TROPO_TERM_GRAD * alt)) / (BASE_TEMP), (GRAV) / (AIR_IDEAL_CONS * TROPO_TERM_GRAD));
    result
}

fn densy(alt: f64) -> f64 {
    let result = f64::powf((BASE_TEMP + (TROPO_TERM_GRAD * alt)) / (BASE_TEMP), ((GRAV) / (AIR_IDEAL_CONS * TROPO_TERM_GRAD)) - 1.0);
    result
}

fn main() {
 let pressure = temp(1524.0);
    println!("{}", pressure);
}
