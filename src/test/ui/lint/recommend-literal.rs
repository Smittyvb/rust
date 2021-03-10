type Real = double;
//~^ ERROR cannot find type `double` in this scope
//~| HELP you may have meant to use a literal instead

fn main() {
    let x: Real = 3.5;
    let y: long = 74802374902374923;
    //~^ ERROR cannot find type `long` in this scope
    //~| HELP you may have meant to use a literal instead
}

fn z(a: boolean) {
    //~^ ERROR cannot find type `boolean` in this scope
    //~| HELP you may have meant to use a literal instead
}

fn a() -> byte {
//~^ ERROR cannot find type `byte` in this scope
//~| HELP you may have meant to use a literal instead
    3
}

struct Data { //~ HELP you might be missing a type parameter
    width: float,
    //~^ ERROR cannot find type `float` in this scope
    //~| HELP you may have meant to use a literal instead
    depth: Option<int>,
    //~^ ERROR cannot find type `int` in this scope
    //~| HELP you may have meant to use a literal instead
}

trait Stuff {}
impl Stuff for short {}
//~^ ERROR cannot find type `short` in this scope
//~| HELP you may have meant to use a literal instead
