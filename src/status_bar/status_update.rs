
#[derive(serde::Serialize)]
pub struct StatusUpdate{
    pub full_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_top: Option<isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_right: Option<isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_bottom: Option<isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_left: Option<isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_width: Option<isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator_block_width: Option<isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markup: Option<String>
}

impl StatusUpdate{
    pub fn to_json_string(&self) -> String{
        serde_json::to_string(self).unwrap()
    }
}
