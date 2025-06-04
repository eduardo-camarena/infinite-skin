use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CreateLibraryDTO {
    pub name: String,
    pub location: String,
    pub is_private: bool,
}

#[derive(Deserialize, Debug)]
pub struct ScanMediaFolderDTO {
    pub library_ids: Option<Vec<i32>>,
}

#[derive(Deserialize, Debug)]
pub struct GetPossibleFoldersDTO {
    pub path: String,
}
