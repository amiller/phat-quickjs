use js::ToJsValue;

use crate::{service::ServiceRef, Service};
use anyhow::{anyhow, bail, Context, Result};

use pink_types::js::{JsCode, JsValue};

struct Args {
    codes: Vec<JsCode>,
    js_args: Vec<String>,
}

#[cfg(feature = "wapo")]
fn load_code(code_hash: &str) -> Result<String> {
    log::info!(target: "js", "loading code with hash: {code_hash}");
    let code_hash = code_hash.trim_start_matches("0x");
    if code_hash.len() != 64 {
        bail!("invalid code hash length: {}", code_hash.len());
    }
    let code_hash = hex::decode(code_hash).context("invalid code hash")?;
    let source_blob =
        wapo::ocall::blob_get(&code_hash, "sha256").context("failed to get source code")?;
    let source_code = String::from_utf8(source_blob).context("source code is not valid utf-8")?;
    Ok(source_code)
}

fn parse_args(args: impl Iterator<Item = String>) -> Result<Args> {
    let mut codes = vec![];
    let mut iter = args.skip(1);
    while let Some(arg) = iter.next() {
        if arg.starts_with("-") {
            if arg == "--" {
                break;
            }
            match arg.as_str() {
                #[cfg(feature = "wapo")]
                "--code-hash" => {
                    let code_hash = iter
                        .next()
                        .ok_or(anyhow!("missing value after --code-hash"))?;
                    let code =
                        load_code(&code_hash).context("failed to load code with given hash")?;
                    codes.push(JsCode::Source(code));
                }
                "-c" => {
                    let code = iter.next().ok_or(anyhow!("missing code after -c"))?;
                    codes.push(JsCode::Source(code));
                }
                _ => {
                    print_usage();
                    bail!("unknown option: {}", arg);
                }
            }
        } else {
            // File name
            let code = std::fs::read_to_string(arg).context("failed to read script file")?;
            codes.push(JsCode::Source(code));
        }
    }
    if codes.is_empty() {
        print_usage();
        bail!("no script file provided");
    }
    let js_args = iter.collect();
    Ok(Args { codes, js_args })
}

fn print_usage() {
    println!("wapojs v{}", env!("CARGO_PKG_VERSION"));
    println!("Usage: wapojs [options] [script..] [-- [args]]");
    println!("");
    println!("Options:");
    println!("  -c <code>        Execute code");
    #[cfg(feature = "wapo")]
    println!("  --code-hash <code_hash>  Execute code");
    println!("  --               Stop processing options");
}

pub async fn run(args: impl Iterator<Item = String>) -> Result<JsValue> {
    let service = Service::new_ref();
    let rv = run_with_service(service.clone(), args).await;
    service.shutdown().await;
    rv
}

async fn run_with_service(
    service: ServiceRef,
    args: impl Iterator<Item = String>,
) -> Result<JsValue> {
    let args = parse_args(args)?;
    let js_ctx = service.context();
    let js_args = args
        .js_args
        .to_js_value(&js_ctx)
        .context("failed to convert args to js value")?;
    js_ctx
        .get_global_object()
        .set_property("scriptArgs", &js_args)
        .context("failed to set scriptArgs")?;
    let mut expr_val = None;
    for code in args.codes.into_iter() {
        let result = match code {
            JsCode::Source(src) => service.exec_script(&src),
            JsCode::Bytecode(bytes) => service.exec_bytecode(&bytes),
        };
        match result {
            Ok(value) => expr_val = value.to_js_value(),
            Err(err) => {
                bail!("failed to execute script: {err}");
            }
        }
    }
    #[cfg(feature = "wapo")]
    loop {
        tokio::select! {
            _ = service.wait_for_tasks() => {
                break;
            }
            query = wapo::channel::incoming_queries().next() => {
                let Some(query) = query else {
                    log::info!(target: "js", "host dropped the channel, exiting...");
                    break;
                };
                crate::host_functions::try_accept_query(service.clone(), query)?;
            }
            request = wapo::channel::incoming_http_requests().next() => {
                let Some(request) = request else {
                    log::info!(target: "js", "host dropped the channel, exiting...");
                    break;
                };
                #[cfg(feature = "js-http-listen")]
                crate::host_functions::try_accept_http_request(service.clone(), request)?;
            }
        }
    }
    #[cfg(not(feature = "wapo"))]
    {
        service.wait_for_tasks().await;
    }
    // If scriptOutput is set, use it as output. Otherwise, use the last expression value.
    let output = js_ctx
        .get_global_object()
        .get_property("scriptOutput")
        .unwrap_or_default();
    let output = if output.is_undefined() {
        expr_val.unwrap_or_default()
    } else {
        output
    };
    convert(output).context("failed to convert output")
}

fn convert(output: js::Value) -> Result<JsValue> {
    if output.is_undefined() {
        return Ok(JsValue::Undefined);
    }
    if output.is_null() {
        return Ok(JsValue::Null);
    }
    if output.is_string() {
        return Ok(JsValue::String(output.decode_string()?));
    }
    if output.is_uint8_array() {
        return Ok(JsValue::Bytes(output.decode_bytes()?));
    }
    return Ok(JsValue::Other(output.to_string()));
}
