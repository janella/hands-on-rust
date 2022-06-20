use std::io::stdin;

fn main() {
    let mut visitor_list = vec![
        Visitor::new("william", VisitorAction::Accept, 31),
        Visitor::new(
            "norrie",
            VisitorAction::AcceptWithNote {
                note: String::from("From out of town"),
            },
            31,
        ),
        Visitor::new("Whiskey", VisitorAction::Refuse, 7),
    ];

    println!("Hello, what's your name?");
    loop {
        let name = get_name();

        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);
        match known_visitor {
            Some(visitor) => {
                visitor.greet_visitor();
                println!("{:?}", visitor)
            }
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("Not on list, let's add you!");
                    visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0))
                }
            }
        }
    }
    println!("The list of visitors: {:#?}", visitor_list)
}

fn get_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("failed to read line");
    name.trim().to_lowercase()
}

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}
#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Come on in {}!", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome {}! (Note: {note}", self.name);
                if self.age < 18 {
                    println!("Don't serve alcohol to {}", self.name)
                }
            }
            VisitorAction::Probation => println!("{} is now a newbie member", self.name),
            _ => println!("No Admittance"),
        }
    }
}
