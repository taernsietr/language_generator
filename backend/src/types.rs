use serde::Deserialize;
use angelspeech::prelude::*;

#[derive(Deserialize)]
pub struct GeneratorNameParams {
    pub generator: String
}

#[derive(Deserialize, Clone)]
pub struct WordParams {
    pub generator: String,
    pub min: Option<u8>,
    pub max: Option<u8>,
    pub bias: Option<f32>,
    pub text_length: Option<u8>,
    pub text_type: Option<TextType>
}

impl From<WordParams> for TextParams {
    fn from(params: WordParams) -> Self {
        TextParams::new(
            params.min.unwrap_or(1u8),
            params.max.unwrap_or(4u8),
            params.bias.unwrap_or(0.5f32),
            params.text_length.unwrap_or(1u8),
            params.text_type.unwrap_or(TextType::GenericWord)
        )
    }
}

