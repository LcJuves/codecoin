// ---------------- [ File: bitcoin-peerman/src/lib.rs ]
#![feature(iter_advance_by)]

#[macro_use] mod imports; use imports::*;

x!{add_address}
x!{accept_to_memory_pool}
x!{find_fork_in_global_index}
x!{add_to_compact_extra_txns}
x!{add_tx_announcement}
x!{addr_send_times}
x!{alredy_have_block}
x!{alredy_have_tx}
x!{announcement_index}
x!{block_checked}
x!{block_connected}
x!{block_disconnected}
x!{block_inv}
x!{block_request_allowed}
x!{block_requested}
x!{can_direct_fetch}
x!{chain_sync_timeout_state}
x!{check_for_stale_tip_and_evict_peers}
x!{compute_tx_hashinfo}
x!{consider_eviction}
x!{evict_extra_outbound_peers}
x!{finalize_node}
x!{find_next_blocks_to_download}
x!{find_tx_for_getdata}
x!{get_node_state_stats}
x!{get_peer_ref}
x!{ignores_incoming_txs}
x!{initialize_node}
x!{is_block_requested}
x!{maybe_discourage_and_disconnect}
x!{maybe_punish_node_for_block}
x!{maybe_punish_node_for_tx}
x!{maybe_send_addr}
x!{maybe_send_fee_filter}
x!{maybe_send_ping}
x!{maybe_set_peer_as_announcing_header_and_ids}
x!{misbehavior}
x!{new_pow_valid_block}
x!{node_state}
x!{orphans}
x!{partial_block}
x!{peer_has_header}
x!{peerinfo}
x!{peerman_inner}
x!{peerman}
x!{peer}
x!{prepare_block_filter_request}
x!{priority_computer}
x!{process_addr_message}
x!{process_block_availability}
x!{process_block_message}
x!{process_blocktxn_message}
x!{process_block}
x!{process_cmpctblock_message}
x!{process_feefilter_message}
x!{process_filteradd_message}
x!{process_filterclear_message}
x!{process_filterload_message}
x!{process_get_data}
x!{process_get_filters}
x!{process_getaddr_message}
x!{process_getblock_data}
x!{process_getblocks_message}
x!{process_getblockstxn_message}
x!{process_getcf_checkpt}
x!{process_getcf_headers}
x!{process_getdata_message}
x!{process_getheaders_message}
x!{process_headers_message}
x!{process_inv_message}
x!{process_mempool_message}
x!{process_messages}
x!{process_message}
x!{process_notfound_message}
x!{process_orphan_tx}
x!{process_ping_message}
x!{process_pong_message}
x!{process_sendaddrv2_message}
x!{process_sendcmpct_message}
x!{process_sendheaders_message}
x!{process_tx_message}
x!{process_verack_message}
x!{process_version_message}
x!{process_wtxidrelay_message}
x!{push_node_version}
x!{reattempt_initial_broadcast}
x!{recent_confirmed_txns}
x!{relay_address}
x!{relay_txn}
x!{remove_block_request}
x!{remove_peer}
x!{send_block_transactions}
x!{send_messages}
x!{send_pings}
x!{set_best_height}
x!{setup_address_relay}
x!{start_scheduled_tasks}
x!{tip_may_be_stale}
x!{traits}
x!{txhashinfo}
x!{txorphanage}
x!{txrequest_comparator}
x!{txrequest_compute}
x!{txrequest_priority}
x!{txrequest_tracker_announcement}
x!{txrequest_tracker_impl}
x!{txrequest_tracker_state}
x!{txrequest_tracker}
x!{txrequest_waitstate}
x!{update_block_availability}
x!{update_last_block_announce_time}
x!{update_preferred_download}
x!{updated_block_tip}
