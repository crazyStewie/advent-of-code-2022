#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
)]

const INPUT: &str = include_str!("inputs/01.txt");

fn run_pt_1() {
    // Collect all the line slices on a single contiuous Vec
    // Note that this vec contains only references to each line, and not the string itself
    // I only did this to simplify the group separation using the split method, instead
    // of spliting the string using "/n/n" or "/r/n/r/n" or doing some more manual approach
    let lines = INPUT.lines().collect::<Vec<_>>();

    // Use the slice split method to split the lines Vec at the empty lines,
    // Separating the data for each elf on it's own slice
    let result = lines.split(|&l| l.is_empty())
        // The map function runs for each group the lines got split into, returning a value for each one
        // i.e, it maps each of the groups to a corresponding value, which in this case is the total
        // amount of food each elf is carrying
        .map(|group_lines| {
            // Iterate over all the lines on each group
            group_lines
                .iter()
                // Map each line into it's parsed version, this assumes the parsing always succeeds
                .map(|s| s.parse::<i32>().unwrap())
                // Sum all of the parsed values on this group, this is the value returned for each group
                // on the outer map function
                .sum::<i32>()
        })
        // Gets the maximum element from all the individual groups total values
        .max()
        // Assumes a maximum always exists (the iterator isn't empty)
        .unwrap();
    println!("Day 1: Calorie Counting part 1");
    println!("{result:?}");
}

fn run_pt_2() {
    let lines = INPUT.lines().collect::<Vec<_>>();

    let result = lines.split(|&l| l.is_empty())
        .map(|group_lines| {
            group_lines
                .iter()
                .map(|s| s.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        //Instead of getting the max value, we fold the iterator into an array containing the largest 3 alements
        .fold([std::i32::MIN; 3], |mut max, element| {
            // Find the smallest element currently on the max array
            let mut min_idx = 0;
            for i in 1..max.len() {
                if max[i] < max[min_idx] {
                    min_idx = i;
                }
            }
            // Overrides the smallest element with the current one if necessary
            if element > max[min_idx] {
                max[min_idx] = element;
            }
            // Propagates the max array to the next element on the iterator
            max
        })
        // Sum the 3 elements from the final max array
        .iter()
        .sum::<i32>();
    println!("Day 1: Calorie Counting part 2");
    println!("{result:?}");
}

fn main(){
    let start = std::time::Instant::now();
    run_pt_1();
    run_pt_2();
    let duration = start.elapsed();
    println!("Finished in {duration:#?}");
}