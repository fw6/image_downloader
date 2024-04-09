use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum Command {
    Avatar(AvatarRequest),
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AvatarRequest {
    pub name: String,
    font_family: Option<String>,
    font_scale: Option<String>,
    font_color: Option<String>,
    background_color: Option<String>,
    pub length: Option<usize>,
    width: Option<u32>,
    height: Option<u32>,
    contrast_ratio: Option<f32>,
    blur: Option<f32>,
}

impl AvatarRequest {
    // pub fn into_avatar_builder(self) -> imagekit::avatar::AvatarBuilder {
    //     let mut builder =
    // imagekit::avatar::AvatarBuilder::default().name(self.name);

    //     if let Some(font_family) = self.font_family {
    //         // builder = builder.font_family(font_family);
    //     }
    //     if let Some(font_scale) = self.font_scale {
    //         builder = builder.font_scale(font_scale);
    //     }
    //     if let Some(font_color) = self.font_color {
    //         builder = builder.font_color(font_color);
    //     }
    //     if let Some(background_color) = self.background_color {
    //         builder = builder.background_color(background_color);
    //     }
    //     if let Some(length) = self.length {
    //         builder = builder.length(length);
    //     }
    //     if let Some(width) = self.width {
    //         builder = builder.width(width);
    //     }
    //     if let Some(height) = self.height {
    //         builder = builder.height(height);
    //     }
    //     if let Some(contrast_ratio) = self.contrast_ratio {
    //         builder = builder.contrast_ratio(contrast_ratio);
    //     }

    //     builder = builder.blur(self.blur);

    //     builder
    // }
}
