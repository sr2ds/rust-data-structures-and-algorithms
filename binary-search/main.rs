/**
 * Simple Rust Binary Search Sample
 * 
 */
fn main() {
    let array = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 15, 17, 18, 19, 20, 35, 38, 40, 50, 60, 70, 80, 90, 100];
    let find = 1;

    find_number_in_array(array, find)
}

fn find_number_in_array(array:Vec<i8>, number: i8) {
 
    let mut low = 0;
    let mut high = array.len() - 1;

    let mut middle;
    let mut to_try;
    let mut try_counter:i8 = 0;

    while low <= high {
        try_counter += 1;
        middle = (low + high) / 2;
        to_try = array[middle];
        
        if to_try == number {
            println!("Number {:?} found on index {:?} after {:?} tentatives", array[middle], middle, try_counter)
        }
        
        if to_try > number  {
            high = middle -1;
        } else {
            low = middle + 1;
        }
    }
}