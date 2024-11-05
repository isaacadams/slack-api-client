/**
 * https://api.slack.com/methods/chat.postMessage#errors
 *
 * e.g.
 * {
 *  "ok": false,
 *  "error": "not_in_channel"
 * }
 *
 * this response occurs when trying to send a message to a channel that the bot has not been added to
 */

#[derive(serde::Serialize, serde::Deserialize)]
struct SlackHttpResponse {
    ok: bool,
    error: Option<String>,
}
