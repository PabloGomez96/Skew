# Skew 
Skew is a Rust script whose purpose is to provide its users with a simple, precise and highly effective tool for atmospheric properties calculation on the server side of the operation.

## :memo: Usage

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
[Rust Physical constants crate](https://crates.io/crates/physical_constants)

## :books: Bibliography
- Section 4-4 Internal energy, enthalpy and specific heats of ideal gases, Thermodynamics 8th Edition, Yunus A. Çengel & Michael A. Boles
- Section 4.9 Speed of Sound, Introduction to Flight 7th Edition, John D. Anderson Jr.
- ICAO Doc 7488/3, Manual of The The ICAO Standard Atmosphere 3th Edition 
