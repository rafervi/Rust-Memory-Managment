use std::rc::Rc;

#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() {
    let i = 5;
    let j = i;
    println!("{}",i);
    println!("{}",j);

    let v = vec![1, 2, 3, 4, 5];
    /*let w = v;
    println!("{:?}",w);
    println!("{:?}",v);*/
    let foo = |v: Vec<i32>| -> Vec<i32> {
        println!("Vector used in foo");
        v
    };
    let v = foo(v);
    println!("{:?}",v);
    //Borrowing
    let mut a = 6;
    {
        let b = &mut a;
        println!("{}", *b);
        *b +=2;
    }

    println!("{}", a);

    let mut v = vec![1, 2, 3, 4, 5];
    for i in &v {
        println!("{}", i);

    }
    let p1= Person{name: String::from("john")};
    let d1 = Dog{name: String::from("Max"), owner: &p1};

    println!("{:?}", d1);

    let mut a: &String;
    {
        let p2 = Person{name:String::from("Mary")};
       // a = p2.get_name();
        a = p1.get_name();
    }
println!("{}", a);
    let brand = Rc::new(String::from("BMW"));
    println!("pointers: {}", Rc::strong_count(&brand));
    {
        let car = Car::new(brand.clone());
        car.drive();
        println!("pointers: {}", Rc::strong_count(&brand));
    }
    println!("My car is a {}",brand);
    println!("pointers: {}", Rc::strong_count(&brand));

}
//lifetimes
#[derive(Debug)]
struct Person {
    name:String
}
#[derive(Debug)]
struct Dog<'l> {
    name:String,
    owner: &'l Person
}
impl Person {
    //fn get_name(&self) -> &String { It's the same  as
    fn get_name<'l>(&'l self) -> &'l String{
        &self.name
    }
}
struct Car {
    brand: Rc<String>
}

impl Car {
    fn new(brand: Rc<String>)-> Car{Car{brand: brand}}
    fn drive(&self){
        println!("{} is driving ", &self.brand);
    }
}