use anyhow::Result;
use hf_hub::api::tokio::Api; // Use tokio API
use std::path::PathBuf;

async fn download_model_if_needed() -> Result<PathBuf> {
    let api = Api::new()?;
    // Switching to the alternative public model
    let repo_id = "lmz/candle-quantized-phi".to_string();
    let model_filename = "model-v1-q4k.gguf";

    println!("Downloading ./{model_filename} from {repo_id}...");
    let model_path = api.model(repo_id).download(model_filename).await?;
    Ok(model_path)
}

#[tokio::main]
async fn main() -> Result<()> {
    match download_model_if_needed().await {
        Ok(path) => {
            println!("Model downloaded to: {:?}", path);
        }
        Err(e) => {
            eprintln!("Error downloading model: {:?}", e);
            return Err(e);
        }
    }
    Ok(())
}
