use ollama_rs::Ollama;
use ollama_rs::error::OllamaError;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::generation::completion::GenerationResponse;

// GenerationResponse を返す関数
async fn request_ollama_raw(prompt: &str, model: Option<&str>) -> Result<GenerationResponse, OllamaError> {
    let ollama = Ollama::default();
    let model = model.unwrap_or("rnj-1:latest");
    ollama.generate(GenerationRequest::new(model.to_string(), prompt)).await
}

// res.response を返す関数
pub async fn request_ollama(prompt: &str, model: Option<&str>) -> String {
    match request_ollama_raw(prompt, model).await {
        Ok(res) => res.response,
        Err(_) => "Failed to get response".to_string(),
    }
}

// pub async fn request_ollama_dummy(prompt: &str, model: Option<&str>) -> String {
//     String::from("dummy ollama method")
// }
