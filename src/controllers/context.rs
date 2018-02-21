use serde::Serialize;

#[derive(Serialize)]
pub struct IndexTemplateContext<TModel: Serialize, TData: Serialize> {
    pub model: Vec<TModel>,
    pub flash: Option<String>,
    pub extra_data: TData,
}

#[derive(Serialize)]
pub struct TemplateContext<TModel: Serialize, TData: Serialize> {
    pub model: TModel,
    pub flash: Option<String>,
    pub extra_data: TData,
}
