// @generated automatically by Diesel CLI.

diesel::table! {
  ark_wallets (id) {
      id -> Int8,
      chain_id -> Int8,
      owner_address -> VarChar,
      balance ->  VarChar,
  }
}

diesel::table! {
  coinflip_games (id) {
      id -> Int8,
      chain_id -> Int8,
      number_of_players -> Int4,
      expiry_timestamp -> Int8,
      creator_address -> VarChar,
      block_number -> Int8,
      wager -> VarChar,
      play_count -> Int4,
      unavailable_coin_side -> Nullable<Int4>,
      outcome -> Nullable<Int4>,
      amount_for_each_winner -> Nullable<VarChar>,
      completed_at -> Nullable<Int8>,
      refunded_amount_per_player -> Nullable<VarChar>,
      refunded_at -> Nullable<Int8>,
      inserted_at -> Int8
  }
}

diesel::table! {
  coinflip_game_plays (id) {
      id -> Int4,
      game_id -> Int8,
      chain_id -> Int8,
      coin_side -> Int4,
      player_address -> VarChar,
      proof_of_chance -> VarChar,
      chance_and_salt -> Nullable<VarChar>,
      status -> VarChar
  }
}

diesel::joinable!(coinflip_game_plays -> coinflip_games (game_id));
diesel::allow_tables_to_appear_in_same_query!(coinflip_games, coinflip_game_plays);

diesel::table! {
  coinflip_game_activities (id) {
      id -> Int8,
      game_id -> Int8,
      chain_id -> Int8,
      trigger_public_address -> VarChar,
      kind ->  VarChar,
      data -> Json,
      occurred_at -> Int8,
      transaction_hash -> Nullable<VarChar>,
  }
}

diesel::table! {
  ark_chain_currencies (id) {
      id -> Int4,
      chain_id -> Int8,
      currency_symbol -> VarChar,
      unit_usd_price -> VarChar
  }
}

diesel::table! {
  ark_total_paid_out_reports (id) {
      id -> Int8,
      amount -> VarChar
  }
}
