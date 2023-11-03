// Run: `cargo run --release -- -p "What do you think about Rust lang?" -m ./open_llama_3b-f16.bin`

use std::{convert::Infallible, io::Write};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    model_path: std::path::PathBuf,

    #[clap(short, long)]
    prompt: String,
}

fn infer() -> anyhow::Result<()> {
    let args = Args::parse();

    // What model we are using?
    let model_architecture = llm::ModelArchitecture::Llama;
    // Path of model binary we are using
    let model_path = args.model_path;
    // Path of tokenizer from llm
    let tokenizer_source = llm::TokenizerSource::Embedded;
    // The prompt to ask
    let prompt = args.prompt;
    // Time at which program is executed
    let now_time = std::time::Instant::now();

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

    let result = session.infer::<Infallible>(
        // model to use
        model.as_ref(),
        &mut rand::thread_rng(),
        // Request for prompt
        &llm::InferenceRequest {
            prompt: (&prompt).into(),
            parameters: &llm::InferenceParameters::default(),
            play_back_previous_tokens: false,
            maximum_token_count: None,
        },
        // Output request
        &mut Default::default(),
        // Inference response
        |r| match r {
            llm::InferenceResponse::PromptToken(t) | llm::InferenceResponse::InferredToken(t) => {
                print!("{t}");
                std::io::stdout().flush().unwrap();

                Ok(llm::InferenceFeedback::Continue)
            }
            _ => Ok(llm::InferenceFeedback::Continue),
        },
    );

    match result {
        Ok(result) => println!("Inference stats: \n {result}"),
        Err(err) => eprintln!("{err}"),
    }

    Ok(())
}

fn main() {
    infer().unwrap();
}
