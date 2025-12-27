use Indicator::Off;
use Indicator::On;
use itertools::Itertools;
use regex_macro::{LazyRegex, lazy_regex};
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;

static RE_FIRST_GROUP: LazyRegex = lazy_regex!(r"\[([.#]*)]");
static RE_SECOND_GROUP: LazyRegex = lazy_regex!(r"\((\d+[,\d+]*)\)");

#[derive(PartialEq, Clone)]
enum Indicator {
    On,
    Off,
}

impl TryFrom<char> for Indicator {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Off),
            '#' => Ok(On),
            _ => Err(()),
        }
    }
}

impl Indicator {
    fn switch(&mut self) {
        *self = match self {
            On => Off,
            Off => On,
        }
    }
}

impl Display for Indicator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                On => "🟢",
                Off => "🔴",
            }
        )
    }
}

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
    indicators: Vec<Indicator>,
    started_indicators: Vec<Indicator>,
    buttons: Vec<Button>,
}

impl Machine {
    fn new(started_indicators: Vec<Indicator>, buttons: Vec<Button>) -> Self {
        Self {
            indicators: vec![Off; started_indicators.len()],
            started_indicators,
            buttons,
        }
    }

    fn press(&mut self, button: usize) {
        let button = self.buttons.get(button);
        if let Some(button) = button {
            for i in &button.indicator {
                self.indicators[*i].switch();
            }
        }
    }

    fn is_started(&self) -> bool {
        self.started_indicators == self.indicators
    }

    fn reset(&mut self) {
        self.indicators = vec![Off; self.started_indicators.len()];
    }
}

impl Display for Machine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for indicator in &self.indicators {
            write!(f, "{}", indicator)?;
        }
        Ok(())
    }
}

impl TryFrom<&str> for Machine {
    type Error = ();

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let mut indicators = Vec::new();
        let mut buttons = Vec::new();

        let mut first_group = true;

        for current_line in line.split_whitespace() {
            if first_group {
                if let Some((_, [result])) =
                    RE_FIRST_GROUP.captures(current_line).map(|c| c.extract())
                {
                    for char in result.chars() {
                        let indicator = Indicator::try_from(char)?;
                        indicators.push(indicator);
                    }
                    first_group = false;
                }
            } else {
                if let Some((_, [result])) =
                    RE_SECOND_GROUP.captures(current_line).map(|c| c.extract())
                {
                    let button = Button::try_from(result)?;
                    buttons.push(button);
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
        let mut machine = Machine::try_from(line)?;
        'end: for i in 1..10 {
            for combo in (0..machine.buttons.len()).combinations(i) {
                machine.reset();
                for &i in &combo {
                    machine.press(i);
                }
                if machine.is_started() {
                    result.push(i);
                    break 'end;
                }
            }
        }
    }

    println!("{:?}", result.iter().sum::<usize>());

    Ok(())
}
