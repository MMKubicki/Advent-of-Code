use std::num::ParseIntError;
use std::ops::{Div, Sub};
use std::str::FromStr;

/// Alias of usize for mass of Module
pub type Mass = usize;

/// Alias of usize for fuel needed for Module
pub type Fuel = usize;

#[derive(Copy, Clone, Debug, Default)]
/// Module of spacecraft, only having a specific mass
pub struct Module {
    mass: Mass,
}

impl Module {
    pub fn new(mass: Mass) -> Self {
        Self { mass }
    }

    fn get_fuel_for_mass(mass: Mass) -> Fuel {
        // floor(mass / 3) - 2
        // .max(0) to make sure that always positive (negative fuel when? :) )
        (mass as f64).div(3f64).floor().sub(2f64).max(0f64) as Fuel
    }

    /// Get fuel needed for module. Simple calculation of floor(mass / 3) - 2 > 0
    pub fn get_simple_fuel_need(self) -> Fuel {
        // Just get simple fuel calculation
        Module::get_fuel_for_mass(self.mass)
    }

    /// Get true fuel needed for module. Respects the weight added by fuel
    pub fn get_true_fuel_need(self) -> Fuel {
        // Start with module mass. get fuel for that
        // set fuel to new remaining mass and accumulate fuel values

        let mut fuel = 0;
        let mut remaining_mass = self.mass;

        while remaining_mass > 0 {
            let needed_fuel = Module::get_fuel_for_mass(remaining_mass);
            fuel += needed_fuel;
            remaining_mass = needed_fuel;
        }

        fuel
    }
}

impl From<Mass> for Module {
    fn from(mass: Mass) -> Self {
        Self::new(mass)
    }
}

impl FromStr for Module {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Module::from(s.parse::<usize>()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_simple_calculated_fuel(mass: Mass, fuel: Fuel) {
        println!("Mass of {} should need {} of fuel", mass, fuel);
        assert_eq!(Module::from(mass).get_simple_fuel_need(), fuel)
    }

    fn test_true_calculated_fuel(mass: Mass, fuel: Fuel) {
        println!("Mass of {} should actually need {} of fuel", mass, fuel);
        assert_eq!(Module::from(mass).get_true_fuel_need(), fuel)
    }

    #[test]
    fn simple_fuel_example1() {
        test_simple_calculated_fuel(12, 2);
    }

    #[test]
    fn simple_fuel_example2() {
        test_simple_calculated_fuel(14, 2);
    }

    #[test]
    fn simple_fuel_example3() {
        test_simple_calculated_fuel(1969, 654);
    }

    #[test]
    fn simple_fuel_example4() {
        test_simple_calculated_fuel(100_756, 33_583);
    }

    #[test]
    fn true_fuel_example1() {
        test_true_calculated_fuel(14, 2);
    }

    #[test]
    fn true_fuel_example2() {
        test_true_calculated_fuel(1969, 966);
    }

    #[test]
    fn true_fuel_example3() {
        test_true_calculated_fuel(100_756, 50_346);
    }
}
