use anyhow::Context;

#[derive(Debug, serde::Deserialize, Clone)]
pub enum ResponseType {
    Text,
    Function,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct CustomResponse {
    pub response_type: ResponseType,
    trigger: String,
    pub response: String,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct CustomResponses(Vec<CustomResponse>);

impl CustomResponses {
    pub fn from_file(path: Option<std::path::PathBuf>) -> anyhow::Result<Self> {
        tracing::debug!("{:?}", path);
        match path {
            Some(path) => std::fs::read_to_string(path)
                .with_context(|| "Could not read path")
                .and_then(|file| {
                    serde_json::from_str(&file)
                        .with_context(|| "Could not parse file into custom responses")
                })
                .map(CustomResponses),
            None => Ok(CustomResponses(Vec::new())),
        }
    }

    pub fn search_matching(&self, trigger: &String) -> Option<CustomResponse> {
        self.0
            .iter()
            .find(|item| &item.trigger == trigger)
            .map(|item| item.clone())
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct FunctionOutput {
    pub name: String,
    pub arguments: serde_json::Value,
}

#[derive(Debug, serde::Deserialize)]
pub struct FunctionOutputs(Vec<FunctionOutput>);

impl FunctionOutputs {
    pub fn iter(&self) -> impl Iterator<Item = &FunctionOutput> {
        self.0.iter()
    }
}
