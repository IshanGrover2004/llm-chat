// TODO: Use clap for the arguments

fn infer(prompt: impl AsRef<str>) -> anyhow::Result<String> {
    // What model we are using?
    let model_architecture = llm::ModelArchitecture::Llama;
    // Path of model binary we are using
    let model_path = std::path::PathBuf::from("./../open_llama_3b-f16.bin");
    // Path of tokenizer from llm
    let tokenizer_source = llm::TokenizerSource::Embedded;
    // The prompt to ask
    let prompt = prompt.as_ref().to_string();
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
    let session = model.start_session(Default::default());
    // Accumulate the tokens here
    let mut tokens = String::new();

    todo!();
}

fn main() {
    let prompt: &str = "What do you think about Rust lang?";

    let response = infer(prompt);
    dbg!(response);
}
