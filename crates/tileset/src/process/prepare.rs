use crate::{
    data::{coords::Dimensions, mapping::CharacterMapping, palette::PaletteSetRgba},
    input_stack::{
        Aseprite, InError, InputConfig, InputEntry, InputImage, InputStack, InputStackError,
    },
    manifest::{Manifest, ManifestEntry},
    process::config::ToConfig,
    profile::Profile,
};
use buddyasm_common::manifest::Manifest as _;
use image::open;

/// Load an input stack from the provided config
pub fn prepare(profile: Profile, manifest: &Manifest) -> Result<InputStack, InputStackError> {
    let config = profile.to_config();

    // Collect errors encountered
    let mut errors = Vec::with_capacity(manifest.entries.len());

    // Load the default palette
    let default_palette = if let Some(path) = &manifest.config.default_palette {
        let path = manifest.evaluate_path(path);
        match PaletteSetRgba::load_palette(&path) {
            Ok(palette) => Some(palette),
            Err(err) => {
                // Default palette is not valid
                errors.push(InError::Palette(err));
                None
            }
        }
    } else {
        None
    };

    // Build a stack from the provided entries
    let mut stack = Vec::with_capacity(manifest.entries.len());

    // Create a context to process the entries
    let mut context = Context {
        manifest,
        default_palette,
        tiled_loader: tiled::Loader::new(),
        config,
    };

    // Process the entries
    for entry in manifest.entries.iter() {
        match context.prepare_entry(entry) {
            Ok(entry) => stack.push(entry),
            Err(err) => errors.push(err),
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

/// Store context to read the manifest file
struct Context<'m> {
    /// Manifest file to read
    manifest: &'m Manifest,

    /// Default palette (if any was defined)
    default_palette: Option<PaletteSetRgba>,

    /// Loader for tiled files
    tiled_loader: tiled::Loader,

    /// Config generated from the profile
    config: InputConfig,
}

impl<'m> Context<'m> {
    fn prepare_entry(&mut self, entry: &ManifestEntry) -> Result<InputEntry, InError> {
        // Check if we have a palette override for this entry
        let palette = if let Some(path_palette) = &entry.palette {
            let path = self.manifest.evaluate_path(path_palette);
            match PaletteSetRgba::load_palette(&path) {
                Ok(palette) => palette,
                Err(err) => {
                    // Palette specified for entry is not valid
                    return Err(InError::Palette(err));
                }
            }
        } else if let Some(palette) = &self.default_palette {
            palette.clone()
        } else {
            // Without a default palette, is not valid
            return Err(InError::NoPalette);
        };

        // Resolve the image path
        let img_path = self.manifest.evaluate_path(&entry.image);

        // Resolve the name of the entry (either set in the manifest or the file name)
        let name = if let Some(name) = &entry.name {
            name.clone()
        } else if let Some(name) = img_path.file_name().and_then(|n| n.to_str()) {
            name.to_string()
        } else {
            return Err(InError::InvalidName);
        };

        // Keep the template path as given: it is used as the lookup key
        // for the Tera instance, which registers templates under their
        // path relative to the manifest (see `render::build_renderer`).
        let template_path = entry.template.clone();

        // Get the extension of the file
        let Some(ext) = img_path.extension() else {
            return Err(InError::FileExt);
        };

        // Check if it is an aseprite file
        if ext.eq_ignore_ascii_case("aseprite") {
            // load an Aseprite file
            let aseprite = Aseprite::load(&img_path)?;
            Ok(InputEntry {
                path: img_path.clone(),
                name,
                image: InputImage::Aseprite(aseprite),
                palette,
                output_json: entry.output_json,
                template: template_path,
            })
        } else if ext.eq_ignore_ascii_case("tsx") {
            let tileset = self.tiled_loader.load_tsx_tileset(&img_path)?;
            let image = InputImage::TiledTileset(Box::new(tileset));
            Ok(InputEntry {
                path: img_path.clone(),
                name,
                image,
                palette,
                output_json: entry.output_json,
                template: template_path,
            })
        } else if ext.eq_ignore_ascii_case("tmx") {
            let tilemap = self.tiled_loader.load_tmx_map(&img_path)?;
            let image = InputImage::TiledMap(Box::new(tilemap));
            Ok(InputEntry {
                path: img_path.clone(),
                name,
                image,
                palette,
                output_json: entry.output_json,
                template: template_path,
            })
        } else {
            // evaluate the number of entries to generate
            let image = open(&img_path)?.to_rgba8();

            // Check if we require a fixed mapping
            let image = if !entry.fixed_mapping.is_empty() {
                // When a fixed mapping is required processing mapping data
                let dims = Dimensions::from_img(image.dimensions(), self.config.tile_size);
                let mapping = CharacterMapping::from_ranges(dims, &entry.fixed_mapping);
                InputImage::FixedPosition { image, mapping }
            } else {
                // Static image to process
                InputImage::Static(image)
            };
            Ok(InputEntry {
                path: img_path.clone(),
                name,
                image,
                palette,
                output_json: entry.output_json,
                template: template_path,
            })
        }
    }
}
