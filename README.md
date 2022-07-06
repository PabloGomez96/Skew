# Skew 
Skew is a Rust script whose purpose is to provide its users with a simple, precise and highly effective tool for atmospheric properties calculation on the server side of the operation.

## :memo: Usage
1. On your terminal, for a single user usage, run the following command
```sh
Skew [ALTITUDE]
```
Knowing that altitude value entered should be in meters, measured from the mean ground level and could have up to 15 positions after the decimal point
Or for an agnostic user usage run the following command on your terminal
```sh
sudo Skew [ALTITUDE]
```

## :computer: Installation
1. Move to your home directory
```sh
cd
```
2. Clone the repo
```sh
git clone git@github.com:PabloGomez96/Skew.git
```
3. Move to Skew directory
```sh
cd Skew
```
4. Build the source code (A Warninig about non snake_case package name will appear but is ok since the author loves CamelCase more than loves Rust :joy:)
```sh
cargo build --release
```
5. Copy the brand new compiled binary to a directory contained by your $PATH, for a single user usage of the program you would run the following command
```sh
cp ./target/release/Skew ~/.local/bin
```
Or run the following command for an agnostic user usage for the program
```sh
cp ./target/release/Skew /usr/local/bin/
```

## :gear: Features
- Geometric to geopotential altittude conversion will be performed in every calculation automatically
- Compute values for Troposphere and Low Stratosphere
- Dynamic viscosity calcution based on Sutherland's Law
- Local Speed of Sound calculation based on gases idealization and altitude particular values
- Thermodynamic properties of gases executed in their polynomial exact form based on absolute temperature

## :warning: Limitations
- The results will be presented in aproximated format only with as many significant figures as the author think are correct according to their experience in the field.
- The calculation methods are valid for Air as ideal gas with a temperature range of [273.15 - 1800]K
- The calculation methods are valid for Earth atmosphere at meand conditions between [0 - 20,000]m

## :world_map: Roadmap
Some options to implement as flags indications:
- [ ] Not aproximated results
- [ ] Print on screen the atmosphere layer of flight
- [ ] Present the results on Imperial Unit System

## :building_construction: Dependencies
1. [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
2. [Cargo - Rust package manager](https://www.rust-lang.org/tools/install)
3. [Rust Physical constants crate](https://crates.io/crates/physical_constants)

## :books: Bibliography
- Section 4-4 Internal energy, enthalpy and specific heats of ideal gases, Thermodynamics 8th Edition, Yunus A. Çengel & Michael A. Boles
- Section 4.9 Speed of Sound, Introduction to Flight 7th Edition, John D. Anderson Jr.
- ICAO Doc 7488/3, Manual of The The ICAO Standard Atmosphere 3th Edition 
