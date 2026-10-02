// origin: src/bin/serve.rs @ B1 seam commit 22ae291, lines 1093-1099

                    "too_large",
                    "body too large",
                    cors.as_deref(),
                );
                return Ok(());
            }
            let mut body = vec![0u8; req.content_length];

// origin: src/bin/serve.rs @ B1 seam commit 22ae291, lines 1140-1147

                &mut writer,
                "405 Method Not Allowed",
                "method_not_allowed",
                "wrong method for this endpoint",
                cors.as_deref(),
            );
        }
        _ => {
