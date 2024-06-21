pub mod user;
use user::user::User;

#[cfg(test)]
mod tests {
    use user::user::UserRoles;

    use super::*;

    #[test]
    fn test_user_default_role() {
        let new_user = User::default();
        assert_eq!(new_user.get_role(), UserRoles::Normal);
    }

    #[test]
    fn test_new_user() {
        let new_user = User::new(
            String::from("Rilwan"),
            String::from("default_password"),
            String::from("useruser@gmail.com"),
            UserRoles::Admin,
        );
        assert_eq!(new_user.get_role(), UserRoles::Admin);
        assert_eq!(new_user.get_name(), "Rilwan");
        assert_eq!(new_user.get_password(), "default_password");
    }

    #[test]
    fn test_new_user_full() {
        let new_user = User::new_full(
            uuid::Uuid::new_v4(),
            String::from("Rilwan"),
            String::from("useruser@gmail.com"),
            String::from("default_password"),
            UserRoles::Admin,
            false,
        );
        assert_eq!(new_user.get_role(), UserRoles::Admin);
        assert_eq!(new_user.get_name(), "Rilwan");
        assert_eq!(new_user.get_password(), "default_password");
        assert_eq!(new_user.get_email(), "useruser@gmail.com");
        assert_eq!(new_user.get_confirmed_status(), false);
    }
}
