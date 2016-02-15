// #[derive(Display)]
enum Animal {
    Cow,
    Dog,
    Cat
}

fn main() {
  let my_cow = Animal::Cow;
  let my_dog = Animal::Dog;
  let my_cat = Animal::Cat;
  paws(my_cow);
  paws(my_dog);
  paws(my_cat);
}

fn paws(animal: Animal){
    use Animal::*;

    println!("{}", match animal {
        Cow => 0,
        Dog => 4,
        Cat => 4
    });
}
