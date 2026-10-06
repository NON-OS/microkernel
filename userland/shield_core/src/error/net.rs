//! Failures on the network path.

/// Failures on the network path. Every connection leaves through a SOCKS5 proxy, so a refusal
/// is usually the proxy, and a sequencer's rejection code is passed through unchanged.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetError {
    EndpointRefused,
    ProxyUnreachable,
    ProxyRefused,
    ProxyProtocol,
    Transport,
    ReplyShape,
    Rejected {
        code: u16,
    },
    ReplyTooLarge,
    /// The RPC serves another chain: nothing is read from it or signed for it.
    WrongChain,
    /// The relayer is not the account every proof pays: nothing was handed to it.
    WrongRelayer,
    /// The wallet locked while the pool was read: the read stopped between two requests.
    Stopped,
}

impl core::fmt::Display for NetError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            NetError::EndpointRefused => f.write_str("that endpoint would leak the connection"),
            NetError::ProxyUnreachable => f.write_str("the privacy proxy is not running"),
            NetError::ProxyRefused => f.write_str("the privacy proxy refused the route"),
            NetError::ProxyProtocol => f.write_str("the privacy proxy is not speaking SOCKS5"),
            NetError::Transport => f.write_str("the connection failed"),
            NetError::ReplyShape => f.write_str("the reply was malformed"),
            NetError::Rejected { code } => write!(f, "the server refused the request ({code})"),
            NetError::ReplyTooLarge => f.write_str("the reply was too large"),
            NetError::WrongChain => f.write_str("the server answered for a different network"),
            NetError::WrongRelayer => f.write_str("the relayer answered as someone else"),
            NetError::Stopped => f.write_str("the wallet locked, so the read was stopped"),
        }
    }
}

impl core::error::Error for NetError {}
