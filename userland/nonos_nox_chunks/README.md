# nonos_nox_chunks

Escrow's fair delivery as plan/22 defines it, shared by the buyer's OS side,
the installer's pin and the seller's host tool (`marketplace-index
escrow-serve`), so the three cannot drift. `no_std`, alloc only; keccak comes
from the caller through marketplace_abi's `Keccak`.

- Chunks (`chunk`, `chunk_count`): exactly 1024 bytes, the last zero-padded
  and hashed padded. Since padding hides the true length, contentLen is
  committed beside chunk_root.
- Leaves (`leaf`): keccak256(abi.encode(uint8 tree, uint64 i, bytes32 h)),
  tree 0 plain and 1 cipher.
- Trees (`root`, `proof`, `verify`): leaves in index order, pairs hashed
  sorted as OpenZeppelin's MerkleProof does, an odd level's last node
  promoted unpaired.
- Roots (`chunk_root`, `cipher_root`, `plain_hashes`, `root_of_hashes`).
- Cipher (`keystream_word`, `xor_chunk`, `key_commit`): word j of chunk i is
  keccak256(abi.encode(K, uint64 i, uint32 j)); the same XOR decrypts.
- Orders (`OrderTerms`, `order_id`, `refund_commit`): the fourteen-word
  terms tuple settle_proto names for fund(T), under ORDER_TAG and REFUND_TAG
  from nonos_nox_config.

Tests check each piece against an independent derivation: a recursive
reference tree, hand-built ABI words, and keccak from sha3.
