use core::fmt;
use std::{io, num::ParseIntError};

#[derive(Debug)]
pub enum Error {
    IO(String),
    Parse(String),
    InvalidInput,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Error::InvalidInput => "Invalid input, please select a number".to_string(),
            Error::IO(e) => e.to_string(),
            Error::Parse(e) => e.to_string(),
        };
        write!(f, "Error: {}", message)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IO(value.to_string())
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::Parse(value.to_string())
    }
}

#[derive(Debug)]
pub struct Step<T>
where
    T: Clone,
{
    title: String,
    items: Vec<StepItem<T>>,
    error: Option<Error>,
}

impl<T> fmt::Display for Step<T>
where
    T: Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let items = &self.items;
        let items_text = items
            .iter()
            .enumerate()
            .fold("".to_string(), |acc, (index, item)| {
                format!("{}\n{}) {}", acc, index + 1, item.title)
            });
        let error_text: String = match &self.error {
            Some(v) => format!("\n{}", v.to_string()),
            None => "".to_string(),
        };
        write!(
            f,
            "\x1B[2J\x1B[1;1H{}
------------------------------------- 
enter q to quit
{}
{}",
            self.title, items_text, error_text,
        )
    }
}

#[derive(Debug, Clone)]
pub struct StepItem<T>
where
    T: Clone,
{
    pub title: String,
    pub value: T,
}

#[derive(Debug)]
pub struct Selection<T>
where
    T: Clone,
{
    pub step: usize,
    pub selected: StepItem<T>,
}

#[derive(Debug)]
pub struct Journey<T>
where
    T: Clone,
{
    current: usize,
    steps: Vec<Step<T>>,
    selected_items: Vec<usize>,
}

impl<T> Journey<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Journey {
            current: 0,
            steps: Vec::new(),
            selected_items: Vec::new(),
        }
    }

    pub fn step<I>(&mut self, title: String, items: Vec<StepItem<T>>) {
        self.steps.push(Step {
            title,
            items,
            error: None,
        });
    }
}

impl<T> Iterator for Journey<T>
where
    T: Clone,
{
    type Item = Selection<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.steps.len() {
            return None;
        }

        let mut step = &mut self.steps[self.current];
        let selected_index = loop {
            println!("{}", step);
            match get_user_action(step.items.len()) {
                Ok(UserAction::Select(v)) => break v,
                Ok(UserAction::Quit) => return None,
                Err(e) => {
                    step.error = Some(e);
                }
            }
        };
        self.selected_items.push(selected_index);
        let selection = Selection {
            step: self.current,
            selected: step.items[selected_index].clone(),
        };
        self.current += 1;
        Some(selection)
    }
}

enum UserAction {
    Select(usize),
    Quit,
}

fn get_user_action(max: usize) -> Result<UserAction, Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();
    if input == "q" {
        return Ok(UserAction::Quit);
    }
    match input {
        "q" => Ok(UserAction::Quit),
        _ => {
            let selected: usize = input.parse()?;
            if selected == 0 {
                return Err(Error::InvalidInput);
            }
            let selected = selected - 1;
            if selected >= max {
                return Err(Error::InvalidInput);
            }
            Ok(UserAction::Select(selected))
        }
    }
}
