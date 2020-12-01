#[allow(dead_code)]
#[allow(unused_variables)]

fn how_many(x:i32) -> &'static str
{
    match x
    {
        0 => "no",
        1|2 => "one or two",
        12 => "dozen",
        9...11 => "lots of",
        _ if (x % 2 == 0) => "some",
        _ => "a few"
    }
}

pub fn pattern_matching(){
    for x in 0..13
    {
        println!("{} : I have {} many oranges",x,how_many(x));
    }

    let point = (3,7);
    match(point){
        (0,0) => println!("origin"),
        (0,y) => println!("x_axis, y = {}",y),
        (x,0) => println!("x = {}, y_axis",x),
        (_,y) => println!("?,y={}",y)
    }
}

struct Point<T>
{
    x: T,
    y: T
}

struct Line<T>
{
    start: Point<T>,
    end : Point<T>
}

pub fn generics_fn(){
    let a = Point {x:0f64,y:3.1};
    let b = Point {x:1f64,y:3.4};

    let myline = Line {start:a,end:b};
}
