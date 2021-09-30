use std::io;

#[derive(Debug, Clone)]
struct MainComponent {
    price: u32,
    priority: u32,
    attached_components: Vec<AttachedComponent>,
}

#[derive(Debug, Clone)]
struct AttachedComponent {
    price: u32,
    priority: u32,
}

impl MainComponent {
    fn get_all_combinations(&self) -> Vec<(u32, u32)> {
        assert!(
            self.attached_components.len() <= 2,
            "Too many attached components"
        );
        match self.attached_components.len() {
            0 => vec![(self.price, self.priority)],
            1 => vec![
                (self.price, self.priority),
                (
                    self.price + self.attached_components[0].price,
                    self.priority + self.attached_components[0].priority,
                ),
            ],
            _ => vec![
                (self.price, self.priority),
                (
                    self.price + self.attached_components[0].price,
                    self.priority + self.attached_components[0].priority,
                ),
                (
                    self.price + self.attached_components[1].price,
                    self.priority + self.attached_components[1].priority,
                ),
                (
                    self.price
                        + self.attached_components[0].price
                        + self.attached_components[1].price,
                    self.priority
                        + self.attached_components[0].priority
                        + self.attached_components[1].priority,
                ),
            ],
        }
    }
}

fn main() {
    let cin = io::stdin();
    let mut input_buffer = String::new();
    cin.read_line(&mut input_buffer).unwrap();
    let mut inputs: Vec<u32> = input_buffer
        .trim()
        .split(&[' ', '\n'][..])
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let total_money = inputs[0] / 10;
    let object_num = inputs[1];

    let mut main_components: Vec<MainComponent> = Vec::new();
    main_components.reserve(object_num as usize);

    for i in 0..object_num as usize {
        input_buffer.clear();
        cin.read_line(&mut input_buffer).unwrap();
        inputs = input_buffer
            .trim()
            .split(&[' ', '\n'][..])
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        if inputs[2] == 0 {
            // Main component
            if i >= main_components.len() {
                main_components.push(MainComponent {
                    price: inputs[0] / 10,
                    priority: inputs[1],
                    attached_components: Vec::new(),
                })
            } else {
                main_components[i].price = inputs[0] / 10;
                main_components[i].priority = inputs[1];
            }
        } else {
            // Attached component
            if inputs[2] as usize > main_components.len() {
                main_components.resize(
                    inputs[2] as usize,
                    MainComponent {
                        price: 0,
                        priority: 0,
                        attached_components: Vec::new(),
                    },
                );
            }
            main_components[(inputs[2] - 1) as usize]
                .attached_components
                .push(AttachedComponent {
                    price: inputs[0] / 10,
                    priority: inputs[1],
                })
        }
    }

    println!("{:?}", main_components);

    let mut previous_stage = vec![0; total_money as usize];
    let mut current_stage = vec![0; total_money as usize];

    for main_component in main_components.iter() {
        for i in (main_component.price - 1)..total_money {
            let combinations = main_component.get_all_combinations();
            for combination in combinations {
                if combination.0 - 1 <= i {
                    let candidates = [
                        previous_stage[i as usize],
                        current_stage[i as usize],
                        previous_stage[(i + 1 - combination.0) as usize]
                            + combination.0 * combination.1,
                    ];
                    current_stage[i as usize] = *candidates.iter().max().unwrap();
                }
            }
        }
        previous_stage = current_stage.clone();
    }
    println!("{}", current_stage.last().unwrap() * 10);
}
