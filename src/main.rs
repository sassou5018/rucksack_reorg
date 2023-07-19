use std::fs;

fn main() {
    let alphabet = create_alphabet();
    let data = fs::read_to_string("./input.txt").expect("Can't open file!");
    let mut duplicates: Vec<&str> = Vec::new();
    println!("{:#?}", alphabet);
    for line in data.trim().lines(){
        let (half1, half2) = line.split_at(line.len()/2);
        for letter in split_letters(half1).iter(){
            if half2.contains(letter){
                duplicates.push(letter);
                //if i don't break i'm going to get duplicates
                break;
            }
        }
    }
    println!("Duplicates: {:#?}", duplicates);
    let mut sum: u32 = 0;
    for letter in duplicates.iter(){
        let letter = letter.clone();
        let value = alphabet.iter().position(|&x| String::from(x) == String::from(letter)).unwrap() as u32;
        sum+=value+1;
    }

    println!("sum: {}", sum);
}



/**
 * Just so i can drop uppercase from heap memory because i don't need it.
 */
fn create_alphabet()->Vec<char>{
    let mut alphabet: Vec<char> = ('a'..='z').into_iter().collect::<Vec<char>>();
    let mut uppercase = ('A'..='Z').into_iter().collect::<Vec<char>>();
    alphabet.append(&mut uppercase);
    alphabet
}

fn split_letters(half: &str)-> Vec<&str>{
    let mut x = half.split("").collect::<Vec<&str>>();
    x.pop();
    x.remove(0);
    x
}