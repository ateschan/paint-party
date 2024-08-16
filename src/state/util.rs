use crate::state::brush::Dot;
pub fn get_unique_dots(dots: &mut Vec<Dot>) -> Vec<Dot> {
    let mut result: Vec<Dot> = Vec::new();
    for dot in dots {
        if !result.contains(dot) {
            result.push(dot.clone());
        }
    }
    result
}
