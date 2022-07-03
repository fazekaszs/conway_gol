use std::io::{stdin, stdout, Write};
use std::{time, thread, fs};
use std::collections::{HashSet, HashMap};

const CONF_OSCILLATOR: [[i32; 2]; 3] = [
    [0, 0], [1, 0], [-1, 0]
];

const CONF_GLIDER: [[i32; 2]; 5] = [
    [0, 1], [1, 0], [1, -1], [0, -1], [-1, -1]
];

const CONF_GLIDERGUN: [[i32; 2]; 36] = [
    [1, 4], [1, 5], [2, 4], [2, 5], 
    [11, 3], [11, 4], [11, 5],
    [12, 2], [12, 6],
    [13, 1], [13, 7],
    [14, 1], [14, 7],
    [15, 4],
    [16, 2], [16, 6],
    [17, 3], [17, 4], [17, 5],
    [18, 4],
    [21, 5], [21, 6], [21, 7],
    [22, 5], [22, 6], [22, 7],
    [23, 4], [23, 8],
    [25, 3], [25, 4], [25, 8], [25, 9],
    [35, 6], [35, 7],
    [36, 6], [36, 7]
];

fn fill_hset(conf: &[[i32; 2]]) -> HashSet<Vec<i32>> {

    let mut output = HashSet::new();
    for &element in conf.iter(){
        output.insert(Vec::from(element));
    }
    output
}

fn read_user_input(input_path: &str) -> HashSet<Vec<i32>> {

    let mut output: HashSet<Vec<i32>> = HashSet::new();
    let (mut x, mut y) = (0, 0);

    let file_content = fs::read_to_string(input_path).unwrap();

    for character in file_content.as_bytes().iter() {
        match character {
            b' ' => x += 1,
            b'\n' => { x = 0; y += 1; },
            b'#' => {
                let mut current_vec = Vec::new();
                current_vec.push(x);
                current_vec.push(y);
                output.insert(current_vec);
                x += 1
            },
            _ => panic!("Unknown character {} in the input file! Only use \' \', \'\\n\', and \'#\'!",
                        *character as char)
        }
    }

    output
}

fn get_init_conf(conf_name: &str) -> HashSet<Vec<i32>> {
    match conf_name {
        "glider" => fill_hset(&CONF_GLIDER),
        "oscillator" => fill_hset(&CONF_OSCILLATOR),
        "glidergun" => fill_hset(&CONF_GLIDERGUN),
        "custom" => {
            print!("Provide a path to the input file: ");
            stdout().flush().unwrap();
            let mut input_path = String::new();
            stdin().read_line(&mut input_path).unwrap();
            read_user_input(input_path.trim())
        },
        _ => panic!("Initial configuration name not known!"),
    }
}

fn step_conf(conf: &HashSet<Vec<i32>>) -> HashSet<Vec<i32>> {

    let mut output: HashSet<Vec<i32>> = HashSet::new();
    let mut neighbours: HashMap<Vec<i32>, i32> = HashMap::new();

    for point in conf.iter() {

        let mut n_of_neighbours = 0;
        
        for delta0 in [-1, 0, 1] {
            for delta1 in [-1, 0, 1] {

                if delta0 == 0 && delta1 == 0 { continue; }

                let mut neighbour = (*point).clone();
                neighbour[0] += delta0;
                neighbour[1] += delta1;

                if conf.contains(&neighbour) { n_of_neighbours += 1; }

                let counter = neighbours.entry(neighbour).or_insert(0);
                *counter += 1;
            }
        }

        if [2, 3].contains(&n_of_neighbours) {
            output.insert(point.clone());
        }
    }

    for (point, n_of_neighbours) in neighbours.iter() {
        if *n_of_neighbours == 3 { output.insert(point.clone()); }
    }

    output
}

fn show_conf(conf: &HashSet<Vec<i32>>) -> String {

    let mut output = String::new();

    // (min_x, max_x, min_y, max_y)
    let mut ranges = (None, None, None, None);

    for point in conf.iter(){
        match ranges.0 {
            None => ranges.0 = Some(point[0]),
            Some(value) => if value > point[0] { ranges.0 = Some(point[0]); }
        }
        match ranges.1 {
            None => ranges.1 = Some(point[0]),
            Some(value) => if value < point[0] { ranges.1 = Some(point[0]); }
        }
        match ranges.2 {
            None => ranges.2 = Some(point[1]),
            Some(value) => if value > point[1] { ranges.2 = Some(point[1]); }
        }
        match ranges.3 {
            None => ranges.3 = Some(point[1]),
            Some(value) => if value < point[1] { ranges.3 = Some(point[1]); }
        }
    }

    if ranges.0 == None {
        return output;
    }

    for idx_y in ranges.2.unwrap()..ranges.3.unwrap() + 1 {
        for idx_x in ranges.0.unwrap()..ranges.1.unwrap() + 1 {
            let mut current_vec = Vec::new();
            current_vec.push(idx_x);
            current_vec.push(idx_y);
            if conf.contains(&current_vec) {
                output.push_str("#");
            } else {
                output.push_str(" ");
            }
        }
        output.push_str("\n");
    }

    output
}

fn main() {

    let wait_time = time::Duration::from_millis(100);

    print!("Choose a configuration: ");
    stdout().flush().unwrap();

    let mut user_input = String::new();
    stdin().read_line(&mut user_input).unwrap();

    let mut configuration = get_init_conf(user_input.trim());

    loop {

        println!("{}", show_conf(&configuration));
        thread::sleep(wait_time);

        configuration = step_conf(&configuration);

        print!("\x1B[2J\x1B[1;1H");
        stdout().flush().unwrap();
    }
}
