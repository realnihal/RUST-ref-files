#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_features)]

mod s_and_h;
mod pm;

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
use crate::pm::pattern_matching;

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

struct Point
{
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

enum Color{
    Red,
    Green,
    Blue,
    RgbColor(u8,u8,u8), //tuple like construct
    Cmyk{cyan:u8,magenta:u8,yellow:u8,black:u8}//struct like construct
}

fn enums(){
    let c = Color::Cmyk{cyan:3,magenta:3,yellow:2,black:255};
    match c {
        Color::Red => println!("r"),
        Color::Blue => println!("b"),
        Color::Green => println!("g"),
        Color::RgbColor(0,0,0) => println!("black"),
        Color::RgbColor(r,g,b) => println!("{},{},{}",r,g,b),
        Color::Cmyk {cyan:_,magenta:_,yellow:_,black:255} => println!("Black"),
        _ => println!("not available")
    }
}

union IntOrFloat{
    i: i32,
    f: f32
}

fn unions(){
    let mut iof = IntOrFloat{i:123};
    iof.i = 234;

    let value = unsafe{ iof.i };
    println!("iof.i = {}",value);

    process_value(IntOrFloat{f:42.0})
}

fn process_value(iof: IntOrFloat){
    unsafe{
        match iof {
            IntOrFloat {i:42} => println!("meaning of life"),
            IntOrFloat {f} => println!("float value = {}",f)

        }
    }
}

fn option_t(){
    let x = 3.0;
    let y = 2.0;

    //option
    let result =
    if y != 0.0 {Some(x/y)} else {None};

    match result{
        Some(z) => println!("{}/{}={}",x,y,z),
        None => println!("Cannot divide by zero")
    }

    if let Some(z) = result{
        println!("result = {}",z)
    }
}

fn array_fun(){
    let mut a = [1,2,3,4,5];

    println!("a has {} elements, first is {}", a.len(), a[0]); //indexing arrays
    a[0] = 321;
    println!("a = {}", a[0]); //mutable array elements

    println!("{:?}",a); //printing out array for debugging

    if a != [1,2,3,4,5]{ //compare with the array
        println!("Does not match");
    }
    let b = [1u16;10]; //b.len() == 10 initialising with u16
    for i in 0..b.len(){
        println!("{}",b[i]);
    }
    println!("b took up {}", mem::size_of_val(&b));

    //multi dimensional arrays
    let mtx:[[f32;3];2] =
    [
        [3.0,2.0,3.0],
        [2.0,4.0,3.0]
    ];
    println!("{:?}",mtx); //printing the whole matrix
}

fn use_slice(slice:&mut[i32]){
    println!("first elem = {}, len = {}", slice[0],slice.len());
    slice[0] = 4321;
}

fn slices(){
    let mut data = [1,2,3,4,5];
    use_slice(&mut data[1..4]);
    println!("{:?}",data);
}

fn tuples(){
    let x= 3;
    let y= 4;
    let sp = sum_and_product(x,y);
    println!("{:?}",sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y ,sp.0,sp.1);

    //destructuring
    let (a,b) = sp;
    println!(" a= {}, b= {}",a,b);

    let sp2 = sum_and_product(4,7);
    let combined = (sp,sp2);
    println!("{:?}",combined);
    println!("last elem = {}", (combined.1).1);

    let foo = (true, 43.0, -1i8);
    println!("{:?}",foo);

    let meaning = (42,);
    println!("{:?}",meaning);
}

fn sum_and_product(x:i32,y:i32)->(i32,i32)
{
    (x+y,x*y)
}

fn vectors(){
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?} ",a);
    a.push(44);
    println!("a = {:?} ",a);

    //usize isize
    let idx:usize = 0;
    a[idx] = 312;
    println!("a[0] = {}",a[idx]);
    //option type
    match a.get(6){
        Some(x) => println!("a[6] = {}",x),
        None => println!("No such element")
    }

    for x in &a {
        println!("{}",x);
    }
    a.push(77);
    println!("{:?}",a);
    let last_elem = a.pop(); //option
    println!("last elem is {:?}, a ={:?}", last_elem, a);

    while let Some(x) = a.pop()
    {
        println!("{}",x);
    }
}
use std::collections::HashMap;
fn hashes(){
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"),3);
    shapes.insert(String::from("square"),4);
    println!("a square has {} sides",shapes["square".into()]);

    shapes.insert("square".into(),5);
    println!("{:?}",shapes);

    shapes.entry("circle".into()).or_insert(1);
    {
        let actual = shapes
            .entry("circle".into())
            .or_insert(2);
        *actual = 0
    }

}

use std::collections::HashSet;
use std::thread;
use std::time;

fn Hashsets(){
    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");
    greeks.insert("delta");
    println!("{:?}",greeks);

    let added_vega = greeks.insert("vega");
    if added_vega{
        println!("we added vega!!")
    }

    if !greeks.contains("kappa") {
        println!("we don't have kappa")
    }

    let removal = greeks.remove("delta");
    if removal {
        println!("we removed delta");
        println!("{:?}",greeks)
    }

    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    //subset
    println!(
        "is {:?} a subset of {:?}? {}",
        _2_8, _1_10,
        _2_8.is_subset(&_1_10)
    );

    //disjoint = no common elements
    println!(
        "is {:?} a disjoint of {:?}? {}",
        _1_5, _6_10,
        _1_5.is_disjoint(&_6_10)
    );

    //union, intersection
    println!(
        "items in either {:?} and {:?} are {:?}",
        _2_8,_6_10,_2_8.union(&_6_10)
    );
    println!(
        "items in both {:?} and {:?} are {:?}",
        _2_8,_6_10,_2_8.intersection(&_6_10)
    );
}

fn iterators(){
    let mut vec = vec![3,2,1];
    let mut vec2 = vec![1,2,3];
    vec2.extend(vec);
    println!("{:?}",vec2);
}

fn strings()
{
    let s:&'static str = "hello world"; //&str = string slice
    //s = 'abc;
    //let  = s[0];
    for c in s.chars().rev()
    {
        println!("{}",c);
    }

    if let Some(first_char) = s.chars().nth(0){
        println!("first char is {}",first_char)
    }
    //heap
    //string
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8)
    {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }
    println!("{:?}",letters);
    let u:&str = &letters;
    println!("{}",u);

    let z = letters + "123";
    println!("{:?}",z);
}

fn print_value(x:i32)
{
    println!("value = {}",x);
}

fn increase(x: &mut i32)
{
    *x += 1;
}

fn product(x:i32,y:i32) -> i32
{
    return x*y;
}

fn functions()
{
    print_value(34);

    let mut z = 1;
    increase(&mut z);
    print_value(z);
    print_value(product(3,5));

}


fn main() {

}


