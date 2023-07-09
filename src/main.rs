mod password_manager {
    use std::collections::HashMap;

    pub struct Locked;
    pub struct Unlocked;

    pub struct PasswordManager<State = Locked> {
        master_password: String,
        passwords: HashMap<String, String>,
        state: State,
    }

    impl PasswordManager {
        pub fn new(master_password: String) -> Self {
            PasswordManager {
                master_password,
                passwords: Default::default(),
                state: Locked,
            }
        }
    }

    impl<State> PasswordManager<State> {
        pub fn version(&self) -> &str {
            "this-is-the-version"
        }

        pub fn encryption(&self) -> &str {
            "this-is-the-encryption"
        }
    }

    impl PasswordManager<Locked> {
        pub fn unlock(self) -> PasswordManager<Unlocked> {
            PasswordManager {
                master_password: self.master_password,
                passwords: self.passwords,
                state: Unlocked,
            }
        }
    }

    impl PasswordManager<Unlocked> {
        pub fn lock(self) -> PasswordManager<Locked> {
            PasswordManager {
                master_password: self.master_password,
                passwords: self.passwords,
                state: Locked,
            }
        }

        pub fn list_passwords(&self) -> &HashMap<String, String> {
            &self.passwords
        }

        pub fn add_password(&mut self, username: String, password: String) {
            self.passwords.insert(username, password);
        }
    }
}

fn main() {
    // Creating a API that uses generics to help the user to user call the right functions for
    // password manager

    // I also wrapped the PasswordManager inside a `mod password_manager` to simulate what it would
    // be like for a API user to be calling the API not within the same module.

    // Create a password manager in the default locked state
    let manager = password_manager::PasswordManager::new("my-master-password".to_string());
    // At this point, we only have access to the unlock function on a locked password manager.
    // Plus the default version and encryption functions.
    let mut manager = manager.unlock();

    // After unlocking, we've access to more functions we didn't have before.
    manager.list_passwords();
    manager.version();
    manager.add_password("user".to_string(), "pass".to_string());
    manager.list_passwords();

    // Finally, lock the password manager again.
    let manager = manager.lock();
    manager.encryption();
}
