// @generated automatically by Diesel CLI.

diesel::table! {
  coinflip_games (id) {
      id -> Int8,
      max_play_count -> Int4,
      expiry_timestamp -> Int8,
      creator_address -> Text,
      block_number -> Int8,
      wager -> Text,
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
      play_hash -> Text
  }
}

diesel::table! {
  coinflip_chain_currencies (id) {
      id -> Int4,
      chain_id -> Int4,
      currency_symbol -> Text,
      unit_value_in_usd -> Text
  }
}
