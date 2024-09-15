// Problem 1:
/* You are tasked with implementing a library management system using Rust.
Your goal is to design a program that can handle books and magazines.
To fulfill the requirements, follow the steps below:

Create a structure called Item with the following fields:
id: An integer representing the unique identifier of the item.
title: A string representing the title of the item.
year: An integer representing the publication year of the item.
type: an enumeration type. The details are coming below.

Create an enumeration called ItemType with two variants:
Book: Represents a book.
Magazine: Represents a magazine.

Implement a function called display_item_info() that takes an Item as input
and displays its information. The function should output
the item's ID, title, publication year, and type (book or magazine).
*/

enum Type {
    Book,
    Magazine,
}

impl Type {
    fn to_string(&self) -> &str {
        match self {
            Type::Book => "BOOK",
            Type::Magazine => "MAGAZINE",
        }
    }
}

struct Item {
    id: i32,
    title: String,
    year: i32,
    _type: Type,
}

impl Item {
    fn display_item_info(&self) {
        println!(
            "Item's id: {}, title: {}, year: {}, type: {}",
            self.id,
            self.title,
            self.year,
            self._type.to_string()
        );
    }
}

fn main() {
    let item = Item {
        id: 10,
        title: String::from("Calçado"),
        year: 2010,
        _type: Type::Book,
    };
    item.display_item_info();
}
