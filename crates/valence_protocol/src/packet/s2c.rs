pub mod status {
    pub use query_pong::QueryPongS2c;
    pub use query_response::QueryResponseS2c;

    pub mod query_pong;
    pub mod query_response;

    packet_group! {
        #[derive(Clone)]
        S2cStatusPacket<'a> {
            0 = QueryResponseS2c<'a>,
            1 = QueryPongS2c,
        }
    }
}

pub mod login {
    pub use login_compression::LoginCompressionS2c;
    pub use login_disconnect::LoginDisconnectS2c;
    pub use login_hello::LoginHelloS2c;
    pub use login_query_request::LoginQueryRequestS2c;
    pub use login_success::LoginSuccessS2c;

    pub mod login_compression;
    pub mod login_disconnect;
    pub mod login_hello;
    pub mod login_query_request;
    pub mod login_success;

    packet_group! {
        #[derive(Clone)]
        S2cLoginPacket<'a> {
            0 = LoginDisconnectS2c<'a>,
            1 = LoginHelloS2c<'a>,
            2 = LoginSuccessS2c<'a>,
            3 = LoginCompressionS2c,
            4 = LoginQueryRequestS2c<'a>,
        }
    }
}

pub mod play {
    pub use advancement_update::AdvancementUpdateS2c;
    pub use block_breaking_progress::BlockBreakingProgressS2c;
    pub use block_entity_update::BlockEntityUpdateS2c;
    pub use block_event::BlockEventS2c;
    pub use block_update::BlockUpdateS2c;
    pub use boss_bar::BossBarS2c;
    pub use chat_message::ChatMessageS2c;
    pub use chat_suggestions::ChatSuggestionsS2c;
    pub use chunk_data::ChunkDataS2c;
    pub use chunk_delta_update::ChunkDeltaUpdateS2c;
    pub use chunk_load_distance::ChunkLoadDistanceS2c;
    pub use chunk_render_distance_center::ChunkRenderDistanceCenterS2c;
    pub use clear_titles::ClearTitlesS2c;
    pub use close_screen::CloseScreenS2c;
    pub use command_suggestions::CommandSuggestionsS2c;
    pub use command_tree::CommandTreeS2c;
    pub use cooldown_update::CooldownUpdateS2c;
    pub use craft_failed_response::CraftFailedResponseS2c;
    pub use custom_payload::CustomPayloadS2c;
    pub use death_message::DeathMessageS2c;
    pub use difficulty::DifficultyS2c;
    pub use disconnect::DisconnectS2c;
    pub use end_combat::EndCombatS2c;
    pub use enter_combat::EnterCombatS2c;
    pub use entities_destroy::EntitiesDestroyS2c;
    pub use entity::{MoveRelativeS2c, RotateAndMoveRelativeS2c, RotateS2c};
    pub use entity_animation::EntityAnimationS2c;
    pub use entity_attach::EntityAttachS2c;
    pub use entity_attributes::EntityAttributesS2c;
    pub use entity_equipment_update::EntityEquipmentUpdateS2c;
    pub use entity_passengers_set::EntityPassengersSetS2c;
    pub use entity_position::EntityPositionS2c;
    pub use entity_set_head_yaw::EntitySetHeadYawS2c;
    pub use entity_spawn::EntitySpawnS2c;
    pub use entity_status::EntityStatusS2c;
    pub use entity_status_effect::EntityStatusEffectS2c;
    pub use entity_tracker_update::EntityTrackerUpdateS2c;
    pub use entity_velocity_update::EntityVelocityUpdateS2c;
    pub use experience_bar_update::ExperienceBarUpdateS2c;
    pub use experience_orb_spawn::ExperienceOrbSpawnS2c;
    pub use explosion::ExplosionS2c;
    pub use features::FeaturesS2c;
    pub use game_join::GameJoinS2c;
    pub use game_message::GameMessageS2c;
    pub use game_state_change::GameStateChangeS2c;
    pub use health_update::HealthUpdateS2c;
    pub use inventory::InventoryS2c;
    pub use item_pickup_animation::ItemPickupAnimationS2c;
    pub use keep_alive::KeepAliveS2c;
    pub use light_update::LightUpdateS2c;
    pub use look_at::LookAtS2c;
    pub use map_update::MapUpdateS2c;
    pub use nbt_query_response::NbtQueryResponseS2c;
    pub use open_horse_screen::OpenHorseScreenS2c;
    pub use open_screen::OpenScreenS2c;
    pub use open_written_book::OpenWrittenBookS2c;
    pub use overlay_message::OverlayMessageS2c;
    pub use particle::ParticleS2c;
    pub use play_ping::PlayPingS2c;
    pub use play_sound::PlaySoundS2c;
    pub use play_sound_from_entity::PlaySoundFromEntityS2c;
    pub use player_abilities::PlayerAbilitiesS2c;
    pub use player_action_response::PlayerActionResponseS2c;
    pub use player_list::PlayerListS2c;
    pub use player_list_header::PlayerListHeaderS2c;
    pub use player_position_look::PlayerPositionLookS2c;
    pub use player_remove::PlayerRemoveS2c;
    pub use player_respawn::PlayerRespawnS2c;
    pub use player_spawn::PlayerSpawnS2c;
    pub use player_spawn_position::PlayerSpawnPositionS2c;
    pub use profileless_chat_message::ProfilelessChatMessageS2c;
    pub use remove_entity_status_effect::RemoveEntityStatusEffectS2c;
    pub use remove_message::RemoveMessageS2c;
    pub use resource_pack_send::ResourcePackSendS2c;
    pub use scoreboard_display::ScoreboardDisplayS2c;
    pub use scoreboard_objective_update::ScoreboardObjectiveUpdateS2c;
    pub use scoreboard_player_update::ScoreboardPlayerUpdateS2c;
    pub use screen_handler_property_update::ScreenHandlerPropertyUpdateS2c;
    pub use screen_handler_slot_update::ScreenHandlerSlotUpdateS2c;
    pub use select_advancements_tab::SelectAdvancementsTabS2c;
    pub use server_metadata::ServerMetadataS2c;
    pub use set_camera_entity::SetCameraEntityS2c;
    pub use set_trade_offers::SetTradeOffersS2c;
    pub use sign_editor_open::SignEditorOpen;
    pub use simulation_distance::SimulationDistanceS2c;
    pub use statistics::StatisticsS2c;
    pub use stop_sound::StopSoundS2c;
    pub use subtitle::SubtitleS2c;
    pub use synchronize_recipes::SynchronizeRecipesS2c;
    pub use synchronize_tags::SynchronizeTagsS2c;
    pub use team::TeamS2c;
    pub use title::TitleS2c;
    pub use title_fade::TitleFadeS2c;
    pub use unload_chunk::UnloadChunkS2c;
    pub use unlock_recipes::UnlockRecipesS2c;
    pub use update_selected_slot::UpdateSelectedSlotS2c;
    pub use vehicle_move::VehicleMoveS2c;
    pub use world_border_center_changed::WorldBorderCenterChangedS2c;
    pub use world_border_initialize::WorldBorderInitializeS2c;
    pub use world_border_interpolate_size::WorldBorderInterpolateSizeS2c;
    pub use world_border_size_changed::WorldBorderSizeChangedS2c;
    pub use world_border_warning_blocks_changed::WorldBorderWarningBlocksChangedS2c;
    pub use world_border_warning_time_changed::WorldBorderWarningTimeChangedS2c;
    pub use world_event::WorldEventS2c;
    pub use world_time_update::WorldTimeUpdateS2c;

