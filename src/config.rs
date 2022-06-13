
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Config {
	pub wt_base_path: String,
	pub target_path: String,
	pub wt_tools_path: String,
}