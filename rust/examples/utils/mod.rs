use std::error::Error;

use aristech_tts_client::{Auth, TlsOptions};

/// This function checks if token and secret or a root certificate are set in the environment
/// variables and returns the TlsOptions accordingly.
pub fn get_tls_options() -> Result<Option<TlsOptions>, Box<dyn Error>> {
    // For self-signed certificates we would need to read the certificate file into a string
    // and set ca_certificate to Some(ca_certificate_string)
    let root_cert = match std::env::var("ROOT_CERT") {
        Ok(root_cert) => match root_cert.is_empty() {
            true => None,
            false => {
                let root_cert = std::fs::read_to_string(root_cert)?;
                Some(root_cert)
            }
        },
        Err(_) => None,
    };
    let tls_options = {
        match (std::env::var("TOKEN"), std::env::var("SECRET")) {
            (Ok(token), Ok(secret)) => Some(TlsOptions {
                ca_certificate: root_cert,
                auth: Some(Auth { token, secret }),
            }),
            _ => match root_cert {
                Some(root_cert) => Some(TlsOptions {
                    ca_certificate: Some(root_cert),
                    auth: None,
                }),
                None => match std::env::var("SSL") {
                    Ok(ssl) => match ssl.parse::<bool>() {
                        Ok(true) => Some(TlsOptions {
                            ca_certificate: None,
                            auth: None,
                        }),
                        _ => None,
                    },
                    _ => None,
                },
            },
        }
    };
    Ok(tls_options)
}
