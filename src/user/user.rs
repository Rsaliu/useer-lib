use serde::{de::Error, Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;
use once_cell::sync::Lazy;
use regex::Regex;


#[derive(Default,Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRoles{
    #[default]
    Normal,
    Admin
}

static USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9A-Za-z_]+$").unwrap());
static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap());
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize,Validate)]
#[serde(rename_all = "camelCase")]

pub struct User
{
    id: Uuid,
    #[validate(length(min = 3, max = 16), regex = "USERNAME_REGEX")]
    name:String,
    #[validate(length(min = 10, max = 100), regex = "USERNAME_REGEX")]
    email:String,
    #[validate(length(min = 8, max = 32))]
    password:String,
    user_role:UserRoles,
    confirmed:bool
}


impl User{
    pub fn new(name:String,password:String,email:String,user_role:UserRoles)->Self{
        User {id:Uuid::nil(),name,email, password, user_role,confirmed:false}
    }
    pub fn new_full(id:uuid::Uuid,name:String,email:String,password:String,user_role:UserRoles,confirmed:bool)->Self{
        User { id, name,email,password, user_role,confirmed}
    }
    pub fn get_name(&self)->&str{
        self.name.as_str()
    }
    pub fn get_id(&self)->Uuid{
        self.id
    }
    pub fn get_password(&self)->&str{
        self.password.as_str()
    }
    pub fn get_role(&self)->UserRoles{
        self.user_role.clone()
    }
    pub fn get_email(&self)->&str{
        self.email.as_str()
    }
    pub fn get_confirmed_status(&self)->bool{
        self.confirmed
    }

}







