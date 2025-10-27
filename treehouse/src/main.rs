use std::io::stdin;



#[derive(Debug)]
struct VisitorName {
    original: String,
    normalized: String,
}
impl VisitorName {
    fn new(input: &str) -> Self {
        Self {
            original: input.trim().to_string(),
            normalized: input.trim().to_lowercase(),
        }
    }
}

#[derive(Debug)]
struct Visitor {
    visitor_name: VisitorName,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            visitor_name: VisitorName::new(name),
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn what_is_your_name() -> VisitorName {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Failed to read line");
  
    VisitorName::new(&name)
//    let trimmed = your_name.trim().to_string();
//    let lowercase = trimmed.to_lowercase();
//    (trimmed, lowercase)
}

fn main() {
    //let visitor_list = ["bert", "steve", "fred"];
    // let visitor_list = Vec::new();
    // visitor_list.push(Vistir::new(...));
    let mut visitor_list = vec![
        Visitor::new("Bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("Steve", "Hi Steve, your milk is in the fridge."),
        Visitor::new("Fred", "Wow, who invited Fred?"),
    ];
    
    loop {
        println!("Hello, what's your name?");
        let visitor_name = what_is_your_name();

        if visitor_name.original.is_empty() {
            break;
        }
        //let mut allow_them_in = false;
        //for visitor in &visitor_list {
        //    if visitor == &name {
        //        allow_them_in = true;
        //    }
        //}
        let known_visitor = visitor_list
            .iter()
            .find(|visitor| visitor.visitor_name.normalized == visitor_name.normalized);
        println!("{:#?}", known_visitor);
        
        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                visitor_list.push(Visitor::new(&visitor_name.original, "New friend"));
                println!("{} is not on the visitor list.", visitor_name.original);
            }
        }
        //if allow_them_in {
        //    println!("Welcome to the Treehouse, {}", name);
        //} else {
        //    println!("Sorry, you aren't on the list/");
        //}
    }
}
