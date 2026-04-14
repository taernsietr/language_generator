use serde::Deserialize;
use angelspeech::prelude::*;

#[derive(Deserialize)]
pub struct GeneratorNameParams {
    pub generator: String
}

#[derive(Deserialize, Clone)]
pub struct WordParams {
    pub generator: String,
    pub min: u8,
    pub max: u8,
    pub bias: f32,
    pub text_length: u8,
    pub text_type: TextType
}

impl From<WordParams> for TextParams {
    fn from(params: WordParams) -> Self {
        TextParams::new(
            params.min,
            params.max,
            params.bias,
            params.text_length,
            params.text_type
        )
    }
}

