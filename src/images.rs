use image::{DynamicImage, RgbaImage, imageops::overlay, imageops};
use crate::settings::ImageCrop;
use crate::jigsaw::PieceState;
pub struct Images {
    pub piece_blank: RgbaImage,
    pub piece_edge: RgbaImage,
    pub piece_edges: [RgbaImage; 6],
}

pub fn load_images() -> Images {
    Images {
        piece_blank: image::open("./images/piece_blank.png").unwrap().to_rgba8(),
        piece_edge: image::open("./images/piece_edge.png").unwrap().to_rgba8(),
        piece_edges: [
            image::open("./images/piece_edge_1.png").unwrap().to_rgba8(),
            image::open("./images/piece_edge_2.png").unwrap().to_rgba8(),
            image::open("./images/piece_edge_3.png").unwrap().to_rgba8(),
            image::open("./images/piece_edge_4.png").unwrap().to_rgba8(),
            image::open("./images/piece_edge_5.png").unwrap().to_rgba8(),
            image::open("./images/piece_edge_6.png").unwrap().to_rgba8(),
        ],
    }
}

pub fn create_solution_images(
    solutions: &Vec<Vec<Vec<Option<PieceState>>>>,
    images: &Images,
    settings: crate::Settings,
    piece_size: (u32, u32),
    overlap: u32,
    solutions_folder: &str,
) {

    let left_crop = ImageCrop { x: 0, y: 0, width: 20, height: 64 };
    let right_crop = ImageCrop { x: 44, y: 0, width: 20, height: 64 };
    let top_crop = ImageCrop { x: 0, y: 0, width: 64, height: 20 };
    let bottom_crop = ImageCrop { x: 0, y: 44, width: 64, height: 20 };

    for (index, solution) in solutions.iter().enumerate() {
        let mut img = DynamicImage::new_rgba8(
            (piece_size.0 - overlap) * settings.grid_size as u32 + overlap,
            (piece_size.1 - overlap) * settings.grid_size as u32 + overlap,
        );
        
        for (y, row) in solution.iter().enumerate() {
            for (x, _piece_option) in row.iter().enumerate() {
                let x_offset = x as u32 * (piece_size.0 - overlap);
                let y_offset = y as u32 * (piece_size.1 - overlap);
                overlay(&mut img, &images.piece_blank, x_offset as i64, y_offset as i64);
            }
        }

        for (y, row) in solution.iter().enumerate() {
            for (x, piece_option) in row.iter().enumerate() {
                let piece = piece_option.as_ref().unwrap();
        
                let x_offset = x as u32 * (piece_size.0 - overlap);
                let y_offset = y as u32 * (piece_size.1 - overlap);
        
                let sides = [
                    (0, &top_crop, x_offset, y_offset), 
                    (1, &right_crop, x_offset + piece_size.0 - right_crop.width, y_offset),
                    (2, &bottom_crop, x_offset, y_offset + piece_size.1 - bottom_crop.height),
                    (3, &left_crop, x_offset, y_offset),
                ];
        
                for &(side_index, crop, offset_x, offset_y) in &sides {
                    let side_value = piece.piece.sides[side_index].0;
                    let at_grid_edge = match side_index {
                        0 => y == 0,
                        1 => x == settings.grid_size - 1,
                        2 => y == settings.grid_size - 1,
                        3 => x == 0,
                        _ => false,
                    };
        
                    if at_grid_edge {
                        if side_value == 0 {
                            let cropped = imageops::crop_imm(&images.piece_edge, crop.x, crop.y, crop.width, crop.height);
                            overlay(&mut img, &cropped.to_image(), offset_x as i64, offset_y as i64);
                        }
                    } else if side_value < 0 && side_value >= -6 {
                        let edge_index = -side_value as usize;
                        let edge_image = &images.piece_edges[edge_index - 1];
                        let cropped = imageops::crop_imm(edge_image, crop.x, crop.y, crop.width, crop.height);
                        overlay(&mut img, &cropped.to_image(), offset_x as i64, offset_y as i64);
                    }
                }
            }
        }
        
        img.save(format!("{}/solution_{}.png", solutions_folder, index + 1)).unwrap();
    }
}
