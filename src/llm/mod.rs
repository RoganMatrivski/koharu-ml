pub mod model;
pub mod providers;

use std::path::PathBuf;
use std::str::FromStr;
use strum::{Display, EnumIter, EnumProperty, EnumString, IntoEnumIterator};

pub use model::{GenerateOptions, Llm};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, EnumIter, EnumProperty)]
pub enum Language {
    #[strum(
        to_string = "Simplified Chinese",
        serialize = "zh-CN",
        serialize = "zh",
        serialize = "zh-Hans",
        props(tag = "zh-CN")
    )]
    ChineseSimplified,
    #[strum(
        to_string = "English",
        serialize = "en-US",
        serialize = "en",
        props(tag = "en-US")
    )]
    English,
    #[strum(
        to_string = "French",
        serialize = "fr-FR",
        serialize = "fr",
        props(tag = "fr-FR")
    )]
    French,
    #[strum(
        to_string = "Portuguese",
        serialize = "pt-PT",
        serialize = "pt",
        props(tag = "pt-PT")
    )]
    Portuguese,
    #[strum(
        to_string = "Brazilian Portuguese",
        serialize = "pt-BR",
        props(tag = "pt-BR")
    )]
    BrazilianPortuguese,
    #[strum(
        to_string = "Spanish",
        serialize = "es-ES",
        serialize = "es",
        props(tag = "es-ES")
    )]
    Spanish,
    #[strum(
        to_string = "Japanese",
        serialize = "ja-JP",
        serialize = "ja",
        props(tag = "ja-JP")
    )]
    Japanese,
    #[strum(
        to_string = "Turkish",
        serialize = "tr-TR",
        serialize = "tr",
        props(tag = "tr-TR")
    )]
    Turkish,
    #[strum(
        to_string = "Russian",
        serialize = "ru-RU",
        serialize = "ru",
        props(tag = "ru-RU")
    )]
    Russian,
    #[strum(
        to_string = "Arabic",
        serialize = "ar-SA",
        serialize = "ar",
        props(tag = "ar-SA")
    )]
    Arabic,
    #[strum(
        to_string = "Korean",
        serialize = "ko-KR",
        serialize = "ko",
        props(tag = "ko-KR")
    )]
    Korean,
    #[strum(
        to_string = "Thai",
        serialize = "th-TH",
        serialize = "th",
        props(tag = "th-TH")
    )]
    Thai,
    #[strum(
        to_string = "Italian",
        serialize = "it-IT",
        serialize = "it",
        props(tag = "it-IT")
    )]
    Italian,
    #[strum(
        to_string = "German",
        serialize = "de-DE",
        serialize = "de",
        props(tag = "de-DE")
    )]
    German,
    #[strum(
        to_string = "Vietnamese",
        serialize = "vi-VN",
        serialize = "vi",
        props(tag = "vi-VN")
    )]
    Vietnamese,
    #[strum(
        to_string = "Malay",
        serialize = "ms-MY",
        serialize = "ms",
        props(tag = "ms-MY")
    )]
    Malay,
    #[strum(
        to_string = "Indonesian",
        serialize = "id-ID",
        serialize = "id",
        props(tag = "id-ID")
    )]
    Indonesian,
    #[strum(
        to_string = "Filipino",
        serialize = "fil-PH",
        serialize = "fil",
        serialize = "tl",
        props(tag = "fil-PH")
    )]
    Filipino,
    #[strum(
        to_string = "Hindi",
        serialize = "hi-IN",
        serialize = "hi",
        props(tag = "hi-IN")
    )]
    Hindi,
    #[strum(
        to_string = "Traditional Chinese",
        serialize = "zh-TW",
        serialize = "zh-Hant",
        props(tag = "zh-TW")
    )]
    ChineseTraditional,
    #[strum(
        to_string = "Polish",
        serialize = "pl-PL",
        serialize = "pl",
        props(tag = "pl-PL")
    )]
    Polish,
    #[strum(
        to_string = "Czech",
        serialize = "cs-CZ",
        serialize = "cs",
        props(tag = "cs-CZ")
    )]
    Czech,
    #[strum(
        to_string = "Dutch",
        serialize = "nl-NL",
        serialize = "nl",
        props(tag = "nl-NL")
    )]
    Dutch,
    #[strum(
        to_string = "Khmer",
        serialize = "km-KH",
        serialize = "km",
        props(tag = "km-KH")
    )]
    Khmer,
    #[strum(
        to_string = "Burmese",
        serialize = "my-MM",
        serialize = "my",
        props(tag = "my-MM")
    )]
    Burmese,
    #[strum(
        to_string = "Persian",
        serialize = "fa-IR",
        serialize = "fa",
        props(tag = "fa-IR")
    )]
    Persian,
    #[strum(
        to_string = "Gujarati",
        serialize = "gu-IN",
        serialize = "gu",
        props(tag = "gu-IN")
    )]
    Gujarati,
    #[strum(
        to_string = "Urdu",
        serialize = "ur-PK",
        serialize = "ur",
        props(tag = "ur-PK")
    )]
    Urdu,
    #[strum(
        to_string = "Telugu",
        serialize = "te-IN",
        serialize = "te",
        props(tag = "te-IN")
    )]
    Telugu,
    #[strum(
        to_string = "Marathi",
        serialize = "mr-IN",
        serialize = "mr",
        props(tag = "mr-IN")
    )]
    Marathi,
    #[strum(
        to_string = "Hebrew",
        serialize = "he-IL",
        serialize = "he",
        props(tag = "he-IL")
    )]
    Hebrew,
    #[strum(
        to_string = "Bengali",
        serialize = "bn-BD",
        serialize = "bn",
        props(tag = "bn-BD")
    )]
    Bengali,
    #[strum(
        to_string = "Bulgarian",
        serialize = "bg-BG",
        serialize = "bg",
        props(tag = "bg-BG")
    )]
    Bulgarian,
    #[strum(
        to_string = "Tamil",
        serialize = "ta-IN",
        serialize = "ta",
        props(tag = "ta-IN")
    )]
    Tamil,
    #[strum(
        to_string = "Ukrainian",
        serialize = "uk-UA",
        serialize = "uk",
        props(tag = "uk-UA")
    )]
    Ukrainian,
    #[strum(
        to_string = "Tibetan",
        serialize = "bo-CN",
        serialize = "bo",
        props(tag = "bo-CN")
    )]
    Tibetan,
    #[strum(
        to_string = "Kazakh",
        serialize = "kk-KZ",
        serialize = "kk",
        props(tag = "kk-KZ")
    )]
    Kazakh,
    #[strum(
        to_string = "Mongolian",
        serialize = "mn-MN",
        serialize = "mn",
        props(tag = "mn-MN")
    )]
    Mongolian,
    #[strum(
        to_string = "Uyghur",
        serialize = "ug-CN",
        serialize = "ug",
        props(tag = "ug-CN")
    )]
    Uyghur,
    #[strum(
        to_string = "Cantonese",
        serialize = "yue-HK",
        serialize = "yue",
        props(tag = "yue-HK")
    )]
    Cantonese,
}

