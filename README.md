安装wasm工具链
rustup target add wasm32-unknown-unknown

构建项目
RUSTFLAGS="-C link-arg=-s" cargo build --target wasm32-unknown-unknown --release

部署合约
near contract deploy crypto-keep.testnet use-file ./target/wasm32-unknown-unknown/release/crypto_keep.wasm   without-init-call network-config testnet sign-with-keychain send


添加密码
near contract call-function as-transaction crypto-keep.testnet add_password json-args  '{"account_id":"chenyutong.testnet","encrypted_password_info":"QtIgl58sVzmXaOp3iqQr7lc6UJEERd3o5p7we6jRs7pvIAtaT8S5W/7AZJE7YYb9w30nPakpIB4+RPyDVA6olJHdTSR2LtisT562d1AaiNJIS7xN6aXahWv4L2EavAFpdc1QSZIOBZs9DEmrjArtUzSHVi93/X0VtJ1IF3iBvw=="}'  prepaid-gas '100.000 TeraGas' attached-deposit '0 NEAR' sign-as chenyutong.testnet network-config testnet sign-with-keychain send

删除密码
near contract call-function as-transaction crypto-keep.testnet delete_password json-args  '{"account_id":"chenyutong.testnet","index":0}'  prepaid-gas '100.000 TeraGas' attached-deposit '0 NEAR' sign-as chenyutong.testnet network-config testnet sign-with-keychain send

查询密码
near contract call-function as-read-only crypto-keep.testnet get_passwords  json-args '{"account_id":"chenyutong.testnet"}'  network-config testnet  now


----
用户自己创建自己的合约，工厂合约

对每一条数据，整体进行加密，然后进行存储。

查询的时候，进行解密，显示结果。


数据结构修改如下所示
pub struct PasswordInfo {
    name: String,
    encrypted_password: String,
    website: String,
    note: String,
}

需要实现对PasswordInfo加密和解密。当使用add_password 添加密码的时候，进行加密然后存储到链上。当使用 get_password 获取密码的时候，进行解密展示出来。

要使每个用户可以存储多个密码，并且能够对每个密码进行单独的管理（添加、更新、获取和删除）