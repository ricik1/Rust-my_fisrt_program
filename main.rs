//add library so we can input from the command line
use std::io;
use colored::*;



//added use to bring in color

// Add out main function, this is the enrty point into our program

fn main(){
    
    
    //print a title to the console
    //let msg will allow me to change the color of my text and style, the header and all questions will be yellow
// I used an extra \t in the middle of my next line to be sure both lines are center to each other
    let msg ="\n\n\t ***Guess the Rust Keyword from the given definition***\n\n\t\t ***A total of 4 questions, one guess each***\n\n".yellow();
   println!("{msg}");
    //create a string to hold the users answer
    // let is used to create a new var in this case a string
    //mut is used to make var mutable(changeable or read only)
    // user_answer is the name of the Var
    let mut user_answer = String::new();

    //print the definition to the word allowing the user to look up the definition and get the corrent word.
    // the fisrt correct answer is "break"- break will stop the loop immediately and continue with the code after the loop
    let msg: ColoredString =  "\" exit a loop immediately.\" - is for what keyword? \n".yellow();
    println!("{msg}");

    // get the user input and store itin the user_answer var

    io::stdin().read_line(&mut user_answer).expect("Failed to read line");

    //TRIM the user input remove any white spaces.
user_answer = user_answer.trim().to_string();

//make the user input all lowercase
user_answer = user_answer.to_lowercase();
// check the user input against the correct awnser
if user_answer == "break" {
    // the user got the correct answer
    let msg: ColoredString ="\n\nYou are correct! A rust wizard You Are!\n\n".bright_purple();
    println!("{msg}");
//msg printed for doing it right will be bright purple and wrong will be red, for every question
}
else{
    //I want to print you are incorrect, next question
    let msg: ColoredString ="\n\nYou are incorecct, try the next question\n\n".red(); 
    println!("{msg}");  
}



    //create a string to hold the users answer
    // let is used to create a new var in this case a string
    //mut is used to make var mutable(changeable or read only)
    // user_answer1 is the name of the Var for my next question
    let mut user_answer1 = String::new();
    //moving on to the next quesiton
    //print the definition to the word allowing the user to look up the definition and get the corrent word.
    // the fisrt correct answer is "extern"- link an external function or variable,The extern keyword is used
    // in two places in Rust. One is in conjunction with the crate keyword to make your Rust code aware of other Rust crates in your project
    //let msg will allow me to change the color of my text and style, the header and all questions will be yellow
    let msg: ColoredString ="\"link an external function or variable .\" - is for what keyword? \n".yellow();
    println!("{msg}"); 

    // get the user input and store itin the user_answer var

io::stdin().read_line(&mut user_answer1).expect("Failed to read line");

    //TRIM the user input remove any white spaces.
user_answer1 = user_answer1.trim().to_string();

    //make the user input all lowercase
user_answer1 = user_answer1.to_lowercase();
    // check the user input against the correct awnser

if user_answer1 == "extern" {
    // the user got the correct answer
    let msg: ColoredString ="\n\nYou care correct! A rust wizard You Are!\n\n".bright_purple();
    println!("{msg}"); 
    //msg printed for doing it right will be bright purple and wrong will be red, for every question
}
else{
    //I want to print you are incorrect, next question
    let msg: ColoredString ="\n\nYou are incorecct, try the next question\n\n".red();   
    println!("{msg}"); 
}



    //create a string to hold the users answer
    // let is used to create a new var in this case a string
    //mut is used to make var mutable(changeable or read only)
    // user_answer2 is the name of the Var for my next question
    let mut user_answer2 = String::new();
    //moving on to the next quesiton
    //print the definition to the word allowing the user to look up the definition and get the corrent word.
    // the fisrt correct answer is "await"- suspend execution until the result of a Future is ready,
    //.await are special pieces of Rust syntax that make it possible to yield control of the current
    // thread rather than blocking, allowing other code to make progress while waiting
    //let msg will allow me to change the color of my text and style, the header and all questions will be yellow
    let msg: ColoredString = "\" suspend execution until the result of a Future is ready.\" - is for what keyword? \n".yellow();
    println!("{msg}"); 


    // get the user input and store itin the user_answer var

io::stdin().read_line(&mut user_answer2).expect("Failed to read line");

    //TRIM the user input remove any white spaces.
user_answer2 = user_answer2.trim().to_string();

    //make the user input all lowercase
user_answer2 = user_answer2.to_lowercase();
// check the user input against the correct awnser

if user_answer2 == "await" {
    // the user got the correct answer
    let msg: ColoredString ="\n\nYou care correct! A rust wizard You Are!\n\n".bright_purple();
    println!("{msg}"); 
    //msg printed for doing it right will be bright purple and wrong will be red, for every question
}
else{
    //I want to print you are incorrect, next question
    let msg: ColoredString ="\n\nYou are incorecct, try the next question\n\n".red();  
    println!("{msg}"); 
}



//create a string to hold the users answer
    // let is used to create a new var in this case a string
    //mut is used to make var mutable(changeable or read only)
    // user_answer3 is the name of the Var for my next question
    let mut user_answer3 = String::new();
//moving on to the next quesiton
//print the definition to the word allowing the user to look up the definition and get the corrent word.
    // the fisrt correct answer is "use"- bring symbols into scope
    //Usually a use keyword is used to shorten the path required to refer to a module item. 
    //The keyword may appear in modules, blocks and even functions, usually at the top.
    //let msg will allow me to change the color of my text and style, the header and all questions will be yellow
    let msg: ColoredString ="\"bring symbols into scope .\" - is for what keyword? \n".yellow();
    println!("{msg}"); 

// get the user input and store itin the user_answer var

io::stdin().read_line(&mut user_answer3).expect("Failed to read line");

//TRIM the user input remove any white spaces.
user_answer3 = user_answer3.trim().to_string();

//make the user input all lowercase
user_answer3 = user_answer3.to_lowercase();
// check the user input against the correct awnser

if user_answer3 == "use" {
    // the user got the correct answer
    let msg: ColoredString = "\n\nYou care correct! A rust wizard You Are!\n\n".bright_purple();// this is the last question which will follow with the goodbye when they get it right
    println!("{msg}"); 
    let msg: ColoredString = "\n\n\t ***Thank you for playing, goodbye***\n\n".bright_magenta();//this is the end of the game and I 
    println!("{msg}"); 
    //msg printed for doing it right will be bright purple and wrong will be red, for every question
    // I added the goodbye msg for both if they get that last question right or if they get if wrong.
}
else{
    //I want to print you are incorrect, next question
    let msg: ColoredString ="\n\n\tYou are incorecct, GAME OVER!\n\n".red();  // this was the last question and notes game over if they miss it 
    println!("{msg}"); 
    // end the program say goodbye
    let msg: ColoredString = "\n\n\t ***Thank you for playing, goodbye***\n\n".bright_magenta();//this is the end of the game and I 
    println!("{msg}"); 
    //msg printed for doing it right will be bright purple and wrong will be red, for every question

    
}
}





