use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();

    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    
    your_name
        .trim()
        .to_lowercase()
}

fn main() {
    //let visitor_list = ["bert", "steve", "fred"];
    // let visitor_list = Vec::new();
    // visitor_list.push(Vistir::new(...));
    let mut visitor_list = vec![
        Visitor::new("bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("steve", "Hi Steve, your milk is in the fridge."),
        Visitor::new("fred", "Wow, who invited Fred?"),
    ];
    
    loop {
        println!("Hello, what's your name?");
        let name = what_is_your_name();

        //let mut allow_them_in = false;
        //for visitor in &visitor_list {
        //    if visitor == &name {
        //        allow_them_in = true;
        //    }
        //}
        let known_visitor = visitor_list
            .iter()
            .find(|visitor| visitor.name == name);
        println!("{:?}", known_visitor);
        
        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not on the visitor list.", name);
                    visitor_list.push(Visitor::new(&name, "New friend"));
                }
            }
        }
        //if allow_them_in {
        //    println!("Welcome to the Treehouse, {}", name);
        //} else {
        //    println!("Sorry, you aren't on the list/");
        //}
    }
}