impl Language {
    pub fn tag(self) -> &'static str {
        self.get_str("tag").expect("language tag property")
    }

    pub fn parse(value: &str) -> Option<Self> {
        let value = value.trim();
        if value.is_empty() {
            return None;
        }
        Self::from_str(value).ok()
    }
}

pub fn supported_locales() -> Vec<String> {
    Language::iter()
        .map(|language| language.tag().to_string())
        .collect()
}

pub fn language_from_tag(value: &str) -> String {
    Language::parse(value)
        .unwrap_or(Language::English)
        .to_string()
}

pub fn tags(languages: &[Language]) -> Vec<String> {
    languages
        .iter()
        .map(|language| language.tag().to_string())
        .collect()
}

pub const BLOCK_TAG_INSTRUCTIONS: &str = "The input uses numbered tags like [1], [2], etc. to mark each text block. Translate only the text after each tag. Keep every tag exactly unchanged, including numbers and order. Output the same tags followed by the translated text. Do not merge, split, or reorder blocks.";

pub fn system_prompt(target_language: Language) -> String {
    format!(
        "You are a professional manga translator. Translate manga dialogue into natural {} that fits inside speech bubbles. Preserve character voice, emotional tone, relationship nuance, emphasis, and sound effects naturally. Keep the wording concise. Do not add notes, explanations, or romanization. {BLOCK_TAG_INSTRUCTIONS}",
        target_language
    )
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    strum::Display,
    strum::EnumString,
    strum::EnumIter,
    strum::EnumProperty,
)]
pub enum ModelId {
    #[strum(
        serialize = "vntl-llama3-8b-v2",
        props(
            repo = "lmg-anon/vntl-llama3-8b-v2-gguf",
            filename = "vntl-llama3-8b-v2-hf-q8_0.gguf",
            languages = "en-US"
        )
    )]
    VntlLlama3_8Bv2,
    #[strum(
        serialize = "lfm2.5-1.2b-instruct",
        props(
            repo = "LiquidAI/LFM2.5-1.2B-Instruct-GGUF",
            filename = "LFM2.5-1.2B-Instruct-Q8_0.gguf",
            languages = "en-US,ar-SA,zh-CN,fr-FR,de-DE,ja-JP,ko-KR,pt-PT,es-ES"
        )
    )]
    Lfm2_5_1_2bInstruct,
    #[strum(
        serialize = "sakura-galtransl-7b-v3.7",
        props(
            repo = "SakuraLLM/Sakura-GalTransl-7B-v3.7",
            filename = "Sakura-Galtransl-7B-v3.7.gguf",
            languages = "zh-CN"
        )
    )]
    SakuraGalTransl7Bv3_7,
    #[strum(
        serialize = "sakura-1.5b-qwen2.5-v1.0",
        props(
            repo = "shing3232/Sakura-1.5B-Qwen2.5-v1.0-GGUF-IMX",
            filename = "sakura-1.5b-qwen2.5-v1.0-Q5KS.gguf",
            languages = "zh-CN"
        )
    )]
    Sakura1_5bQwen2_5v1_0,
    #[strum(
        serialize = "hunyuan-mt-7b",
        props(
            repo = "Mungert/Hunyuan-MT-7B-GGUF",
            filename = "Hunyuan-MT-7B-q6_k_m.gguf",
            languages = "zh-CN,en-US,fr-FR,pt-PT,pt-BR,es-ES,ja-JP,tr-TR,ru-RU,ar-SA,ko-KR,th-TH,it-IT,de-DE,vi-VN,ms-MY,id-ID,fil-PH,hi-IN,zh-TW,pl-PL,cs-CZ,nl-NL,km-KH,my-MM,fa-IR,gu-IN,ur-PK,te-IN,mr-IN,he-IL,bn-BD,ta-IN,uk-UA,bo-CN,kk-KZ,mn-MN,ug-CN,yue-HK"
        )
    )]
    HunyuanMT7B,
    #[strum(
        serialize = "sugoi-14b-ultra",
        props(
            repo = "sugoitoolkit/Sugoi-14B-Ultra-GGUF",
            filename = "Sugoi-14B-Ultra-Q8_0.gguf",
            languages = "en-US"
        )
    )]
    Sugoi14bUltra,
    #[strum(
        serialize = "sugoi-32b-ultra",
        props(
            repo = "sugoitoolkit/Sugoi-32B-Ultra-GGUF",
            filename = "Sugoi-32B-Ultra-Q4_K_M.gguf",
            languages = "en-US"
        )
    )]
    Sugoi32bUltra,
    #[strum(
        serialize = "gemma4-e2b-it",
        props(
            repo = "unsloth/gemma-4-E2B-it-GGUF",
            filename = "gemma-4-e2b-it-Q8_0.gguf",
            languages = "*"
        )
    )]
    Gemma4E2bIt,
    #[strum(
        serialize = "gemma4-e4b-it",
        props(
            repo = "unsloth/gemma-4-E4B-it-GGUF",
            filename = "gemma-4-e4b-it-Q8_0.gguf",
            languages = "*"
        )
    )]
    Gemma4E4bIt,
    #[strum(
        serialize = "gemma4-26b-a4b-it",
        props(
            repo = "unsloth/gemma-4-26B-A4B-it-GGUF",
            filename = "gemma-4-26B-A4B-it-Q8_0.gguf",
            languages = "*"
        )
    )]
    Gemma4_26bA4bIt,
    #[strum(
        serialize = "gemma4-31b-it",
        props(
            repo = "unsloth/gemma-4-31B-it-GGUF",
            filename = "gemma-4-31B-it-Q4_K_M.gguf",
            languages = "*"
        )
    )]
    Gemma4_31bIt,
    #[strum(
        serialize = "gemma4-e2b-uncensored",
        props(
            repo = "HauhauCS/Gemma-4-E2B-Uncensored-HauhauCS-Aggressive",
            filename = "Gemma-4-E2B-Uncensored-HauhauCS-Aggressive-Q8_K_P.gguf",
            languages = "*"
        )
    )]
    Gemma4E2bUncensored,
    #[strum(
        serialize = "gemma4-e4b-uncensored",
        props(
            repo = "HauhauCS/Gemma-4-E4B-Uncensored-HauhauCS-Aggressive",
            filename = "Gemma-4-E4B-Uncensored-HauhauCS-Aggressive-Q4_K_M.gguf",
            languages = "*"
        )
    )]
    Gemma4E4bUncensored,
    #[strum(
        serialize = "qwen3.5-0.8b",
        props(
            repo = "unsloth/Qwen3.5-0.8B-GGUF",
            filename = "Qwen3.5-0.8B-Q8_0.gguf",
            languages = "*"
        )
    )]
    Qwen3_5_0_8b,
    #[strum(
        serialize = "qwen3.5-2b",
        props(
            repo = "unsloth/Qwen3.5-2B-GGUF",
            filename = "Qwen3.5-2B-Q8_0.gguf",
            languages = "*"
        )
    )]
    Qwen3_5_2b,
    #[strum(
        serialize = "qwen3.5-4b",
        props(
            repo = "unsloth/Qwen3.5-4B-GGUF",
            filename = "Qwen3.5-4B-Q8_0.gguf",
            languages = "*"
        )
    )]
    Qwen3_5_4b,
    #[strum(
        serialize = "qwen3.5-9b",
        props(
            repo = "unsloth/Qwen3.5-9B-GGUF",
            filename = "Qwen3.5-9B-Q8_0.gguf",
            languages = "*"
        )
    )]
    Qwen3_5_9b,
    #[strum(
        serialize = "qwen3.5-27b",
        props(
            repo = "unsloth/Qwen3.5-27B-GGUF",
            filename = "Qwen3.5-27B-Q4_K_M.gguf",
            languages = "*"
        )
    )]
    Qwen3_5_27b,
    #[strum(
        serialize = "qwen3.5-35b-a3b",
        props(
            repo = "unsloth/Qwen3.5-35B-A3B-GGUF",
            filename = "Qwen3.5-35B-A3B-Q8_0.gguf",
            languages = "*"
        )
    )]
    Qwen3_5_35bA3b,
    #[strum(
        serialize = "qwen3.5-2b-uncensored",
        props(
            repo = "HauhauCS/Qwen3.5-2B-Uncensored-HauhauCS-Aggressive",
            filename = "Qwen3.5-2B-Uncensored-HauhauCS-Aggressive-Q8_0.gguf",
            languages = "*"
        )
    )]
    Qwen3_5_2bUncensored,
    #[strum(
        serialize = "qwen3.5-4b-uncensored",
        props(
            repo = "HauhauCS/Qwen3.5-4B-Uncensored-HauhauCS-Aggressive",
            filename = "Qwen3.5-2B-Uncensored-HauhauCS-Aggressive-Q8_0.gguf",
            languages = "*"
        )
    )]
    Qwen3_5_4bUncensored,
    #[strum(
        serialize = "qwen3.5-9b-uncensored",
        props(
            repo = "HauhauCS/Qwen3.5-9B-Uncensored-HauhauCS-Aggressive",
            filename = "Qwen3.5-9B-Uncensored-HauhauCS-Aggressive-Q8_0.gguf",
            languages = "*"
        )
    )]
    Qwen3_5_9bUncensored,
    #[strum(
        serialize = "qwen3.5-27b-uncensored",
        props(
            repo = "HauhauCS/Qwen3.5-27B-Uncensored-HauhauCS-Aggressive",
            filename = "Qwen3.5-27B-Uncensored-HauhauCS-Aggressive-Q4_K_M.gguf",
            languages = "*"
        )
    )]
    Qwen3_5_27bUncensored,
    #[strum(
        serialize = "qwen3.5-35b-a3b-uncensored",
        props(
            repo = "HauhauCS/Qwen3.5-35B-A3B-Uncensored-HauhauCS-Aggressive",
            filename = "Qwen3.5-35B-A3B-Uncensored-HauhauCS-Aggressive-Q8_0.gguf",
            languages = "*"
        )
    )]
    Qwen3_5_35bA3bUncensored,
}

