use global::config::{CAConfig, JWTConfig};
use global::io::Log;
use rust_i18n::t;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::spawn;
use tokio::task::JoinHandle;
use tokio_rustls::rustls::pki_types::{CertificateDer, PrivateKeyDer};
use tokio_rustls::rustls::server::WebPkiClientVerifier;
use tokio_rustls::rustls::{RootCertStore, ServerConfig};
use tokio_rustls::TlsAcceptor;

pub async fn start_net_service(
	addr: String,
	port: u16,
	ca: &CAConfig,
	jwt: &JWTConfig
) -> Result<JoinHandle<Result<(), std::io::Error>>, Box<dyn std::error::Error>> {
    // 加载 CA 证书
    let cas = load_certs(&ca.cert_ca)?;
    let mut client_auth_roots = RootCertStore::empty();
    let mut count = 0;
    for ca in cas {
        client_auth_roots.add(ca)?;
        count += 1;
    }
    Log::i(
        t!("tag.netController_service.self"),
        t!("log.netService.ca.load", count = count),
    ).print();
    // 创建允许双向认证的校验器
    let client_verifier = WebPkiClientVerifier::builder(client_auth_roots.into()).build()?;
    // 服务端配置
    let private_cert = load_certs(&ca.cert_server)?;
    let private_key = load_key(&ca.cert_key)?;
    let config = ServerConfig::builder()
        .with_client_cert_verifier(client_verifier)
        .with_single_cert(private_cert, private_key)?;
	let acceptor = TlsAcceptor::from(Arc::new(config));
	let listener = TcpListener::bind(format!("{}:{}", addr, port)).await?;
	Log::i(t!("tag.netController_service"), t!("log.netService.start", port = port)).print();
	let handle = spawn(async move {
		loop {
			let (stream, _peer_addr) = listener.accept().await?;
			let acceptor = acceptor.clone();
			spawn(async move {
				match acceptor.accept(stream).await {
					Ok(tls_stream) => {
						// 握手成功：此时 mTLS 已确保客户端持有合法的 CA 证书
						handle_client_connection(tls_stream).await;
					}
					Err(e) => Log::e("TLS", &format!("Accept error: {}", e)).print(),
				}
			});
		}
	});

	Ok(handle)
}

async fn handle_client_connection(mut tls_stream: tokio_rustls::server::TlsStream<tokio::net::TcpStream>) {
	// 这里是你之后处理 JWT 验证和业务逻辑的地方
	Log::i("Net", "New client connected via mTLS").print();
}

fn load_certs(path: &str) -> std::io::Result<Vec<CertificateDer<'static>>> {
    let file = std::fs::File::open(path)?;
    let mut reader = std::io::BufReader::new(file);
    rustls_pemfile::certs(&mut reader).collect()
}

fn load_key(path: &str) -> std::io::Result<PrivateKeyDer<'static>> {
    let file = std::fs::File::open(path)?;
    let mut reader = std::io::BufReader::new(file);
    rustls_pemfile::private_key(&mut reader)?
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "No private key found"))
}
