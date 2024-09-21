use std::{collections::HashSet, fmt::Display};

use crate::{color::Color, length::Length};

pub struct Rect(HashSet<RectProps>);

impl Rect {
    pub fn x(mut self, length: Length) -> Self {
        self.0.insert(RectProps::X(length));
        self
    }
}

impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut props = self.0.iter().fold(String::from("<rect"), |acc, x| {
            let i = match x {
                RectProps::X(x) => format!("x={}", x),
                RectProps::Y(x) => format!("y={}", x),
                RectProps::Rx(x) => format!("rx={}", x),
                RectProps::Ry(x) => format!("ry={}", x),
                RectProps::Width(x) => format!("width={}", x),
                RectProps::Height(x) => format!("height={}", x),
                RectProps::Stroke(x) => format!("stroke={}", x),
                RectProps::Fill(x) => format!("fill={}", x),
                RectProps::StrokeWidth(x) => format!("stroke-width={}", x),
            };
            acc + " " + &i
        });
        props.push_str("/>");
        write!(f, "{props}")
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum RectProps {
    X(Length),
    Y(Length),
    Width(Length),
    Height(Length),
    Rx(Length),
    Ry(Length),
    Stroke(Color),
    Fill(Color),
    StrokeWidth(Length),
}