impl ModelId {
    fn property(&self, name: &str) -> &'static str {
        self.get_str(name).expect("missing model property")
    }

    pub fn default_generate_options(&self) -> GenerateOptions {
        match self {
            // LFM2.5: temp=0.1, top_k=50, repeat=1.05
            Self::Lfm2_5_1_2bInstruct => GenerateOptions {
                temperature: 0.1,
                top_k: Some(50),
                repeat_penalty: 1.05,
                ..Default::default()
            },
            // Gemma 4: temp=1.0, top_p=0.95, top_k=64
            Self::Gemma4E2bIt
            | Self::Gemma4E4bIt
            | Self::Gemma4_26bA4bIt
            | Self::Gemma4_31bIt
            | Self::Gemma4E2bUncensored
            | Self::Gemma4E4bUncensored => GenerateOptions {
                temperature: 1.0,
                top_k: Some(64),
                top_p: Some(0.95),
                repeat_penalty: 1.0,
                ..Default::default()
            },
            // Qwen3.5 non-thinking: temp=1.0, top_k=20, top_p=1.0, presence=2.0
            Self::Qwen3_5_0_8b
            | Self::Qwen3_5_2b
            | Self::Qwen3_5_4b
            | Self::Qwen3_5_9b
            | Self::Qwen3_5_27b
            | Self::Qwen3_5_35bA3b
            | Self::Qwen3_5_2bUncensored
            | Self::Qwen3_5_4bUncensored
            | Self::Qwen3_5_9bUncensored
            | Self::Qwen3_5_27bUncensored
            | Self::Qwen3_5_35bA3bUncensored => GenerateOptions {
                temperature: 1.0,
                top_k: Some(20),
                top_p: Some(1.0),
                min_p: Some(0.0),
                presence_penalty: 2.0,
                repeat_penalty: 1.0,
                ..Default::default()
            },
            // Sugoi: temp=0.1, top_k=40, top_p=0.95, min_p=0.05, repeat=1.1
            Self::Sugoi14bUltra | Self::Sugoi32bUltra => GenerateOptions {
                temperature: 0.1,
                top_k: Some(40),
                top_p: Some(0.95),
                min_p: Some(0.05),
                repeat_penalty: 1.1,
                ..Default::default()
            },
            // Default for other models
            _ => GenerateOptions::default(),
        }
    }

    pub fn languages(&self) -> Vec<Language> {
        let langs = self.property("languages");
        if langs == "*" {
            return Language::iter().collect();
        }
        langs
            .split(',')
            .map(|tag| Language::parse(tag).expect("invalid model language tag"))
            .collect()
    }
}

pub async fn prefetch() -> anyhow::Result<()> {
    Ok(())
}
