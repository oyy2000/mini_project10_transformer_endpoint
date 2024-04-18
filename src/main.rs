use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};
use std::convert::Infallible;
use std::io::Write;
use std::path::PathBuf;

fn generate_response(prompt: String) -> Result<String, Box<dyn std::error::Error>> {
    // Set the model's details and the tokenizer
    let tokenizer = llm::TokenizerSource::Embedded;
    let architecture = llm::ModelArchitecture::GptNeoX;

    // Define model's path based on environment
    let model_file = PathBuf::from("src/pythia-1b-q4_0-ggjt.bin");

    let model = llm::load_dynamic(
        Some(architecture),
        &model_file,
        tokenizer,
        Default::default(),
        llm::load_progress_callback_stdout,
    )?;

    let mut session = model.start_session(Default::default());
    let mut response_text = String::new();

    let inference_result = session.infer::<Infallible>(
        model.as_ref(),
        &mut rand::thread_rng(),
        &llm::InferenceRequest {
            prompt: (&prompt).into(),
            parameters: &llm::InferenceParameters::default(),
            play_back_previous_tokens: false,
            maximum_token_count: Some(15),
        },
        &mut Default::default(),
        |response| match response {
            llm::InferenceResponse::PromptToken(token)
            | llm::InferenceResponse::InferredToken(token) => {
                print!("{token}");
                std::io::stdout().flush().unwrap();
                response_text.push_str(&token);
                Ok(llm::InferenceFeedback::Continue)
            }
            _ => Ok(llm::InferenceFeedback::Continue),
        },
    );

    // Handle inference result
    match inference_result {
        Ok(_) => Ok(response_text),
        Err(e) => Err(Box::new(e)),
    }
}

async fn handle_request(req: Request) -> Result<Response<Body>, Error> {
    let user_query = req
        .query_string_parameters_ref()
        .and_then(|params| params.first("query"))
        .unwrap_or("Today is a");

    let output_message = match generate_response(user_query.to_string()) {
        Ok(result) => result,
        Err(e) => format!("Inference error: {:?}", e),
    };
    println!("Response from model: {:?}", output_message);

    let response = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(Body::from(output_message.replace("<|padding|>", "")))
        .map_err(Box::new)?;

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    run(service_fn(handle_request)).await
}
