#[derive(Debug, Clone)]
pub struct Settings {
    pub grid_size: usize,
    pub min_solutions: usize,
    pub side_types: u8,
}

pub struct ImageCrop {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}