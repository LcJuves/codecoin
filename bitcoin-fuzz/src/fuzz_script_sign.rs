// ---------------- [ File: bitcoin-fuzz/src/fuzz_script_sign.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/script_sign.cpp]

pub fn initialize_script_sign()  {
    
    todo!();
        /*
            static const ECCVerifyHandle ecc_verify_handle;
        ECC_Start();
        SelectParams(CBaseChainParams::REGTEST);
        */
}

#[fuzz_test(initializer = "initialize_script_sign")]
fn script_sign() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());
        const std::vector<uint8_t> key = ConsumeRandomLengthByteVector(fuzzed_data_provider, 128);

        {
            DataStream random_data_stream = ConsumeDataStream(fuzzed_data_provider);
            std::map<CPubKey, KeyOriginInfo> hd_keypaths;
            try {
                DeserializeHDKeypaths(random_data_stream, key, hd_keypaths);
            } catch (const std::ios_base::failure&) {
            }
            DataStream serialized{SER_NETWORK, PROTOCOL_VERSION};
            SerializeHDKeypaths(serialized, hd_keypaths, fuzzed_data_provider.ConsumeIntegral<uint8_t>());
        }

        {
            std::map<CPubKey, KeyOriginInfo> hd_keypaths;
            while (fuzzed_data_provider.ConsumeBool()) {
                const std::optional<CPubKey> pub_key = ConsumeDeserializable<CPubKey>(fuzzed_data_provider);
                if (!pub_key) {
                    break;
                }
                const std::optional<KeyOriginInfo> key_origin_info = ConsumeDeserializable<KeyOriginInfo>(fuzzed_data_provider);
                if (!key_origin_info) {
                    break;
                }
                hd_keypaths[*pub_key] = *key_origin_info;
            }
            DataStream serialized{SER_NETWORK, PROTOCOL_VERSION};
            try {
                SerializeHDKeypaths(serialized, hd_keypaths, fuzzed_data_provider.ConsumeIntegral<uint8_t>());
            } catch (const std::ios_base::failure&) {
            }
            std::map<CPubKey, KeyOriginInfo> deserialized_hd_keypaths;
            try {
                DeserializeHDKeypaths(serialized, key, hd_keypaths);
            } catch (const std::ios_base::failure&) {
            }
            assert(hd_keypaths.size() >= deserialized_hd_keypaths.size());
        }

        {
            SignatureData signature_data_1{ConsumeScript(fuzzed_data_provider)};
            SignatureData signature_data_2{ConsumeScript(fuzzed_data_provider)};
            signature_data_1.MergeSignatureData(signature_data_2);
        }

        FillableSigningProvider provider;
        CKey k;
        const std::vector<uint8_t> key_data = ConsumeRandomLengthByteVector(fuzzed_data_provider);
        k.Set(key_data.begin(), key_data.end(), fuzzed_data_provider.ConsumeBool());
        if (k.IsValid()) {
            provider.AddKey(k);
        }

        {
            const std::optional<CMutableTransaction> mutable_transaction = ConsumeDeserializable<CMutableTransaction>(fuzzed_data_provider);
            const std::optional<CTxOut> tx_out = ConsumeDeserializable<CTxOut>(fuzzed_data_provider);
            const unsigned int n_in = fuzzed_data_provider.ConsumeIntegral<unsigned int>();
            if (mutable_transaction && tx_out && mutable_transaction->vin.size() > n_in) {
                SignatureData signature_data_1 = DataFromTransaction(*mutable_transaction, n_in, *tx_out);
                CTxIn input;
                UpdateInput(input, signature_data_1);
                const CScript script = ConsumeScript(fuzzed_data_provider);
                SignatureData signature_data_2{script};
                signature_data_1.MergeSignatureData(signature_data_2);
            }
            if (mutable_transaction) {
                CTransaction tx_from{*mutable_transaction};
                CMutableTransaction tx_to;
                const std::optional<CMutableTransaction> opt_tx_to = ConsumeDeserializable<CMutableTransaction>(fuzzed_data_provider);
                if (opt_tx_to) {
                    tx_to = *opt_tx_to;
                }
                CMutableTransaction script_tx_to = tx_to;
                CMutableTransaction sign_transaction_tx_to = tx_to;
                if (n_in < tx_to.vin.size() && tx_to.vin[n_in].prevout.n < tx_from.vout.size()) {
                    (c_void)SignSignature(provider, tx_from, tx_to, n_in, fuzzed_data_provider.ConsumeIntegral<int>());
                }
                if (n_in < script_tx_to.vin.size()) {
                    (c_void)SignSignature(provider, ConsumeScript(fuzzed_data_provider), script_tx_to, n_in, ConsumeMoney(fuzzed_data_provider), fuzzed_data_provider.ConsumeIntegral<int>());
                    MutableTransactionSignatureCreator signature_creator{&tx_to, n_in, ConsumeMoney(fuzzed_data_provider), fuzzed_data_provider.ConsumeIntegral<int>()};
                    std::vector<unsigned char> vch_sig;
                    CKeyID address;
                    if (fuzzed_data_provider.ConsumeBool()) {
                        if (k.IsValid()) {
                            address = k.GetPubKey().GetID();
                        }
                    } else {
                        address = CKeyID{ConsumeUInt160(fuzzed_data_provider)};
                    }
                    (c_void)signature_creator.CreateSig(provider, vch_sig, address, ConsumeScript(fuzzed_data_provider), fuzzed_data_provider.PickValueInArray({SigVersion::BASE, SigVersion::WITNESS_V0}));
                }
                std::map<OutPoint, Coin> coins;
                while (fuzzed_data_provider.ConsumeBool()) {
                    const std::optional<OutPoint> outpoint = ConsumeDeserializable<OutPoint>(fuzzed_data_provider);
                    if (!outpoint) {
                        break;
                    }
                    const std::optional<Coin> coin = ConsumeDeserializable<Coin>(fuzzed_data_provider);
                    if (!coin) {
                        break;
                    }
                    coins[*outpoint] = *coin;
                }
                std::map<int, bilingual_str> input_errors;
                (c_void)SignTransaction(sign_transaction_tx_to, &provider, coins, fuzzed_data_provider.ConsumeIntegral<int>(), input_errors);
            }
        }

        {
            SignatureData signature_data_1;
            (c_void)ProduceSignature(provider, DUMMY_SIGNATURE_CREATOR, ConsumeScript(fuzzed_data_provider), signature_data_1);
            SignatureData signature_data_2;
            (c_void)ProduceSignature(provider, DUMMY_MAXIMUM_SIGNATURE_CREATOR, ConsumeScript(fuzzed_data_provider), signature_data_2);
        }

    */
}