    pub mod advancement_update;
    pub mod block_breaking_progress;
    pub mod block_entity_update;
    pub mod block_event;
    pub mod block_update;
    pub mod boss_bar;
    pub mod chat_message;
    pub mod chat_suggestions;
    pub mod chunk_data;
    pub mod chunk_delta_update;
    pub mod chunk_load_distance;
    pub mod chunk_render_distance_center;
    pub mod clear_titles;
    pub mod close_screen;
    pub mod command_suggestions;
    pub mod command_tree;
    pub mod cooldown_update;
    pub mod craft_failed_response;
    pub mod custom_payload;
    pub mod death_message;
    pub mod difficulty;
    pub mod disconnect;
    pub mod end_combat;
    pub mod enter_combat;
    pub mod entities_destroy;
    pub mod entity;
    pub mod entity_animation;
    pub mod entity_attach;
    pub mod entity_attributes;
    pub mod entity_equipment_update;
    pub mod entity_passengers_set;
    pub mod entity_position;
    pub mod entity_set_head_yaw;
    pub mod entity_spawn;
    pub mod entity_status;
    pub mod entity_status_effect;
    pub mod entity_tracker_update;
    pub mod entity_velocity_update;
    pub mod experience_bar_update;
    pub mod experience_orb_spawn;
    pub mod explosion;
    pub mod features;
    pub mod game_join;
    pub mod game_message;
    pub mod game_state_change;
    pub mod health_update;
    pub mod inventory;
    pub mod item_pickup_animation;
    pub mod keep_alive;
    pub mod light_update;
    pub mod look_at;
    pub mod map_update;
    pub mod nbt_query_response;
    pub mod open_horse_screen;
    pub mod open_screen;
    pub mod open_written_book;
    pub mod overlay_message;
    pub mod particle;
    pub mod play_ping;
    pub mod play_sound;
    pub mod play_sound_from_entity;
    pub mod player_abilities;
    pub mod player_action_response;
    pub mod player_list;
    pub mod player_list_header;
    pub mod player_position_look;
    pub mod player_remove;
    pub mod player_respawn;
    pub mod player_spawn;
    pub mod player_spawn_position;
    pub mod profileless_chat_message;
    pub mod remove_entity_status_effect;
    pub mod remove_message;
    pub mod resource_pack_send;
    pub mod scoreboard_display;
    pub mod scoreboard_objective_update;
    pub mod scoreboard_player_update;
    pub mod screen_handler_property_update;
    pub mod screen_handler_slot_update;
    pub mod select_advancements_tab;
    pub mod server_metadata;
    pub mod set_camera_entity;
    pub mod set_trade_offers;
    pub mod sign_editor_open;
    pub mod simulation_distance;
    pub mod statistics;
    pub mod stop_sound;
    pub mod subtitle;
    pub mod synchronize_recipes;
    pub mod synchronize_tags;
    pub mod team;
    pub mod title;
    pub mod title_fade;
    pub mod unload_chunk;
    pub mod unlock_recipes;
    pub mod update_selected_slot;
    pub mod vehicle_move;
    pub mod world_border_center_changed;
    pub mod world_border_initialize;
    pub mod world_border_interpolate_size;
    pub mod world_border_size_changed;
    pub mod world_border_warning_blocks_changed;
    pub mod world_border_warning_time_changed;
    pub mod world_event;
    pub mod world_time_update;

