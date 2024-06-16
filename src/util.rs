pub type Pnt = (f32, f32);
pub type FigResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn pnt_to_buf(pnt: Pnt, width: usize) -> usize {
    let x = pnt.0.round() as usize;
    let y = pnt.1.round() as usize;

    return x + y * width;
}