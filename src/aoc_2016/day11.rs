use std::collections::VecDeque;

use nohash_hasher::IntSet;
use itertools::Itertools;

// SAMPLE
// The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
// The second floor contains a hydrogen generator.
// The third floor contains a lithium generator.
// The fourth floor contains nothing relevant.

// INPUT
// The first floor contains a polonium generator, a thulium generator, a thulium-compatible microchip, a promethium generator, a ruthenium generator, a ruthenium-compatible microchip, a cobalt generator, and a cobalt-compatible microchip.
// The second floor contains a polonium-compatible microchip and a promethium-compatible microchip.
// The third floor contains nothing relevant.
// The fourth floor contains nothing relevant.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Element {
    // Part 1
    Hydrogen,
    Lithium,

    // Part 2
    Thulium,
    Ruthenium,
    Cobalt,
    Polonium,
    Promethium,
}

impl Element {
    fn base_value(&self) -> u64 {
        // For the 8 bits that make up the specific machines on each floor,
        // each element is a bit in a specific place
        match self {
            Hydrogen   => 0x01, // 1 << 0
            Lithium    => 0x02, // 1 << 1
            Thulium    => 0x04, // 1 << 2
            Ruthenium  => 0x08, // 1 << 3
            Cobalt     => 0x10, // 1 << 4
            Polonium   => 0x20, // 1 << 5
            Promethium => 0x40, // 1 << 6
        }
    }

    fn has_element(&self, value: u64) -> bool {
        match self {
            Hydrogen => value & 0x01 != 0,
            Lithium => value & 0x02 != 0,
            Thulium => value & 0x04 != 0,
            Ruthenium => value & 0x08 != 0,
            Cobalt => value & 0x10 != 0,
            Polonium => value & 0x20 != 0,
            Promethium => value & 0x40 != 0,
        }
    }
}

use Element::*;

#[derive(Clone, PartialEq, Eq, Debug)]
enum Machine {
    Microchip(Element),
    Generator(Element),
}

impl Machine {
    fn base_value(&self) -> u64 {
        // For the 16 bits that make up a single floor in the building bitset, the RTC Generators
        // use up the most significant byte and the chips take up the least significant byte.
        match self {
            Generator(e) => e.base_value() << 8,
            Microchip(e) => e.base_value(),
        }
    }

    fn is_rtg(&self) -> bool {
        match self {
            Microchip(_) => false,
            Generator(_) => true,
        }
    }

    fn to_element(&self) -> Element {
        match self {
            Microchip(el) => *el,
            Generator(el) => *el,
        }
    }
}

use Machine::*;

#[derive(PartialEq, Eq, Clone, Debug)]
struct Building {
    elevator: u8,
    floor0: Vec<Machine>,
    floor1: Vec<Machine>,
    floor2: Vec<Machine>,
    floor3: Vec<Machine>,
}

// A valid floor is one where either there are no generators, there are no microchips, or every microchip has a matching generator.
fn is_floor_valid(floor: &Vec<Machine>) -> bool {
    let rtg_elems: Vec<Element> = floor.iter().filter(|m| m.is_rtg()).map(|m| m.to_element()).collect();
    if rtg_elems.is_empty() {
        return true;
    }
    floor.iter().all(|machine| {
        match machine {
            Generator(_) => true,
            Microchip(el) => rtg_elems.contains(el),
        }
    })
}

impl Building {
    // Parsing these does not sound like fun
    fn create_sample() -> Self {
        Self {
            elevator: 0,
            floor0: vec![Microchip(Hydrogen), Microchip(Lithium)],
            floor1: vec![Generator(Hydrogen)],
            floor2: vec![Generator(Lithium)],
            floor3: Vec::new(),
        }
    }

    fn create_input() -> Self {
        Self {
            elevator: 0,
            floor0: vec![
                Generator(Polonium),
                Generator(Thulium),
                Generator(Promethium),
                Generator(Ruthenium),
                Generator(Cobalt),
                Microchip(Thulium),
                Microchip(Ruthenium),
                Microchip(Cobalt),
            ],
            floor1: vec![
                Microchip(Polonium),
                Microchip(Promethium),
            ],
            floor2: Vec::new(),
            floor3: Vec::new(),
        }
    }

