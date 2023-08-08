




//adaptee aka services


pub struct SpecificTarget;

impl SpecificTarget {
    fn specific_request(&self) -> String {
        return "Specific request from specific target...".to_string();
    }
}


// adapter 

pub struct TargetAdapter {
    adaptee: SpecificTarget
}

impl TargetAdapter {
    pub fn new(adaptee: SpecificTarget) -> Self {
        Self {adaptee: adaptee}
    } 
}

impl Target for TargetAdapter {
    fn request(&self) -> String {
        self.adaptee.specific_request()
    }
}


// Compatible target 
pub trait Target {
    fn request(&self) -> String;
}

pub struct OrdinaryTarget;

impl Target for OrdinaryTarget {
    fn request(&self) -> String {
        return "Ordinary target".to_string();
    }
}


// Main logic

fn call(target: impl Target) {
    println!("'{}'", target.request());
}

fn main() {
    let target = OrdinaryTarget;

    print!("A compatible target can be directly called: ");
    call(target);

    let adaptee = SpecificTarget;

    println!(
        "Adaptee is incompatible with client: '{}'",
        adaptee.specific_request()
    );

    let adapter = TargetAdapter::new(adaptee);

    print!("But with adapter client can call its method: ");
    call(adapter);
}
