# Peel NAT

> 本项目旨在创建一个基于Rust语言的分布式网络系统，帮助客户端进行远程互联，并帮助客户端进行内外穿透、NAT打洞及P2P网络互联

## 项目结构

```
peel/
├── crates/   ------------------------ 模块
│   ├── controller_core/   ----------- (lib) 控制台服务 -> 客户端接口服务
│   ├── controller_tui/   -------------- (bin) 客户端TUI控制台
│   ├── controller_web_ui/   --------- (lib) WebUI控制台服务
│   ├── core_global/   --------------- (lib) 公共库
│   ├── core_server/   ----------------- (bin) 核心服务
│   ├── net_client/   ---------------- (lib) 客户端库 -> PeeL-Client
│   └── net_server/   ---------------- (lib) 面向普通网络客户端的网络服务
└── Cargo.toml
```

## 开始使用

> 本项目支持自建服务器，也可以使用官方服务器

### 配置管理员密钥，推荐采用OpenSSL生成

```bash
# 定义路径
export KEY_PATH=/path/to/key

# 生成根 CA，可同时用于WebUI和TUI客户端的登录
openssl genrsa -out "$KEY_PATH/rootCA.key" 4096
openssl req -x509 -new -nodes -key "$KEY_PATH/rootCA.key" -sha256 -days 1024 -out "$KEY_PATH/rootCA.crt"

# 为服务端签发证书

openssl genrsa -out "$KEY_PATH/server.key" 2048
openssl req -new -key "$KEY_PATH/server.key" -out "$KEY_PATH/server.csr"
openssl x509 -req -in "$KEY_PATH/server.csr" -CA "$KEY_PATH/rootCA.crt" -CAkey "$KEY_PATH/rootCA.key" -CAcreateserial -out "$KEY_PATH/server.crt" -days 500 -sha256

# 为客户端签发证书
openssl genrsa -out "$KEY_PATH/client.key" 2048
openssl req -new -key "$KEY_PATH/client.key" -out "$KEY_PATH/client.csr"
openssl x509 -req -in "$KEY_PATH/client.csr" -CA "$KEY_PATH/rootCA.crt" -CAkey "$KEY_PATH/rootCA.key" -CAcreateserial -out "$KEY_PATH/client.crt" -days 500 -sha256

# 将客户端证书和私钥打包为 .p12 格式以便安卓系统识别
openssl pkcs12 -export \
    -out "$KEY_PATH/client_android.p12" \
    -inkey "$KEY_PATH/client.key" \
    -in "$KEY_PATH/client.crt" \
    -certfile "$KEY_PATH/rootCA.crt" \
    -name "client_cert" \
    -passout pass:123456 \
    -legacy
```

**浏览器访问提示**

1. 手动将 rootCA.crt 导入浏览器或安卓设备的“受信任的根证书颁发机构（CA）”

2. 双击并导入 client.p12（安装时需输入生成时设置的密码）

3. 访问 https://Your_server_ip:9771，浏览器会弹出窗口要求选择证书，确认即可

### 打开配置文件，填入相应信息

```toml
# 根证书
cert_ca = "/path/to/rootCA.crt" # 填入根证书路径 `$KEY_PATH/rootCA.crt`
# 服务端证书
cert_server = "/path/to/server.crt" # 填入服务端证书路径 `$KEY_PATH/core_server.crt`
# 服务端密钥 严格保密
cert_key = "/path/to/server.key" # 填入服务端密钥路径 `$KEY_PATH/core_server.key`
```
