// @generated automatically by Diesel CLI.

diesel::table! {
  coinflip_games (id) {
      id -> Int8,
      max_play_count -> Int4,
      expiry_timestamp -> Int8,
      creator_address -> VarChar,
      block_number -> Int8,
      wager -> VarChar,
      chain_id -> Int4,
      play_count -> Int4,
      is_completed -> Bool
  }
}

diesel::table! {
  coinflip_game_plays (id) {
      id -> Int4,
      game_id -> Int8,
      coin_side -> Bool,
      player_address -> VarChar,
      play_hash -> VarChar
  }
}

diesel::table! {
  coinflip_game_play_proofs (id) {
      id -> Int8,
      game_id -> Int8,
      game_play_id -> Int4,
      player_address -> VarChar,
      play_proof -> VarChar
  }
}

diesel::table! {
  coinflip_game_activities (id) {
      id -> Int8,
      game_id -> Int8,
      trigger_public_address -> VarChar,
      kind ->  VarChar,
      data -> Nullable<Jsonb>,
      block_timestamp -> Int8,
      transaction_hash -> VarChar,
  }
}

diesel::table! {
  coinflip_chain_currencies (id) {
      id -> Int4,
      chain_id -> Int4,
      currency_symbol -> VarChar,
      unit_usd_price -> VarChar
  }
}
