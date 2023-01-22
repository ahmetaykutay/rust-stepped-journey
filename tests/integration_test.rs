use std::io;
use stepped_journey::{Journey, StepItem};

#[test]
fn run() {
    let mut j: Journey<i32> = Journey::new();
    j.step::<i32>(
        "first step".to_string(),
        vec![
            StepItem {
                title: "item 1".to_string(),
                value: 10,
            },
            StepItem {
                title: "item 2".to_string(),
                value: 11,
            },
        ],
    );

    j.step::<i32>(
        "second step".to_string(),
        vec![
            StepItem {
                title: "second item 1".to_string(),
                value: 20,
            },
            StepItem {
                title: "second item 2".to_string(),
                value: 21,
            },
        ],
    );

    for selection in j {
        println!(
            "step: {} | selected: {} | selected value: {}",
            selection.step, selection.selected.title, selection.selected.value
        );
        press_any();
    }
}

fn press_any() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
}
