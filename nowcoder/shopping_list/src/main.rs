use std::io;

#[derive(Debug, Clone)]
struct BaseProperty {
    price: u32,
    priority: u32,
}

#[derive(Debug, Clone)]
struct CandidateObject {
    base_property: BaseProperty,
    attachments: Vec<BaseProperty>,
}

impl BaseProperty {
    fn new() -> Self {
        Self {
            price: 0,
            priority: 0,
        }
    }
}

impl CandidateObject {
    fn new() -> Self {
        Self {
            base_property: BaseProperty::new(),
            attachments: Vec::new(),
        }
    }

    fn get_all_combinations(&self) -> Vec<(u32, u32)> {
        assert!(self.attachments.len() <= 2, "Too many attached components");
        let mut result = vec![(
            self.base_property.price,
            self.base_property.price * self.base_property.priority,
        )];
        match self.attachments.len() {
            1 => result.push((
                self.base_property.price + self.attachments[0].price,
                self.base_property.price * self.base_property.priority
                    + self.attachments[0].price * self.attachments[0].priority,
            )),
            2 => {
                result.push((
                    self.base_property.price + self.attachments[0].price,
                    self.base_property.price * self.base_property.priority
                        + self.attachments[0].price * self.attachments[0].priority,
                ));
                result.push((
                    self.base_property.price + self.attachments[1].price,
                    self.base_property.price * self.base_property.priority
                        + self.attachments[1].price * self.attachments[1].priority,
                ));
                result.push((
                    self.base_property.price
                        + self.attachments[0].price
                        + self.attachments[1].price,
                    self.base_property.price * self.base_property.priority
                        + self.attachments[0].price * self.attachments[0].priority
                        + self.attachments[1].price * self.attachments[1].priority,
                ));
            }
            _ => (),
        }
        result
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

    let mut candidate_objects: Vec<Option<CandidateObject>> = Vec::new();
    candidate_objects.resize(object_num as usize, Some(CandidateObject::new()));

    for i in 0..object_num as usize {
        input_buffer.clear();
        cin.read_line(&mut input_buffer).unwrap();
        inputs = input_buffer
            .trim()
            .split(&[' ', '\n'][..])
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        if inputs[2] == 0 {
            candidate_objects[i].as_mut().unwrap().base_property.price = inputs[0] / 10;
            candidate_objects[i]
                .as_mut()
                .unwrap()
                .base_property
                .priority = inputs[1];
        } else {
            // Attached component
            candidate_objects[i] = None;
            candidate_objects[(inputs[2] - 1) as usize]
                .as_mut()
                .unwrap()
                .attachments
                .push(BaseProperty {
                    price: inputs[0] / 10,
                    priority: inputs[1],
                });
        }
    }

    let mut previous_stage = vec![0; (total_money + 1) as usize];
    let mut current_stage = vec![0; (total_money + 1) as usize];

    for candidate in candidate_objects.iter() {
        if let Some(obj) = candidate {
            let combinations = obj.get_all_combinations();
            for i in (obj.base_property.price)..total_money + 1 {
                for combination in combinations.iter() {
                    if combination.0 <= i {
                        let candidates = [
                            previous_stage[i as usize],
                            current_stage[i as usize],
                            previous_stage[(i - combination.0) as usize] + combination.1,
                        ];
                        current_stage[i as usize] = *candidates.iter().max().unwrap();
                    }
                }
            }
            previous_stage = current_stage.clone();
        }
    }
    println!("{}", current_stage.last().unwrap() * 10);
}