    fn _create_full_input() -> Self {
        let mut input = Self::create_input();
        // The problem for part 2 states that there are 2 new elements, elerium and dilithium. However, since
        // hydrogen and lithium are not used in the real input, I can sub those instead. It won't change
        // the output what the elements are called.
        input.floor0.push(Microchip(Hydrogen));
        input.floor0.push(Microchip(Lithium));
        input.floor0.push(Generator(Hydrogen));
        input.floor0.push(Generator(Lithium));
        input
    }

    // (has_elevator, floor machines)
    fn hash_to_floor(floor_value: u64) -> (bool, Vec<Machine>) {
        let generators = floor_value >> 8;
        let microchips = floor_value & 0xFF;
        let has_elevator = floor_value & 0x8000 != 0;
        let mut machines: Vec<Machine> = Vec::new();
        // could be more elegant, but whatever
        if generators > 0 {
            if Hydrogen.has_element(generators) {
                machines.push(Generator(Hydrogen));
            }
            if Lithium.has_element(generators) {
                machines.push(Generator(Lithium));
            }
            if Thulium.has_element(generators) {
                machines.push(Generator(Thulium));
            }
            if Ruthenium.has_element(generators) {
                machines.push(Generator(Ruthenium));
            }
            if Cobalt.has_element(generators) {
                machines.push(Generator(Cobalt));
            }
            if Polonium.has_element(generators) {
                machines.push(Generator(Polonium));
            }
            if Promethium.has_element(generators) {
                machines.push(Generator(Promethium));
            }
        }
        if microchips > 0 {
            if Hydrogen.has_element(microchips) {
                machines.push(Microchip(Hydrogen));
            }
            if Lithium.has_element(microchips) {
                machines.push(Microchip(Lithium));
            }
            if Thulium.has_element(microchips) {
                machines.push(Microchip(Thulium));
            }
            if Ruthenium.has_element(microchips) {
                machines.push(Microchip(Ruthenium));
            }
            if Cobalt.has_element(microchips) {
                machines.push(Microchip(Cobalt));
            }
            if Polonium.has_element(microchips) {
                machines.push(Microchip(Polonium));
            }
            if Promethium.has_element(microchips) {
                machines.push(Microchip(Promethium));
            }
        }
        (has_elevator, machines)
    }

    fn from_hash(hash: u64) -> Self {
        let (floor0_elevator, floor0) = Self::hash_to_floor(0xFFFF & hash);
        let (floor1_elevator, floor1) = Self::hash_to_floor(0xFFFF & (hash >> 16));
        let (floor2_elevator, floor2) = Self::hash_to_floor(0xFFFF & (hash >> 32));
        let (_, floor3) = Self::hash_to_floor(0xFFFF & (hash >> 48));
        let elevator = if floor0_elevator {
            0
        } else if floor1_elevator {
            1
        } else if floor2_elevator {
            2
        } else {
            3
        };
        Self {
            elevator,
            floor0,
            floor1,
            floor2,
            floor3,
        }
    }

    fn get_floor(&self, at: u8) -> &Vec<Machine> {
        match at {
            0 => &self.floor0,
            1 => &self.floor1,
            2 => &self.floor2,
            3 => &self.floor3,
            _ => unreachable!(),
        }
    }

    fn set_floor(&mut self, at: u8, floor: Vec<Machine>) {
        match at {
            0 => self.floor0 = floor,
            1 => self.floor1 = floor,
            2 => self.floor2 = floor,
            3 => self.floor3 = floor,
            _ => unreachable!(),
        }
    }

