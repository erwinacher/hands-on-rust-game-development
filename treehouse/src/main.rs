use std::io::stdin;


#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote{ note: String },
    Refuse,
    Probation,
}


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
impl PartialEq for VisitorName {
    fn eq(&self, other: &Self) -> bool {
        self.normalized == other.normalized
    }
}


#[derive(Debug)]
struct Visitor {
    visitor_name: VisitorName,
    //greeting: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            visitor_name: VisitorName::new(name),
            //greeting: greeting.to_string(),
            action,
            age,
        }
    }

    fn greet_visitor(&self) {
        //println!("{}", self.greeting);
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the treehouse, {}", self.visitor_name.original),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the treehouse, {}", self.visitor_name.original);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alchol to {}", self.visitor_name.original);
                }          
            }
            VisitorAction::Probation => println!("{} is now a probationary member", self.visitor_name.original),
            VisitorAction::Refuse => println!("Do not allow {} in!", self.visitor_name.original),
        }
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
        Visitor::new("Bert", VisitorAction::Accept, 45),
        Visitor::new("Steve", VisitorAction::AcceptWithNote { note: String::from("Lactose-free milk is in the fridge") }, 15),
        Visitor::new("Fred", VisitorAction::Refuse, 30),
    ];
    
    loop {
        println!("Hello, what's your name?");
        let visitor_name = what_is_your_name();

        if visitor_name.original.is_empty() {
            println!("{:#?}", visitor_list);
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
            .find(|v| v.visitor_name == visitor_name);
        println!("{:#?}", known_visitor);
        
        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                visitor_list.push(Visitor::new(&visitor_name.original, VisitorAction::Probation, 0));
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