    packet_group! {
        #[derive(Clone)]
        S2cPlayPacket<'a> {
            0 = EntitySpawnS2c,
            1 = ExperienceOrbSpawnS2c,
            2 = PlayerSpawnS2c,
            3 = EntityAnimationS2c,
            4 = StatisticsS2c,
            5 = PlayerActionResponseS2c,
            6 = BlockBreakingProgressS2c,
            7 = BlockEntityUpdateS2c<'a>,
            8 = BlockEventS2c,
            9 = BlockUpdateS2c,
            10 = BossBarS2c,
            11 = DifficultyS2c,
            12 = ClearTitlesS2c,
            13 = CommandSuggestionsS2c<'a>,
            14 = CommandTreeS2c<'a>,
            15 = CloseScreenS2c,
            16 = InventoryS2c<'a>,
            17 = ScreenHandlerPropertyUpdateS2c,
            18 = ScreenHandlerSlotUpdateS2c<'a>,
            19 = CooldownUpdateS2c,
            20 = ChatSuggestionsS2c<'a>,
            21 = CustomPayloadS2c<'a>,
            22 = RemoveMessageS2c<'a>,
            23 = DisconnectS2c<'a>,
            24 = ProfilelessChatMessageS2c<'a>,
            25 = EntityStatusS2c,
            26 = ExplosionS2c<'a>,
            27 = UnloadChunkS2c,
            28 = GameStateChangeS2c,
            29 = OpenHorseScreenS2c,
            30 = WorldBorderInitializeS2c,
            31 = KeepAliveS2c,
            32 = ChunkDataS2c<'a>,
            33 = WorldEventS2c,
            34 = ParticleS2c<'a>,
            35 = LightUpdateS2c,
            36 = GameJoinS2c<'a>,
            37 = MapUpdateS2c<'a>,
            38 = SetTradeOffersS2c,
            39 = MoveRelativeS2c,
            40 = RotateAndMoveRelativeS2c,
            41 = RotateS2c,
            42 = VehicleMoveS2c,
            43 = OpenWrittenBookS2c,
            44 = OpenScreenS2c<'a>,
            45 = SignEditorOpen,
            46 = PlayPingS2c,
            47 = CraftFailedResponseS2c<'a>,
            48 = PlayerAbilitiesS2c,
            49 = ChatMessageS2c<'a>,
            50 = EndCombatS2c,
            51 = EnterCombatS2c,
            52 = DeathMessageS2c<'a>,
            53 = PlayerRemoveS2c<'a>,
            54 = PlayerListS2c<'a>,
            55 = LookAtS2c,
            56 = PlayerPositionLookS2c,
            57 = UnlockRecipesS2c<'a>,
            58 = EntitiesDestroyS2c<'a>,
            59 = RemoveEntityStatusEffectS2c,
            60 = ResourcePackSendS2c<'a>,
            61 = PlayerRespawnS2c<'a>,
            62 = EntitySetHeadYawS2c,
            63 = ChunkDeltaUpdateS2c<'a>,
            64 = SelectAdvancementsTabS2c<'a>,
            65 = ServerMetadataS2c<'a>,
            66 = OverlayMessageS2c<'a>,
            67 = WorldBorderCenterChangedS2c,
            68 = WorldBorderInterpolateSizeS2c,
            69 = WorldBorderSizeChangedS2c,
            70 = WorldBorderWarningTimeChangedS2c,
            71 = WorldBorderWarningBlocksChangedS2c,
            72 = SetCameraEntityS2c,
            73 = UpdateSelectedSlotS2c,
            74 = ChunkRenderDistanceCenterS2c,
            75 = ChunkLoadDistanceS2c,
            76 = PlayerSpawnPositionS2c,
            77 = ScoreboardDisplayS2c<'a>,
            78 = EntityTrackerUpdateS2c<'a>,
            79 = EntityAttachS2c,
            80 = EntityVelocityUpdateS2c,
            81 = EntityEquipmentUpdateS2c,
            82 = ExperienceBarUpdateS2c,
            83 = HealthUpdateS2c,
            84 = ScoreboardObjectiveUpdateS2c<'a>,
            85 = EntityPassengersSetS2c,
            86 = TeamS2c<'a>,
            87 = ScoreboardPlayerUpdateS2c<'a>,
            88 = SimulationDistanceS2c,
            89 = SubtitleS2c<'a>,
            90 = WorldTimeUpdateS2c,
            91 = TitleS2c<'a>,
            92 = TitleFadeS2c,
            93 = PlaySoundFromEntityS2c,
            94 = PlaySoundS2c<'a>,
            95 = StopSoundS2c<'a>,
            96 = GameMessageS2c<'a>,
            97 = PlayerListHeaderS2c<'a>,
            98 = NbtQueryResponseS2c,
            99 = ItemPickupAnimationS2c,
            100 = EntityPositionS2c,
            101 = AdvancementUpdateS2c<'a>,
            102 = EntityAttributesS2c<'a>,
            103 = FeaturesS2c<'a>,
            104 = EntityStatusEffectS2c,
            105 = SynchronizeRecipesS2c<'a>,
            106 = SynchronizeTagsS2c<'a>,
        }
    }
}
