安装wasm工具链
rustup target add wasm32-unknown-unknown

构建项目
RUSTFLAGS="-C link-arg=-s" cargo build --target wasm32-unknown-unknown --release

部署合约
near contract deploy crypto-keep.testnet use-file ./target/wasm32-unknown-unknown/release/crypto_keep.wasm   without-init-call network-config testnet sign-with-keychain send


添加密码
near contract call-function as-transaction crypto-keep.testnet add_password json-args  '{"account_id":"chenyutong.testnet","encrypted_password":"dsfasdfkiewr2341","website":"https://www.baidu.com","note":"baidu chenyutong","tags":["a","b"]}'  prepaid-gas '100.000 TeraGas' attached-deposit '0 NEAR' sign-as chenyutong.testnet network-config testnet sign-with-keychain send

near contract call-function as-transaction crypto-keep.testnet add_password json-args  '{"account_id":"xiamu.testnet","encrypted_password":"dsfasdfkiewr2341","website":"https://www.baidu.com","note":"baidu chenyutong","tags":["a","b"]}'  prepaid-gas '100.000 TeraGas' attached-deposit '0 NEAR' sign-as chenyutong.testnet network-config testnet sign-with-keychain send 

查询密码
near contract call-function as-read-only crypto-keep.testnet get_password  json-args '{"account_id":"chenyutong.testnet"}'  network-config testnet now