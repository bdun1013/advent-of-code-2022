use std::{
    collections::VecDeque,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use hashbrown::HashMap;

struct Monkey {
    items: VecDeque<i32>,
    operation: Box<dyn Fn(i32) -> i32>,
    test: Box<dyn Fn(i32) -> bool>,
    true_monkey: u8,
    false_monkey: u8,

    num_inspected_items: u32,
}

impl Monkey {
    fn new(
        items: VecDeque<i32>,
        operation: impl Fn(i32) -> i32 + 'static,
        test: impl Fn(i32) -> bool + 'static,
        true_monkey: u8,
        false_monkey: u8,
    ) -> Self {
        Self {
            items,
            operation: Box::new(operation),
            test: Box::new(test),
            true_monkey,
            false_monkey,
            num_inspected_items: 0,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut monkeys: HashMap<u8, Monkey> = HashMap::new();
    let num_rounds = 20;

    let mut lines = reader.lines();
    loop {
        let id_line: String;
        if let Some(s) = lines.next() {
            id_line = s.unwrap();
        } else {
            break;
        }

        let id = id_line[(id_line.len() - 2)..(id_line.len() - 1)]
            .parse::<u8>()
            .unwrap();

        let items_line = lines.next().unwrap().unwrap();
        let items = (&items_line[18..])
            .split(", ")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<VecDeque<i32>>();

        let operation: Box<dyn Fn(i32) -> i32>;
        let operation_line = lines.next().unwrap().unwrap();
        let operation_line_split = operation_line
            .split(" ")
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        let operator = &operation_line_split[6];
        let operand = &operation_line_split[7];

        if operator == "*" {
            if operand == "old" {
                operation = Box::new(|x: i32| x * x);
            } else {
                let operand_int = operand.parse::<i32>().unwrap();
                operation = Box::new(move |x| x * operand_int);
            }
        } else {
            if operand == "old" {
                operation = Box::new(|x: i32| x + x);
            } else {
                let operand_int = operand.parse::<i32>().unwrap();
                operation = Box::new(move |x| x + operand_int);
            }
        }

        let test_line = lines.next().unwrap().unwrap();
        let test_line_split = test_line
            .split(" ")
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        let divisible_by = (&test_line_split[5]).parse::<i32>().unwrap();
        println!("{divisible_by}");
        let test = Box::new(move |x: i32| x % divisible_by == 0);

        let true_monkey_line = lines.next().unwrap().unwrap();
        let true_monkey = (&true_monkey_line[(true_monkey_line.len() - 1)..true_monkey_line.len()])
            .parse::<u8>()
            .unwrap();

        let false_monkey_line = lines.next().unwrap().unwrap();
        let false_monkey = (&false_monkey_line
            [(false_monkey_line.len() - 1)..false_monkey_line.len()])
            .parse::<u8>()
            .unwrap();

        lines.next();

        let monkey = Monkey::new(items, operation, test, true_monkey, false_monkey);

        monkeys.insert(id, monkey);
    }

    for round in 1..=num_rounds {
        println!("-----------ROUND {}---------------", round);
        for id in 0..(monkeys.len() as u8) {
            println!("Monkey {}:", id);
            let monkey = monkeys.get(&id).unwrap();
            let true_monkey_id = monkey.true_monkey;
            let false_monkey_id = monkey.false_monkey;

            if let Some([monkey, true_monkey, false_monkey]) =
                monkeys.get_many_mut([&id, &true_monkey_id, &false_monkey_id])
            {
                for id in 0..monkey.items.len() {
                    monkey.num_inspected_items = monkey.num_inspected_items + 1;
                    let mut item = monkey.items.pop_front().unwrap();
                    println!("  Monkey inspects an item with a worry level of {item}.");
                    item = monkey.operation.as_ref()(item);
                    println!("    Worry level is multiplied by X to {}.", item);
                    item = item / 3;
                    println!(
                        "    Monkey gets bored with item. Worry level is divided by 3 to {item}."
                    );
                    let passed_test = monkey.test.as_ref()(item);
                    if passed_test {
                        println!("    Current worry level is divisible.");
                        println!(
                            "    Item with worry level {item} is thrown to monkey {}.",
                            true_monkey_id
                        );
                        true_monkey.items.push_back(item);
                    } else {
                        println!("    Current worry level is not divisible.");
                        println!(
                            "    Item with worry level {item} is thrown to monkey {}.",
                            false_monkey_id
                        );
                        false_monkey.items.push_back(item);
                    }
                }
            }
        }
    }

    for id in 0..(monkeys.len() as u8) {
        let monkey = monkeys.get(&id).unwrap();
        println!(
            "Monkey {}: {:?} {}",
            id, monkey.items, monkey.num_inspected_items
        );
    }

    Ok(())
}
