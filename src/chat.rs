use std::{convert::Infallible, io::Write, path::PathBuf};

use axum::{response::IntoResponse, Json};
use llm::Model;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(thiserror::Error, Debug)]
enum InferenceError {
    #[error("Failed to load the model: {0}")]
    UnableToLoadModel(String),

    #[error("Failed to perform inference: {0}")]
    UnableToCreateResponse(String),
}

/// Represents a user prompt & corresponding response from LLM.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Prompt {
    prompt: Option<String>,

    #[serde(skip_deserializing)]
    response: Option<String>,
}

impl Prompt {
    /// Creates a new Prompt instance.
    pub fn new(prompt: Option<String>, response: Option<String>) -> Self {
        Self { prompt, response }
    }

    /// Gets the prompt string.
    pub fn get_prompt(&self) -> Option<String> {
        self.prompt.to_owned()
    }

    /// Gets the response string.
    pub fn get_response(&self) -> Option<String> {
        self.response.to_owned()
    }

    /// Generates a reply for the given prompt.
    pub fn generate_reply_for_prompt(&mut self) -> impl IntoResponse {
        self.get_prompt().map(|prompt_str| -> Json<_> {
            self.infer().expect("Unable to generate LLM response");
            json!({"prompt": prompt_str, "response": self.get_response()}).into()
        })
        .unwrap_or(json!({"Suggestion":"To initiate a chat, add \"/chat?prompt=my prompt\" or \"/chat/my prompt\""}).into())
    }

    /// Performs inference based on the prompt and updates the response.
    pub fn infer(&mut self) -> anyhow::Result<()> {
        let model = self
            .load_model()
            .map_err(|err| InferenceError::UnableToLoadModel(err.to_string()))?;

        let start_time = std::time::Instant::now().elapsed().as_millis();
        println!("Model fully loaded! Elapsed: {}ms", start_time);

        // ---------- Starting session --------------
        let mut session = model.start_session(Default::default());

        let inference_request = &llm::InferenceRequest {
            prompt: (self.prompt.as_deref().unwrap()).into(),
            parameters: &llm::InferenceParameters::default(),
            play_back_previous_tokens: false,
            maximum_token_count: Some(100),
        };

        session
            .infer::<Infallible>(
                // Loaded model
                model.as_ref(),
                // Random range
                &mut rand::thread_rng(),
                // Request for inference
                inference_request,
                // Output request
                &mut Default::default(),
                // Inference response
                |r| match r {
                    llm::InferenceResponse::PromptToken(t)
                    | llm::InferenceResponse::InferredToken(t) => {
                        print!("{t}");
                        self.response = Some(t);

                        std::io::stdout().flush().unwrap();

                        Ok(llm::InferenceFeedback::Continue)
                    }
                    _ => Ok(llm::InferenceFeedback::Continue),
                },
            )
            .map_err(|e| InferenceError::UnableToCreateResponse(e.to_string()))?;

        Ok(())
    }

    /// Loads the model
    pub fn load_model(&self) -> anyhow::Result<Box<dyn Model>> {
        let model_architecture = llm::ModelArchitecture::Llama;
        let model_path = PathBuf::from("./assets/open_llama_3b-f16.bin");
        let tokenizer_source = llm::TokenizerSource::Embedded;

        Ok(llm::load_dynamic(
            Some(model_architecture),
            &model_path,
            tokenizer_source,
            Default::default(),
            llm::load_progress_callback_stdout,
        )?)
    }
}
