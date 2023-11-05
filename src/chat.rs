use std::{convert::Infallible, io::Write, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Prompt {
    prompt: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Response {
    response: String,
}

impl Response {
    pub fn new(response: String) -> Self {
        Self { response }
    }

    pub fn response(&self) -> String {
        self.response.to_owned()
    }
}

impl Prompt {
    pub fn new(prompt: Option<String>) -> Self {
        Self { prompt }
    }

    pub fn prompt(&self) -> Option<String> {
        self.prompt.to_owned()
    }

    pub fn infer(&self) -> anyhow::Result<Response> {
        // What model to use
        let model_architecture = llm::ModelArchitecture::Llama;
        // Path of model binary
        let model_path =
            PathBuf::from("/home/tan/Documents/Projects/llm_task/my_llm/open_llama_3b-f16.bin");
        // Path of tokenizer
        let tokenizer_source = llm::TokenizerSource::Embedded;
        // Prompt to ask
        let prompt = self.prompt.as_deref().unwrap();
        // Time at which program is executed
        let now_time = std::time::Instant::now();

        // Loading model with necessary details
        let model = llm::load_dynamic(
            Some(model_architecture),
            &model_path,
            tokenizer_source,
            Default::default(),
            llm::load_progress_callback_stdout,
        )?;

        println!(
            "Model fully loaded! Elapsed: {}ms",
            now_time.elapsed().as_millis()
        );

        // Starting session
        let mut session = model.start_session(Default::default());

        let inference_request = llm::InferenceRequest {
            prompt: (prompt).into(),
            parameters: &llm::InferenceParameters::default(),
            play_back_previous_tokens: false,
            maximum_token_count: Some(100),
        };

        let mut result = String::new();

        session.infer::<Infallible>(
            // Model to use
            model.as_ref(),
            &mut rand::thread_rng(),
            // Request for inference
            &inference_request,
            // Output request
            &mut Default::default(),
            // Inference response
            |r| match r {
                llm::InferenceResponse::PromptToken(t)
                | llm::InferenceResponse::InferredToken(t) => {
                    print!("{t}");
                    result.push_str(&t);

                    std::io::stdout().flush().unwrap();

                    Ok(llm::InferenceFeedback::Continue)
                }
                _ => Ok(llm::InferenceFeedback::Continue),
            },
        )?;

        Ok(Response::new(result.to_string()))
    }
}
