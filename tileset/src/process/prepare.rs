use crate::{
    data::{coords::Dimensions, mapping::CharacterMapping, palette::PaletteSetRgba},
    input_stack::{InError, InputEntry, InputImage, InputStack, InputStackError},
    manifest::Manifest,
    process::config::ToConfig,
    profile::Profile,
};
use asefile::AsepriteFile;
use buddyasm_common::manifest::Manifest as _;
use image::open;

/// Load an input stack from the provided config
pub fn prepare(profile: Profile, manifest: &Manifest) -> Result<InputStack, InputStackError> {
    let config = profile.to_config();

    // Collect errors encountered
    let mut errors = Vec::with_capacity(manifest.entries.len());

    // Load the default palette
    let palette = if let Some(path) = &manifest.config.default_palette {
        let path = manifest.evaluate_path(path);
        match PaletteSetRgba::load_palette(&path) {
            Ok(palette) => Some(palette),
            Err(err) => {
                // Default palette is not valid
                errors.push(InError::Palette(path, err));
                None
            }
        }
    } else {
        None
    };

    // Create a loader for tiled files
    let mut tiled_loader = tiled::Loader::new();

    // Build a stack from the provided entries
    let mut stack = Vec::with_capacity(manifest.entries.len());

    // Process the entries
    for entry in manifest.entries.iter() {
        // Check if we have a palette override for this entry
        let palette = if let Some(path_palette) = &entry.palette {
            let path = manifest.evaluate_path(path_palette);
            match PaletteSetRgba::load_palette(&path) {
                Ok(palette) => palette,
                Err(err) => {
                    // Palette specified for entry is not valid
                    errors.push(InError::Palette(path, err));
                    continue;
                }
            }
        } else if let Some(palette) = &palette {
            palette.clone()
        } else {
            // Without a default palette, is not valid
            errors.push(InError::NoPalette);
            continue;
        };

        // Resolve the image path
        let img_path = manifest.evaluate_path(&entry.image);

        // Resolve the name of the entry (either set in the manifest or the file name)
        let name = if let Some(name) = &entry.name {
            name.clone()
        } else if let Some(name) = img_path.file_name().and_then(|n| n.to_str()) {
            name.to_string()
        } else {
            errors.push(InError::InvalidName(img_path.clone()));
            continue;
        };

        // Keep the template path as given: it is used as the lookup key
        // for the Tera instance, which registers templates under their
        // path relative to the manifest (see `render::build_renderer`).
        let template_path = entry.template.clone();

        // Get the extension of the file
        let Some(ext) = img_path.extension() else {
            errors.push(InError::FileExt(img_path.clone()));
            continue;
        };

        // Check if it is an aseprite file
        if ext.eq_ignore_ascii_case("aseprite") {
            match AsepriteFile::read_file(&img_path) {
                Ok(image) => {
                    let image = InputImage::Aseprite(Box::new(image));

                    let in_entry = stack.push(InputEntry {
                        path: img_path.clone(),
                        name,
                        image,
                        palette,
                        output_json: entry.output_json,
                        template: template_path,
                    });
                }
                Err(err) => {
                    errors.push(InError::Aseprite(img_path.clone(), err));
                }
            }
        } else if ext.eq_ignore_ascii_case("tsx") {
            match tiled_loader.load_tsx_tileset(&img_path) {
                Ok(tileset) => {
                    let image = InputImage::TiledTileset(Box::new(tileset));
                    stack.push(InputEntry {
                        path: img_path.clone(),
                        name,
                        image,
                        palette,
                        output_json: entry.output_json,
                        template: template_path,
                    });
                }
                Err(err) => {
                    errors.push(InError::TiledTSX(img_path.clone(), err));
                }
            }
        } else if ext.eq_ignore_ascii_case("tmx") {
            match tiled_loader.load_tmx_map(&img_path) {
                Ok(map) => {
                    let image = InputImage::TiledMap(Box::new(map));
                    stack.push(InputEntry {
                        path: img_path.clone(),
                        name,
                        image,
                        palette,
                        output_json: entry.output_json,
                        template: template_path,
                    });
                }
                Err(err) => {
                    errors.push(InError::TiledTMX(img_path.clone(), err));
                }
            }
        } else {
            // evaluate the number of entries to generate
            match open(&img_path) {
                Ok(image) => {
                    // We only handle RGBA images
                    let image = image.to_rgba8();

                    // Check if we require a fixed mapping
                    let image = if !entry.fixed_mapping.is_empty() {
                        // When a fixed mapping is required processing mapping data
                        let dims = Dimensions::from_img(image.dimensions(), config.tile_size);
                        let mapping = CharacterMapping::from_ranges(dims, &entry.fixed_mapping);
                        InputImage::FixedPosition { image, mapping }
                    } else {
                        // Static image to process
                        InputImage::Static(image)
                    };
                    stack.push(InputEntry {
                        path: img_path.clone(),
                        name,
                        image,
                        palette,
                        output_json: entry.output_json,
                        template: template_path,
                    });
                }
                Err(err) => {
                    errors.push(InError::Image(img_path.clone(), err));
                }
            }
        }
    }

    // Check if we encountered errors
    if errors.is_empty() {
        // Complete the stack
        Ok(InputStack {
            profile,
            config,
            stack,
        })
    } else {
        Err(InputStackError { errors })
    }
}
