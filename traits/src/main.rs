// #[derive(Display)]
#[allow(dead_code)]

pub struct Animal {
    pub name: &'static str,
    pub age: i32,
}

pub struct Person {
    name: &'static str,
    age: i32,
}

/* We can add `new` method directly: */
//
// impl Animal {
//     pub fn new(name: &'static str, age: i32) -> Animal {
//         Animal {
//             name: name,
//             age: age,
//         }
//     }
// }
//
// impl Person {
//     pub fn new(name: &'static str, age: i32) -> Animal {
//         Animal {
//             name: name,
//             age: age,
//         }
//     }
// }

/* Or use a construct trait? */
//
trait Construct {
    fn new(name: &'static str, age: i32) -> Self;
}

impl Construct for Animal {
    fn new(name: &'static str, age: i32) -> Self {
        Animal {
            name: name,
            age: age,
        }
    }
}

impl Construct for Person {
    fn new(name: &'static str, age: i32) -> Self {
        Person {
            name: name,
            age: age,
        }
    }
}

// Implement Speak trait for Animal and Person
trait Speak {
    fn say_name(&self) -> &'static str;
    fn say_age(&self) -> i32;
}

impl Speak for Animal {
    fn say_name(&self) -> &'static str {
        self.name
    }

    fn say_age(&self) -> i32 {
        self.age
    }
}

impl Speak for Person {
    fn say_name(&self) -> &'static str {
        self.name
    }

    fn say_age(&self) -> i32 {
        self.age
    }
}

fn main() {
    let a = Animal::new("Dexter", 4);
    let b = Person::new("Bryan", 34);

    greet(a);
    greet(b);
}

/* Greet function that accepts any type that implements Speak */
fn greet<T: Speak>(being: T){
    println!(
        "Hello, I am {} and am {} years old.",
        being.say_name(),
        being.say_age()
    );
}
