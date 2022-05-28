// Need to include rand dependency in .toml
use rand::Rng;
use std::io::{stdin,stdout,Write};
use std::string::ToString;


#[derive(Copy, Clone)]
pub enum aChoice
{
    cNone,
    cRock,
    cPaper,
    cScissors,
}

trait WriteName
{
    fn write(choice : aChoice);
}

impl WriteName for aChoice
{
    fn write(choice: aChoice)
    {
        let nameStr : &str;
        match choice
        {
            aChoice::cNone=> nameStr = "None",
            aChoice::cRock=> nameStr = "Rock",
            aChoice::cScissors=> nameStr = "Scissors",
            aChoice::cPaper=> nameStr = "Paper",
            _=> nameStr = "_",
        }
        println!("{}", nameStr);
    }
}

// Struct - member data (private/public?)
pub struct BotRand
{
    // name of member: member type    
    pub choice_: aChoice,
}

pub struct BotRepeat
{
    choice_: aChoice,
}

// Implement : Class functions go here
impl BotRand
{    
    pub fn SetChoice(&mut self, chz : aChoice)
    {
        //aChoice::write(chz);
        self.choice_ = chz;
    }    
}

// Traits - used to virtualize.
// Overloaded functions, interface. 
pub trait MakeChoice
{
    // Method signatures that define behavior of types
    //      that implement MakeChoice
    fn GetChoice(&self) -> aChoice; // This is the declaration.
                                    // Each type must provide
                                    //  its own implementation.
    fn PrintChoice(&self);
}

// Implementation of MakeChoice for BotRand type
impl MakeChoice for BotRand
{
    // Access specifier is implied pub 
    fn GetChoice(&self) -> aChoice
    {
        // mutable type - type can be changed (wooooah also scary)
        // random init for rand thread
        let mut rand = rand::thread_rng();
                                // double dot (..) -> range of data
        let rand0: i16 = rand.gen_range(0..3);
        //let randStr: &str = to_string(rand0);
        // println!("{}" , rand0);
        match rand0
        {
            0=>aChoice::cRock,
            1=>aChoice::cPaper,
            2=>aChoice::cScissors,
            _=>aChoice::cNone,
        }
        
    }

    // give function access to self (this)
    fn PrintChoice(&self)
    {           
        // Rust case statement
        match self.choice_
        {
            aChoice::cRock=>println!("Rock"),
            aChoice::cPaper=>println!("Paper"),
            aChoice::cScissors=>println!("Scissors"),
            // "default"
            _=>println!("no choice"), 
        }
    }


}

fn GetInput() -> aChoice
{    
    let mut s = String::new();
    print!("Please pick: Rock , Paper, Scissors: \n");
    // Flushing out - why? *Ari - probs just best practice to clear. 
    let _ = stdout().flush();
                                // Expect - error messaging
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    // 
    if let Some('\n') = s.chars().next_back() 
    {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() 
    {
        s.pop();
    }
    use crate::aChoice::cRock;
    return cRock;
}
//////////////
// Notes
//////////////////////////////////////////
// Owned / String concat
/*
        // "let" - modern type declaration, analogous to "auto"
        // this should deduce string type
        let b = "z";
        let a : &str = "z";
        // owned - by the scope. Aka a reference / ptr
        // borrowed - pass by value, from outside the scope.
        // String concatenation requires a "base" reference 
        //      to build from. 
        
        if(b == a)
        {
            println!("{}" , a.to_owned() + " == " 
                          + b + "\n");
        }
        else
        {
            println!("{}" , a.to_owned() + " != " 
                          + b + "\n");
        }
        */