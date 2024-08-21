use crate::state::brush::Dot;

pub fn get_unique_dots(dots: Vec<Dot>) -> Vec<Dot> {
    let mut unique_dots: Vec<Dot> = Vec::new();
    for dot in dots.iter() {
        if !unique_dots.contains(dot) {
            unique_dots.push(dot.clone());
        }
    }
    unique_dots
}
