// @generated automatically by Diesel CLI.

diesel::table! {
  coinflip_games (id) {
      id -> Int8,
      max_play_count -> Int4,
      expiry_timestamp -> Int8,
      creator_address -> Text,
      block_number -> Int8
  }
}
