mod jigsaw;
mod settings;
mod files;
mod images;

use files::handle_solutions_folder;
use images::{load_images, create_solution_images};
use jigsaw::Jigsaw;
use settings::Settings;

fn main() {
    let settings = Settings {
        grid_size: 10,
        min_solutions: 2,
        side_types: 6,
    };

    let mut solutions = Vec::new();
    let solutions_folder = "./solutions";
    handle_solutions_folder(solutions_folder);

    let images = load_images();


    if settings.side_types > images.piece_edges.len() as u8
    {
        panic!("You have more side types than edge images, either create more edge images or reduce the number of side types");
    }

    let piece_size = images.piece_blank.dimensions();
    let overlap = 1;

    while solutions.len() < settings.min_solutions {
        let mut jigsaw = Jigsaw::generate(settings.grid_size, &settings);
        solutions.clear();
        jigsaw.try_solve(&mut solutions);
    }

    println!("Creating images for {} solutions", solutions.len());
    create_solution_images(&solutions, &images, settings, piece_size, overlap, solutions_folder);
}
