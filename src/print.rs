pub fn run(){
    //print to console
    println!("Hello from the print.rs file");
   //basic formatting
    println!("{} is from {}", "Brad", "Skateboarding");
   //Positional Arguments
   println!("{0} is form {1} and {0} likes to {2}","Brad","Skateboarding", "Code");
   //Named Arguments
   println!("{name} likes to play {activity}", name="Kausa", activity="video games");
   //PlaceHolder traits 
   println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);
}