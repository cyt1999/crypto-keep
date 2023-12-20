use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
use near_sdk::collections::LookupMap;

// 合约结构和初始化
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct CryptoKeep {
    passwords: LookupMap<String, Vec<String>>, // 映射到字符串的向量
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

    // 创建密码   
    pub fn add_password(&mut self, account_id: String, encrypted_password_info: String) {
        let mut passwords_list = self.passwords.get(&account_id).unwrap_or_else(Vec::new);
        passwords_list.push(encrypted_password_info);
        self.passwords.insert(&account_id, &passwords_list);
    }
    
    
    // 更新密码
    pub fn update_password(&mut self, account_id: String, index: usize, new_encrypted_password_info: String) {
        let mut passwords_list = self.passwords.get(&account_id).unwrap_or_else(Vec::new);
        if passwords_list.get(index).is_some() {
            passwords_list[index] = new_encrypted_password_info;
            self.passwords.insert(&account_id, &passwords_list);
        } else {
            env::panic_str("No password entry exists at the specified index for this account.");
        }
    }
    

    // 获取密码
    pub fn get_passwords(&self, account_id: String)  -> Vec<(usize, String)> {
        self.passwords.get(&account_id).unwrap_or_else(Vec::new)
        .into_iter().enumerate().collect()
    }
    

    // 删除密码
    pub fn delete_password(&mut self, account_id: String, index: usize) {
        let mut passwords_list = self.passwords.get(&account_id).unwrap_or_else(Vec::new);
        if passwords_list.get(index).is_some() {
            passwords_list.remove(index);
            self.passwords.insert(&account_id, &passwords_list);
        } else {
            env::panic_str("No password entry exists at the specified index for this account.");
        }
    }
    

}


