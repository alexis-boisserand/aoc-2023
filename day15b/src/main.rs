
fn hash(s: &str) -> u32 {
    s.chars()
        .fold(0u32, |value, c| ((value + u32::from(c)) * 17) % 256)
}

#[derive(Debug)]
struct Lens<'a> {
    label: &'a str,
    value: u32,
}

#[derive(Debug)]
struct LensesBox<'a> {
    lenses: Vec<Lens<'a>>,
}

fn main() {
    let line = include_str!("input.txt").lines().next().unwrap();
    let mut boxes: [LensesBox<'_>; 256] = std::array::from_fn(|_| LensesBox { lenses: Vec::new() });
    for step in line.split(",") {
        if step.ends_with('-') {
            let label = step.trim_end_matches('-');
            let box_ = &mut boxes[hash(label) as usize];
            if let Some(index) = box_.lenses.iter().position(|lens| lens.label == label) {
                box_.lenses.remove(index);
            }
        } else {
            let (label, value) = step.split_once('=').unwrap();
            let value = value.parse().unwrap();
            let box_index = hash(label);
            let box_ = &mut boxes[box_index as usize];
            if let Some(index) = box_.lenses.iter().position(|lens| lens.label == label) {
                box_.lenses[index].value = value;
            } else {
                box_.lenses.push(Lens { label, value });
            }
        }
    }

    let mut power = 0;
    for (box_index, box_) in boxes.iter().enumerate() {
        for (lens_index, lens) in box_.lenses.iter().enumerate() {
            power += (1 + box_index as u32) * (1 + lens_index as u32) * lens.value;
        }
    }
    println!("{power}");
}
