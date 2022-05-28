// ! -> inner attribute
#![allow(warnings, unused)]

mod Bot;
use Bot::aChoice;
use Bot::BotRand;
use Bot::MakeChoice;
fn main() 
{
    let mut bot0 = BotRand { choice_: aChoice::cNone};
    for i in 0..10
    {
        bot0.SetChoice(bot0.GetChoice());
        bot0.PrintChoice();
    }
    
    
}


// Option types - Some and contains a value, or None and does not.
fn divide(numerator: f64, denominator: f64) -> Option<f64> 
{
    // return None if D is 0  
    if denominator == 0.0 
    {        
        None
    }
    else 
    {
        Some(numerator / denominator)
    }
    
}
