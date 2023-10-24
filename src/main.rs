#![allow(unused)]
use std::char::MAX;
use std::{io, vec, any};
use std::error::Error;
use std::num::ParseIntError;
use rand::Rng;
use rand::seq::index;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;

fn is_num_in(num: i32, container: &Vec<i32>) -> bool{
    for item in container{
        if item == &num{
            return true
        }
    }
    return false
}

fn list_values(values: &Vec<i32>){
    for item in values{
        print!("{} ", item)
    }
    println!("");
}

use std::ops::Sub;

fn sub_two_gen<T:Sub<Output = T>>(x: T, y:T) -> T{
    x - y
}

use std::ops::Add;

fn add_two_gen<T:Add<Output = T>>(x: T, y:T) -> T{
    x + y
}

trait Enemy {
    fn new(health: i32, dmg: i32) -> Self;
    fn take_dmg(&mut self, dmg_took: i32);
    fn deal_dmg(&self) -> i32;
}

struct Goomba{health: i32, dmg: i32}

impl Enemy for Goomba{
    fn new(mut health: i32, dmg: i32) -> Goomba {
        return Goomba{health, dmg}
    }

    fn take_dmg(&mut self, dmg_took: i32) {
        self.health -= dmg_took;
    }

    fn deal_dmg(&self) -> i32{
        return  self.dmg;
    }
}



fn preform_operation(x: i32, y:i32, operator: &str) -> i32{
    match operator{
        "+" => return x + y,
        "-" => return x-y,
        "*" => return x*y,
        "/" => return x/y,
        "%" => return x%y,
        _ => {println!("Invalid Operation! Returning 0");
        return 0}
    }
}


fn main(){
    println!("");
    
    let path: &str = "groceries.txt";
    
    /* let output = File::create(path);
    let mut output:File = match output{
        Ok(file) => file,
        Err(error) =>{
            panic!("Woah!")
        },
    }; */
/* write!(output, "Bananas,\nChicken,\nMilk - 2 Gallons,\nPudding Mix - Vanilla,\nKibby Bites for Eevee\nLettuce,\nToilet Paper").expect("Failed to write to file, eevee sad."); */

    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);

    for line in buffered.lines(){
        println!("{}", line.unwrap());
    }

    
    let path2: &str = "bee_movie.txt";
    let output2 = File::open(path2).unwrap();

    let buff2 = BufReader::new(output2);

    for line in buff2.lines(){
        println!("{}", line.unwrap());
    }

    let output3 = File::open("rand.txt");
    let output3 = match output3 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file: {:?}", e),
            },
            _other_error => panic!("Problem opening file : {:?}", error),
        },
    };

    
}
/* fn main() {
    println!("Hello there!");
    let mut name: String = String::new();
    let greeting: &str = "Nice to meet you";
    io::stdin().read_line(&mut name)
        .expect("Didn't Receive Input");

    println!("Hola {}! {}", name.trim_end(), greeting)
} */
/* let string_cheese: &str = "String cheese";
    let grocery_list: String = String::from("Groceries:\n");

    let result = grocery_list + string_cheese;
    println!("{}",result);
    println!("{}",string_cheese) */
/* Strings are defined with "" while characters are defined with '' */

/* let num1: u8 = 20;
    let num2: u8 = 236;
    let num3:u32 = (num1 as u32) + (num2 as u32);
    println!("result: {}", (u16::MAX))
    
    
    let num1: u32 = 64;
    let console = String::from("Nintendo ") ;
    
    println!("result: {}", console + stringify!()) */



/*  enum Month {
        January,
        Febuary,
        March,
        April,
        May,
        June,
        July,
        August,
        September,
        October,
        November,
        December
    }

    impl Month {
        fn is_winter(&self)-> bool {
            match self {
                Month::December | Month::January | Month::Febuary => true,
                _ => false
            }
        }
    }

    let dis_month: Month = Month::Febuary;

    println!("It is winter in Febuary. {}", dis_month.is_winter()) */

    /* let mut vec1: Vec<i32> = vec![1991, 1992, 1994, 1998, 2001];

    let mut sequel: &i32 = &vec1[1];

    for item in &vec1{
        println!("{}", item);
    }

    match &vec1.get(1){
        Some(&i32) => println!("Success"),
        None => println!("Failure")
    }

    vec1.push(2006);
    sequel = &vec1[5];

    match &vec1[5]{
        sequel  => println!("Success"),
        _ => println!("Failure")
    }

    for item in &vec1{
        println!("{}", item);
    } */

    /* let result1: bool = is_num_in(5, &nums);
    let result2: bool = is_num_in(8, &nums);

    println!("Is five in nums: {}",result1);
    println!("Is eight in nums: {}",result2);

    let result3 = preform_operation(nums[2], 7, "+");

    println!("\n{}", result3);
    println!("\n{}", preform_operation(4, 5, "*"));
    println!("\n{}", preform_operation(5, 4, "%"));
    
     let nums = vec![1,2,3,4,5];

    list_values(&nums);

    println!("8 - 5 = {}", sub_two_gen(8, 5));
    println!("10 + 12 = {}", add_two_gen(10, 12));
    */


    /* let mut points:HashMap<_, _>  = HashMap::new();
    points.insert("Elly", "5");
    points.insert("Mario", "6");
    points.insert("Earl", "3");
    points.insert("Mickey", "9");

    for (k,v) in points.iter(){
        println!("{} had {} points.", k, v)
    } */

    /* let mut goom:Goomba = Enemy::new(5,3);
    println!("Goomba - Health:{} Damage:{}", goom.health, goom.dmg);
    println!("\nYou deal 2 hits to the goomba.\n");
    goom.take_dmg(2);
    println!("Goomba - Health:{} Damage:{}", goom.health, goom.dmg); */