extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::iterators::{Pair,Pairs};

#[derive(Parser)]
#[grammar = "dir.pest"]
pub struct DIRParser;

fn main() {
    let successful_parse = DIRParser::parse(Rule::statement, " (up[10]) :: 8").expect("unsuccessful parse")
        .next().unwrap();
    process(successful_parse);
}

fn process(pairs: Pair<Rule>) {
    match pairs.as_rule() {
        Rule::statement => {
            process(pairs.into_inner().next().unwrap())
        } //then do action}
        Rule::expressions => {
            let pairs = pairs.into_inner();
            for pair in pairs {
                //add to que?
                process(pair)
            }
        }
        Rule::expression => {
            process(pairs.into_inner().next().unwrap());
        }
        Rule::if_ => {
            let mut pairs = pairs.into_inner();
            let mut cond = pairs.next().unwrap().into_inner();
            let t = pairs.next().unwrap();
            if comp(&mut cond) {
                process(t);
            } else if let Some(else_) = pairs.next() {
                process(else_);
            }
        }
        Rule::while_ => {
            let mut pairs = pairs.into_inner();
            let mut cond = pairs.next().unwrap().into_inner();
            let exp = pairs.next().unwrap();
            while comp(&mut cond) {
                process(exp.clone());
            }
        }
        Rule::repeat => {
            let mut pairs = pairs.into_inner();
            let exp = pairs.next().unwrap();
            let times = pairs.next().unwrap().as_str().parse::<u32>().unwrap();
            for _ in 0..times {
                process(exp.clone());
            }
        }
        Rule::actions => {

        }
        _ => { println!("{:?}", pairs.as_rule()) }
    }
}

fn comp( pairs: &mut Pairs<Rule>) -> bool{
    let a = goodnum(pairs.next().unwrap());
    let comp = pairs.next().unwrap().into_inner().next().unwrap();
    let b = goodnum(pairs.next().unwrap());
        print!("{:?}",comp);
        match comp.as_rule() {
            Rule::gt => return a > b,
            Rule::lt => return a < b,
            Rule::ge => return a >= b,
            Rule::le => return a <= b,
            Rule::eq => return a == b,
            _ => unreachable!()
        }
}

fn goodnum( pair: Pair<Rule>) -> f64{
    match pair.as_rule(){
        Rule :: num => return pair.as_str().parse::<f64>().unwrap(),
        // Rule :: xspot => return func
        // Rule :: yspot => return func2
        _ => {
            println!("hello");
            unreachable!();
        }
    }

}