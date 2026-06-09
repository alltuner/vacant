// ABOUTME: End-to-end test for the `vacant mcp` stdio server over real JSON-RPC.
// ABOUTME: Spawns the built binary, runs the MCP handshake, and checks check_domains.

use std::io::Write;
use std::process::{Command, Stdio};

use serde_json::Value;

/// Drive the server through initialize → tools/list → tools/call and return the
/// parsed JSON-RPC responses keyed by id.
fn run_session(requests: &[&str]) -> Vec<Value> {
    let mut child = Command::new(env!("CARGO_BIN_EXE_vacant"))
        .arg("mcp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn vacant mcp");

    {
        let stdin = child.stdin.as_mut().expect("child stdin");
        for line in requests {
            writeln!(stdin, "{line}").expect("write request");
        }
    } // drop stdin → EOF, so the server shuts down after replying

    let output = child.wait_with_output().expect("wait for child");
    assert!(
        output.stderr.is_empty(),
        "server logged to stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .expect("utf8 stdout")
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).expect("response is json"))
        .collect()
}

fn response(responses: &[Value], id: i64) -> &Value {
    responses
        .iter()
        .find(|r| r["id"] == id)
        .unwrap_or_else(|| panic!("no response with id {id}"))
}

#[test]
fn lists_and_calls_check_domains() {
    let responses = run_session(&[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#,
        r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"check_domains","arguments":{"domains":["google.com","this-is-clearly-not-a-domain.example"]}}}"#,
    ]);

    let tools = &response(&responses, 2)["result"]["tools"];
    assert_eq!(tools[0]["name"], "check_domains");

    let result = &response(&responses, 3)["result"]["structuredContent"]["result"];
    let entries = result.as_array().expect("result is an array");
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0]["domain"], "google.com");
    let google = entries[0]["status"].as_str().unwrap();
    assert!(
        google == "registered" || google == "reserved",
        "google.com should be registered/reserved, got {google}"
    );
    assert_eq!(entries[1]["domain"], "this-is-clearly-not-a-domain.example");
    assert_eq!(entries[1]["status"], "invalid");
}

#[test]
fn skips_blank_and_comment_entries() {
    let responses = run_session(&[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r##"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"check_domains","arguments":{"domains":["","  ","# a comment","google.com"]}}}"##,
    ]);

    let result = &response(&responses, 2)["result"]["structuredContent"]["result"];
    let entries = result.as_array().expect("result is an array");
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0]["domain"], "google.com");
}
