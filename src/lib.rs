use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
use near_sdk::collections::LookupMap;
use serde::{Deserialize, Serialize};

// 合约结构和初始化
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct CryptoKeep {
    passwords: LookupMap<String, PasswordInfo>, // 用于存储密码的映射
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct PasswordInfo {
    encrypted_password: String,
    website: String,
    note: String,
    tags: Vec<String>,
}

impl PasswordInfo {
    pub fn new(encrypted_password: String, website: String, note: String, tags: Vec<String>) -> Self {
        PasswordInfo {
            encrypted_password,
            website,
            note,
            tags,
        }
    }
}

impl Default for CryptoKeep {
    fn default() -> Self {
        Self { 
            passwords: LookupMap::new(b"p"),
        }
    }
}


#[near_bindgen]
impl CryptoKeep {

    // #[init]
    // pub fn init() -> Self {
    //     Self {
    //         passwords: LookupMap::new(b"p"),
    //     }
    // }

    // 创建密码
    pub fn add_password(&mut self, account_id: String, encrypted_password: String, website: String, note: String, tags: Vec<String>) {
        let password_info = PasswordInfo::new(encrypted_password, website, note, tags);
        assert!(
            self.passwords.get(&account_id).is_none(),
            "An entry already exists for this account."
        );
        self.passwords.insert(&account_id, &password_info);
    }

    // 更新密码
    pub fn update_password(&mut self, account_id: String, new_encrypted_password: String, new_website: String, new_note: String, new_tags: Vec<String>) {
        match self.passwords.get(&account_id) {
            Some(mut password_info) => {
                password_info.encrypted_password = new_encrypted_password;
                password_info.website = new_website;
                password_info.note = new_note;
                password_info.tags = new_tags;
                self.passwords.insert(&account_id, &password_info);
            },
            None => env::panic_str("No password entry exists for this account."),
        }
    }

    // 获取密码
    pub fn get_password(&self, account_id: String) -> Option<PasswordInfo> {
        self.passwords.get(&account_id)
    }

    // 删除密码
    pub fn delete_password(&mut self, account_id: String) {
        self.passwords.remove(&account_id);
    }

}


