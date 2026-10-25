# nonos_nox_chunks

Escrow's fair delivery as the plan defines it, shared by the buyer's OS side,
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
- The buyer's checks: `check_offer` before funding (the hash list makes
  chunk_root for content_len, the ciphertext makes cipherRoot, no key
  needed); `open_delivery` once K is revealed (K against keyCommit, every
  decrypted chunk against its hash, else the first bad index); `dispute`
  builds dispute()'s arguments for that index.
- Deposit addresses (`deposit_address`, `create2`, `clone_initcode_hash`):
  CREATE2 by the escrow, salt orderId, of an EIP-1167 clone of the
  DepositForwarder; `create2` meets EIP-1014's examples.
- The seller's agent (`reveal_ok`, `may_send`, `Mine`): the plan step 4.
- The buyer's terms (`buyer_terms`, `Sale`, `Buyer`): token NOX, refundTo
  behind refundCommit, a deadline in [now + ORDER_MIN_LEAD, now +
  MAX_DELIVERY - DELIVER_MARGIN]; a test shows the seller's `reveal_ok`
  accepts what the buyer writes.
- The escrow's calls as calldata (`fund`, `fund_from`, `release`, `refund`,
  `dispute_call`), each taken by settle_proto's `kind_of` as its kind.
- Orders (`OrderTerms`, `order_id`, `refund_commit`): the fourteen-word
  terms tuple settle_proto names for fund(T), under ORDER_TAG and REFUND_TAG
  from nonos_nox_config.

Tests check each piece against an independent derivation: a recursive
reference tree, hand-built ABI words, and keccak from sha3.
