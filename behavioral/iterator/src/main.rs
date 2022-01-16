use iterator::{StringList, StringListReverseIterator};

pub fn main() {
    let mut string_list = StringList::new();
    string_list.add_items(vec!["a", "b", "c", "d", "e"]);

    let a = string_list.iter().next().unwrap();
    let b = string_list.iter().next().unwrap();

    println!("ROUBADOS {} {}", a, b);
    println!("VÁRIAS LINHAS DE CÓDIGOS...");

    let c = string_list.iter().next().unwrap();
    let mut rest = vec![];

    for s in string_list.iter() {
        rest.push(s);
    }

    println!("{} {:?}", c, rest);

    string_list.reset_iterator();

    for s in string_list.iter() {
        println!("{}", s);
    }

    println!();
    string_list.set_iterator(Box::new(StringListReverseIterator::new(
        string_list.items(),
    )));

    for s in string_list.iter() {
        println!("{}", s);
    }

    println!();
    println!("PRECISA-SE RESETAR O ITERATOR");

    for s in string_list.iter() {
        println!("{}", s);
    }
}