    // I don't want to have to clone my building to insert it into a set, and this lets me use the no-hash hasher
    // since every unique building creates a distinct u64 hash value
    fn building_hash(&self) -> u64 {
        let elevator_bit: u64 = 0x8000 << (self.elevator * 16);

        elevator_bit
        | self.floor0.iter().fold(0, |acc, machine| acc | machine.base_value()) << 0
        | self.floor1.iter().fold(0, |acc, machine| acc | machine.base_value()) << 16
        | self.floor2.iter().fold(0, |acc, machine| acc | machine.base_value()) << 32
        | self.floor3.iter().fold(0, |acc, machine| acc | machine.base_value()) << 48
    }

    fn is_done(&self) -> bool {
        self.floor0.is_empty() && self.floor1.is_empty() && self.floor2.is_empty()
    }

    fn next_possible_states(&self) -> Vec<Self> {
        let mut next_states = Vec::new();

        // Prioritize going up by adding the upward moves to next_states first.
        if self.elevator < 3 {
            self.states_between_floors(&mut next_states, self.elevator, self.elevator + 1);
        }
        if self.elevator > 0 {
            self.states_between_floors(&mut next_states, self.elevator, self.elevator - 1);
        }

        next_states
    }

    fn states_between_floors(&self, list: &mut Vec<Building>, from: u8, to: u8) {
        let to_floor = self.get_floor(to);
        let mut elevator_powerset = self.get_floor(from).iter().powerset();
        elevator_powerset.next(); // Get rid of the initial empty set since the elevator must always have at least 1 item
        let mut single_item_move = Vec::new();
        let mut double_item_move = Vec::new();
        while let Some(in_elevator) = elevator_powerset.next() {
            if in_elevator.len() == 1 {
                single_item_move.push(in_elevator);
            } else if in_elevator.len() == 2 {
                double_item_move.push(in_elevator);
            } else if in_elevator.len() > 2 {
                break;
            }
        }
        // Never take 2 items down together
        if to > from {
            for elevator in double_item_move.iter() {
                let new_to_floor: Vec<Machine> = to_floor.iter().chain(elevator.to_owned()).cloned().collect();
                let new_from_floor: Vec<Machine> = self.get_floor(from).iter().filter(|m| !elevator.contains(m)).cloned().collect();
                if is_floor_valid(&new_to_floor) && is_floor_valid(&new_from_floor) {
                    let mut new_building = self.clone();
                    new_building.elevator = to;
                    new_building.set_floor(from, new_from_floor);
                    new_building.set_floor(to, new_to_floor);
                    list.push(new_building);
                }
            }
        }
        for elevator in single_item_move {
            let new_to_floor: Vec<Machine> = to_floor.iter().chain(elevator.to_owned()).cloned().collect();
            let new_from_floor: Vec<Machine> = self.get_floor(from).iter().filter(|m| !elevator.contains(m)).cloned().collect();
            if is_floor_valid(&new_to_floor) && is_floor_valid(&new_from_floor) {
                let mut new_building = self.clone();
                new_building.elevator = to;
                new_building.set_floor(from, new_from_floor);
                new_building.set_floor(to, new_to_floor);
                list.push(new_building);
            }
        }
    }
}

fn find_shortest_arrangement(start: Building) -> u64 {
    let mut known_states: IntSet<u64> = IntSet::default();
    let mut next_states: VecDeque<(u64, u64)> = VecDeque::new();

    for b in start.next_possible_states() {
        let building_value = b.building_hash();
        known_states.insert(building_value);
        next_states.push_back((1, building_value));
    }

    while let Some((iter, cur_hash)) = next_states.pop_front() {
        let cur_building = Building::from_hash(cur_hash);
        for next_building in cur_building.next_possible_states() {
            if next_building.is_done() {
                return iter + 1;
            }
            let next_hash = next_building.building_hash();
            if !known_states.contains(&next_hash) {
                known_states.insert(next_hash);
                next_states.push_back((iter + 1, next_hash));
            }
        }
    };

    panic!("Could not find solution");
}

#[test]
fn part_1() {
    assert_eq!(11, find_shortest_arrangement(Building::create_sample()));
    assert_eq!(47, find_shortest_arrangement(Building::create_input()));
    // Takes >1 minute
    // assert_eq!(71, find_shortest_arrangement(Building::_create_full_input()));
}
