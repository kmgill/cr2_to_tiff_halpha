
use crate::imagebuffer::ImageBuffer;
use crate::path;
use crate::constants;

use std::process;



pub fn run_mean_stack(file_list:Vec<&str>, output:&str) {

    // This feels hacky....
    let mut stack = ImageBuffer::new(1, 1).unwrap();
    let mut cnt = 0;

    for in_file in file_list.iter() {
        if path::file_exists(in_file) {
            println!("Processing File: {}", in_file);
            
            let image = ImageBuffer::from_cr2(&in_file).unwrap().red().unwrap();
            let imagemm = image.get_min_max(-1.0).unwrap();
            println!("    Image Min/Max : {}, {}", imagemm.min, imagemm.max);

            if cnt == 0 {
                stack = image;
            } else {
                stack = stack.add(&image).unwrap();
            }

            cnt = cnt + 1;
        } else {
            eprintln!("File not found: {}", in_file);
        }
    }

    if cnt > 0 {
        stack = stack.scale(1.0 / cnt as f32).unwrap();
        let stackmm = stack.get_min_max(-1.0).unwrap();
        println!("    Stack Min/Max : {}, {} ({} images)", stackmm.min, stackmm.max, cnt);

        stack.save(output).expect(constants::OK);
    } else {
        eprintln!("No files used");
        process::exit(1);
    }

}