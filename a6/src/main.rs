use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let buf_reader = BufReader::new(File::open("input.txt").unwrap());
    let relationships: Vec<String> = buf_reader
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>();
    let mut objects: Vec<Object> = Vec::new();
    for line in relationships {
        add_connection(&mut objects, line);
    }
    println!("RESULT: {}", find_orbit_count(&objects, None, 0));
}

struct Object {
    name: String,
    parent: Option<String>,
}

fn object_exists(objects: &mut Vec<Object>, name: &String) -> bool {
    for object in objects {
        if &object.name == name {
            return true;
        }
    }
    false
}

fn try_add_parent(objects: &mut Vec<Object>, child: &String, parent: &String) -> bool {
    for object in objects {
        if &object.name == child {
            object.add_parent(parent.clone());
            return true;
        }
    }
    false
}

fn add_connection(objects: &mut Vec<Object>, line: String) {
    let lines = Object::split_orbit(line);
    match try_add_parent(objects, &lines.1, &lines.0) {
        true => (),
        false => objects.push(Object {
            name: lines.1,
            parent: Some(lines.0.clone()),
        }),
    }
    match object_exists(objects, &lines.0) {
        true => (),
        false => objects.push(Object {
            name: lines.0,
            parent: None,
        }),
    }
}

fn find_orbit_count(objects: &Vec<Object>, name: Option<String>, orbit_depth: i32) -> i32 {
    let mut orbit_count = 0;
    for object in objects {
        if &object.parent == &name {
            let jou =
                orbit_depth + find_orbit_count(objects, Some(object.name.clone()), orbit_depth + 1);
            orbit_count = orbit_count + jou
        }
    }
    orbit_count
}

impl Object {
    fn add_parent(&mut self, name: String) {
        self.parent = Some(name);
    }

    fn split_orbit(line: String) -> (String, String) {
        let lines = line.split(')').collect::<Vec<_>>();
        (String::from(lines[0]), String::from(lines[1]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_test() {
        let test_input = vec![
            String::from("COM)B"),
            String::from("B)C"),
            String::from("C)D"),
            String::from("D)E"),
            String::from("E)F"),
            String::from("B)G"),
            String::from("G)H"),
            String::from("D)I"),
            String::from("E)J"),
            String::from("J)K"),
            String::from("K)L"),
        ];
        let mut objects: Vec<Object> = Vec::new();
        for line in test_input {
            add_connection(&mut objects, line);
        }
        assert_eq!(42, find_orbit_count(&objects, None, 0));
    }
}
