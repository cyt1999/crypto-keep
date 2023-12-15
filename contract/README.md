**安装wasm工具链**

rustup target add wasm32-unknown-unknown

**构建项目**

RUSTFLAGS="-C link-arg=-s" cargo build --target wasm32-unknown-unknown --release

**部署合约**

near contract deploy crypto-keep.testnet use-file ./target/wasm32-unknown-unknown/release/crypto_keep.wasm   without-init-call network-config testnet sign-with-keychain send


**添加密码**

near contract call-function as-transaction crypto-keep.testnet add_password json-args  '{"account_id":"chenyutong.testnet","encrypted_password_info":"QtIgl58sVzmXaOp3iqQr7lc6UJEERd3o5p7we6jRs7pvIAtaT8S5W/7AZJE7YYb9w30nPakpIB4+RPyDVA6olJHdTSR2LtisT562d1AaiNJIS7xN6aXahWv4L2EavAFpdc1QSZIOBZs9DEmrjArtUzSHVi93/X0VtJ1IF3iBvw=="}'  prepaid-gas '100.000 TeraGas' attached-deposit '0 NEAR' sign-as chenyutong.testnet network-config testnet sign-with-keychain send

**删除密码**

near contract call-function as-transaction crypto-keep.testnet delete_password json-args  '{"account_id":"chenyutong.testnet","index":0}'  prepaid-gas '100.000 TeraGas' attached-deposit '0 NEAR' sign-as chenyutong.testnet network-config testnet sign-with-keychain send

**查询密码**

near contract call-function as-read-only crypto-keep.testnet get_passwords  json-args '{"account_id":"chenyutong.testnet"}'  network-config testnet  now
