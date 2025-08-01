use askama::Template;
use crate::models::{UserProfile, JournalListItem, JournalDetails, User, Journal};

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    pub title: String,
    pub journals: Vec<JournalListItem>,
    pub user: Option<User>,
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    pub title: String,
    pub error: Option<String>,
    pub user: Option<User>,
}

#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate {
    pub title: String,
    pub error: Option<String>,
    pub user: Option<User>,
}

#[derive(Template)]
#[template(path = "profile.html")]
pub struct ProfileTemplate {
    pub title: String,
    pub profile: UserProfile,
    pub user: Option<User>,
}

#[derive(Template)]
#[template(path = "profile_edit.html")]
pub struct ProfileEditTemplate {
    pub title: String,
    pub profile_user: User,
    pub user: Option<User>,
}

#[derive(Template)]
#[template(path = "journal_list.html")]
pub struct JournalListTemplate {
    pub title: String,
    pub journals: Vec<JournalListItem>,
    pub user: Option<User>,
}

#[derive(Template)]
#[template(path = "journal_show.html")]
pub struct JournalShowTemplate {
    pub title: String,
    pub journal: JournalDetails,
    pub user: Option<User>,
}

#[derive(Template)]
#[template(path = "journal_new.html")]
pub struct JournalNewTemplate {
    pub title: String,
    pub error: Option<String>,
    pub user: Option<User>,
}

#[derive(Template)]
#[template(path = "journal_edit.html")]
pub struct JournalEditTemplate {
    pub title: String,
    pub journal: Journal,
    pub error: Option<String>,
    pub user: Option<User>,
}

#[derive(Template)]
#[template(path = "my_journals.html")]
pub struct MyJournalsTemplate {
    pub journals: Vec<Journal>,
    pub user: Option<User>,
}
