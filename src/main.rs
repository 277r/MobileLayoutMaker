use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

// struct voor elke letter en opvolgende letter count
struct LetterOrder {
	first_letter : char,
	second_letter : char,
	count: u64
}
// andere struct voor elke letter en hoevaak die voorkomt (eerste sortering, voor ergo locatie, dan andere sortering voor 2e gedeelte)
struct LetterCount {
	letter : char,
	count: u64
}

fn vector_contains(list : &Vec<LetterCount>, el : char) -> i64 {
// check first field only
	for i in 0..list.len() {
		let n = &list[i];
		if n.letter == el{
			return i as i64;
		}
	}
	return -1;
}

fn vector_contains_lo(list : &Vec<LetterOrder>, el1 : char, el2 : char) -> i64 {
	// check first field only
		for i in 0..list.len() {
			let n = &list[i];
			if n.first_letter == el1 && n.second_letter == el2 {
				return i as i64;
			}
		}
		return -1;
	}
	







fn main() {
	let args: Vec<String> = env::args().collect();


	let mut data_str = String::new();
	
	if (args.len()) > 1 {
		let p = Path::new(&args[1]);
		
		
		
		// probeer naar een lange string te lezen
		match File::open(p){
			Ok(mut file) => {
				
				file.read_to_string(&mut data_str).unwrap();
			}
			Err(error) => {
				println!("error {} while opening file {}", error, args[1]);
			}
			
		}
	}
	else {
		println!("error: no file provided");
	}
	// inefficiently lowercase-ify
	data_str = data_str.to_lowercase();
		

	let mut letters_az_aantallen : Vec<LetterCount> = Vec::new();

	// check each letter individually
	for i in data_str.chars() {
		let n = vector_contains(&letters_az_aantallen, i);
		
		// register existing letters and their counts in letters_az_aantallen
		if n == -1 {
			letters_az_aantallen.push(LetterCount {letter: i, count: 1});			
		}


		else {
			letters_az_aantallen[n as usize].count += 1;
		}

	}

	// remove unallowed letters
	let alphabet="abcdefghijklmnopqrstuvwxyz";

	let mut i : usize = 0;
	let mut max_size = letters_az_aantallen.len();
	while i < max_size {
		if !alphabet.contains(letters_az_aantallen[i].letter) {
			letters_az_aantallen.remove(i);
			max_size -= 1;
		}
		else {
			i += 1;
		}
	}

	// sort by count HiLo
	letters_az_aantallen.sort_by_key(|k| k.count);
	letters_az_aantallen.reverse();

	for el in letters_az_aantallen {
		println!("{} : {}", el.letter, el.count);
	}

	// after showing which letters are most often existent, make a list of letters that follow other letters most often
	// this code sucks, i'm sorry 

	let mut letters_az_aantallen : Vec<LetterOrder> = Vec::new();

	
	// check each letter double
	let data_charray : Vec<char> = data_str.chars().collect();
	
	for i in 0..(data_charray.len() -1) {
		let n = vector_contains_lo(&letters_az_aantallen, data_charray[i], data_charray[i+1]);
		
		// register existing letters and their counts in letters_az_aantallen
		if n == -1 {
			letters_az_aantallen.push(LetterOrder {first_letter: data_charray[i], second_letter: data_charray[i+1], count: 1});			
		}


		else {
			letters_az_aantallen[n as usize].count += 1;
		}

	}

	// remove unallowed letters
	let alphabet="abcdefghijklmnopqrstuvwxyz";

	let mut i : usize = 0;
	let mut max_size = letters_az_aantallen.len();
	while i < max_size {
		if !alphabet.contains(letters_az_aantallen[i].first_letter) {
			letters_az_aantallen.remove(i);
			max_size -= 1;
		}
		else {
			i += 1;
		}
	}
	let mut i : usize = 0;
	let mut max_size = letters_az_aantallen.len();
	while i < max_size {
		if !alphabet.contains(letters_az_aantallen[i].second_letter) {
			letters_az_aantallen.remove(i);
			max_size -= 1;
		}
		else {
			i += 1;
		}
	}

	// sort by count HiLo
	letters_az_aantallen.sort_by_key(|k| k.count);
	letters_az_aantallen.reverse();

	// TODO:
	// sort out letter combinations in the best way, only 26^2 elements, so only (26^2)! options if i calculated correctly
	// the calculation above was NOT correct



	for el in letters_az_aantallen {
		println!("{}, {} : {}", el.first_letter, el.second_letter, el.count);
	}





	

} 