// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../src/security/capsule_manifest/schema/constants.rs"]
pub mod constants;

#[path = "../../../../../../../../src/security/capsule_manifest/schema/endpoint.rs"]
pub mod endpoint;

pub use constants::{MANIFEST_SCHEMA_VERSION, MAX_ENDPOINTS, MAX_ENDPOINT_NAME_LEN, MAX_NAMESPACE_LEN, MAX_PUBLISHER_SIGNATURES, MAX_TARGET_TRIPLE_LEN, NONOS_ID_CERT_ID_LEN, PAYLOAD_HASH_LEN, PUBLISHER_KEY_ID_LEN};
pub use endpoint::{EndpointDecl, EndpointKind};
