fn parse_input(input: &str) -> Vec<i64> {
    input
        .trim()
        .split("\n")
        .map(|x| x.parse().unwrap())
        .collect()
}

const INPUT: &str = include_str!("./inputs/day1.txt");

fn fuel_required(masses: &Vec<i64>) -> i64 {
    masses.iter().map(|mass| mass / 3 - 2).sum()
}

fn real_fuel_required(masses: &Vec<i64>) -> i64 {
    masses.iter().map(|&mass| fuel_for_mass(mass)).sum()
}

fn fuel_for_mass(mass: i64) -> i64 {
    if mass < 6 {
        0
    } else {
        let new_mass = mass / 3 - 2;
        new_mass + fuel_for_mass(new_mass)
    }
}

#[test]
fn solutions() {
    let module_masses = parse_input(INPUT);
    assert_eq!(3390830, fuel_required(&module_masses));
    assert_eq!(5083370, real_fuel_required(&module_masses));
}
