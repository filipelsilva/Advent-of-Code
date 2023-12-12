use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let mut seeds: Vec<Vec<u64>> = Vec::new();
    let mut seed_to_soil: Vec<(u64, u64, u64)> = Vec::new();
    let mut soil_to_fertilizer: Vec<(u64, u64, u64)> = Vec::new();
    let mut fertilizer_to_water: Vec<(u64, u64, u64)> = Vec::new();
    let mut water_to_light: Vec<(u64, u64, u64)> = Vec::new();
    let mut light_to_temperature: Vec<(u64, u64, u64)> = Vec::new();
    let mut temperature_to_humidity: Vec<(u64, u64, u64)> = Vec::new();
    let mut humidity_to_location: Vec<(u64, u64, u64)> = Vec::new();

    input.split("\n\n").enumerate().for_each(|(i, line)| {
        if i == 0 {
            seeds = line
                .split(": ")
                .last()
                .unwrap()
                .split(" ")
                .map(|num| vec![num.parse::<u64>().unwrap()])
                .collect();
            return;
        }

        line.lines().enumerate().for_each(|(ii, map)| {
            if ii == 0 {
                return;
            }
            let nums: Vec<u64> = map
                .split(" ")
                .map(|num| num.parse::<u64>().unwrap())
                .collect();
            let (dest, src, range) = (nums[0], nums[1], nums[2]);
            match i {
                1 => seed_to_soil.push((src, range, dest)),
                2 => soil_to_fertilizer.push((src, range, dest)),
                3 => fertilizer_to_water.push((src, range, dest)),
                4 => water_to_light.push((src, range, dest)),
                5 => light_to_temperature.push((src, range, dest)),
                6 => temperature_to_humidity.push((src, range, dest)),
                7 => humidity_to_location.push((src, range, dest)),
                _ => panic!("Unknown map"),
            }
        });
    });

    println!("{:?}", seeds);
    println!("{:?}", seed_to_soil);
    println!("{:?}", soil_to_fertilizer);
    println!("{:?}", fertilizer_to_water);
    println!("{:?}", water_to_light);
    println!("{:?}", light_to_temperature);
    println!("{:?}", temperature_to_humidity);
    println!("{:?}", humidity_to_location);

    for seed in &mut seeds {
        let mut flag = false;
        let num = seed.last().unwrap().clone();
        for (src, range, dest) in &seed_to_soil {
            if (src..&(src + range)).contains(&&num) {
                seed.push(dest + num - src);
                flag = true;
                break;
            }
        }
        if !flag {
            seed.push(num);
        }
    }

    for seed in &mut seeds {
        let mut flag = false;
        let num = seed.last().unwrap().clone();
        for (src, range, dest) in &soil_to_fertilizer {
            if (src..&(src + range)).contains(&&num) {
                seed.push(dest + num - src);
                flag = true;
                break;
            }
        }
        if !flag {
            seed.push(num);
        }
    }

    for seed in &mut seeds {
        let mut flag = false;
        let num = seed.last().unwrap().clone();
        for (src, range, dest) in &fertilizer_to_water {
            if (src..&(src + range)).contains(&&num) {
                seed.push(dest + num - src);
                flag = true;
                break;
            }
        }
        if !flag {
            seed.push(num);
        }
    }

    for seed in &mut seeds {
        let mut flag = false;
        let num = seed.last().unwrap().clone();
        for (src, range, dest) in &water_to_light {
            if (src..&(src + range)).contains(&&num) {
                seed.push(dest + num - src);
                flag = true;
                break;
            }
        }
        if !flag {
            seed.push(num);
        }
    }

    for seed in &mut seeds {
        let mut flag = false;
        let num = seed.last().unwrap().clone();
        for (src, range, dest) in &light_to_temperature {
            if (src..&(src + range)).contains(&&num) {
                seed.push(dest + num - src);
                flag = true;
                break;
            }
        }
        if !flag {
            seed.push(num);
        }
    }

    for seed in &mut seeds {
        let mut flag = false;
        let num = seed.last().unwrap().clone();
        for (src, range, dest) in &temperature_to_humidity {
            if (src..&(src + range)).contains(&&num) {
                seed.push(dest + num - src);
                flag = true;
                break;
            }
        }
        if !flag {
            seed.push(num);
        }
    }

    for seed in &mut seeds {
        let mut flag = false;
        let num = seed.last().unwrap().clone();
        for (src, range, dest) in &humidity_to_location {
            if (src..&(src + range)).contains(&&num) {
                seed.push(dest + num - src);
                flag = true;
                break;
            }
        }
        if !flag {
            seed.push(num);
        }
    }

    println!("{:?}", seeds);

    println!(
        "Part 1: {:?}",
        seeds.iter().map(|seed| seed.last().unwrap()).min().unwrap()
    );
    // println!("Part 2: {:?}", part2);
}
