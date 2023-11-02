/* Inference */

use std::{convert::Infallible, io::Write};

fn infer(prompt: impl AsRef<str>) -> anyhow::Result<()> {
    // What model we are using?
    let model_architecture = llm::ModelArchitecture::Llama;
    // Path of model binary we are using
    let model_path = std::path::PathBuf::from(
        "/home/tan/Documents/Projects/llm_task/my_llm/open_llama_3b-f16.bin",
    );
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
    let mut session = model.start_session(Default::default());

    let result = session.infer::<Infallible>(
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
    let prompt: &str = "What do you think about Rust lang?";

    infer(prompt).unwrap();
}

/* Embeddings */

// use clap::Parser;
//
// #[derive(Parser)]
// struct Args {
//     #[arg(long, short = 'q')]
//     pub query: Option<String>,
//     #[arg(long, short = 'c')]
//     pub comparands: Vec<String>,
// }
//
// fn main() {
//     let args = Args::parse();
//
//     let tokenizer_source = llm::TokenizerSource::Embedded;
//     let model_architecture = llm::ModelArchitecture::Llama;
//     let model_path = std::path::PathBuf::from(
//         "/home/tan/Documents/Projects/llm_task/my_llm/open_llama_3b-f16.bin",
//     );
//     let query = args
//         .query
//         .as_deref()
//         .unwrap_or("My favourite animal is the dog");
//     let comparands = match args.comparands.is_empty() {
//         true => vec![
//             "My favourite animal is the dog".to_string(),
//             "I have just adopted a cute dog".to_string(),
//             "My favourite animal is the cat".to_string(),
//         ],
//         false => args.comparands,
//     };
//
//     // Load model
//     let model_params = llm::ModelParameters::default();
//     let model = llm::load_dynamic(
//         Some(model_architecture),
//         &model_path,
//         tokenizer_source,
//         model_params,
//         llm::load_progress_callback_stdout,
//     )
//     .unwrap_or_else(|err| {
//         panic!("Failed to load {model_architecture} model from {model_path:?}: {err}")
//     });
//     let inference_parameters = llm::InferenceParameters::default();
//
//     // Generate embeddings for query and comparands
//     let query_embeddings = get_embeddings(model.as_ref(), &inference_parameters, query);
//     let comparand_embeddings: Vec<(String, Vec<f32>)> = comparands
//         .iter()
//         .map(|text| {
//             (
//                 text.clone(),
//                 get_embeddings(model.as_ref(), &inference_parameters, text),
//             )
//         })
//         .collect();
//
//     // Print embeddings
//     fn print_embeddings(text: &str, embeddings: &[f32]) {
//         println!("{text}");
//         println!("  Embeddings length: {}", embeddings.len());
//         println!("  Embeddings first 10: {:.02?}", embeddings.get(0..10));
//     }
//
//     print_embeddings(query, &query_embeddings);
//     println!("---");
//     for (text, embeddings) in &comparand_embeddings {
//         print_embeddings(text, embeddings);
//     }
//
//     // Calculate the cosine similarity between the query and each comparand, and sort by similarity
//     let mut similarities: Vec<(&str, f32)> = comparand_embeddings
//         .iter()
//         .map(|(text, embeddings)| {
//             (
//                 text.as_str(),
//                 cosine_similarity(&query_embeddings, embeddings),
//             )
//         })
//         .collect();
//     similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
//
//     // Print similarities
//     println!("---");
//     println!("Similarities:");
//     for (text, score) in similarities {
//         println!("  {text}: {score}");
//     }
// }
//
// fn get_embeddings(
//     model: &dyn llm::Model,
//     inference_parameters: &llm::InferenceParameters,
//     query: &str,
// ) -> Vec<f32> {
//     let mut session = model.start_session(Default::default());
//     let mut output_request = llm::OutputRequest {
//         all_logits: None,
//         embeddings: Some(Vec::new()),
//     };
//     let vocab = model.tokenizer();
//     let beginning_of_sentence = true;
//     let query_token_ids = vocab
//         .tokenize(query, beginning_of_sentence)
//         .unwrap()
//         .iter()
//         .map(|(_, tok)| *tok)
//         .collect::<Vec<_>>();
//     model.evaluate(&mut session, &query_token_ids, &mut output_request);
//     output_request.embeddings.unwrap()
// }
//
// fn cosine_similarity(v1: &[f32], v2: &[f32]) -> f32 {
//     let dot_product = dot(v1, v2);
//     let magnitude1 = magnitude(v1);
//     let magnitude2 = magnitude(v2);
//
//     dot_product / (magnitude1 * magnitude2)
// }
//
// fn dot(v1: &[f32], v2: &[f32]) -> f32 {
//     v1.iter().zip(v2.iter()).map(|(&x, &y)| x * y).sum()
// }
//
// fn magnitude(v: &[f32]) -> f32 {
//     v.iter().map(|&x| x * x).sum::<f32>().sqrt()
// }

// TODO: Use clap for the taking prompts and path of model
