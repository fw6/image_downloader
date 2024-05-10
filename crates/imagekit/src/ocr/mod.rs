use std::{fs, path::Path};

use anyhow::{anyhow, Result};
use derive_builder::Builder;
use ocrs::{OcrEngine, OcrEngineParams};
use rten::Model;
use rten_imageio::read_image;
use rten_tensor::prelude::*;
#[cfg(feature = "openapi")]
use salvo::oapi::ToParameters;
#[cfg(feature = "openapi")]
use serde::Deserialize;

/// OCRs an image
#[derive(Builder)]
#[cfg_attr(feature = "openapi", builder(derive(ToParameters, Deserialize, Debug)), builder_struct_attr(salvo(parameters(default_parameter_in = Query))))]
#[cfg_attr(feature = "cli", derive(clap::Parser, Debug, Clone))]
pub struct Ocrs {
    /// Initial name of the avatar
    #[cfg_attr(feature = "openapi", builder_field_attr(salvo(parameter(value_type = Option<String>, required = true))))]
    #[cfg_attr(feature = "cli", clap(short, long))]
    pub image: String,
}

impl Ocrs {
    pub async fn recognize(&self) -> Result<()> {
        let workspace_dir = project_root::get_project_root()?;
        let detection_model_data =
            fs::read(workspace_dir.join(Path::new("models/text-detection.rten")))?;
        let rec_model_data =
            fs::read(workspace_dir.join(Path::new("models/text-recognition.rten")))?;

        let detection_model = Model::load(&detection_model_data)?;
        let recognition_model = Model::load(&rec_model_data)?;
        let engine = OcrEngine::new(OcrEngineParams {
            detection_model: Some(detection_model),
            recognition_model: Some(recognition_model),
            ..Default::default()
        })?;

        let image = read_image(&self.image).map_err(|_| anyhow!("The image path is incorrect!"))?;
        let ocr_input = engine.prepare_input(image.view())?;

        let word_rects = engine.detect_words(&ocr_input)?;

        let line_rects = engine.find_text_lines(&ocr_input, &word_rects);

        let line_texts = engine.recognize_text(&ocr_input, &line_rects)?;

        for line in line_texts
            .iter()
            .flatten()
            // Filter likely spurious detections. With future model improvements
            // this should become unnecessary.
            .filter(|l| l.to_string().len() > 1)
        {
            println!("{}", line);
        }

        anyhow::Ok(())
    }
}
