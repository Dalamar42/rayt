use cli::ImagePath;
use data::image::Image;
use failure::Error;
use io::load_image;
use std::collections::HashMap;

#[derive(Debug, Fail)]
pub enum AssetValidationError {
    #[fail(display = "asset with name <{}> has not been loaded", asset_name)]
    MissingAsset { asset_name: String },
}

pub struct Assets {
    assets: HashMap<String, Image>,
}

impl Assets {
    pub fn new(asset_paths: &[ImagePath]) -> Result<Assets, Error> {
        let mut assets: HashMap<String, Image> = HashMap::new();
        for asset_path in asset_paths {
            assets.insert(
                String::from(asset_path.file_name()),
                load_image(asset_path)?,
            );
        }

        Ok(Assets { assets })
    }

    pub fn get_asset(&self, asset_name: &str) -> &Image {
        &self.assets[asset_name]
    }

    pub fn validate(&self, asset_name: &str) -> Result<(), AssetValidationError> {
        if self.assets.get(asset_name).is_some() {
            Ok(())
        } else {
            Err(AssetValidationError::MissingAsset {
                asset_name: String::from(asset_name),
            })
        }
    }
}
