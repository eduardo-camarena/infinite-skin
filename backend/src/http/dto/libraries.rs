use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateLibraryDTO {
    pub name: String,
    pub location: String,
    pub is_private: bool,
}

#[derive(Deserialize, Debug)]
pub struct ScanMediaFolderDTO {
    pub libraries: Option<Vec<i32>>,
}
