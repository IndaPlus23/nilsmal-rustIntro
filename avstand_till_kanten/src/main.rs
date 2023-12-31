/***
 * Template to a Kattis solution.
 * See: https://open.kattis.com/help/rust
 * Author: Viola Söderlund <violaso@kth.se>
 * Edit: Benjamin Widman <bwidman@kth.se>
 */

use std::io;

/// Kattis calls main function to run your solution.
fn main() {
    // get standard input stream
    let input = io::stdin();

    // Get input lines as a vector of strings
    // see: https://doc.rust-lang.org/std/io/trait.BufRead.html
    let lines = input.lines()
        .map(|_line| _line.ok().unwrap())
        .collect::<Vec<String>>();          // Converts iterator into vector. Not necessary, see example solution.
    // The map acts on every element in the iterator, getting the value inside the returned Result, assuming the result is Ok(value) and not Err(error_message).
    // ok() returns an Option, so we unwrap it to get the value inside.
    
    //println!("{}", lines[0]);


    avstand(lines);

    //eprintln!("Kattis skips this comment!");
    //println!("Print to standard output.");
}




fn avstand(input: Vec<String>){

    let list:Vec<&str> = input[0].split(" ").collect();

    let width: usize = list[0].parse::<usize>().unwrap();
    let height: usize = list[1].parse::<usize>().unwrap();

    let mut arr: Vec<Vec<u64>> = vec![vec![0; height]; width];

    for (i_horisontal, _) in vec![0;list[0].parse::<usize>().unwrap()].iter().enumerate(){
        for (i_vertical, _) in vec![0;list[1].parse::<usize>().unwrap()].iter().enumerate(){
            let distance_map: [u64; 4] = [
                i_horisontal as u64,
                i_vertical as u64,
                list[1].parse::<usize>().unwrap() as u64 - i_vertical as u64 -1,
                list[0].parse::<usize>().unwrap() as u64 - i_horisontal as u64 -1
            ];
        
            let min = *distance_map.iter().min().unwrap();

            arr[i_horisontal][i_vertical] = min as u64 + 1;
        }
    }

    for (i_e1, _) in arr.iter().enumerate(){
        for e2 in arr[i_e1].iter(){
            if *e2 > 9 {
                print!(".");
                continue;
            }
            print!("{}", *e2 as u64);
        }
        println!("")
    }
}