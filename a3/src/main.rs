use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    println!("Hello, world!");
    read_data();
}

fn open_file() -> File {
    let file = match File::open("input.txt") {
        Err(_) => panic!("couldn't open the file"),
        Ok(file) => file,
    };
    file
}

struct Movement {
    x : i32,
    y : i32,
    length : i32,
    is_vertical : bool,
}

struct WiringCommand {
    command : String
}

impl WiringCommand {
    fn create_movement(&self, current_x : i32, current_y: i32) -> (Movement, i32, i32){
        let length =  i32::from_str_radix(&self.command.chars().skip(1).filter(|c| c.is_digit(10)).collect::<String>(), 10).unwrap();
        match self.command.chars().next().unwrap() {
            'U' => (Movement{x : current_x, y : current_y + 1 , length : length, is_vertical : true}, current_x, current_y + length),
            'R' => (Movement{x : current_x + 1, y : current_y, length : length, is_vertical : false}, current_x + length, current_y),
            'D' => (Movement{x : current_x, y : current_y - length, length : length, is_vertical : true}, current_x, current_y - length),
            'L' => (Movement{x : current_x - length, y : current_y, length : length, is_vertical : false}, current_x - length, current_y),
            _ => ::std::process::exit(1) //Impossible
        }
    }
}

struct Intersection(i32, i32);

impl Intersection {
    fn get_distance(&self) -> i32{
        i32::abs(self.0) + i32::abs(self.1)
    }
}

impl Movement {
    fn get_closest_intersection(&self, other : &Movement) -> Option<Intersection>{
        match self.is_vertical == other.is_vertical {
            true if self.x == other.x && self.is_vertical => {
                match Movement::compare_parallel(self.y, other.y, self.length, other.length) {
                    Some(y) => Some(Intersection(self.x, y)),
                    None => None,
                }
            },
            true if self.y == other.y && !self.is_vertical => {
                match Movement::compare_parallel(self.x, other.x, self.length, other.length) {
                    Some(x) => Some(Intersection(x, self.y)),
                    None => None,
                }
            }
            true => None,
            false if self.y <= other.y && other.x <= self.x && self.is_vertical => Movement::compare_orthogonal(other.x, other.y, self.x, self.y, other.length, self.length),
            false if self.x <= other.x && other.y <= self.y && !self.is_vertical => Movement::compare_orthogonal(self.x, self.y, other.x, other.y, self.length, other.length),
            _ => None,
        }
    }



    fn compare_orthogonal(right_x: i32, right_y: i32, up_x: i32, up_y: i32, right_length: i32, up_length: i32) -> Option<Intersection>{
        match (right_x + right_length >= up_x) && (up_y + up_length >= right_y) {
            true => Some(Intersection(up_x, right_y)),
            false => None,
        }
    }

    fn compare_parallel(self_coord : i32, other_coord : i32, self_length : i32, other_length : i32) -> Option<i32>{
        match self_coord.cmp(&other_coord){
            Ordering::Equal if self_coord >= 0 => Some(self_coord),
            Ordering::Equal if self_length <= other_length && self_length + self_coord >= 0 => Some(0),
            Ordering::Equal if self_length <= other_length => Some(self_length + self_coord),
            Ordering::Equal if other_length + other_coord >= 0 => Some(0),
            Ordering::Equal => Some(other_coord + other_length),
            Ordering::Less if self_coord + self_length >= other_coord => Movement::compare_parallel(other_coord, other_coord, other_coord-self_coord+self_length, other_length),
            Ordering::Greater if other_coord + other_length >= self_coord => Movement::compare_parallel(self_coord, self_coord, self_coord-other_coord+other_length, self_length),
            _ => None,
        }
    }
}

fn read_data() {
    let mut buf_reader = BufReader::new(open_file());
    let mut first = String::new();
    buf_reader.read_line(&mut first);
    let mut second = String::new();
    buf_reader.read_line(&mut second);
    let first_movements = get_movements(first);
    let second_movements = get_movements(second);
    let intersections = find_intersections(first_movements, second_movements);
    println!("{}", get_closest_intersection_distance(intersections));
}

fn get_movements(data : String) -> Vec<Movement>{
    let mut current_x = 0;
    let mut current_y = 0;
    let wiring_commands = data.as_str().split(",").map(|c| WiringCommand{command : String::from(c)}).collect::<Vec<WiringCommand>>();
    let mut movements : Vec<Movement> = Vec::new();
    for wiring_command in wiring_commands {
        let (movement, new_x, new_y) = wiring_command.create_movement(current_x, current_y);
        current_x = new_x;
        current_y = new_y;
        movements.push(movement);
    };
    movements
}

fn find_intersections(first_movements : Vec<Movement>, second_movements : Vec<Movement>) -> Vec<Intersection>{
    let mut intersections : Vec<Intersection> = Vec::new();
    for first_movement in first_movements {
        for second_movement in &second_movements {
            let intersection = first_movement.get_closest_intersection(second_movement);
            if intersection.is_some() {
                intersections.push(intersection.unwrap());
            }
        }
    }
    intersections
}

fn get_closest_intersection_distance(intersections : Vec<Intersection>) -> i32{
    let mut closest_intersection = std::i32::MAX;
    for intersection in intersections {
        if intersection.get_distance() < closest_intersection {
            closest_intersection = intersection.get_distance();
        }
    }
    closest_intersection
}