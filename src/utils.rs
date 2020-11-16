
use crate::Person;
/// pretty print a person instance
pub fn print_person(person: &Person) {
    println!("Person");
    let Person{id, first, last, login} = person;
    println!("\tid:    {}", id);
    if let Some(first) = first {
        println!("\tfirst: {}", first);
    }
    if let Some(last) = last {
        println!("\tlast:  {}", last);
    }
    if let Some(login) = login {
        println!("\tlogin: {}", login);
    }
   
}