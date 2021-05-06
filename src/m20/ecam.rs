use crate::{
    vprintln, 
    rgbimage, 
    enums, 
    path,
    util
};



pub fn process_file(input_file:&str, red_scalar:f32, green_scalar:f32, blue_scalar:f32, _no_ilt:bool, only_new:bool) {
    let out_file = input_file.replace(".png", "-rjcal.png").replace(".PNG", "-rjcal.png");
    if path::file_exists(&out_file) && only_new {
        vprintln!("Output file exists, skipping. ({})", out_file);
        return;
    }

    let mut instrument = enums::Instrument::M20NavcamRight;

    // Attempt to figure out camera from file name
    if util::filename_char_at_pos(&input_file, 0) == 'N' {         // NAVCAMS
        if util::filename_char_at_pos(&input_file, 1) == 'L' {     // Left
            instrument = enums::Instrument::M20NavcamLeft;
        } else {                                   // Assume Right
            instrument = enums::Instrument::M20NavcamRight;
        }
    } else if util::filename_char_at_pos(&input_file, 0) == 'F' {  // FHAZ
        if util::filename_char_at_pos(&input_file, 1)  == 'L' {     // Left
            instrument = enums::Instrument::M20FrontHazLeft;
        } else {                                   // Assume Right
            instrument = enums::Instrument::M20FrontHazRight;
        }  
    } else if util::filename_char_at_pos(&input_file, 0) == 'R' {  // RHAZ
        if util::filename_char_at_pos(&input_file, 1)  == 'L' {     // Left
            instrument = enums::Instrument::M20RearHazLeft;
        } else {                                   // Assume Right
            instrument = enums::Instrument::M20RearHazRight;
        }
    }

    let mut raw = rgbimage::RgbImage::open(String::from(input_file), instrument).unwrap();

    let data_max = 255.0;

    // if ! no_ilt {
    //     vprintln!("Decompanding...");
    //     raw.decompand().unwrap();
    //     data_max = decompanding::get_max_for_instrument(instrument) as f32;
    // }

    // Looks like 'ECM' in the name seems to indicate that it still have the bayer pattern
    if raw.is_grayscale() {
        vprintln!("Debayering...");
        raw.debayer().unwrap();
    }

    // We're going to need a reliable way of figuring out what part of the sensor
    // is represented before we can flatfield or apply an inpainting mask
    //vprintln!("Inpainting...");
    //raw.apply_inpaint_fix().unwrap();

    vprintln!("Applying color weights...");
    raw.apply_weight(red_scalar, green_scalar, blue_scalar).unwrap();

    vprintln!("Normalizing...");
    raw.normalize_to_16bit_with_max(data_max).unwrap();

    // Trim off border pixels
    let crop_to_width = raw.width - 4;
    let crop_to_height = raw.height - 4;
    raw.crop(2, 2, crop_to_width, crop_to_height).unwrap();

    vprintln!("Writing to disk...");
    raw.save(&out_file).unwrap();
}