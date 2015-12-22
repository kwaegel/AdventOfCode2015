
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::cmp;

struct Raindeer {
    name: String,
    speed: u32,
    flying_time: u32,
    rest_time: u32,
}

fn get_input_lines(filename: &str) -> Vec<String> {
    let path = Path::new(filename);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let lines = reader.lines();

    lines.map(|x| x.unwrap()).collect()
}

// --------------------------------------------------------

fn get_raindeer(strings: &Vec<String>) -> Vec<Raindeer> {

    let mut raindeer = Vec::new();

    for string in strings {
        let tokens: Vec<&str> = string.split_whitespace().collect();
        let name = tokens[0].to_string();
        let speed = tokens[3].parse::<u32>().unwrap();
        let flying_time = tokens[6].parse::<u32>().unwrap();
        let rest_time = tokens[13].parse::<u32>().unwrap();

        let rd = Raindeer {
            name: name,
            speed: speed,
            flying_time: flying_time,
            rest_time: rest_time,
        };

        raindeer.push(rd);
    }

    raindeer
}

// --------------------------------------------------------

fn main() {
    let strings = get_input_lines("day14.txt");
    let raindeer = get_raindeer(&strings);

    let total_seconds = 2503;
    // let total_seconds = 1000; // 1000 seconds for test data

    {
        let mut best_distance = 0;
        for rd in &raindeer {
            let cycle_time = rd.flying_time + rd.rest_time;
            let num_full_cycles = total_seconds / cycle_time;
            let full_cycle_dist = num_full_cycles * rd.flying_time * rd.speed;

            let remaining_seconds = total_seconds % cycle_time;
            let remaining_fly_time = cmp::min(remaining_seconds, rd.flying_time);
            let remaining_dist = remaining_fly_time * rd.speed;

            let total_dist = full_cycle_dist + remaining_dist;

            println!("{} went {} cycles covering {} km, then flew an extra {} km, for a total of \
                      {} km",
                     rd.name,
                     num_full_cycles,
                     full_cycle_dist,
                     remaining_dist,
                     total_dist);

            best_distance = cmp::max(best_distance, total_dist);
        }

        println!("Part 1: overall, the best distance was {} km",
                 best_distance);
    }

    // Part 2: scoring by the second
    {
        let num_raindeer = raindeer.len();
        let mut points = vec![0; num_raindeer];
        for seconds in 1..total_seconds {

            let mut distances = vec![0; num_raindeer];
            for rd_idx in 0..num_raindeer {
                let rd = &raindeer[rd_idx];

                let cycle_time = rd.flying_time + rd.rest_time;
                let num_full_cycles = seconds / cycle_time;
                let full_cycle_dist = num_full_cycles * rd.flying_time * rd.speed;

                let remaining_seconds = seconds % cycle_time;
                let remaining_fly_time = cmp::min(remaining_seconds, rd.flying_time);
                let remaining_dist = remaining_fly_time * rd.speed;

                let total_dist = full_cycle_dist + remaining_dist;
                distances[rd_idx] = total_dist;
            }

            let best_distance = *distances.iter().max().unwrap();
            for rd_idx in 0..num_raindeer {
                if distances[rd_idx] == best_distance {
                    points[rd_idx] += 1;
                }
            }
        }

        let best_score = *points.iter().max().unwrap();
        println!("Part 2: the best score is {}", best_score);
    }
}
