#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_features)]

mod s_and_h;
use std::mem;
use crate::s_and_h::stack_and_heap;

const MEANING_OF_LIFE:u8 = 42; // no fixed address
static mut Z:i32 = 123; // this cannot be mutable to prevent memory safety compromise

fn operators(){

    //arithmetic
    let mut a = 2+3*4; // +-/*
    println!("{}",a);
    a = a+1;
    a -= 2; //a = a-2
            // -= += *= /= %=
    println!("remainder of {}/{} = {}",a,3,(a%3));

    let a_cubed = i32::pow(a,3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b,3);
    let b_to_pi = f64::powf(b,std::f64::consts::PI);
    println!("{} cubed is {}, to the pi is {}",b,b_cubed,b_to_pi);

    //bitwise
    let c = 1 | 2; // | or, & and, ^Xor, ! nor
    println!("1|2 = {}",c);
    let two_to_10 = 1<<10;
    println!("2^10 = {}",two_to_10);

    //logical
    let pi_less_4 = std::f64::consts::PI < 4.0; //true
    // > <= >= ==
    let x = 5;
    let x_is_5 = x ==5;
}

fn unsafe_rust(){
    unsafe{
        let mut a = 10;
        a = 324;
        println!("using unsafe rust, a = {}",a);
    }
}

fn scope_and_shadowing(){
    let a = 123;
    { //scoping and shadowing
        let a = 777; // inner variable shadows the outer variable
        let b = 456;
        println!("inside b = {}",b); // b exists only inside the braces
        println!("inside a = {}",a);
    }
    println!("a = {}",a);
}

fn fundamental_data_types() {
    let a: u8 = 123; // unsigned , 8 bits, 0-255
    println!("a = {}", a); //immutable

    // u = unsigned
    // i = signed, -128 to 127
    let mut b: i8 = 0;
    println!("b = {} before", b);
    b = 42;
    println!("b = {} after", b);

    let mut c = 123456789;
    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {}", c);

    //usize isize
    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {}, {}-bit OS",
             z, size_of_z, size_of_z * 8);

    let d: char = 'x';
    println!("{} is a char, size = {} bytes", d, mem::size_of_val(&d));

    //f32 f64 IEEE754 signed!

    let e = 2.5;
    println!("{} is a float, size = {} bytes",e, mem::size_of_val(&e));

    let g= false; //true
    println!("{} is a bool, size = {} bytes", g, mem::size_of_val(&g));
}

fn if_statement(){
    let temp = 35;
    if temp > 30{
        println!("Its really hot outside");
    }else if temp < 10{
        println!("Its really cold");
    }else{
    println!("Temp is okay");
    }
    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("today is {}",day);
    println!("It is {}",
    if temp >20{
    if temp >30 {"Very Hot"} else {"Hot"}
    }else if temp < 10 {"cold"}else {"ok"});
}

fn while_and_loop()
{
    let mut x = 1;
    while x < 1000
    {
        x *= 2;
        if x == 64 {continue;} //skip the rest of the loop at that value
        println!("x = {}",x)
    }
    let mut y = 1;
    loop //while true
    {
        y *= 2;
        println!("y = {}", y);

        if y == 1 << 10 { break; }
    }
}

fn for_loop()
{
    for x in 1..11
    {
        if x  == 3 {continue;} //skips for 3 value
        if x == 8 {break;} //breaks out of the loop at 8
        println!("x = {}",x);
    }
    for (pos,y) in (30..41).enumerate()
    {
        println!("{} : {}",pos,y);
    }
}

fn match_state(){
    let country_code = 7;

    let country = match country_code{
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "unknown",
        _ => "invalid"
    };

    println!("the country with code {} is {}", country_code,country);
    let x = true;
    let state = match x{
        true => "yes",
        false => "no"
    };
}

enum State{
    Locked,
    Failed,
    Unlocked
}
use std::io::stdin;
fn combination_lock(){
    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();
    loop{
        match state{
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input){
                    Ok(_) => {
                        entry.push_str(&input.trim_end());
                    }
                    Err(_) => continue
                }
                if entry == code{
                    state = State::Unlocked;
                    continue;
                }
                if !code.starts_with(&entry){
                    state = State::Failed;
                }
            }
            State::Failed => {
                println!("FAILED");
                entry.clear();
                state = State::Locked;
                continue;
            }
            State::Unlocked =>{
                println!("UNLOCKED");
                return;
            }
        }
    }
}

struct Point{
    x: f32,
    y: f32
}

struct Line{
    start: Point,
    end: Point
}

fn structures(){
    let p1 = Point{x:3.4,y:3.3};
    println!("x of pi = {}, y of p1 is {}", p1.x,p1.y);
    let p2 = Point{x:4.5,y:2.4};
    let my_line = Line{start:p1,end:p2};
    println!("my line starts at ({},{}), and ends at ({},{})",my_line.start.x,my_line.start.y,my_line.end.x,my_line.end.y);
}

fn main() {
    //if_statement();
    //while_and_loop();
    //for_loop();
    //match_state()
    //combination_lock()
    structures()
}