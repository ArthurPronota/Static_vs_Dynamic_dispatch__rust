trait Op { // Is dyn-compatible !!!
    fn run(&self) ->i32 ;
}

struct A ;

impl Op for A {
    fn run(&self) ->i32 {
        1
    }
}

struct B ;

impl Op for B {
    fn run(&self) ->i32 {
        2
    }
}

// статическая сумма (generic trait)
// оба аргумента должны быть одного конкретного типа T
fn sum_static<T: Op>(x: &T, y: &T) ->i32 {
    x.run() + y.run()
}

// динамичесая сумма (dynamic trait)
fn sum_dyn(v: &[Box<dyn Op>]) ->i32 {
    v
        .iter()
        .map(|x| x.run())
        .sum()
}

fn main() {
    let a = A ;
    let a1 = A ;

    println!("static sum: {}", sum_static(&a, &a1)) ;   // Out: static sum: 2

    let b = B ;

    println!("dyn sum: {}", sum_dyn(&[Box::new(a), Box::new(a1), Box::new(b)])) ; // Out: dyn sum: 4
}
