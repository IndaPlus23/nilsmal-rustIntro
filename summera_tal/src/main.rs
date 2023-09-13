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
    
    summeratal(lines);

}


fn summeratal(input: Vec<String>){
    // println!("input: {:?}", input);

    let list_len: u128 = input[0].trim().parse::<u128>().unwrap();
    let mut list_string: String = input[1].trim().to_string();
    list_string = list_string.replace("  ", "");
    let list:Vec<&str> = list_string.split(" ").collect();
    
    let mut list_u128: Vec<u128>= list.iter().map(|&value| value.parse::<u128>().unwrap()).collect();

    let mut v_i128: Vec<i128> = list_u128.iter().map(|&x| x as i128).collect();

    v_i128.sort();

    list_u128 = v_i128.iter().map(|&x| x as u128).collect();
    

    let middle_index: u128;

    if list_len%2 as u128 == 0{
        middle_index = list_len/2 + 1;
    } else{
        middle_index = list_len/2 + 1;
    }

    let result_sum: u128 = list_u128[(middle_index -1) as usize..list_len as usize].iter().sum::<u128>();

    // println!("middle_index: {:?}", middle_index);
    // println!("arr_len: {:?}", list_len);
    // println!("list u128: {:?}", list_u128);
    // println!("RESULT: {:?}", result_sum)
    println!("{:?}", result_sum)

 }