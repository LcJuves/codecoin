// ---------------- [ File: bitcoin-fuzz/src/fuzz_merkleblock.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/merkleblock.cpp]

#[fuzz_test] fn merkleblock() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());
        CPartialMerkleTree partial_merkle_tree;
        CallOneOf(
            fuzzed_data_provider,
            [&] {
                const std::optional<CPartialMerkleTree> opt_partial_merkle_tree = ConsumeDeserializable<CPartialMerkleTree>(fuzzed_data_provider);
                if (opt_partial_merkle_tree) {
                    partial_merkle_tree = *opt_partial_merkle_tree;
                }
            },
            [&] {
                CMerkleBlock merkle_block;
                const std::optional<CBlock> opt_block = ConsumeDeserializable<CBlock>(fuzzed_data_provider);
                CBloomFilter bloom_filter;
                std::set<uint256> txids;
                if (opt_block && !opt_block->vtx.empty()) {
                    if (fuzzed_data_provider.ConsumeBool()) {
                        merkle_block = CMerkleBlock{*opt_block, bloom_filter};
                    } else if (fuzzed_data_provider.ConsumeBool()) {
                        while (fuzzed_data_provider.ConsumeBool()) {
                            txids.insert(ConsumeUInt256(fuzzed_data_provider));
                        }
                        merkle_block = CMerkleBlock{*opt_block, txids};
                    }
                }
                partial_merkle_tree = merkle_block.txn;
            });
        (c_void)partial_merkle_tree.GetNumTransactions();
        std::vector<uint256> matches;
        std::vector<unsigned int> indices;
        (c_void)partial_merkle_tree.ExtractMatches(matches, indices);

    */
}
