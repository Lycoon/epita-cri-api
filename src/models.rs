use serde::Deserialize;

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Group {
    pub url: String,
    pub slug: String,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Campus {
    pub url: String,
    pub slug: String,
    pub group: Group,
    pub name: String,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct SimpleCRIUser {
    pub url: String,
    pub login: String,
    pub old_accounts: Vec<String>,
    pub new_account: Option<String>,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct CRIGroup {
    pub url: String,
    pub slug: String,
    pub gid: Option<u16>,
    pub name: String,
    pub kind: String,
    pub members_url: String,
    pub history_url: String,
    pub managers: Vec<SimpleCRIUser>,
    pub private: bool,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct CRIComputedMembership {
    pub group: Group,
    pub user: Option<SimpleCRIUser>,
    pub begin_at: String,
    pub end_at: Option<String>,
    pub graduation_year: Option<u32>,
    pub is_current: bool,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct CRIUser {
    pub url: String,
    pub login: String,
    pub uid: u16,
    pub primary_group: Group,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub groups_history: Vec<CRIComputedMembership>,
    pub current_groups: Vec<Group>,
    pub old_accounts: Vec<String>,
    pub new_account: Option<String>,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Profile {
    pub url: String,
    pub login: String,
    pub uid: u16,
    pub primary_group: Vec<Group>,
    pub first_name: String,
    pub last_name: String,
    pub legal_first_name: String,
    pub legal_last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub birthdate: Option<String>,
    pub groups_history: Vec<CRIComputedMembership>,
    pub current_groups: Vec<Group>,
    pub old_accounts: Vec<String>,
    pub new_account: Option<String>,
}