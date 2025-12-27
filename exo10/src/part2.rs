use nalgebra::{DMatrix, DVector};
use regex_macro::{LazyRegex, lazy_regex};
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;

static RE_FIRST_GROUP: LazyRegex = lazy_regex!(r"\[([.#]*)]");
static RE_SECOND_GROUP: LazyRegex = lazy_regex!(r"\((\d+[,\d+]*)\)");
static RE_THIRD_GROUP: LazyRegex = lazy_regex!(r"\{(\d+[,\d+]*)}");

struct Button {
    indicator: Vec<usize>,
}

impl Button {
    fn new(indicator: Vec<usize>) -> Self {
        Self { indicator }
    }
}

impl TryFrom<&str> for Button {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut result = Vec::new();
        for x in value.split(',') {
            result.push(x.parse::<usize>().map_err(|_| ())?);
        }
        Ok(Button::new(result))
    }
}

struct Machine {
    indicators: Vec<f64>,
    joltage_indicators: Vec<f64>,
    buttons: Vec<Button>,
}

impl Machine {
    fn new(joltage_indicators: Vec<f64>, buttons: Vec<Button>) -> Self {
        Self {
            indicators: vec![0f64; joltage_indicators.len()],
            joltage_indicators,
            buttons,
        }
    }

    fn press(&mut self, button: usize) {
        let button = self.buttons.get(button);
        if let Some(button) = button {
            for i in &button.indicator {
                self.indicators[*i] = self.indicators[*i] + 1f64;
            }
        }
    }
    fn press_many(&mut self, button: usize, number: f64) {
        let button = self.buttons.get(button);
        if let Some(button) = button {
            for i in &button.indicator {
                self.indicators[*i] = self.indicators[*i] + number;
            }
        }
    }

    fn is_joltaged_ok(&self) -> bool {
        self.joltage_indicators == self.indicators
    }

    fn reset(&mut self) {
        self.indicators = vec![0f64; self.indicators.len()];
    }
}

impl Display for Machine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for indicator in &self.indicators {
            write!(f, "{}", indicator)?;
        }
        write!(f, "{:?}", self.joltage_indicators)?;

        Ok(())
    }
}

impl TryFrom<&str> for Machine {
    type Error = ();

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let mut indicators = Vec::new();
        let mut buttons = Vec::new();

        let mut first_group = 0;

        for current_line in line.split_whitespace() {
            if first_group == 0 {
                if let Some((_, [_])) = RE_FIRST_GROUP.captures(current_line).map(|c| c.extract()) {
                    first_group = 1;
                }
            } else if first_group == 1 {
                if let Some((_, [result])) =
                    RE_SECOND_GROUP.captures(current_line).map(|c| c.extract())
                {
                    let button = Button::try_from(result)?;
                    buttons.push(button);
                } else {
                    if let Some((_, [result])) =
                        RE_THIRD_GROUP.captures(current_line).map(|c| c.extract())
                    {
                        for number in result.split(",") {
                            indicators.push(number.parse::<f64>().unwrap_or(0f64));
                        }
                    }
                }
            }
        }

        let machine = Machine::new(indicators, buttons);
        Ok(machine)
    }
}

fn main() -> Result<(), ()> {
    let mut result = Vec::new();

    let string = read_to_string("exo10/example/example2.txt").unwrap();

    for line in string.lines() {
        let machine = Machine::try_from(line)?;
        println!("{line}");
        let matrix = DMatrix::from_fn(
            machine.joltage_indicators.len(),
            machine.buttons.len(),
            |i, j| {
                if machine
                    .buttons
                    .get(j)
                    .filter(|b| b.indicator.contains(&i))
                    .iter()
                    .count()
                    == 1
                {
                    1f64
                } else {
                    0f64
                }
            },
        );
        let b = DVector::from_column_slice(&machine.joltage_indicators[0..]);
        //println!("{}", &matrix);
        //println!("{}", b);

        let lu = matrix.lu();

        //println!("L matrix is {}", lu.l());
        //println!("U matrix is {}", lu.u());

        if machine.buttons.len() == machine.joltage_indicators.len() {
            let option = lu.solve(&b);
            println!(
                "la solution est: {:?}",
                option.map(|x| x.iter().sum::<f64>())
            );
            println!("{}", lu.determinant());
        };

        println!(
            "nombre de boutons (nb inconnus): {}, nombre de voltages (nb d'équation): {}",
            machine.buttons.len(),
            machine.joltage_indicators.len()
        )

        /*        for combo in (0..10).combinations_with_replacement(machine.buttons.len()- machine.joltage_indicators.len()) {
                    machine.reset();
                    for (i, &j) in combo.iter().enumerate() {
                        machine.press(i, j as isize);
                    }
                    if machine.is_joltaged_ok() {
                        result.push(combo.iter().sum::<usize>());
                        println!("machine ended {} for line {line}", combo.iter().sum::<usize>());
                        break;
                    } else {
                        println!("machine not ended {:?}", combo);
                    }
                }
        */
    }

    println!("{}", result.iter().sum::<usize>());
    println!("{:?}", result);

    Ok(())
}
