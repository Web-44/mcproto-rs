use crate::{types::*, uuid::*, *};
use alloc::fmt;
use alloc::{
    borrow::ToOwned,
    // boxed::Box,
    string::{String, ToString},
    vec::Vec,
};
use fmt::Debug;

#[cfg(all(test, feature = "std"))]
use crate::protocol::TestRandom;

define_protocol!(763, Packet763, RawPacket763, RawPacket763Body, Packet763Kind => {
    // handshaking
    Handshake, 0x00, Handshaking, ServerBound => HandshakeSpec {
        version: VarInt,
        server_address: String,
        server_port: u16,
        next_state: HandshakeNextState
    },

    // status
    StatusResponse, 0x00, Status, ClientBound => StatusResponseSpec {
        response: super::status::StatusSpec
    },
    StatusPong, 0x01, Status, ClientBound => StatusPongSpec {
        payload: i64
    },
    StatusRequest, 0x00, Status, ServerBound => StatusRequestSpec {},
    StatusPing, 0x01, Status, ServerBound => StatusPingSpec {
        payload: i64
    },

    // login
    LoginDisconnect, 0x00, Login, ClientBound => LoginDisconnectSpec {
        message: Chat
    },
    LoginEncryptionRequest, 0x01, Login, ClientBound => LoginEncryptionRequestSpec {
        server_id: String,
        public_key: CountedArray<u8, VarInt>,
        verify_token: CountedArray<u8, VarInt>
    },
    LoginSuccess, 0x02, Login, ClientBound => LoginSuccessSpec {
        uuid: UUID4,
        username: String,
        properties: CountedArray<PlayerAddProperty, VarInt>
    },
    LoginSetCompression, 0x03, Login, ClientBound => LoginSetCompressionSpec {
        threshold: VarInt
    },
    LoginPluginRequest, 0x04, Login, ClientBound => LoginPluginRequestSpec {
        message_id: VarInt,
        channel: String,
        data: RemainingBytes
    },
    LoginStart, 0x00, Login, ServerBound => LoginStartSpec {
        name: String,
        uuid: Option<UUID4>
    },
    LoginEncryptionResponse, 0x01, Login, ServerBound => LoginEncryptionResponseSpec {
        shared_secret: CountedArray<u8, VarInt>,
        verify_token: CountedArray<u8, VarInt>
    },
    LoginPluginResponse, 0x02, Login, ServerBound => LoginPluginResponseSpec {
        message_id: VarInt,
        successful: bool,
        data: RemainingBytes
    },

    // play
    // client bound
    PlayBundleDelimiter, 0x00, Play, ClientBound => PlayBundleDelimiterSpec {},
    PlaySpawnEntity, 0x01, Play, ClientBound => PlaySpawnEntitySpec {
        entity_id: VarInt,
        object_uuid: UUID4,
        entity_type: VarInt,
        position: Vec3<f64>,
        pitch: Angle,
        yaw: Angle,
        head_yaw: Angle,
        data: VarInt,
        velocity: Vec3<i16>
    },
    PlaySpawnExperienceOrb, 0x02, Play, ClientBound => PlaySpawnExperienceOrbSpec {
        entity_id: VarInt,
        position: Vec3<f64>,
        count: i16
    },
    PlaySpawnLivingEntity, 0x02, Play, ClientBound => PlaySpawnLivingEntitySpec {
        entity_id: VarInt,
        entity_uuid: UUID4,
        entity_type: VarInt,
        location: EntityLocation<f64, Angle>,
        head_pitch: Angle,
        velocity: Vec3<i16>
    },
    PlaySpawnPlayer, 0x03, Play, ClientBound => PlaySpawnPlayerSpec {
        entity_id: VarInt,
        uuid: UUID4,
        location: EntityLocation<f64, Angle>
    },
    PlayEntityAnimation, 0x04, Play, ClientBound => PlayEntityAnimationSpec {
        entity_id: VarInt,
        animation: EntityAnimationKind
    },
    PlayStatistics, 0x05, Play, ClientBound => PlayStatisticsSpec {
        entries: CountedArray<Statistic, VarInt>
    },
    PlayAcknowledgeBlockChange, 0x06, Play, ClientBound => PlayAcknowledgeBlockChangeSpec {
        id: VarInt
    },
    PlayBlockBreakAnimation, 0x07, Play, ClientBound => PlayBlockBreakAnimationSpec {
        entity_id: VarInt,
        location: IntPosition,
        destroy_stage: i8
    },
    PlayBlockEntityData, 0x08, Play, ClientBound => PlayBlockEntityDataSpec {
        location: IntPosition,
        action: BlockEntityDataAction,
        nbt_data: NamedNbtTag
    },
    PlayBlockAction, 0x09, Play, ClientBound => PlayBlockActionSpec {
        location: IntPosition,
        action_id: u8,
        action_payload: u8,
        block_type: VarInt
    },
    PlayBlockChange, 0x0A, Play, ClientBound => PlayBlockChangeSpec {
        location: IntPosition,
        block_id: VarInt
    },
    PlayBossBar, 0x0B, Play, ClientBound => PlayBossBarSpec {
        uuid: UUID4,
        action: BossBarAction
    },
    PlayServerDifficulty, 0x0C, Play, ClientBound => PlayServerDifficultySpec {
        difficulty: Difficulty,
        locked: bool
    },
    PlayChunkBiomes, 0x0D, Play, ClientBound => PlayChunkBiomesSpec {
        chunks: CountedArray<ChunkBiomeData, VarInt>
    },
    PlayClearTitles, 0x0E, Play, ClientBound => PlayClearTitlesSpec {
        reset: bool
    },
    PlayTabComplete, 0x0F, Play, ClientBound => PlayTabCompleteSpec {
        id: VarInt,
        start: VarInt,
        length: VarInt,
        matches: CountedArray<TabCompleteMatch, VarInt>
    },
    PlayDeclareCommands, 0x10, Play, ClientBound => PlayDeclareCommandsSpec {
        nodes: CountedArray<CommandNodeSpec, VarInt>,
        root_index: VarInt
    },
    PlayServerCloseWindow, 0x11, Play, ClientBound => PlayServerCloseWindowSpec {
        window_id: u8
    },
    PlayWindowItems, 0x12, Play, ClientBound => PlayWindowItemsSpec {
        window_id: u8,
        state_id: VarInt,
        slots: CountedArray<Slot, VarInt>,
        carried_item: Slot
    },
    PlayWindowProperty, 0x13, Play, ClientBound => PlayWindowPropertySpec {
        window_id: u8,
        property: i16,
        value: i16
    },
    PlaySetSlot, 0x14, Play, ClientBound => PlaySetSlotSpec {
        window_id: u8,
        state_id: VarInt,
        slot: i16,
        slot_data: Slot
    },
    PlaySetCooldown, 0x15, Play, ClientBound => PlaySetCooldownSpec {
        item_id: VarInt,
        cooldown_ticks: VarInt
    },
    PlayChatSuggestions, 0x16, Play, ClientBound => PlayChatSuggestionsSpec {
        action: ChatSuggestionAction,
        entries: CountedArray<String, VarInt>
    },
    PlayServerPluginMessage, 0x17, Play, ClientBound => PlayServerPluginMessageSpec {
        channel: String,
        data: RemainingBytes
    },
    PlayDamageEvent, 0x18, Play, ClientBound => PlayDamageEventSpec {
        entity_id: VarInt,
        source_type_id: VarInt,
        source_cause_id: VarInt,
        source_direct_id: VarInt,
        source_position: Option<Vec3<f64>>
    },
    PlayDeleteMessage, 0x19, Play, ClientBound => PlayDeleteMessageSpec {
        signature: CountedArray<u8, VarInt>
    },
    PlayDisconnect, 0x1A, Play, ClientBound => PlayDisconnectSpec {
        reason: Chat
    },
    PlayDisguisedChatMessage, 0x1B, Play, ClientBound => PlayServerChatMessageSpec {
        message: Chat,
        position: ChatPosition,
        sender: Chat,
        target: Option<Chat>
    },
    PlayEntityStatus, 0x1C, Play, ClientBound => PlayEntityStatusSpec {
        entity_id: i32,
        raw_status: u8 // todo deal with the gigantic table
    },
    PlayExplosion, 0x1D, Play, ClientBound => PlayExplosionSpec {
        position: Vec3<f64>,
        strength: f32,
        records: CountedArray<Vec3<i8>, VarInt>,
        player_motion: Vec3<f32>
    },
    PlayUnloadChunk, 0x1E, Play, ClientBound => PlayUnloadChunkSpec {
        position: ChunkPosition<i32>
    },
    PlayChangeGameState, 0x1F, Play, ClientBound => PlayChangeGameStateSpec {
        reason: GameChangeReason
    },
    PlayOpenHorseWindow, 0x20, Play, ClientBound => PlayOpenHorseWindowSpec {
        window_id: u8,
        number_of_slots: VarInt,
        entity_id: i32
    },
    PlayHurtAnimation, 0x21, Play, ClientBound => PlayHurtAnimationSpec {
        entity_id: VarInt,
        yaw: f32
    },
    PlayInitializeWorldBorder, 0x22, Play, ClientBound => PlayInitializeWorldBorderSpec {
        x: f64,
        z: f64,
        old_diameter: f64,
        new_diameter: f64,
        speed: VarLong,
        portal_teleport_boundary: VarInt,
        warning_blocks: VarInt,
        warning_time: VarInt
    },
    PlayServerKeepAlive, 0x23, Play, ClientBound => PlayServerKeepAliveSpec {
        id: i64
    },
    PlayChunkData, 0x24, Play, ClientBound => PlayChunkDataWrapper {
        x: i32,
        z: i32,
        heightmaps: NamedNbtTag,
        data: CountedArray<u8, VarInt>,
        block_entities: CountedArray<BlockEntityData, VarInt>,
		sky_light_mask: CountedArray<i64, VarInt>,
        block_light_mask: CountedArray<i64, VarInt>,
		empty_sky_light_mask: CountedArray<i64, VarInt>,
        empty_block_light_mask: CountedArray<i64, VarInt>,
        sky_lights: CountedArray<CountedArray<i8, VarInt>, VarInt>,
        block_lights: CountedArray<CountedArray<i8, VarInt>, VarInt>
    },
    PlayEffect, 0x25, Play, ClientBound => PlayEffectSpec {
        effect_id: EffectKind,
        location: IntPosition,
        data: i32,
        disable_relative_volume: bool
    },
    PlayParticle, 0x26, Play, ClientBound => PlayParticleSpec {
        particle_id: i32,
        long_distance: bool,
        position: Vec3<f64>,
        offset: Vec3<f32>,
        max_speed: f32,
        particle_data: i32,
        data: RemainingBytes // todo
    },
    PlayUpdateLight, 0x27, Play, ClientBound => PlayUpdateLightSpec {
        chunk: ChunkPosition<VarInt>,
        sky_light_mask: CountedArray<i64, VarInt>,
        block_light_mask: CountedArray<i64, VarInt>,
        empty_sky_light_mask: CountedArray<i64, VarInt>,
        empty_block_light_mask: CountedArray<i64, VarInt>,
        sky_lights: CountedArray<CountedArray<i8, VarInt>, VarInt>,
        block_lights: CountedArray<CountedArray<i8, VarInt>, VarInt>
    },
    PlayJoinGame, 0x28, Play, ClientBound => PlayJoinGameSpec {
        entity_id: i32,
        is_hardcore: bool,
        gamemode: GameMode,
        previous_gamemode: PreviousGameMode,
        worlds: CountedArray<String, VarInt>,
        dimension_codec: NamedNbtTag,
        dimension: String,
        world_name: String,
        hashed_seed: i64,
        max_players: VarInt,
        view_distance: VarInt,
        simulation_distance: VarInt,
        reduced_debug_info: bool,
        enable_respawn_screen: bool,
        is_debug: bool,
        is_flat: bool,
        death_location: Option<DeathLocation>,
        portal_cooldown: VarInt
    },
    PlayMapData, 0x29, Play, ClientBound => PlayMapDataSpec {
        map_id: VarInt,
        scale: i8,
        locked: bool,
        icons: Option<CountedArray<MapIconSpec, VarInt>>,
        columns: MapColumns
    },
    PlayTradeList, 0x2A, Play, ClientBound => PlayTradeListSpec {
        window_id: VarInt,
        trades: CountedArray<TradeSpec, i8>,
        villager_level: VarInt,
        experience: VarInt,
        regular_villager: bool,
        can_restock: bool
    },
    PlayEntityPosition, 0x2B, Play, ClientBound => PlayEntityPositionSpec {
        entity_id: VarInt,
        delta: Vec3<i16>,
        on_ground: bool
    },
    PlayEntityPositionAndRotation, 0x2C, Play, ClientBound => PlayEntityPositionAndRotationSpec {
        entity_id: VarInt,
        delta: EntityLocation<i16, Angle>,
        on_ground: bool
    },
    PlayEntityRotation, 0x2D, Play, ClientBound => PlayEntityRotationSpec {
        entity_id: VarInt,
        rotation: EntityRotation<Angle>,
        on_ground: bool
    },
    PlayServerVehicleMove, 0x2E, Play, ClientBound => PlayEntityVehicleMoveSpec {
        location: EntityLocation<f64, f32>
    },
    PlayOpenBook, 0x2F, Play, ClientBound => PlayOpenBookSpec {
        hand: Hand
    },
    PlayOpenWindow, 0x30, Play, ClientBound => PlayOpenWindowSpec {
        id: VarInt,
        kind: WindowType,
        title: Chat
    },
    PlayOpenSignEditor, 0x31, Play, ClientBound => PlayOpenSignEditorSpec {
        location: IntPosition,
        is_front_text: bool
    },
    PlayPing, 0x32, Play, ClientBound => PlayPingSpec {
        id: i32
    },
    PlayCraftRecipeResponse, 0x33, Play, ClientBound => PlayCraftRecipeResponseSpec {
        window_id: u8, // TODO: should be i8?
        recipe: String
    },
    PlayServerPlayerAbilities, 0x34, Play, ClientBound => PlayServerPlayerAbilitiesSpec {
        flags: PlayerAbilityFlags,
        flying_speed: f32,
        field_of_view_modifier: f32
    },
    //TODO: Chat Message
    PlayEndCombatEvent, 0x36, Play, ClientBound => PlayEndCombatEventSpec {
        duration_ticks: VarInt
    },
    PlayEnterCombatEvent, 0x37, Play, ClientBound => PlayEnterCombatEventSpec {},
    PlayDeathCombatEvent, 0x38, Play, ClientBound => PlayDeathCombatEventSpec {
        player_id: VarInt,
        message: Chat
    },
    PlayPlayerInfoRemove, 0x39, Play, ClientBound => PlayPlayerInfoRemoveSpec {
        players: CountedArray<UUID4, VarInt>
    },
    PlayPlayerInfo, 0x3A, Play, ClientBound => PlayPlayerInfoSpec {
        actions: PlayerInfoActionList
    },
    PlayFacePlayer, 0x3B, Play, ClientBound => PlayFacePlayerSpec {
        face_kind: FacePlayerKind,
        target: Vec3<f64>,
        entity: Option<FacePlayerEntityTarget>
    },
    PlayServerPlayerPositionAndLook, 0x3C, Play, ClientBound => PlayServerPlayerPositionAndLookSpec {
        location: EntityLocation<f64, f32>,
        flags: PositionAndLookFlags,
        teleport_id: VarInt
    },
    PlayUnlockRecipes, 0x3D, Play, ClientBound => PlayUnlockRecipesSpec {
        action: RecipeUnlockAction,
        crafting_book_open: bool,
        crafting_book_active: bool,
        smelting_book_open: bool,
        smelting_book_active: bool,
        blast_furnace_recipe_book_open: bool,
        blast_furnace_recipe_book_active: bool,
        smoke_recipe_book_open: bool,
        smoke_recipe_book_active: bool,
        recipe_ids: CountedArray<String, VarInt>,
        other_recipe_ids: RemainingBytes // todo
    },
    PlayDestroyEntities, 0x3E, Play, ClientBound => PlayDestroyEntitySpec {
        entity_ids: CountedArray<VarInt, VarInt>
    },
    PlayRemoveEntityEffect, 0x3F, Play, ClientBound => PlayRemoveEntityEffectSpec {
        entity_id: VarInt,
        effect: EntityEffectKind
    },
    PlayResourcePackSend, 0x40, Play, ClientBound => PlayResourcePackSendSpec {
        url: String,
        hash: String,
        forced: bool,
        prompt_message: Option<Chat>
    },
    PlayRespawn, 0x41, Play, ClientBound => PlayRespawnSpec {
        dimension: String,
        world_name: String,
        hashed_seed: i64,
        gamemode: GameMode,
        previous_gamemode: GameMode,
        is_debug: bool,
        is_flat: bool,
        data_copy: u8,
        death_location: Option<DeathLocation>,
        portal_cooldown: VarInt
    },
    PlayEntityHeadLook, 0x42, Play, ClientBound => PlayEntityHeadLookSpec {
        entity_id: VarInt,
        head_yaw: Angle
    },
    PlayMultiBlockChange, 0x43, Play, ClientBound => PlayMultiBlockChangeSpec {
        chunk: ChunkSectionPosition,
        blocks: CountedArray<MultiBlockChangeRecord, VarInt>
    },
    PlaySelectAdvancementTab, 0x44, Play, ClientBound => PlaySelectAdvancementTabSpec {
        identifier: Option<String>
    },
    PlayServerData, 0x45, Play, ClientBound => PlayServerDataSpec {
        motd: Chat,
        icon: Option<String>,
        enforces_secure_chat: bool
    },
    PlayActionBar, 0x46, Play, ClientBound => PlayActionBarSpec {
        text: Chat
    },
    PlayWorldBorderCenter, 0x47, Play, ClientBound => PlayWorldBorderCenterSpec {
        x: f64,
        z: f64
    },
    PlayWorldBorderLerpSize, 0x48, Play, ClientBound => PlayWorldBorderLerpSizeSpec {
        old_diameter: f64,
        new_diameter: f64,
        speed: VarLong
    },
    PlayWorldBorderSize, 0x49, Play, ClientBound => PlayWorldBorderSizeSpec {
        diameter: f64
    },
    PlayWorldBorderWarningDelay, 0x4A, Play, ClientBound => PlayWorldBorderWarningDelaySpec {
        warning_time: VarInt
    },
    PlayWorldBorderWarningReach, 0x4B, Play, ClientBound => PlayWorldBorderWarningReachSpec {
        warning_blocks: VarInt
    },
    PlayCamera, 0x4C, Play, ClientBound => PlayCameraSpec {
        camera_id: VarInt
    },
    PlayServerHeldItemChange, 0x4D, Play, ClientBound => PlayServerHeldItemChangeSpec {
        slot: i8
    },
    PlayUpdateViewPosition, 0x4E, Play, ClientBound => PlayUpdateViewPositionSpec {
        chunk: ChunkPosition<VarInt>
    },
    PlayUpdateViewDistance, 0x4F, Play, ClientBound => PlayUpdateViewDistanceSpec {
        view_distance: VarInt
    },
    PlaySpawnPosition, 0x50, Play, ClientBound => PlaySpawnPositionSpec {
        location: IntPosition,
        angle: f32
    },
    PlayDisplayScoreboard, 0x51, Play, ClientBound => PlayDisplayScoreboardSpec {
        position: ScoreboardPosition,
        score_name: String
    },
    PlayEntityMetadata, 0x52, Play, ClientBound => PlayEntityMetadataSpec {
        entity_id: VarInt,
        metadata: EntityMetadata
    },
    PlayAttachEntity, 0x53, Play, ClientBound => PlayAttachEntitySpec {
        attached_entity_id: i32,
        holding_entity_id: i32
    },
    PlayEntityVelocity, 0x54, Play, ClientBound => PlayEntityVelocitySpec {
        entity_id: VarInt,
        velocity: Vec3<i16>
    },
    PlayEntityEquipment, 0x55, Play, ClientBound => PlayEntityEquiptmentSpec {
        entity_id: VarInt,
        equipment: EntityEquipmentArray
    },
    PlaySetExperience, 0x56, Play, ClientBound => PlaySetExperienceSpec {
        experience_bar: f32,
        level: VarInt,
        total_experience: VarInt
    },
    PlayUpdateHealth, 0x57, Play, ClientBound => PlayUpdateHealthSpec {
        health: f32,
        food: VarInt,
        saturation: f32
    },
    PlayScoreboardObjective, 0x58, Play, ClientBound => PlayScoreboardObjectiveSpec {
        objective_name: String,
        action: ScoreboardObjectiveAction
    },
    PlaySetPassengers, 0x59, Play, ClientBound => PlaySetPassengersSpec {
        entity_id: VarInt,
        passenger_entitiy_ids: CountedArray<VarInt, VarInt>
    },
    PlayTeams, 0x5A, Play, ClientBound => PlayTeamsSpec {
        team_name: String,
        action: TeamAction
    },
    PlayUpdateScore, 0x5B, Play, ClientBound => PlayUpdateScoreSpec {
        entity_name: TeamMember,
        update: UpdateScoreSpec
    },
    PlayUpdateSimulationDistance, 0x5C, Play, ClientBound => PlayUpdateSimulationDistanceSpec {
        simulation_distance: VarInt
    },
    PlaySetTitleSubTitle, 0x5D, Play, ClientBound => PlaySetTitleSubTitleSpec {
        text: Chat
    },
    PlayTimeUpdate, 0x5E, Play, ClientBound => PlayTimeUpdateSpec {
        world_age: i64,
        time_of_day: i64
    },
    PlaySetTitleText, 0x5F, Play, ClientBound => PlaySetTitleTextSpec {
        text: Chat
    },
    PlaySetTitleTimes, 0x60, Play, ClientBound => PlaySetTitleTimesSpec {
        fade_in: i32,
        stay: i32,
        fade_out: i32
    },
    PlayEntitySoundEffect, 0x61, Play, ClientBound => PlayEntitySoundEffectSpec {
        sound_id: VarInt,
        sound_category: SoundCategory,
        entity_id: VarInt,
        volume: f32,
        pitch: f32,
        seed: i64
    },
    PlaySoundEffect, 0x62, Play, ClientBound => PlaySoundEffectSpec {
        sound_id: VarInt,
        sound_category: SoundCategory,
        position: Vec3<FixedInt>,
        volume: f32,
        pitch: f32,
        seed: i64
    },
    PlayStopSound, 0x63, Play, ClientBound => PlayStopSoundSpec {
        spec: StopSoundSpec
    },
    PlaySystemChatMessage, 0x64, Play, ClientBound => PlaySystemChatMessageSpec {
        content: Chat,
        actionbar: bool
    },
    PlayerPlayerListHeaderAndFooter, 0x65, Play, ClientBound => PlayPlayerListHeaderAndFooterSpec {
        header: Chat,
        footer: Chat
    },
    PlayNbtQueryResponse, 0x66, Play, ClientBound => PlayNbtQueryResponseSpec {
        transaction_id: VarInt,
        nbt: NamedNbtTag
    },
    PlayCollectItem, 0x67, Play, ClientBound => PlayCollectItemSpec {
        collected_entity_id: VarInt,
        collector_entity_id: VarInt,
        pickup_item_count: VarInt
    },
    PlayEntityTeleport, 0x68, Play, ClientBound => PlayEntityTeleportSpec {
        entity_id: VarInt,
        location: EntityLocation<f64, Angle>,
        on_ground: bool
    },
    PlayAdvancements, 0x69, Play, ClientBound => PlayAdvancementsSpec {
        reset: bool,
        mappings: CountedArray<AdvancementMappingEntrySpec, VarInt>,
        identifiers: CountedArray<String, VarInt>,
        progress: CountedArray<AdvancementProgressEntrySpec, VarInt>
    },
    PlayEntityProperties, 0x6A, Play, ClientBound => PlayEntityPropertiesSpec {
        entity_id: VarInt,
        properties: CountedArray<EntityPropertySpec, VarInt>
    },
    PlayFeatureFlags, 0x6B, Play, ClientBound => PlayFeatureFlagsSpec {
        features: CountedArray<String, VarInt>
    },
    PlayEntityEffect, 0x6C, Play, ClientBound => PlayEntityEffectSpec {
        entity_id: VarInt,
        effect_id: EntityEffectKind,
        amplifier: i8,
        duration_ticks: VarInt,
        flags: EntityEffectFlags
    },
    PlayDeclareRecipes, 0x6D, Play, ClientBound => PlayDeclareRecipesSpec {
        recipes: CountedArray<RecipeSpec, VarInt>
    },
    PlayTags, 0x6E, Play, ClientBound => PlayTagsSpec {
        tags: CountedArray<TypedTagList, VarInt>
    },

    // play server bound
    PlayTeleportConfirm, 0x00, Play, ServerBound => PlayTeleportConfirmSpec {
        teleport_id: VarInt
    },
    PlayQueryBlockNbt, 0x01, Play, ServerBound => PlayQueryBlockNbtSpec {
        transaction_id: VarInt,
        location: IntPosition
    },
    PlaySetDifficulty, 0x02, Play, ServerBound => PlaySetDifficultySpec {
        new_difficulty: Difficulty
    },
    PlayMessageAcknowledgement, 0x03, Play, ServerBound => PlayMessageAcknowledgementSpec {
        count: VarInt
    },
    PlayClientChatCommand, 0x04, Play, ServerBound => PlayClientChatCommandSpec {
        command: String,
        timestamp: u64,
        salt: i64,
        argument_signatures: CountedArray<ArgumentSignature, VarInt>,
        message_count: VarInt,
        acknowledged: RemainingBytes
    },
    PlayClientChatMessage, 0x05, Play, ServerBound => PlayClientChatMessageSpec {
        message: String,
        timestamp: u64,
        salt: i64,
        signature: Option<ByteArray256>,
        message_count: VarInt,
        acknowledged: RemainingBytes
    },
    PlayPlayerSession, 0x06, Play, ServerBound => PlayPlayerSessionSpec {
        session_id: UUID4,
        expires_at: u64,
        public_key: CountedArray<u8, VarInt>,
        key_signature: CountedArray<u8, VarInt>
    },
    PlayClientStatus, 0x07, Play, ServerBound => PlayClientStatusSpec {
        action: ClientStatusAction
    },
    PlayClientSettings, 0x08, Play, ServerBound => PlayClientSettingsSpec {
        locale: String,
        view_distance: i8,
        chat_mode: ClientChatMode,
        chat_colors: bool,
        displayed_skin_parts: ClientDisplayedSkinParts,
        main_hand: ClientMainHand,
        enable_text_filtering: bool,
        allow_server_listings: bool
    },
    PlayClientTabComplete, 0x09, Play, ServerBound => PlayClientTabCompleteSpec {
        transaction_id: VarInt,
        text: String
    },
    PlayClickWindowButton, 0x0A, Play, ServerBound => PlayClickWindowButtonSpec {
        window_id: i8,
        button_id: i8
    },
    PlayClickWindow, 0x0B, Play, ServerBound => PlayClickWindowSpec {
        window_id: u8, // TODO: should be u8?
        state_id: VarInt,
        slot: i16,
        button: i8,
        mode: InventoryOperationMode,
        slots: CountedArray<SlotAtSpec, VarInt>,
        clicked_item: Slot
    },
    PlayClientCloseWindow, 0x0C, Play, ServerBound => PlayClientCloseWindowSpec {
        window_id: u8
    },
    PlayClientPluginMessage, 0x0D, Play, ServerBound => PlayClientPluginMessageSpec {
        channel: String,
        data: RemainingBytes
    },
    PlayEditBook, 0x0E, Play, ServerBound => PlayEditBookSpec {
        slot: VarInt,
        pages: CountedArray<String, VarInt>,
        title: Option<String>
    },
    PlayQueryEntityNbt, 0x0F, Play, ServerBound => PlayQueryEntityNbtSpec {
        transaction_id: VarInt,
        entity_id: VarInt
    },
    PlayInteractEntity, 0x10, Play, ServerBound => PlayInteractEntitySpec {
        entity_id: VarInt,
        kind: InteractKind,
        sneaking: bool
    },
    PlayGenerateStructure, 0x11, Play, ServerBound => PlayGenerateStructureSpec {
        location: IntPosition,
        levels: VarInt,
        keep_jigsaws: bool
    },
    PlayClientKeepAlive, 0x12, Play, ServerBound => PlayClientKeepAliveSpec {
        id: i64
    },
    PlayLockDifficulty, 0x13, Play, ServerBound => PlayLockDifficultySpec {
        locked: bool
    },
    PlayPlayerPosition, 0x14, Play, ServerBound => PlayPlayerPositionSpec {
        feet_position: Vec3<f64>,
        on_ground: bool
    },
    PlayClientPlayerPositionAndRotation, 0x15, Play, ServerBound => PlayClientPlayerPositionAndRotationSpec {
        feet_location: EntityLocation<f64, f32>,
        on_ground: bool
    },
    PlayPlayerRotation, 0x16, Play, ServerBound => PlayPlayerRotationSpec {
        rotation: EntityRotation<f32>,
        on_ground: bool
    },
    PlayPlayerMovement, 0x17, Play, ServerBound => PlayPlayerMovementSpec {
        on_ground: bool
    },
    PlayClientVehicleMove, 0x18, Play, ServerBound => PlayClientVehicleMoveSpec {
        location: EntityLocation<f64, f32>
    },
    PlaySteerBoat, 0x19, Play, ServerBound => PlaySteerBoatSpec {
        left_paddle_turning: bool,
        right_paddle_turning: bool
    },
    PlayPickItem, 0x1A, Play, ServerBound => PlayPickItemSpec {
        slot_idx: VarInt
    },
    PlayCraftRecipeRequest, 0x1B, Play, ServerBound => PlayCraftRecipeRequestSpec {
        window_id: i8,
        recipe: String,
        make_all: bool
    },
    PlayClientPlayerAbilities, 0x1C, Play, ServerBound => PlayClientPlayerAbilitiesSpec {
        flags: ClientPlayerAbilities
    },
    PlayPlayerDigging, 0x1D, Play, ServerBound => PlayPlayerDiggingSpec {
        status: PlayerDiggingStatus,
        location: IntPosition,
        face: DiggingFace,
        sequence: VarInt
    },
    PlayEntityAction, 0x1E, Play, ServerBound => PlayEntityActionSpec {
        entity_id: VarInt,
        action: EntityActionKind,
        jump_boost: VarInt
    },
    PlaySteerVehicle, 0x1F, Play, ServerBound => PlaySteerVehicleSpec {
        sideways: f32,
        forward: f32,
        flags: SteerVehicleFlags
    },
    PlayPong, 0x20, Play, ServerBound => PlayPongSpec {
        id: i32
    },
    PlaySetRecipeBookState, 0x21, Play, ServerBound => PlaySetRecipeBookStateSpec {
        book_id: RecipeBookType,
        open: bool,
        filtered: bool
    },
    PlaySetDisplayedRecipe, 0x22, Play, ServerBound => PlaySetDisplayedRecipeSpec {
        recipe_id: String
    },
    PlayNameItem, 0x23, Play, ServerBound => PlayNameItemSpec {
        name: String
    },
    PlayResourcePackStatus, 0x24, Play, ServerBound => PlayResourcePackStatusSpec {
        status: ResourcePackStatus
    },
    PlayAdvancementTab, 0x25, Play, ServerBound => PlayAdvancementTabSpec {
        action: AdvancementTabAction
    },
    PlaySelectTrade, 0x26, Play, ServerBound => PlaySelectTradeSpec {
        selected_slot: VarInt
    },
    PlaySetBeaconEffect, 0x27, Play, ServerBound => PlaySetBeaconEffectSpec {
        primary_effect: VarInt,
        secondary_effect: VarInt
    },
    PlayClientHeldItemChange, 0x28, Play, ServerBound => PlayClientHeldItemChangeSpec {
        slot: i16
    },
    PlayUpdateCommandBlock, 0x29, Play, ServerBound => PlayUpdateCommandBlockSpec {
        location: IntPosition,
        command: String,
        mode: CommandBlockMode,
        flags: CommandBlockFlags
    },
    PlayUpdateCommandBlockMinecart, 0x2A, Play, ServerBound => PlayUpdateCommandBlockMinecartSpec {
        entity_id: VarInt,
        command: String,
        track_output: bool
    },
    PlayCreativeInventoryAction, 0x2B, Play, ServerBound => PlayCreativeInventoryActionSpec {
        slot: i16,
        clicked_item: Slot
    },
    PlayUpdateJigsawBlock, 0x2C, Play, ServerBound => PlayUpdateJigsawBlockSpec {
        location: IntPosition,
        name: String,
        target: String,
        pool: String,
        final_state: String,
        joint_type: String
    },
    PlayUpdateStructureBlock, 0x2D, Play, ServerBound => PlayUpdateStructureBlockSpec {
        location: IntPosition,
        action: UpdateStructureBlockAction,
        mode: UpdateStructureBlockMode,
        name: String,
        offset: Vec3<i8>,
        size: Vec3<i8>,
        mirror: UpdateStructureBlockMirror,
        rotation: UpdateStructureBlockRotation,
        metadata: String,
        integrity: f32,
        seed: VarLong,
        flags: UpdateStructureBlockFlags
    },
    PlayUpdateSign, 0x2E, Play, ServerBound => PlayUpdateSignSpec {
        location: IntPosition,
        is_front_text: bool,
        line1: String,
        line2: String,
        line3: String,
        line4: String
    },
    PlayClientAnimation, 0x2F, Play, ServerBound => PlayClientAnimationSpec {
        hand: Hand
    },
    PlaySpectate, 0x30, Play, ServerBound => PlaySpectateSpec {
        target: UUID4
    },
    PlayBlockPlacement, 0x31, Play, ServerBound => PlayBlockPlacementSpec {
        hand: Hand,
        location: IntPosition,
        face: DiggingFace,
        cursor_position: Vec3<f32>,
        inside_block: bool,
        sequence: VarInt
    },
    PlayUseItem, 0x32, Play, ServerBound => PlayUseItemSpec {
        hand: Hand,
        sequence: VarInt
    }
});

// helper types

// handshake enum
proto_byte_enum!(HandshakeNextState,
    0x01 :: Status,
    0x02 :: Login
);

proto_byte_enum!(CardinalDirection,
    0x00 :: South,
    0x01 :: West,
    0x02 :: North,
    0x03:: East
);

proto_byte_enum!(EntityAnimationKind,
    0x00 :: SwingMainArm,
    0x01 :: TakeDamage,
    0x02 :: LeaveBed,
    0x03 :: SwingOffHand,
    0x04 :: CriticalEffect,
    0x05 :: MagicCriticalEffect
);

proto_varint_enum!(StatisticCategory,
    0x00 :: Mined(VarInt),
    0x01 :: Crafted(VarInt),
    0x02 :: Used(VarInt),
    0x03 :: Broken(VarInt),
    0x04 :: PickedUp(VarInt),
    0x05 :: Dropped(VarInt),
    0x06 :: Killed(VarInt),
    0x07 :: KilledBy(VarInt),
    0x08 :: Custom(StatisticKind)
);

proto_varint_enum!(StatisticKind,
    0x00 :: LeaveGame,
    0x01 :: PlayOneMinute,
    0x02 :: TimeSinceDeath,
    0x03 :: SneakTime,
    0x04 :: WealkOneCm,
    0x05 :: CrouchOneCm,
    0x06 :: SprintOneCm,
    0x07 :: SwimOneCm,
    0x08 :: FallOneCm,
    0x09 :: ClimbOneCm,
    0x0A :: FlyOneCm,
    0x0B :: DiveOneCm,
    0x0C :: MinecartOneCm,
    0x0D :: BoatOneCm,
    0x0E :: PigOneCm,
    0x0F :: HorseOneCm,
    0x10 :: AviateOneCm,
    0x11 :: Jumps,
    0x12 :: Drops,
    0x13 :: DamageDealt,
    0x14 :: DamageTaken,
    0x15 :: Deaths,
    0x16 :: MobKills,
    0x17 :: AnimalsBread,
    0x18 :: PlayerKills,
    0x19 :: FishCaught,
    0x1A :: TalkedToVillager,
    0x1B :: TradedWithVillager,
    0x1C :: EatCakeSlice,
    0x1D :: FillCauldron,
    0x1E :: UseCauldron,
    0x1F :: CleanArmor,
    0x20 :: CleanBanner,
    0x21 :: InteractWithBrewingStand,
    0x22 :: InteractWithBeaccon,
    0x23 :: InspectDropper,
    0x24 :: InspectHopper,
    0x25 :: InspectDispenser,
    0x26 :: PlayNoteBlock,
    0x27 :: TuneNoteBlock,
    0x28 :: PotFlower,
    0x29 :: TriggerTrappedChest,
    0x2A :: OpenEnderChest,
    0x2B :: EnchantItem,
    0x2C :: PlayRecord,
    0x2D :: InteractWithFurnace,
    0x2E :: InteractWithCraftingTable,
    0x2F :: OpenChest,
    0x30 :: SleepInBed,
    0x31 :: OpenShulkerBox
);

proto_struct!(Statistic {
    kind: StatisticCategory,
    statistic: VarInt,
    value: VarInt
});

proto_byte_enum!(DiggingStatus,
    0x00 :: Started,
    0x01 :: Cancelled,
    0x02 :: Finished
);

proto_byte_enum!(BlockEntityDataAction,
    0x01 :: SetMobSpawnerData,
    0x02 :: SetCommandBlockText,
    0x03 :: SetBeaconLevelAndPower,
    0x04 :: SetMobHeadRotationAndSkin,
    0x05 :: DeclareConduit,
    0x06 :: SetBannerColorAndPatterns,
    0x07 :: SetStructureTileEntityData,
    0x08 :: SetEndGatewayDestination,
    0x09 :: SetSignText,
    0x0B :: DeclareBed,
    0x0C :: SetJigsawBlockData,
    0x0D :: SetCampfireItems,
    0x0E :: BeehiveInformation
);

proto_byte_enum!(Difficulty,
    0x00 :: Peaceful,
    0x01 :: Easy,
    0x02 :: Normal,
    0x03 :: Hard
);

proto_struct!(ChunkBiomeData {
    position: ChunkPosition<i32>,
    data: CountedArray<u8, VarInt>
});

proto_varint_enum!(ChatSuggestionAction,
    0x00 :: Add,
    0x01 :: Remove,
    0x02 :: Set
);

proto_byte_enum!(ChatPosition,
    0x00 :: ChatBox,
    0x01 :: SystemMessage,
    0x02 :: Hotbar
);

proto_struct!(BlockEntityData {
    xz: u8,
    y: u16,
    r#type: VarInt,
    data: NamedNbtTag
});

proto_str_enum!(SculkDestinationIdentifier,
    "block" :: Block(IntPosition),
    "entity" :: Entity(VarInt)
);

proto_int_enum!(EffectKind,
    1000 :: DispenserDispenses,
    1001 :: DispenserFails,
    1002 :: DispenserShoots,
    1003 :: EnderEyeLaunched,
    1004 :: FireworkShot,
    1005 :: IronDoorOpened,
    1006 :: WoodenDoorOpened,
    1007 :: WoodenTrapdoorOpened,
    1008 :: FenceGateOpened,
    1009 :: FireExtinguished,
    1010 :: PlayRecord,
    1011 :: IronDoorClosed,
    1012 :: WoodenDoorClosed,
    1013 :: WoodenTrapdoorClosed,
    1014 :: FenceGateClosed,
    1015 :: GhastWarns,
    1016 :: GhastShoots,
    1017 :: EnderDragonShoots,
    1018 :: BlazeShoots,
    1019 :: ZombieAttacksWoodDoor,
    1020 :: ZombieAttacksIronDoor,
    1021 :: ZombieBreaksWoodDoor,
    1022 :: WitherBreaksBlock,
    1023 :: WitherSpawned,
    1024 :: WitherShoots,
    1025 :: BatTakesOff,
    1026 :: ZombieInfects,
    1027 :: ZombieVillagerConverted,
    1028 :: EnderDragonDeath,
    1029 :: AnvilDestroyed,
    1030 :: AnvilUsed,
    1031 :: AnvilLanded,
    1032 :: PortalTravel,
    1033 :: ChorusFlowerGrown,
    1034 :: ChorusFlowerDied,
    1035 :: BrewingStandBrewed,
    1036 :: IronTrapdoorOpened,
    1037 :: IronTrapdoorClosed,
    1038 :: EndPortalCreatedInOverworld,
    1039 :: PhantomBites,
    1040 :: ZombieConvertsToDrowned,
    1041 :: HuskConvertsToZombieByDrowning,
    1042 :: GrindstoneUsed,
    1043 :: BookPageTurned,
    // particles:
    1500 :: ComposterComposts,
    1501 :: LavaConvertsBlock,
    1502 :: RedstoneTorchBurnsOut,
    1503 :: EnderEyePlaced,
    2000 :: Spawns10SmokeParticles,
    2001 :: BlockBreak,
    2002 :: SplashPotion,
    2003 :: EyeOfEnderBreak,
    2004 :: MobSpawn,
    2005 :: Bonemeal,
    2006 :: DragonBreath,
    2007 :: InstantSplashPotion,
    2008 :: EnderDragonDestroysBlock,
    2009 :: WetSpongeVaporizes,
    3000 :: EndGatewaySpawn,
    3001 :: EnderDragonGrowl,
    3002 :: ElectricSpark,
    3003 :: CopperApplyWax,
    3004 :: CopperRemoveWax,
    3005 :: CopperScrapeOxidation
);

proto_struct!(DeathLocation {
    name: String,
    location: IntPosition
});

proto_str_enum!(TagType,
    "minecraft:block" :: Block,
    "minecraft:item" :: Item,
    "minecraft:fluid" :: Fluid,
    "minecraft:entity_type" :: EntityType,
    "minecraft:game_event" :: GameEvent
);

proto_struct!(TypedTagList {
    tag_type: TagType,
    tags: CountedArray<TagSpec, VarInt>
});

proto_struct!(ArgumentSignature {
    name: String,
    signature: ByteArray256
});

#[derive(Clone, PartialEq, Debug)]
pub struct ByteArray256 {
    pub inner: Vec<u8>
}

impl Serialize for ByteArray256 {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        to.serialize_bytes(self.inner.as_slice())
    }
}

impl Deserialize for ByteArray256 {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        Ok(mc_deserialize_fixed_byte_array(&data, 256).map(|inner| ByteArray256 {
            inner
        }))
    }
}

fn mc_deserialize_fixed_byte_array<T>(mut data: &[u8], size: usize) -> Deserialized<'_, T> {
    let mut inner = Vec::with_capacity(size);

    for _ in 0..size {
        let Deserialized { value, data: rest } = u8::mc_deserialize(&data)?;
        data = rest;
        inner.push(value);
    }

    Deserialized::create(inner, &data)
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct BlockChangeHorizontalPosition {
    pub rel_x: u8,
    pub rel_z: u8,
}

impl Serialize for BlockChangeHorizontalPosition {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        to.serialize_byte((self.rel_x & 0xF) << 4 | (self.rel_z & 0xF))
    }
}

impl Deserialize for BlockChangeHorizontalPosition {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        Ok(
            u8::mc_deserialize(data)?.map(move |b| BlockChangeHorizontalPosition {
                rel_x: (b >> 4) & 0xF,
                rel_z: b & 0xF,
            }),
        )
    }
}

#[cfg(all(test, feature = "std"))]
impl TestRandom for BlockChangeHorizontalPosition {
    fn test_gen_random() -> Self {
        BlockChangeHorizontalPosition {
            rel_x: rand::random::<u8>() % 16,
            rel_z: rand::random::<u8>() % 16,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ChunkSectionPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Serialize for ChunkSectionPosition {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        let num = (((self.x as u64) & 0x3FFFFF) << 42)
            | (((self.z as u64) & 0x3FFFFF) << 20)
            | ((self.y as u64) & 0xFFFFF);

        to.serialize_other(&num)
    }
}

impl Deserialize for ChunkSectionPosition {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        let Deserialized { value: raw, data } = u64::mc_deserialize(data)?;
        let y = (raw & 0xFFFFF) as i32;
        let z = ((raw >> 20) & 0x3FFFFF) as i32;
        let x = ((raw >> 42) & 0x3FFFFF) as i32;
        Deserialized::ok(Self { x, y, z }, data)
    }
}

#[cfg(all(test, feature = "std"))]
impl TestRandom for ChunkSectionPosition {
    fn test_gen_random() -> Self {
        Self {
            x: (u32::test_gen_random() & 0x3FFFFF) as i32,
            y: (u32::test_gen_random() & 0xFFFFF) as i32,
            z: (u32::test_gen_random() & 0x3FFFFF) as i32,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct MultiBlockChangeRecord {
    pub block_id: u64,
    pub rel_position: Vec3<i8>,
}

impl Serialize for MultiBlockChangeRecord {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        let raw = (self.block_id << 12)
            | (((self.rel_position.x as u64) & 0xF) << 8)
            | (((self.rel_position.y as u64) & 0xF) << 4)
            | ((self.rel_position.z as u64) & 0xF);
        let raw = VarLong(raw as i64);
        to.serialize_other(&raw)
    }
}

impl Deserialize for MultiBlockChangeRecord {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        let Deserialized { value: raw, data } = VarLong::mc_deserialize(data)?;
        let raw = raw.0 as u64;
        let block_id = raw >> 12;
        let x = ((raw >> 8) & 0xF) as i8;
        let y = ((raw >> 4) & 0xF) as i8;
        let z = (raw & 0xF) as i8;
        let rel_position = (x, y, z).into();
        Deserialized::ok(
            Self {
                block_id,
                rel_position,
            },
            data,
        )
    }
}

#[cfg(all(test, feature = "std"))]
impl TestRandom for MultiBlockChangeRecord {
    fn test_gen_random() -> Self {
        let mut rel_position = <Vec3<i8>>::test_gen_random();
        rel_position.x &= 0xF;
        rel_position.y &= 0xF;
        rel_position.z &= 0xF;
        let block_id = u64::test_gen_random() & ((1 << 52) - 1);

        Self {
            block_id,
            rel_position,
        }
    }
}

proto_varint_enum!(BossBarAction,
    0x00 :: Add(BossBarAddSpec),
    0x01 :: Remove,
    0x02 :: UpdateHealth(BossBarUpdateHealthSpec),
    0x03 :: UpdateTitle(BossBarUpdateTitleSpec),
    0x04 :: UpdateStyle(BossBarUpdateStyleSpec),
    0x05 :: UpdateFlags(BossBarUpdateFlagsSpec)
);

proto_varint_enum!(BossBarColor,
    0x00 :: Pink,
    0x01 :: Blue,
    0x02 :: Red,
    0x03 :: Green,
    0x04 :: Yellow,
    0x05 :: Purple,
    0x06 :: White
);

proto_varint_enum!(BossBarDivision,
    0x00 :: NoDivision,
    0x01 :: SixNotches,
    0x02 :: TenNotches,
    0x03 :: TwelveNotches,
    0x04 :: TwentyNotches
);

proto_byte_flag!(BossBarFlags,
    0x01 :: is_darken_sky set_darken_sky,
    0x02 :: is_dragon_bar set_dragon_bar,
    0x04 :: is_create_fog set_create_fog
);

proto_struct!(BossBarAddSpec {
    title: Chat,
    health: f32,
    color: BossBarColor,
    division: BossBarDivision,
    flags: BossBarFlags
});

proto_struct!(BossBarUpdateHealthSpec { health: f32 });

proto_struct!(BossBarUpdateTitleSpec { title: String });

proto_struct!(BossBarUpdateStyleSpec {
    color: BossBarColor,
    dividers: BossBarDivision
});

proto_struct!(BossBarUpdateFlagsSpec {
    flags: BossBarFlags
});

proto_struct!(TabCompleteMatch {
    match_: String,
    tooltip: Option<Chat>
});

#[derive(Clone, Debug, PartialEq)]
pub struct CommandNodeSpec {
    pub children_indices: CountedArray<VarInt, VarInt>,
    pub redirect_node: Option<VarInt>,
    pub is_executable: bool,
    pub node: CommandNode,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CommandNode {
    Root,
    Argument(CommandArgumentNodeSpec),
    Literal(CommandLiteralNodeSpec),
}

impl Serialize for CommandNodeSpec {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        let mut flags: u8 = 0;

        use CommandNode::*;

        flags |= match &self.node {
            Root => 0x00,
            Literal(_) => 0x01,
            Argument(_) => 0x02,
        };

        if self.is_executable {
            flags |= 0x04;
        }

        if self.redirect_node.is_some() {
            flags |= 0x08;
        }

        if let Argument(body) = &self.node {
            if body.suggestions_types.is_some() {
                flags |= 0x10
            }
        }

        to.serialize_byte(flags)?;
        to.serialize_other(&self.children_indices)?;
        if let Some(redirect_node) = &self.redirect_node {
            to.serialize_other(redirect_node)?;
        }

        match &self.node {
            Root => Ok(()),
            Argument(body) => body.serialize(to),
            Literal(body) => to.serialize_other(body),
        }
    }
}

impl Deserialize for CommandNodeSpec {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        let Deserialized { value: flags, data } = u8::mc_deserialize(data)?;
        let Deserialized {
            value: children_indices,
            data,
        } = <CountedArray<VarInt, VarInt>>::mc_deserialize(data)?;
        let (redirect_node, data) = if flags & 0x08 != 0 {
            let Deserialized {
                value: redirect_node,
                data,
            } = VarInt::mc_deserialize(data)?;
            (Some(redirect_node), data)
        } else {
            (None, data)
        };
        let is_executable = flags & 0x04 != 0;

        use CommandNode::*;
        let Deserialized { value: node, data } = match flags & 0x03 {
            0x00 => Deserialized::ok(Root, data),
            0x01 => {
                Ok(CommandLiteralNodeSpec::mc_deserialize(data)?.map(move |body| Literal(body)))
            }
            0x02 => Ok(
                CommandArgumentNodeSpec::deserialize(flags & 0x10 != 0, data)?
                    .map(move |body| Argument(body)),
            ),
            other => panic!("impossible condition (bitmask) {}", other),
        }?;

        Deserialized::ok(
            Self {
                children_indices,
                redirect_node,
                is_executable,
                node,
            },
            data,
        )
    }
}

#[cfg(all(test, feature = "std"))]
impl TestRandom for CommandNodeSpec {
    fn test_gen_random() -> Self {
        let children_indices = <CountedArray<VarInt, VarInt>>::test_gen_random();
        let redirect_node = <Option<VarInt>>::test_gen_random();
        let is_executable = rand::random::<bool>();
        let idx = rand::random::<usize>() % 3;
        let node = match idx {
            0 => CommandNode::Root,
            1 => CommandNode::Argument(CommandArgumentNodeSpec::test_gen_random()),
            2 => CommandNode::Literal(CommandLiteralNodeSpec::test_gen_random()),
            other => panic!("impossible state {}", other),
        };

        Self {
            children_indices,
            redirect_node,
            is_executable,
            node,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommandArgumentNodeSpec {
    pub name: String,
    pub parser: CommandParserSpec,
    pub suggestions_types: Option<SuggestionsTypeSpec>,
}

impl CommandArgumentNodeSpec {
    fn serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        to.serialize_other(&self.name)?;
        to.serialize_other(&self.parser)?;
        if let Some(suggestions_types) = &self.suggestions_types {
            to.serialize_other(suggestions_types)?;
        }

        Ok(())
    }

    fn deserialize(has_suggestion_types: bool, data: &[u8]) -> DeserializeResult<Self> {
        let Deserialized { value: name, data } = String::mc_deserialize(data)?;
        let Deserialized {
            value: parser,
            data,
        } = CommandParserSpec::mc_deserialize(data)?;
        let (suggestions_types, data) = if has_suggestion_types {
            let Deserialized {
                value: suggestions_types,
                data,
            } = SuggestionsTypeSpec::mc_deserialize(data)?;
            (Some(suggestions_types), data)
        } else {
            (None, data)
        };

        Deserialized::ok(
            Self {
                name,
                parser,
                suggestions_types,
            },
            data,
        )
    }
}

#[cfg(all(test, feature = "std"))]
impl TestRandom for CommandArgumentNodeSpec {
    fn test_gen_random() -> Self {
        let name = String::test_gen_random();
        let suggestions_types = <Option<SuggestionsTypeSpec>>::test_gen_random();
        let parser = CommandParserSpec::test_gen_random();

        Self {
            name,
            parser,
            suggestions_types,
        }
    }
}

proto_str_enum!(SuggestionsTypeSpec,
    "minecraft:ask_server" :: AskServer,
    "minecraft:all_recipes" :: AllRecipes,
    "minecraft:available_sounds" :: AvailableSounds,
    "minecraft:summonable_entities" :: SummonableEntities,
    "minecraft:available_biomes" :: AvailableBiomes
);

proto_struct!(CommandLiteralNodeSpec { name: String });

proto_str_enum!(CommandParserSpec,
    "brigadier:bool" :: Bool,
    "brigadier:double" :: Double(DoubleParserProps),
    "brigadier:float" :: Float(FloatParserProps),
    "brigadier:integer" :: Integer(IntegerParserProps),
    "brigadier:string" :: StringParser(StringParserMode),
    "minecraft:entity" :: Entity(EntityParserFlags),
    "minecraft:game_profile" :: GameProfile,
    "minecraft:block_pos" :: BlockPosition,
    "minecraft:column_pos" :: ColumnPosition,
    "minecraft:vec3" :: Vec3,
    "minecraft:vec2" :: Vec2,
    "minecraft:block_state" :: BlockState,
    "minecraft:block_predicate" :: BlockPredicate,
    "minecraft:item_stack" :: ItemStack,
    "minecraft:item_predicate" :: ItemPredicate,
    "minecraft:color" :: Color,
    "minecraft:component" :: Component,
    "minecraft:message" :: Message,
    "minecraft:nbt" :: Nbt,
    "minecraft:nbt_path" :: NbtPath,
    "minecraft:objective" :: Objective,
    "minecraft:objective_criteria" :: ObjectiveCriteria,
    "minecraft:operation" :: Operation,
    "minecraft:particle" :: Particle,
    "minecraft:rotation" :: Rotation,
    "minecraft:angle" :: Angle,
    "minecraft:scoreboard_slot" :: ScoreboardSlot,
    "minecraft:score_holder" :: ScoreHolder(ScoreHolderFlags),
    "minecraft:swizzle" :: Swizzle,
    "minecraft:team" :: Team,
    "minecraft:item_slot" :: ItemSlot,
    "minecraft:resource_location" :: ResourceLocation,
    "minecraft:mob_effect" :: MobEffect,
    "minecraft:function" :: Function,
    "minecraft:entity_anchor" :: EntityAnchor,
    "minecraft:range" :: Range(RangeParserProps),
    "minecraft:int_range" :: IntRange,
    "minecraft:float_range" :: FloatRange,
    "minecraft:item_enchantment" :: ItemEnchantment,
    "minecraft:entity_summon" :: EntitySummon,
    "minecraft:dimension" :: Dimension,
    "minecraft:uuid" :: UUID,
    "minecraft:nbt_tag" :: NbtTag,
    "minecraft:nbt_compound_tag" :: NbtCompoundTag,
    "minecraft:time" :: Time
);

pub struct NumParserProps<T> {
    pub min: Option<T>,
    pub max: Option<T>,
}

pub type DoubleParserProps = NumParserProps<f64>;

pub type FloatParserProps = NumParserProps<f32>;

pub type IntegerParserProps = NumParserProps<i32>;

impl<T> Copy for NumParserProps<T> where T: Copy {}

impl<T> Clone for NumParserProps<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            min: self.min.clone(),
            max: self.max.clone(),
        }
    }
}

impl<T> Debug for NumParserProps<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NumParserProps(min={:?}, max={:?})", self.min, self.max)
    }
}

impl<T> PartialEq for NumParserProps<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        other.max.eq(&self.max) && other.min.eq(&self.min)
    }
}

impl<T> Serialize for NumParserProps<T>
where
    T: Serialize,
{
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        let mut flags: u8 = 0;
        if self.min.is_some() {
            flags |= 0x01;
        }

        if self.max.is_some() {
            flags |= 0x02;
        }

        to.serialize_other(&flags)?;
        if let Some(min) = &self.min {
            to.serialize_other(min)?;
        }

        if let Some(max) = &self.max {
            to.serialize_other(max)?;
        }

        Ok(())
    }
}

impl<T> Deserialize for NumParserProps<T>
where
    T: Deserialize,
{
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        let Deserialized { value: flags, data } = u8::mc_deserialize(data)?;
        let (min, data) = if flags & 0x01 != 0 {
            let Deserialized { value: min, data } = T::mc_deserialize(data)?;
            (Some(min), data)
        } else {
            (None, data)
        };

        let (max, data) = if flags & 0x02 != 0 {
            let Deserialized { value: max, data } = T::mc_deserialize(data)?;
            (Some(max), data)
        } else {
            (None, data)
        };

        let out = Self { min, max };
        Deserialized::ok(out, data)
    }
}

#[cfg(all(test, feature = "std"))]
impl<T> TestRandom for NumParserProps<T>
where
    T: TestRandom + std::cmp::PartialOrd,
    rand::distributions::Standard: rand::distributions::Distribution<T>,
{
    fn test_gen_random() -> Self {
        let has_min = rand::random::<bool>();
        let has_max = rand::random::<bool>();
        let (min, max) = if has_min && has_max {
            let a = rand::random::<T>();
            let b = rand::random::<T>();
            if a < b {
                (Some(a), Some(b))
            } else {
                (Some(b), Some(a))
            }
        } else if !has_min && !has_max {
            (None, None)
        } else {
            let v = rand::random::<T>();
            if has_min {
                (Some(v), None)
            } else {
                (None, Some(v))
            }
        };

        Self { min, max }
    }
}

proto_varint_enum!(StringParserMode,
    0x00 :: SingleWord,
    0x01 :: QuotablePharse,
    0x02 :: GreedyPhrase
);

proto_byte_flag!(EntityParserFlags,
    0x01 :: is_single_target set_single_target,
    0x02 :: is_players_only set_players_only
);

proto_byte_flag!(ScoreHolderFlags,
    0x01 :: is_multiple set_multiple
);

proto_struct!(RangeParserProps { decimal: bool });

proto_byte_enum!(TeamAction,
    0x00 :: Create(TeamActionCreateSpec),
    0x01 :: Remove,
    0x02 :: UpdateInfo(TeamActionUpdateInfoSpec),
    0x03 :: AddPlayers(TeamActionPlayerList),
    0x04 :: RemovePlayers(TeamActionPlayerList)
);

#[derive(Clone, Debug, PartialEq)]
pub enum TeamMember {
    Player(String),
    Entity(UUID4),
}

impl Serialize for TeamMember {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        use TeamMember::*;
        match self {
            Player(username) => username.mc_serialize(to),
            Entity(entity_id) => entity_id.to_string().mc_serialize(to),
        }
    }
}

impl Deserialize for TeamMember {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        use TeamMember::*;

        Ok(String::mc_deserialize(data)?.map(move |raw| {
            if let Some(entity_id) = UUID4::parse(raw.as_str()) {
                Entity(entity_id)
            } else {
                Player(raw)
            }
        }))
    }
}

#[cfg(all(test, feature = "std"))]
impl TestRandom for TeamMember {
    fn test_gen_random() -> Self {
        use TeamMember::*;

        let rand_bool: bool = rand::random();
        if rand_bool {
            Player(String::test_gen_random())
        } else {
            Entity(UUID4::random())
        }
    }
}

proto_str_enum!(TeamTagNameVisibility,
    "always" :: Always,
    "hideForOtherTeams" :: HideForOtherTeams,
    "hideForOwnTeam" :: HideForOwnTeam,
    "never" :: Never
);

proto_str_enum!(TeamCollisionRule,
    "always" :: Always,
    "pushForOtherTeams" :: PushForOtherTeams,
    "pushOwnTeam" :: PushOwnTeam,
    "never" :: Never
);

proto_struct!(TeamActionPlayerList {
    entities: CountedArray<TeamMember, VarInt>
});

proto_struct!(TeamActionCreateSpec {
    display_name: Chat,
    friendly_flags: TeamFriendlyFlags,
    tag_name_visibility: TeamTagNameVisibility,
    collision_rule: TeamCollisionRule,
    color: VarInt,
    prefix: Chat,
    suffix: Chat,
    entities: CountedArray<TeamMember, VarInt>
});

proto_struct!(TeamActionUpdateInfoSpec {
    display_name: Chat,
    friendly_flags: TeamFriendlyFlags,
    tag_name_visibility: TeamTagNameVisibility,
    collision_rule: TeamCollisionRule,
    color: VarInt,
    prefix: Chat,
    suffix: Chat
});

proto_byte_flag!(TeamFriendlyFlags,
    0x01 :: allow_friendly_fire set_friendly_fire,
    0x02 :: show_invisible_teammates set_show_invisible_teammates
);

proto_byte_enum!(UpdateScoreAction,
    0x00 :: Upsert(VarInt),
    0x01 :: Remove
);

#[derive(Clone, Debug, PartialEq)]
pub struct UpdateScoreSpec {
    pub objective_name: String,
    pub action: UpdateScoreAction,
}

impl Serialize for UpdateScoreSpec {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        to.serialize_byte(self.action.id())?;
        to.serialize_other(&self.objective_name)?;
        self.action.serialize_body(to)?;

        Ok(())
    }
}

impl Deserialize for UpdateScoreSpec {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        let Deserialized {
            value: action_id,
            data,
        } = u8::mc_deserialize(data)?;
        let Deserialized {
            value: objective_name,
            data,
        } = String::mc_deserialize(data)?;

        Ok(
            UpdateScoreAction::deserialize_with_id(action_id, data)?.map(move |action| Self {
                objective_name,
                action,
            }),
        )
    }
}

#[cfg(all(test, feature = "std"))]
impl TestRandom for UpdateScoreSpec {
    fn test_gen_random() -> Self {
        Self {
            objective_name: String::test_gen_random(),
            action: UpdateScoreAction::test_gen_random(),
        }
    }
}

// proto_varint_enum!(TitleActionSpec,
//     0x00 :: SetTitle(Chat),
//     0x01 :: SetSubtitle(Chat),
//     0x02 :: SetActionBar(Chat),
//     0x03 :: SetTimesAndDisplay(TitleTimesSpec),
//     0x04 :: Hide,
//     0x05 :: Reset
// );

// proto_struct!(TitleTimesSpec {
//     fade_in: i32,
//     stay: i32,
//     fade_out: i32
// });

proto_varint_enum!(SoundCategory,
    0x00 :: Master,
    0x01 :: Music,
    0x02 :: Records,
    0x03 :: Weather,
    0x04 :: Block,
    0x05 :: Hostile,
    0x06 :: Neutral,
    0x07 :: Player,
    0x08 :: Ambient,
    0x09 :: Voice
);

#[derive(Clone, Debug, PartialEq)]
pub struct StopSoundSpec {
    pub source: Option<SoundCategory>,
    pub sound: Option<String>,
}

impl Serialize for StopSoundSpec {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        let has_sound = self.sound.is_some();
        let has_source = self.source.is_some();
        let mut flags = 0;
        if has_sound {
            flags |= 0x02;
        }
        if has_source {
            flags |= 0x01;
        }

        to.serialize_byte(flags)?;
        if let Some(source) = &self.source {
            to.serialize_other(source)?;
        }

        if let Some(sound) = &self.sound {
            to.serialize_other(sound)?;
        }

        Ok(())
    }
}

impl Deserialize for StopSoundSpec {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        let Deserialized { value: flags, data } = u8::mc_deserialize(data)?;

        let is_source_present = flags & 0x01 != 0;
        let is_sound_present = flags & 0x02 != 0;

        let (source, data) = if is_source_present {
            let Deserialized {
                value: source,
                data,
            } = SoundCategory::mc_deserialize(data)?;
            (Some(source), data)
        } else {
            (None, data)
        };

        let (sound, data) = if is_sound_present {
            let Deserialized { value: sound, data } = String::mc_deserialize(data)?;
            (Some(sound), data)
        } else {
            (None, data)
        };

        Deserialized::ok(Self { source, sound }, data)
    }
}

#[cfg(all(test, feature = "std"))]
impl TestRandom for StopSoundSpec {
    fn test_gen_random() -> Self {
        let source = if rand::random::<bool>() {
            Some(SoundCategory::test_gen_random())
        } else {
            None
        };

        let sound = if source.is_none() || rand::random::<bool>() {
            Some(String::test_gen_random())
        } else {
            None
        };

        Self { source, sound }
    }
}

proto_byte_enum!(GameMode,
    0x00 :: Survival,
    0x01 :: Creative,
    0x02 :: Adventure,
    0x03 :: Spectator
);

#[derive(Clone, Debug, PartialEq)]
pub enum PreviousGameMode {
    NoPrevious,
    Previous(GameMode),
}

impl PreviousGameMode {
    pub fn id(&self) -> i8 {
        use PreviousGameMode::*;
        match self {
            NoPrevious => -1,
            Previous(mode) => mode.id() as i8,
        }
    }
}

impl Serialize for PreviousGameMode {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        to.serialize_byte(self.id() as u8)
    }
}

impl Deserialize for PreviousGameMode {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        let Deserialized { value: id, data } = i8::mc_deserialize(data)?;

        use PreviousGameMode::*;
        match id {
            -1 => Deserialized::ok(NoPrevious, data),
            other => {
                Ok(GameMode::deserialize_with_id(other as u8, data)?.map(move |gm| Previous(gm)))
            }
        }
    }
}

impl Into<Option<GameMode>> for PreviousGameMode {
    fn into(self) -> Option<GameMode> {
        use PreviousGameMode::*;
        match self {
            NoPrevious => None,
            Previous(mode) => Some(mode),
        }
    }
}

#[cfg(all(test, feature = "std"))]
impl TestRandom for PreviousGameMode {
    fn test_gen_random() -> Self {
        use PreviousGameMode::*;
        match <Option<GameMode> as TestRandom>::test_gen_random() {
            Some(gamemode) => Previous(gamemode),
            None => NoPrevious,
        }
    }
}

proto_byte_enum!(WinGameAction,
    0x00 :: Respawn,
    0x01 :: RollCreditsAndRespawn
);

proto_byte_enum!(DemoEvent,
    0x00 :: ShowWelcomeScreen,
    0x65 :: TellMovementControls,
    0x66 :: TellJumpControl,
    0x67 :: TellInventoryControl,
    0x68 :: EndDemo
);

proto_byte_enum!(RespawnRequestType,
    0x00 :: Screen,
    0x01 :: Immediate
);

#[derive(Clone, Debug, PartialEq)]
pub enum GameChangeReason {
    NoRespawnAvailable,
    EndRaining,
    BeginRaining,
    ChangeGameMode(GameMode),
    WinGame(WinGameAction),
    Demo(DemoEvent),
    ArrowHitPlayer,
    RainLevelChange(f32),
    ThunderLevelChange(f32),
    PufferfishSting,
    ElderGuardianMobAppearance,
    Respawn(RespawnRequestType),
}

impl Serialize for GameChangeReason {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        use GameChangeReason::*;
        to.serialize_byte(match self {
            NoRespawnAvailable => 0x00,
            EndRaining => 0x01,
            BeginRaining => 0x02,
            ChangeGameMode(_) => 0x03,
            WinGame(_) => 0x04,
            Demo(_) => 0x05,
            ArrowHitPlayer => 0x06,
            RainLevelChange(_) => 0x07,
            ThunderLevelChange(_) => 0x08,
            PufferfishSting => 0x09,
            ElderGuardianMobAppearance => 0x0A,
            Respawn(_) => 0x0B,
        })?;

        let value = match self {
            ChangeGameMode(body) => body.id() as f32,
            WinGame(body) => body.id() as f32,
            Demo(body) => body.id() as f32,
            RainLevelChange(body) => *body,
            ThunderLevelChange(body) => *body,
            Respawn(body) => body.id() as f32,
            _ => 0 as f32,
        };
        to.serialize_other(&value)
    }
}

impl Deserialize for GameChangeReason {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        let Deserialized {
            value: reason_id,
            data,
        } = u8::mc_deserialize(data)?;
        let Deserialized { value, data } = f32::mc_deserialize(data)?;
        use GameChangeReason::*;
        match reason_id {
            0x00 => Deserialized::ok(NoRespawnAvailable, data),
            0x01 => Deserialized::ok(EndRaining, data),
            0x02 => Deserialized::ok(BeginRaining, data),
            0x03 => Ok(GameMode::deserialize_with_id(value as u8, data)?
                .map(move |mode| ChangeGameMode(mode))),
            0x04 => Ok(WinGameAction::deserialize_with_id(value as u8, data)?
                .map(move |mode| WinGame(mode))),
            0x05 => {
                Ok(DemoEvent::deserialize_with_id(value as u8, data)?.map(move |mode| Demo(mode)))
            }
            0x06 => Deserialized::ok(ArrowHitPlayer, data),
            0x07 => Deserialized::ok(RainLevelChange(value), data),
            0x08 => Deserialized::ok(ThunderLevelChange(value), data),
            0x09 => Deserialized::ok(PufferfishSting, data),
            0x0A => Deserialized::ok(ElderGuardianMobAppearance, data),
            0x0B => Ok(RespawnRequestType::deserialize_with_id(value as u8, data)?
                .map(move |mode| Respawn(mode))),
            other => Err(DeserializeErr::CannotUnderstandValue(alloc::format!(
                "invalid game change reason id {}",
                other
            ))),
        }
    }
}

#[cfg(all(test, feature = "std"))]
impl TestRandom for GameChangeReason {
    fn test_gen_random() -> Self {
        // todo
        GameChangeReason::PufferfishSting
    }
}

proto_varint_enum!(MapIconType,
    0x00 :: WhiteArrow,
    0x01 :: GreenArrow,
    0x02 :: RedArrow,
    0x03 :: BlueArrow,
    0x04 :: WhiteCross,
    0x05 :: RedPointer,
    0x06 :: WhiteCircle,
    0x07 :: SmallWhiteCircle,
    0x08 :: Mansion,
    0x09 :: Temple,
    0x0A :: WhiteBanner,
    0x0B :: OrangeBanner,
    0x0C :: MagentaBanner,
    0x0D :: LightBlueBanner,
    0x0E :: YellowBanner,
    0x0F :: LimeBanner,
    0x10 :: PinkBanner,
    0x11 :: GrayBanner,
    0x12 :: LightGrayBanner,
    0x13 :: CyanBanner,
    0x14 :: PurpleBanner,
    0x15 :: BlueBanner,
    0x16 :: BrownBanner,
    0x17 :: GreenBanner,
    0x18 :: RedBanner,
    0x19 :: BlackBanner,
    0x1A :: TreasureMarker
);

proto_struct!(MapIconSpec {
    kind: MapIconType,
    position: TopDownPosition<i8>,
    direction: i8,
    display_name: Option<Chat>
});

#[derive(Clone, PartialEq, Debug)]
pub enum MapColumns {
    NoUpdates,
    Updated(MapColumnsSpec),
}

proto_struct!(MapColumnsSpec {
    columns: u8,
    rows: u8, // TODO: docs just say "Byte", should be i8?
    position: TopDownPosition<u8>,
    data: CountedArray<u8, VarInt>
});

impl Serialize for MapColumns {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        use MapColumns::*;
        match self {
            NoUpdates => to.serialize_other(&0u8),
            Updated(body) => to.serialize_other(body),
        }
    }
}

impl Deserialize for MapColumns {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        let Deserialized {
            value: columns,
            data: rest,
        } = u8::mc_deserialize(data)?;
        use MapColumns::*;
        match columns {
            0x00 => Deserialized::ok(NoUpdates, rest),
            _ => Ok(MapColumnsSpec::mc_deserialize(data)?.map(move |v| Updated(v))),
        }
    }
}

impl Into<Option<MapColumnsSpec>> for MapColumns {
    fn into(self) -> Option<MapColumnsSpec> {
        use MapColumns::*;
        match self {
            NoUpdates => None,
            Updated(body) => Some(body),
        }
    }
}

impl From<Option<MapColumnsSpec>> for MapColumns {
    fn from(other: Option<MapColumnsSpec>) -> Self {
        use MapColumns::*;
        match other {
            Some(body) => {
                if body.columns == 0 {
                    NoUpdates
                } else {
                    Updated(body)
                }
            }
            None => NoUpdates,
        }
    }
}

#[cfg(all(test, feature = "std"))]
impl TestRandom for MapColumns {
    fn test_gen_random() -> Self {
        <Option<MapColumnsSpec>>::test_gen_random().into()
    }
}

proto_struct!(TradeSpec {
    input_item_1: Slot,
    output_item: Slot,
    input_item_2: Slot,
    trade_disabled: bool,
    trade_uses: i32,
    max_trade_uses: i32,
    xp: i32,
    special_price: i32,
    price_multiplier: f32,
    demand: i32
});

proto_varint_enum!(Hand,
    0x00 :: MainHand,
    0x01 :: OffHand
);

proto_varint_enum!(WindowType, // TODO: does 0x00 actually map to GenericOneRow?
    0x00 :: GenericOneRow,
    0x01 :: GenericTwoRow,
    0x02 :: GenericThreeRow,
    0x03 :: GenericFourRow,
    0x04 :: GenericFiveRow,
    0x05 :: GenericSixRow,
    0x06 :: GenericSquare,
    0x07 :: Anvil,
    0x08 :: Beacon,
    0x09 :: BlastFurnace,
    0x0A :: BrewingStand,
    0x0B :: CraftingTable,
    0x0C :: EnchantmentTable,
    0x0D :: Furnace,
    0x0E :: Grindstone,
    0x0F :: Hopper,
    0x10 :: Lectern,
    0x11 :: Loom,
    0x12 :: Merchant,
    0x13 :: ShulkerBox,
    0x14 :: Smoker,
    0x15 :: Cartography,
    0x16 :: StoneCutter
);

proto_byte_flag!(PlayerAbilityFlags,
    0x01 :: is_invulnerable set_invulnerable,
    0x02 :: is_flying set_flying,
    0x04 :: is_flight_allowed set_flight_allowed,
    0x08 :: is_instant_break set_instant_break
);

// proto_varint_enum!(CombatEvent,
//     0x00 :: Enter,
//     0x01 :: End(CombatEndSpec),
//     0x02 :: EntityDead(CombatEntityDeadSpec)
// );

// proto_struct!(CombatEndSpec {
//     duration_ticks: VarInt,
//     entity_id: i32
// });

// proto_struct!(CombatEntityDeadSpec {
//     player_id: VarInt,
//     entity_id: i32,
//     message: Chat
// });

proto_struct!(PlayerInfoAction<A> {
    uuid: UUID4,
    action: A
});

proto_varint_enum!(PlayerInfoActionList,
    0x00 :: Add(CountedArray<PlayerInfoAction<PlayerAddActionSpec>, VarInt>),
    0x01 :: InitChat(CountedArray<Option<PlayerInitChatSignatureData>, VarInt>),
    0x02 :: UpdateGameMode(CountedArray<PlayerInfoAction<GameMode>, VarInt>),
    0x03 :: UpdateListed(CountedArray<PlayerInfoAction<bool>, VarInt>),
    0x04 :: UpdateLatency(CountedArray<PlayerInfoAction<VarInt>, VarInt>),
    0x05 :: UpdateDisplayName(CountedArray<PlayerInfoAction<Option<Chat>>, VarInt>)
);

proto_struct!(PlayerAddActionSpec {
    name: String,
    properties: CountedArray<PlayerAddProperty, VarInt>,
    game_mode: GameMode,
    ping_ms: VarInt,
    display_name: Option<Chat>
});

proto_struct!(PlayerInitChatSignatureData {
    session_id: UUID4,
    public_key_expiry: u64,
    encoded_public_key: CountedArray<u8, VarInt>,
    public_key_signature: CountedArray<u8, VarInt>
});

proto_struct!(PlayerAddProperty {
    name: String,
    value: String,
    signature: Option<String>
});

proto_varint_enum!(FacePlayerKind,
    0x00 :: Feet,
    0x01 :: Eyes
);

proto_struct!(FacePlayerEntityTarget {
    entity_id: VarInt,
    kind: FacePlayerKind
});

proto_byte_flag!(PositionAndLookFlags,
    0x01 :: is_x_rel set_x_rel,
    0x02 :: is_y_rel set_y_rel,
    0x04 :: is_z_rel set_z_rel,
    0x08 :: is_y_rotation_rel set_y_rotation_rel,
    0x10 :: is_x_rotation_rel set_x_rotation_rel
);

proto_byte_enum!(EntityEffectKind,
    0x01 :: Speed,
    0x02 :: Slowness,
    0x03 :: Haste,
    0x04 :: MiningFatigue,
    0x05 :: Strength,
    0x06 :: InstantHealth,
    0x07 :: InstantDamage,
    0x08 :: JumpBoost,
    0x09 :: Nausea,
    0x0A :: Regeneration,
    0x0B :: Resistance,
    0x0C :: FireResistance,
    0x0D :: WaterBreathing,
    0x0E :: Invisibility,
    0x0F :: Blindness,
    0x10 :: NightVision,
    0x11 :: Hunger,
    0x12 :: Weakness,
    0x13 :: Poison,
    0x14 :: Wither,
    0x15 :: HealthBoost,
    0x16 :: Absorption,
    0x17 :: Saturation,
    0x18 :: Glowing,
    0x19 :: Levetation,
    0x1A :: Luck,
    0x1B :: Unluck,
    0x1C :: SlowFalling,
    0x1D :: ConduitPower,
    0x1E :: DolphinsGrace,
    0x1F :: BadOmen,
    0x20 :: HeroOfTheVillage
);

// proto_varint_enum!(WorldBorderAction,
//     0x00 :: SetSize(WorldBorderSetSizeSpec),
//     0x01 :: LerpSize(WorldBorderLerpSizeSpec),
//     0x02 :: SetCenter(TopDownPosition<f64>),
//     0x03 :: Initialize(WorldBorderInitiaializeSpec),
//     0x04 :: SetWarningTime(WorldBorderWarningTimeSpec),
//     0x05 :: SetWarningBlocks(WorldBorderWarningBlocksSpec)
// );

// proto_struct!(WorldBorderSetSizeSpec {
//     diameter: f64
// });

// proto_struct!(WorldBorderLerpSizeSpec {
//     old_diameter: f64,
//     new_diameter: f64,
//     speed: VarLong
// });

// proto_struct!(WorldBorderInitializeSpec {
//     position: TopDownPosition<f64>,
//     old_diameter: f64,
//     new_diameter: f64,
//     speed: VarLong,
//     portal_teleport_boundary: VarLong,
//     warning_time: VarInt,
//     warning_blocks: VarInt
// });

// proto_struct!(WorldBorderWarningTimeSpec {
//     warning_time: VarInt
// });

// proto_struct!(WorldBorderWarningBlocksSpec {
//     warning_blocks: VarInt
// });

proto_byte_enum!(ScoreboardPosition,
    0x00 :: List,
    0x01 :: Sidebar,
    0x02 :: BelowName,
    0x03 :: TeamSpecific(i8)
);

#[derive(Clone, Debug, PartialEq)]
pub struct EntityEquipmentEntry {
    pub slot: EquipmentSlot,
    pub item: Slot,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EntityEquipmentArray {
    data: Vec<EntityEquipmentEntry>,
}

impl Serialize for EntityEquipmentArray {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        if self.data.is_empty() {
            return Err(SerializeErr::CannotSerialize(
                "entity equipment must be non-empty to send!".to_owned(),
            ));
        }

        let n = self.data.len();

        for i in 0..n {
            let is_last = i == n - 1;
            let entry = self.data.get(i).expect("iter in bounds");
            let mut slot_raw = entry.slot.id();
            if !is_last {
                slot_raw |= 0x80;
            }
            to.serialize_byte(slot_raw)?;
            to.serialize_other(&entry.item)?;
        }

        Ok(())
    }
}

impl Deserialize for EntityEquipmentArray {
    fn mc_deserialize(mut data: &[u8]) -> DeserializeResult<'_, Self> {
        if data.is_empty() {
            return DeserializeErr::Eof.into();
        }

        let mut out = Vec::new();
        let mut has_next = true;
        while has_next {
            let Deserialized {
                value: raw_slot_id,
                data: rest,
            } = u8::mc_deserialize(data)?;
            has_next = raw_slot_id & 0x80 != 0;
            let slot_id = raw_slot_id & 0x7F;
            let Deserialized {
                value: slot,
                data: rest,
            } = EquipmentSlot::deserialize_with_id(slot_id, rest)?;
            let Deserialized {
                value: item,
                data: rest,
            } = Slot::mc_deserialize(rest)?;
            out.push(EntityEquipmentEntry { slot, item });
            data = rest;
        }

        Deserialized::ok(Self { data: out }, data)
    }
}

impl core::ops::Deref for EntityEquipmentArray {
    type Target = Vec<EntityEquipmentEntry>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl core::ops::DerefMut for EntityEquipmentArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl From<Vec<EntityEquipmentEntry>> for EntityEquipmentArray {
    fn from(data: Vec<EntityEquipmentEntry>) -> Self {
        Self { data }
    }
}

impl From<EntityEquipmentArray> for Vec<EntityEquipmentEntry> {
    fn from(other: EntityEquipmentArray) -> Self {
        other.data
    }
}

#[cfg(all(test, feature = "std"))]
impl TestRandom for EntityEquipmentArray {
    fn test_gen_random() -> Self {
        let mut out = Vec::new();

        for slot in &[
            EquipmentSlot::MainHand,
            EquipmentSlot::OffHand,
            EquipmentSlot::ArmorBoots,
            EquipmentSlot::ArmorLeggings,
            EquipmentSlot::ArmorChestplate,
            EquipmentSlot::ArmorHelmet,
        ] {
            out.push(EntityEquipmentEntry {
                slot: slot.clone(),
                item: Slot::test_gen_random(),
            });
        }

        out.into()
    }
}

proto_byte_enum!(EquipmentSlot,
    0x00 :: MainHand,
    0x01 :: OffHand,
    0x02 :: ArmorBoots,
    0x03 :: ArmorLeggings,
    0x04 :: ArmorChestplate,
    0x05 :: ArmorHelmet
);

proto_byte_enum!(ScoreboardObjectiveAction,
    0x00 :: Create(ScoreboardObjectiveSpec),
    0x01 :: Remove,
    0x02 :: UpdateText(ScoreboardObjectiveSpec)
);

proto_varint_enum!(ScoreboardObjectiveKind,
    0x00 :: Integer,
    0x01 :: Hearts
);

proto_struct!(ScoreboardObjectiveSpec {
    text: Chat,
    kind: ScoreboardObjectiveKind
});

proto_struct!(AdvancementMappingEntrySpec {
    key: String,
    value: AdvancementSpec
});

proto_struct!(AdvancementSpec {
    parent: Option<String>,
    display: Option<AdvancementDisplaySpec>,
    criteria: CountedArray<String, VarInt>,
    requirements: CountedArray<CountedArray<String, VarInt>, VarInt>
});

proto_struct!(AdvancementDisplaySpec {
    title: Chat,
    description: Chat,
    icon: Slot,
    frame_type: AdvancementFrameType,
    flags: AdvancementDisplayFlags,
    position: Vec2<f32>
});

proto_struct!(SlotAtSpec {
    number: i16,
    data: Slot
});

#[derive(Clone, Debug, PartialEq)]
pub struct AdvancementDisplayFlags {
    pub background_texture: Option<String>,
    pub show_toast: bool,
    pub hidden: bool,
}

impl Serialize for AdvancementDisplayFlags {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        let mut raw_flags: i32 = 0;
        if self.background_texture.is_some() {
            raw_flags |= 0x01;
        }
        if self.show_toast {
            raw_flags |= 0x02;
        }
        if self.hidden {
            raw_flags |= 0x04;
        }

        to.serialize_other(&raw_flags)?;
        if let Some(texture) = &self.background_texture {
            to.serialize_other(texture)?;
        }

        Ok(())
    }
}

impl Deserialize for AdvancementDisplayFlags {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        let Deserialized {
            value: raw_flags,
            data,
        } = i32::mc_deserialize(data)?;
        let has_background_texture = raw_flags & 0x01 != 0;
        let show_toast = raw_flags & 0x02 != 0;
        let hidden = raw_flags & 0x04 != 0;

        Ok(if has_background_texture {
            String::mc_deserialize(data)?.map(move |id| Some(id))
        } else {
            Deserialized { value: None, data }
        }
        .map(move |background_texture| Self {
            background_texture,
            show_toast,
            hidden,
        }))
    }
}

#[cfg(all(test, feature = "std"))]
impl TestRandom for AdvancementDisplayFlags {
    fn test_gen_random() -> Self {
        let background_texture = if rand::random::<bool>() {
            Some(String::test_gen_random())
        } else {
            None
        };
        let show_toast = rand::random::<bool>();
        let hidden = rand::random::<bool>();

        Self {
            background_texture,
            show_toast,
            hidden,
        }
    }
}

proto_varint_enum!(AdvancementFrameType,
    0x00 :: Task,
    0x01 :: Challenge,
    0x02 :: Goal
);

proto_struct!(AdvancementProgressEntrySpec {
    key: String,
    value: AdvancementProgressSpec
});

proto_struct!(AdvancementProgressSpec {
    criteria: CountedArray<AdvancementCriteriaSpec, VarInt>
});

proto_struct!(AdvancementCriteriaSpec {
    identifier: String,
    progress: AdvancementCriterionProgressSpec
});

proto_struct!(AdvancementCriterionProgressSpec {
    achieved_at: Option<i64>
});

proto_struct!(EntityPropertySpec {
    key: String,
    value: f64,
    modifiers: CountedArray<EntityPropertyModifierSpec, VarInt>
});

proto_struct!(EntityPropertyModifierSpec {
    uuid: UUID4,
    amount: f64,
    operation: EntityPropertyModifierOperation
});

proto_byte_enum!(EntityPropertyModifierOperation,
    0x00 :: AddSubtractAmount,
    0x01 :: AddSubtractAmountPercentOfCurrent,
    0x02 :: MultiplyByAmountPercent
);

proto_byte_flag!(EntityEffectFlags,
    0x01 :: is_ambient set_ambient,
    0x02 :: is_show_particles set_show_particles,
    0x04 :: is_show_icon set_show_icon
);

proto_struct!(TagSpec {
    name: String,
    entries: CountedArray<VarInt, VarInt>
});

proto_varint_enum!(ClientStatusAction,
    0x00 :: PerformRespawn,
    0x01 :: RequestStats
);

proto_varint_enum!(ClientChatMode,
    0x00 :: Enabled,
    0x01 :: CommandsOnly,
    0x02 :: Hidden
);

proto_varint_enum!(ClientMainHand,
    0x00 :: Left,
    0x01 :: Right
);

proto_byte_flag!(ClientDisplayedSkinParts,
    0x01 :: is_cape_enabled set_cape_enabled,
    0x02 :: is_jacket_enabled set_jacket_enabled,
    0x04 :: is_left_sleeve_enabled set_left_sleeve_enabled,
    0x08 :: is_right_sleeve_enabled set_right_sleeve_enabled,
    0x10 :: is_left_pants_leg_enabled set_left_pants_leg_enabled,
    0x20 :: is_right_pant_legs_enabled set_right_pant_legs_enabled,
    0x40 :: is_hat_enabled set_hat_enabled
);

proto_varint_enum!(InventoryOperationMode,
    0x00 :: MouseClick,
    0x01 :: ShiftClick,
    0x02 :: NumberClick,
    0x03 :: MiddleClick,
    0x04 :: DropClick,
    0x05 :: Drag,
    0x06 :: DoubleClick
);

proto_struct!(InteractAtSpec {
    target_position: Vec3<f32>,
    hand: Hand
});

proto_varint_enum!(InteractKind,
    0x00 :: Interact(Hand),
    0x01 :: Attack,
    0x02 :: InteractAt(InteractAtSpec)
);

proto_byte_flag!(ClientPlayerAbilities,
    0x01 :: is_creative set_creative,
    0x02 :: is_flying set_flying,
    0x04 :: is_fly_enabled set_fly_enabled,
    0x08 :: is_damaged_disabled set_damaged_disabled
);

proto_varint_enum!(PlayerDiggingStatus,
    0x00 :: Started,
    0x01 :: Cancelled,
    0x02 :: Finished,
    0x03 :: DropStack,
    0x04 :: DropItem,
    0x05 :: ShootArrowOrFishEating,
    0x06 :: SwapItemInHand
);

proto_byte_enum!(DiggingFace,
    0x00 :: Bottom,
    0x01 :: Top,
    0x02 :: North,
    0x03 :: South,
    0x04 :: West,
    0x05 :: East
);

proto_varint_enum!(EntityActionKind,
    0x00 :: StartSneaking,
    0x01 :: StopSneaking,
    0x02 :: LeaveBed,
    0x03 :: StartSprinting,
    0x04 :: StopSprinting,
    0x05 :: StartJumpWithHorse,
    0x06 :: StopJumpWithHorse,
    0x07 :: OpenHorseInventory,
    0x08 :: StartFlyingWithElytra
);

proto_byte_flag!(SteerVehicleFlags,
    0x01 :: is_jump set_jump,
    0x02 :: is_unmount set_unmount
);

proto_varint_enum!(RecipeBookType,
    0x00 :: Crafting,
    0x01 :: Furnace,
    0x02 :: BlastFurnace,
    0x03 :: Smoker
);

proto_varint_enum!(ResourcePackStatus,
    0x00 :: Loaded,
    0x01 :: Declined,
    0x02 :: FailedDownload,
    0x03 :: Accepted
);

proto_varint_enum!(AdvancementTabAction,
    0x00 :: Opened(String),
    0x01 :: Closed
);

proto_varint_enum!(CommandBlockMode,
    0x00 :: Sequence,
    0x01 :: Auto,
    0x02 :: Redstone
);

proto_byte_flag!(CommandBlockFlags,
    0x01 :: is_track_output set_track_output,
    0x02 :: is_conditional set_conditional,
    0x04 :: is_automatic set_automatic
);

proto_varint_enum!(UpdateStructureBlockAction,
    0x00 :: UpdateData,
    0x01 :: SaveStructure,
    0x02 :: LoadStructure,
    0x03 :: DetectSize
);

proto_varint_enum!(UpdateStructureBlockMode,
    0x00 :: Save,
    0x01 :: Load,
    0x02 :: Corner,
    0x03 :: Data
);

proto_varint_enum!(UpdateStructureBlockMirror,
    0x00 :: NoMirror,
    0x01 :: LeftRight,
    0x02 :: FrontBack
);

proto_varint_enum!(UpdateStructureBlockRotation,
    0x00 :: NoRotation,
    0x01 :: Clockwise90,
    0x02 :: Clockwise180,
    0x03 :: CounterClockwise90
);

proto_byte_flag!(UpdateStructureBlockFlags,
    0x01 :: is_ignore_entities set_ignore_entities,
    0x02 :: is_show_air set_show_air,
    0x04 :: is_show_bounding_box set_show_bounding_box
);

#[derive(Clone, PartialEq, Debug)]
pub struct RecipeSpec {
    pub recipe: Recipe,
    pub id: String,
}

proto_str_enum!(Recipe,
    "minecraft:crafting_shapeless" :: CraftingShapeless(RecipeCraftingShapelessSpec),
    "minecraft:crafting_shaped" :: CraftingShaped(RecipeCraftingShapedSpec),
    "minecraft:crafting_special_armordye" :: CraftingArmorDye,
    "minecraft:crafting_special_bookcloning" :: CraftingBookCloning,
    "minecraft:crafting_special_mapcloning" :: CraftingMapCloning,
    "minecraft:crafting_special_mapextending" :: CraftingMapExtending,
    "minecraft:crafting_special_firework_rocket" :: CraftingFireworkRocket,
    "minecraft:crafting_special_firework_star" :: CraftingFireworkStar,
    "minecraft:crafting_special_firework_star_fade" :: CraftingFireworkStarFade,
    "minecraft:crafting_special_repairitem" :: CraftingRepairItem,
    "minecraft:crafting_special_tippedarrow" :: CraftingTippedArrow,
    "minecraft:crafting_special_bannerduplicate" :: CraftingBannerDuplicate,
    "minecraft:crafting_special_banneraddpattern" :: CraftingBannerAddPattern,
    "minecraft:crafting_special_shielddecoration" :: CraftingShieldDecoration,
    "minecraft:crafting_special_shulkerboxcoloring" :: CraftingShulkerBoxColoring,
    "minecraft:crafting_special_suspiciousstew" :: CraftingSuspiciousStew,
    "minecraft:smelting" :: Smelting(RecipeSmeltingSpec),
    "minecraft:blasting" :: Blasting(RecipeSmeltingSpec),
    "minecraft:smoking" :: Smoking(RecipeSmeltingSpec),
    "minecraft:campfire_cooking" :: CampfireCooking(RecipeSmeltingSpec),
    "minecraft:stonecutting" :: StoneCutting(RecipeStonecuttingSpec),
    "minecraft:smithing" :: Smithing(RecipeSmithingSpec)
);

impl Serialize for RecipeSpec {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        let _type = self.recipe.id();
        to.serialize_other(&_type)?;
        to.serialize_other(&self.id)?;
        self.recipe.serialize_body(to)
    }
}

impl Deserialize for RecipeSpec {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        let Deserialized { value: _type, data } = String::mc_deserialize(data)?;
        let Deserialized {
            value: recipe_id,
            data,
        } = String::mc_deserialize(data)?;

        Ok(
            Recipe::deserialize_with_id(_type.as_str(), data)?.map(move |recipe| RecipeSpec {
                id: recipe_id,
                recipe,
            }),
        )
    }
}

#[cfg(all(test, feature = "std"))]
impl TestRandom for RecipeSpec {
    fn test_gen_random() -> Self {
        RecipeSpec {
            recipe: Recipe::test_gen_random(),
            id: String::test_gen_random(),
        }
    }
}

proto_struct!(RecipeIngredient {
    items: CountedArray<Slot, VarInt>
});

proto_struct!(RecipeCraftingShapelessSpec {
    group: String,
    ingredients: CountedArray<RecipeIngredient, VarInt>,
    result: Slot
});

#[derive(Debug, Clone, PartialEq)]
pub struct RecipeCraftingShapedSpec {
    pub width: VarInt,
    pub height: VarInt,
    pub group: String,
    pub ingredients: Vec<RecipeIngredient>,
    pub result: Slot,
}

impl Serialize for RecipeCraftingShapedSpec {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        to.serialize_other(&self.width)?;
        to.serialize_other(&self.height)?;
        to.serialize_other(&self.group)?;
        for elem in &self.ingredients {
            to.serialize_other(elem)?;
        }
        to.serialize_other(&self.result)?;
        Ok(())
    }
}

impl Deserialize for RecipeCraftingShapedSpec {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        let Deserialized { value: width, data } = <VarInt>::mc_deserialize(data)?;
        let Deserialized {
            value: height,
            data,
        } = <VarInt>::mc_deserialize(data)?;
        let Deserialized {
            value: group,
            mut data,
        } = <String>::mc_deserialize(data)?;

        let ingredients_count = width.0 as usize * height.0 as usize;
        let mut ingredients: Vec<RecipeIngredient> = Vec::with_capacity(ingredients_count);
        for _ in 0..ingredients_count {
            let Deserialized {
                value: elem,
                data: rest,
            } = RecipeIngredient::mc_deserialize(data)?;
            data = rest;
            ingredients.push(elem);
        }

        let Deserialized {
            value: result,
            data,
        } = Slot::mc_deserialize(data)?;

        Deserialized::ok(
            Self {
                width,
                height,
                group,
                ingredients,
                result,
            },
            data,
        )
    }
}

#[cfg(all(test, feature = "std"))]
impl TestRandom for RecipeCraftingShapedSpec {
    fn test_gen_random() -> Self {
        use rand::distributions::Distribution;
        let size_distr = rand::distributions::Uniform::new(1, 9);
        let mut rng = rand::thread_rng();

        let width: VarInt = size_distr.sample(&mut rng).into();
        let height: VarInt = size_distr.sample(&mut rng).into();
        let n_ingredients = (width.0 as usize) * (height.0 as usize);
        let mut ingredients = Vec::with_capacity(n_ingredients);
        for _ in 0..n_ingredients {
            ingredients.push(RecipeIngredient::test_gen_random());
        }

        RecipeCraftingShapedSpec {
            width,
            height,
            group: String::test_gen_random(),
            ingredients,
            result: Some(ItemStack::test_gen_random()),
        }
    }
}

proto_struct!(RecipeSmeltingSpec {
    group: String,
    ingredient: RecipeIngredient,
    result: Slot,
    experience: f32,
    cooking_time: VarInt
});

proto_struct!(RecipeStonecuttingSpec {
    group: String,
    ingredient: RecipeIngredient,
    result: Slot
});

proto_struct!(RecipeSmithingSpec {
    base: RecipeIngredient,
    addition: RecipeIngredient,
    result: Slot
});

proto_varint_enum!(RecipeUnlockAction,
    0x00 :: Init,
    0x01 :: Add,
    0x02 :: Remove
);

// #[derive(Clone, PartialEq, Debug)]
// pub struct ChunkData {
//     pub position: ChunkPosition<i32>,
//     pub primary_bit_mask: VarInt,
//     pub heightmaps: NamedNbtTag,
//     pub biomes: Option<CountedArray<VarInt, VarInt>>,
//     pub data: CountedArray<u8, VarInt>,
//     pub block_entities: Vec<NamedNbtTag>,
// }

// impl Serialize for ChunkData {
//     fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
//         to.serialize_other(&self.position)?;
//         let full_chunk = self.biomes.is_some();
//         to.serialize_other(&full_chunk)?;
//         to.serialize_other(&self.primary_bit_mask)?;
//         to.serialize_other(&self.heightmaps)?;

//         if full_chunk {
//             to.serialize_other(self.biomes.as_ref().unwrap())?;
//         }

//         to.serialize_other(&self.data)?;
//         let num_block_entities = VarInt(self.block_entities.len() as i32);
//         to.serialize_other(&num_block_entities)?;
//         for entity in &self.block_entities {
//             to.serialize_other(entity)?;
//         }

//         Ok(())
//     }
// }

// impl Deserialize for ChunkData {
//     fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
//         let Deserialized { value: position, data } = <ChunkPosition<i32>>::mc_deserialize(data)?;
//         let Deserialized { value: is_full_chunk, data } = bool::mc_deserialize(data)?;
//         let Deserialized { value: primary_bit_mask, data } = VarInt::mc_deserialize(data)?;
//         let Deserialized { value: heightmaps, mut data } = NamedNbtTag::mc_deserialize(data)?;
//         let biomes = if is_full_chunk {
//             let Deserialized { value: biomes, data: rest } = <CountedArray<VarInt, VarInt>>::mc_deserialize(data)?;
//             data = rest;
//             Some(biomes)
//         } else {
//             None
//         };
//         let Deserialized { value: chunk_data, data } = <CountedArray<u8, VarInt>>::mc_deserialize(data)?;
//         let Deserialized { value: n_block_entities_raw, mut data } = VarInt::mc_deserialize(data)?;
//         let n_block_entities = n_block_entities_raw.0 as usize;
//         let mut block_entities = Vec::with_capacity(n_block_entities);
//         for _ in 0..n_block_entities {
//             let Deserialized { value: entity, data: rest } = NamedNbtTag::mc_deserialize(data)?;
//             data = rest;
//             block_entities.push(entity);
//         }

//         Deserialized::ok(ChunkData {
//             position,
//             primary_bit_mask,
//             heightmaps,
//             biomes,
//             data: chunk_data,
//             block_entities,
//         }, data)
//     }
// // }

// #[cfg(all(test, feature = "std"))]
// impl TestRandom for ChunkData {
//     fn test_gen_random() -> Self {
//         ChunkData {
//             position: <ChunkPosition<i32>>::test_gen_random(),
//             primary_bit_mask: VarInt::test_gen_random(),
//             heightmaps: NamedNbtTag::test_gen_random(),
//             biomes: None,
//             data: <CountedArray<u8, VarInt>>::test_gen_random(),
//             block_entities: vec![],
//         }
//     }
// }

// pub const LIGHT_DATA_LENGTH: usize = 2048;
// pub const LIGHT_DATA_SECTIONS: usize = 18;

// #[derive(Clone, PartialEq)]
// pub struct LightingData {
//     pub data: Box<[Option<[u8; LIGHT_DATA_LENGTH]>; LIGHT_DATA_SECTIONS]>,
// }

// impl LightingData {
//     fn deserialize(update_mask: VarInt, mut data: &[u8]) -> DeserializeResult<Self> {
//         let mut out = Box::new([None; LIGHT_DATA_SECTIONS]);
//         for i in 0..LIGHT_DATA_SECTIONS {
//             // gotta read the var int
//             if update_mask.0 & (1 << i) != 0 {
//                 let Deserialized { value: length, data: rest } = VarInt::mc_deserialize(data)?;
//                 if (length.0 as usize) != LIGHT_DATA_LENGTH {
//                     return Err(DeserializeErr::CannotUnderstandValue(alloc::format!("bad data length in light update {}", length)));
//                 }

//                 data = rest;
//                 if data.len() < LIGHT_DATA_LENGTH {
//                     return Err(DeserializeErr::Eof);
//                 }

//                 let (section, rest) = data.split_at(LIGHT_DATA_LENGTH);
//                 let mut to_vec = [0u8; LIGHT_DATA_LENGTH];
//                 to_vec.copy_from_slice(section);
//                 out[i] = Some(to_vec);
//                 data = rest;
//             }
//         }

//         let result = Self {
//             data: out,
//         };

//         Deserialized::ok(result, data)
//     }

//     fn update_mask(&self) -> VarInt {
//         self.compute_has_mask(true)
//     }

//     fn reset_mask(&self) -> VarInt {
//         self.compute_has_mask(false)
//     }

//     fn compute_has_mask(&self, has: bool) -> VarInt {
//         let mut out: u32 = 0;
//         for i in 0..LIGHT_DATA_SECTIONS {
//             if self.data[i].is_some() == has {
//                 out |= 1 << i;
//             }
//         }

//         VarInt(out as i32)
//     }

//     fn serialize_data<S: Serializer>(&self, to: &mut S) -> SerializeResult {
//         for item in self.data.iter() {
//             if let Some(contents) = item {
//                 to.serialize_other(&VarInt(2048))?;
//                 to.serialize_bytes(&contents[..])?;
//             }
//         }

//         Ok(())
//     }
// }

// impl fmt::Debug for LightingData {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "LightingData(update={:018b}, reset={:018b}, size={}, bytes={})",
//             self.update_mask().0,
//             self.reset_mask().0,
//             self.data.iter().filter(move |v| v.is_some()).count(),
//             self.data.iter()
//                 .filter_map(move |v| v.
//                     map(move |arr| arr.len()))
//                 .sum::<usize>())
//     }
// }

// impl fmt::Display for LightingData {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         <dyn fmt::Debug>::fmt(self, f)
//     }
// }

// #[cfg(all(test, feature = "std"))]
// impl LightingData {
//     fn gen_random_mask() -> i32 {
//         let rand: u32 = rand::random();
//         (rand & ((1 << 19) - 1)) as i32
//     }
// }

// #[cfg(all(test, feature = "std"))]
// impl TestRandom for LightingData {
//     fn test_gen_random() -> Self {
//         let set_mask = Self::gen_random_mask();
//         let mut data = Box::new([None; LIGHT_DATA_SECTIONS]);
//         for i in 0..LIGHT_DATA_SECTIONS {
//             if (set_mask & (1 << i)) != 0 {
//                 let mut data_arr = [0u8; LIGHT_DATA_LENGTH];
//                 for k in 0..LIGHT_DATA_LENGTH {
//                     data_arr[k] = rand::random();
//                 }
//                 data[i] = Some(data_arr);
//             }
//         }

//         Self {
//             data,
//         }
//     }
// }

// #[derive(Clone, PartialEq, Debug)]
// pub struct LightingUpdateSpec {
//     pub skylight_data: LightingData,
//     pub blocklight_data: LightingData,
// }

// impl Serialize for LightingUpdateSpec {
//     fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
//         self.skylight_data.update_mask().mc_serialize(to)?;
//         self.blocklight_data.update_mask().mc_serialize(to)?;
//         self.skylight_data.reset_mask().mc_serialize(to)?;
//         self.blocklight_data.reset_mask().mc_serialize(to)?;
//         self.skylight_data.serialize_data(to)?;
//         self.blocklight_data.serialize_data(to)
//     }
// }

// impl Deserialize for LightingUpdateSpec {
//     fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
//         let Deserialized { value: skylight_update_mask, data } = VarInt::mc_deserialize(data)?;
//         let Deserialized { value: blocklight_update_mask, data } = VarInt::mc_deserialize(data)?;
//         let Deserialized { value: _, data } = VarInt::mc_deserialize(data)?;
//         let Deserialized { value: _, data } = VarInt::mc_deserialize(data)?;

//         let Deserialized { value: skylight_data, data } = LightingData::deserialize(skylight_update_mask, data)?;
//         let Deserialized { value: blocklight_data, data } = LightingData::deserialize(blocklight_update_mask, data)?;

//         Deserialized::ok(Self {
//             skylight_data,
//             blocklight_data,
//         }, data)
//     }
// }

// #[cfg(all(test, feature = "std"))]
// impl TestRandom for LightingUpdateSpec {
//     fn test_gen_random() -> Self {
//         Self {
//             skylight_data: LightingData::test_gen_random(),
//             blocklight_data: LightingData::test_gen_random(),
//         }
//     }
// }

#[derive(Clone, Debug, PartialEq, Default)]
pub struct EntityMetadata {
    pub fields: Vec<EntityMetadataField>,
}

impl Serialize for EntityMetadata {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        for field in &self.fields {
            to.serialize_byte(field.index)?;
            to.serialize_other(&field.data)?;
        }
        to.serialize_byte(0xFF)
    }
}

impl Deserialize for EntityMetadata {
    fn mc_deserialize(mut data: &[u8]) -> DeserializeResult<'_, Self> {
        let mut fields = Vec::new();
        loop {
            let Deserialized {
                value: index,
                data: rest,
            } = u8::mc_deserialize(data)?;
            data = rest;
            if index == 0xFF {
                break;
            }

            let Deserialized {
                value: field,
                data: rest,
            } = EntityMetadataFieldData::mc_deserialize(data)?;
            data = rest;
            fields.push(EntityMetadataField { index, data: field });
        }

        Deserialized::ok(Self { fields }, data)
    }
}

#[cfg(all(test, feature = "std"))]
impl TestRandom for EntityMetadata {
    fn test_gen_random() -> Self {
        let n_fields = rand::random::<usize>() % 10;
        let mut fields = Vec::with_capacity(n_fields);
        for i in 0..n_fields {
            fields.push(EntityMetadataField {
                index: i as u8,
                data: EntityMetadataFieldData::test_gen_random(),
            });
        }

        Self { fields }
    }
}

impl EntityMetadata {
    pub fn set(&mut self, index: u8, data: EntityMetadataFieldData) {
        for field in &mut self.fields {
            if field.index == index {
                field.data = data;
                return;
            }
        }

        self.fields.push(EntityMetadataField { index, data })
    }

    pub fn remove(&mut self, index: u8) -> bool {
        for i in 0..self.fields.len() {
            let field = self
                .fields
                .get(i)
                .expect("iterating through this vec, definitely have this index");
            if field.index == index {
                self.fields.remove(i);
                return true;
            }
        }

        false
    }

    pub fn get(&self, index: u8) -> Option<&EntityMetadataFieldData> {
        for field in &self.fields {
            if field.index == index {
                return Some(&field.data);
            }
        }

        None
    }

    pub fn get_mut(&mut self, index: u8) -> Option<&mut EntityMetadataFieldData> {
        for field in &mut self.fields {
            if field.index == index {
                return Some(&mut field.data);
            }
        }

        None
    }
}

impl<'a> core::iter::IntoIterator for &'a EntityMetadata {
    type Item = (u8, &'a EntityMetadataFieldData);
    type IntoIter = FieldIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        FieldIter { data: self, at: 0 }
    }
}

pub struct FieldIter<'a> {
    data: &'a EntityMetadata,
    at: usize,
}

impl<'a> core::iter::Iterator for FieldIter<'a> {
    type Item = (u8, &'a EntityMetadataFieldData);

    fn next(&mut self) -> Option<Self::Item> {
        self.data
            .fields
            .get(self.at)
            .map(move |field| (field.index, &field.data))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.data.fields.len();
        (len, Some(len))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EntityMetadataField {
    pub index: u8,
    pub data: EntityMetadataFieldData,
}

proto_varint_enum!(EntityMetadataFieldData,
    0x00 :: Byte(i8),
    0x01 :: VarInt(VarInt),
    0x02 :: Float(f32),
    0x03 :: String(String),
    0x04 :: Chat(Chat),
    0x05 :: OptChat(Option<Chat>),
    0x06 :: Slot(Slot),
    0x07 :: Boolean(bool),
    0x08 :: Rotation(Vec3<f32>),
    0x09 :: Position(IntPosition),
    0x0A :: OptPosition(Option<IntPosition>),
    0x0B :: Direction(EntityDirection),
    0x0C :: OptUUID(Option<UUID4>),
    0x0D :: OptBlockId(VarInt),
    0x0E :: NBT(NamedNbtTag),
    0x0F :: Particle(ParticleSpec),
    0x10 :: VillagerData(EntityVillagerData),
    0x11 :: OptVarInt(VarInt),
    0x12 :: Pose(EntityPose)
);

proto_varint_enum!(EntityDirection,
    0x00 :: Down,
    0x01 :: Up,
    0x02 :: North,
    0x03 :: South,
    0x04 :: West,
    0x05 :: East
);

proto_struct!(EntityVillagerData {
    villager_type: VillagerType,
    villager_profession: VillagerProfession,
    level: VarInt
});

proto_varint_enum!(VillagerType,
    0x00 :: Desert,
    0x01 :: Jungle,
    0x02 :: Plains,
    0x03 :: Savanna,
    0x04 :: Snow,
    0x05 :: Swamp,
    0x06 :: Taiga
);

proto_varint_enum!(VillagerProfession,
    0x00 :: None,
    0x01 :: Armorer,
    0x02 :: Butcher,
    0x03 :: Cartographer,
    0x04 :: Cleric,
    0x05 :: Farmer,
    0x06 :: Fisherman,
    0x07 :: Fletcher,
    0x08 :: LeatherWorker,
    0x09 :: Librarian,
    0x0A :: Mason,
    0x0B :: Nitwit,
    0x0C :: Shepherd,
    0x0D :: Toolsmith,
    0x0E :: Weaponsmith
);

proto_varint_enum!(EntityPose,
    0x00 :: Standing,
    0x01 :: FallFlying,
    0x02 :: Sleeping,
    0x03 :: Swimming,
    0x04 :: SpinAttack,
    0x05 :: Sneaking,
    0x06 :: LongJumping,
    0x07 :: Dying
);

proto_varint_enum!(ParticleSpec,
    0x00 :: AmbientEntityEffect,
    0x01 :: AngryVillager,
    0x02 :: Barrier,
    0x03 :: Light,
    0x04 :: Block(BlockParticleData),
    0x05 :: Bubble,
    0x06 :: Cloud,
    0x07 :: Crit,
    0x08 :: DamageIndicator,
    0x09 :: DragonBreath,
    0x0A :: DrippingLava,
    0x0B :: FallingLava,
    0x0C :: LandingLava,
    0x0D :: DrippingWater,
    0x0E :: FallingWater,
    0x0F :: Dust(DustParticleData),
    0x10 :: DustColorTransition(DustColorTransitionData),
    0x11 :: Effect,
    0x12 :: ElderGuardian,
    0x13 :: EnchantedHit,
    0x14 :: Enchant,
    0x15 :: EndRod,
    0x16 :: EntityEffect,
    0x17 :: ExplosionEmitter,
    0x18 :: Explosion,
    0x19 :: FallingDust(DustParticleData),
    0x1A :: Firework,
    0x1B :: Fishing,
    0x1C :: Flame,
    0x1D :: SoulFireFlame,
    0x1E :: Soul,
    0x1F :: Flash,
    0x20 :: HappyVillager,
    0x21 :: Composter,
    0x22 :: Heart,
    0x23 :: InstantEffect,
    0x24 :: Item(Slot),
    0x25 :: Vibration(VibrationData),
    0x26 :: ItemSlime,
    0x27 :: ItemSnowball,
    0x28 :: LargeSmoke,
    0x29 :: Lava,
    0x2A :: Mycelium,
    0x2B :: Note,
    0x2C :: Poof,
    0x2D :: Portal,
    0x2E :: Rain,
    0x2F :: Smoke,
    0x30 :: Sneeze,
    0x31 :: Spit,
    0x32 :: SquidInk,
    0x33 :: SweepAttack,
    0x34 :: TotemOfUndying,
    0x35 :: Underwater,
    0x36 :: Splash,
    0x37 :: Witch,
    0x38 :: BubblePop,
    0x39 :: CurrentDown,
    0x3A :: BubbleColumnUp,
    0x3B :: Nautilus,
    0x3C :: Dolphin,
    0x3D :: CampfireCosySmoke,
    0x3E :: CampfireSignalSmoke,
    0x3F :: DrippingHoney,
    0x40 :: FallingHoney,
    0x41 :: LandingHoney,
    0x42 :: FallingNectar,
    0x43 :: FallingSporeBlossom,
    0x44 :: Ash,
    0x45 :: CrimsonSpore,
    0x46 :: WarpedSpore,
    0x47 :: SporeBlossomAir,
    0x48 :: DrippingObsidianTear,
    0x49 :: FallingObsidianTear,
    0x4A :: LandingObsidianTear,
    0x4B :: ReversePortal,
    0x4C :: WhiteAsh,
    0x4D :: SmallFlame,
    0x4E :: Snowflake,
    0x4F :: DrippingDripstoneLava,
    0x50 :: FallingDripstoneLava,
    0x51 :: DrippingDripstoneWater,
    0x52 :: FallingDripstoneWater,
    0x53 :: GlowSquidInk,
    0x54 :: Glow,
    0x55 :: WaxOn,
    0x56 :: WaxOff,
    0x57 :: ElectricSpark,
    0x58 :: Scrape
);

proto_struct!(BlockParticleData {
    block_state: VarInt
});

proto_struct!(DustParticleData {
    red: f32,
    green: f32,
    blue: f32,
    scale: f32
});

proto_struct!(DustColorTransitionData {
    from_red: f32,
    from_green: f32,
    from_blue: f32,
    scale: f32,
    to_red: f32,
    to_green: f32,
    to_blue: f32
});

proto_struct!(VibrationData {
    origin_x: f64,
    origin_y: f64,
    origin_z: f64,
    dest_x: f64,
    dest_y: f64,
    dest_z: f64
});

#[cfg(all(test, feature = "std"))]
pub mod tests {
    use super::*;
    use crate::packet_test_cases;

    packet_test_cases!(
        RawPacket759,
        Packet759,
        Handshake,
        HandshakeSpec,
        test_handshake,
        bench_write_handshake,
        bench_read_handshake
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        StatusRequest,
        StatusRequestSpec,
        test_status_request,
        bench_write_status_request,
        bench_read_status_request
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        StatusPing,
        StatusPingSpec,
        test_status_ping,
        bench_write_status_ping,
        bench_read_status_ping
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        StatusResponse,
        StatusResponseSpec,
        test_status_response,
        bench_write_status_response,
        bench_read_status_response
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        StatusPong,
        StatusPongSpec,
        test_status_pong,
        bench_write_status_pong,
        bench_read_status_pong
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        LoginDisconnect,
        LoginDisconnectSpec,
        test_login_disconnect,
        bench_write_login_disconnect,
        bench_read_login_disconnect
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        LoginEncryptionRequest,
        LoginEncryptionRequestSpec,
        test_login_encryption_request,
        bench_write_login_encryption_request,
        bench_read_login_encryption_request
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        LoginSuccess,
        LoginSuccessSpec,
        test_login_success,
        bench_write_login_success,
        bench_read_login_success
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        LoginSetCompression,
        LoginSetCompressionSpec,
        test_login_set_compression,
        bench_write_login_set_compression,
        bench_read_login_set_compression
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        LoginPluginRequest,
        LoginPluginRequestSpec,
        test_login_plugin_request,
        bench_write_login_plugin_request,
        bench_read_login_plugin_request
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        LoginStart,
        LoginStartSpec,
        test_login_start,
        bench_write_login_start,
        bench_read_login_start
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        LoginEncryptionResponse,
        LoginEncryptionResponseSpec,
        test_login_encryption_response,
        bench_write_login_encryption_response,
        bench_read_login_encryption_response
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        LoginPluginResponse,
        LoginPluginResponseSpec,
        test_login_plugin_response,
        bench_write_login_plugin_response,
        bench_read_login_plugin_response
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlaySpawnEntity,
        PlaySpawnEntitySpec,
        test_play_spawn_entity,
        bench_write_play_spawn_entity,
        bench_read_play_spawn_entity
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlaySpawnExperienceOrb,
        PlaySpawnExperienceOrbSpec,
        test_play_spawn_experience_orb,
        bench_write_play_spawn_experience_orb,
        bench_read_play_spawn_experience_orb
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlaySpawnLivingEntity,
        PlaySpawnLivingEntitySpec,
        test_play_spawn_living_entity,
        bench_write_play_spawn_living_entity,
        bench_read_play_spawn_living_entity
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlaySpawnPainting,
        PlaySpawnPaintingSpec,
        test_play_spawn_painting,
        bench_write_play_spawn_painting,
        bench_read_play_spawn_painting
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlaySpawnPlayer,
        PlaySpawnPlayerSpec,
        test_play_spawn_player,
        bench_write_play_spawn_player,
        bench_read_play_spawn_player
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayEntityAnimation,
        PlayEntityAnimationSpec,
        test_play_entity_animation,
        bench_write_play_entity_animation,
        bench_read_play_entity_animation
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayStatistics,
        PlayStatisticsSpec,
        test_play_statistics,
        bench_write_play_statistics,
        bench_read_play_statistics
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayAcknowledgePlayerDigging,
        PlayAcknowledgePlayerDiggingSpec,
        test_play_acknowledge_player_digging,
        bench_write_play_acknowledge_player_digging,
        bench_read_play_acknowledge_player_digging
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayBlockBreakAnimation,
        PlayBlockBreakAnimationSpec,
        test_play_block_break_animation,
        bench_write_play_block_break_animation,
        bench_read_play_block_break_animation
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayBlockEntityData,
        PlayBlockEntityDataSpec,
        test_play_block_entity_data,
        bench_write_play_block_entity_data,
        bench_read_play_block_entity_data
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayBlockAction,
        PlayBlockActionSpec,
        test_play_block_action,
        bench_write_play_block_action,
        bench_read_play_block_action
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayBlockChange,
        PlayBlockChangeSpec,
        test_play_block_change,
        bench_write_play_block_change,
        bench_read_play_block_change
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayBossBar,
        PlayBossBarSpec,
        test_play_boss_bar,
        bench_write_play_boss_bar,
        bench_read_play_boss_bar
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayServerDifficulty,
        PlayServerDifficultySpec,
        test_play_server_difficulty,
        bench_write_play_server_difficulty,
        bench_read_play_server_difficulty
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayServerChatMessage,
        PlayServerChatMessageSpec,
        test_play_server_chat_message,
        bench_write_play_server_chat_message,
        bench_read_play_server_chat_message
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayTabComplete,
        PlayTabCompleteSpec,
        test_play_tab_complete,
        bench_write_play_tab_complete,
        bench_read_play_tab_complete
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayDeclareCommands,
        PlayDeclareCommandsSpec,
        test_play_declare_commands,
        bench_write_play_declare_commands,
        bench_read_play_declare_commands
    );

    // packet_test_cases!(
    //     RawPacket759,
    //     Packet759,
    //     PlayServerWindowConfirmation,
    //     PlayServerWindowConfirmationSpec,
    //     test_play_server_window_confirmation,
    //     bench_write_play_server_window_confirmation,
    //     bench_read_play_server_window_confirmation
    // );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayServerCloseWindow,
        PlayServerCloseWindowSpec,
        test_play_server_close_window,
        bench_write_play_server_close_window,
        bench_read_play_server_close_window
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayWindowItems,
        PlayWindowItemsSpec,
        test_play_window_items,
        bench_write_play_window_items,
        bench_read_play_window_items
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayWindowProperty,
        PlayWindowPropertySpec,
        test_play_window_property,
        bench_write_play_window_property,
        bench_read_play_window_property
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlaySetSlot,
        PlaySetSlotSpec,
        test_play_set_slot,
        bench_write_play_set_slot,
        bench_read_play_set_slot
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlaySetCooldown,
        PlaySetCooldownSpec,
        test_play_set_cooldown,
        bench_write_play_set_cooldown,
        bench_read_play_set_cooldown
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayServerPluginMessage,
        PlayServerPluginMessageSpec,
        test_play_server_plugin_message,
        bench_write_play_server_plugin_message,
        bench_read_play_server_plugin_message
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayNamedSoundEffect,
        PlayNamedSoundEffectSpec,
        test_play_named_sound_effect,
        bench_write_play_named_sound_effect,
        bench_read_play_named_sound_effect
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayDisconnect,
        PlayDisconnectSpec,
        test_play_disconnect,
        bench_write_play_disconnect,
        bench_read_play_disconnect
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayEntityStatus,
        PlayEntityStatusSpec,
        test_play_entity_status,
        bench_write_play_entity_status,
        bench_read_play_entity_status
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayExplosion,
        PlayExplosionSpec,
        test_play_explosion,
        bench_write_play_explosion,
        bench_read_play_explosion
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayUnloadChunk,
        PlayUnloadChunkSpec,
        test_play_unload_chunk,
        bench_write_play_unload_chunk,
        bench_read_play_unload_chunk
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayChangeGameState,
        PlayChangeGameStateSpec,
        test_play_change_game_state,
        bench_write_play_change_game_state,
        bench_read_play_change_game_state
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayOpenHorseWindow,
        PlayOpenHorseWindowSpec,
        test_play_open_horse_window,
        bench_write_play_open_horse_window,
        bench_read_play_open_horse_window
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayServerKeepAlive,
        PlayServerKeepAliveSpec,
        test_play_server_keep_alive,
        bench_write_play_server_keep_alive,
        bench_read_play_server_keep_alive
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayChunkData,
        PlayChunkDataWrapper,
        test_play_chunk_data,
        bench_write_play_chunk_data,
        bench_read_play_chunk_data
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayEffect,
        PlayEffectSpec,
        test_play_effect,
        bench_write_play_effect,
        bench_read_play_effect
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayParticle,
        PlayParticleSpec,
        test_play_particle,
        bench_write_play_particle,
        bench_read_play_particle
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayUpdateLight,
        PlayUpdateLightSpec,
        test_play_update_light,
        bench_write_play_update_light,
        bench_read_play_update_light
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayJoinGame,
        PlayJoinGameSpec,
        test_play_join_game,
        bench_write_play_join_game,
        bench_read_play_join_game
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayMapData,
        PlayMapDataSpec,
        test_play_map_data,
        bench_write_play_map_data,
        bench_read_play_map_data
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayTradeList,
        PlayTradeListSpec,
        test_play_trade_list,
        bench_write_play_trade_list,
        bench_read_play_trade_list
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayEntityPosition,
        PlayEntityPositionSpec,
        test_play_entity_position,
        bench_write_play_entity_position,
        bench_read_play_entity_position
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayEntityPositionAndRotation,
        PlayEntityPositionAndRotationSpec,
        test_play_entity_position_and_rotation,
        bench_write_play_entity_position_and_rotation,
        bench_read_play_entity_position_and_rotation
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayEntityRotation,
        PlayEntityRotationSpec,
        test_play_entity_rotation,
        bench_write_play_entity_rotation,
        bench_read_play_entity_rotation
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayEntityMovement,
        PlayEntityMovementSpec,
        test_play_entity_movement,
        bench_write_play_entity_movement,
        bench_read_play_entity_movement
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayServerVehicleMove,
        PlayEntityVehicleMoveSpec,
        test_play_server_vehicle_move,
        bench_write_play_server_vehicle_move,
        bench_read_play_server_vehicle_move
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayOpenBook,
        PlayOpenBookSpec,
        test_play_open_book,
        bench_write_play_open_book,
        bench_read_play_open_book
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayOpenWindow,
        PlayOpenWindowSpec,
        test_play_open_window,
        bench_write_play_open_window,
        bench_read_play_open_window
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayOpenSignEditor,
        PlayOpenSignEditorSpec,
        test_play_open_sign_editor,
        bench_write_play_open_sign_editor,
        bench_read_play_open_sign_editor
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayCraftRecipeResponse,
        PlayCraftRecipeResponseSpec,
        test_play_craft_recipe_response,
        bench_write_play_craft_recipe_response,
        bench_read_play_craft_recipe_response
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayServerPlayerAbilities,
        PlayServerPlayerAbilitiesSpec,
        test_play_server_player_abilities,
        bench_write_play_server_player_abilities,
        bench_read_play_server_player_abilities
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayCombatEvent,
        PlayCombatEventSpec,
        test_play_combat_event,
        bench_write_play_combat_event,
        bench_read_play_combat_event
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayPlayerInfo,
        PlayPlayerInfoSpec,
        test_play_player_info,
        bench_write_play_player_info,
        bench_read_play_player_info
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayFacePlayer,
        PlayFacePlayerSpec,
        test_play_face_player,
        bench_write_play_face_player,
        bench_read_play_face_player
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayServerPlayerPositionAndLook,
        PlayServerPlayerPositionAndLookSpec,
        test_play_server_player_position_and_look,
        bench_write_play_server_player_position_and_look,
        bench_read_play_server_player_position_and_look
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayUnlockRecipes,
        PlayUnlockRecipesSpec,
        test_play_unlock_recipes,
        bench_write_play_unlock_recipes,
        bench_read_play_unlock_recipes
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayDestroyEntities,
        PlayDestroyEntitiesSpec,
        test_play_destroy_entities,
        bench_write_play_destroy_entities,
        bench_read_play_destroy_entities
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayRemoveEntityEffect,
        PlayRemoveEntityEffectSpec,
        test_play_remove_entity_effect,
        bench_write_play_remove_entity_effect,
        bench_read_play_remove_entity_effect
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayResourcePackSend,
        PlayResourcePackSendSpec,
        test_play_resource_pack_send,
        bench_write_play_resource_pack_send,
        bench_read_play_resource_pack_send
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayRespawn,
        PlayRespawnSpec,
        test_play_respawn,
        bench_write_play_respawn,
        bench_read_play_respawn
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayEntityHeadLook,
        PlayEntityHeadLookSpec,
        test_play_entity_head_look,
        bench_write_play_entity_head_look,
        bench_read_play_entity_head_look
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayMultiBlockChange,
        PlayMultiBlockChangeSpec,
        test_play_multi_block_change,
        bench_write_play_multi_block_change,
        bench_read_play_multi_block_change
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlaySelectAdvancementTab,
        PlaySelectAdvancementTabSpec,
        test_play_select_advancement_tab,
        bench_write_play_select_advancement_tab,
        bench_read_play_select_advancement_tab
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayWorldBorder,
        PlayWorldBorderSpec,
        test_play_world_border,
        bench_write_play_world_border,
        bench_read_play_world_border
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayCamera,
        PlayCameraSpec,
        test_play_camera,
        bench_write_play_camera,
        bench_read_play_camera
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayServerHeldItemChange,
        PlayServerHeldItemChangeSpec,
        test_play_server_held_item_change,
        bench_write_play_server_held_item_change,
        bench_read_play_server_held_item_change
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayUpdateViewPosition,
        PlayUpdateViewPositionSpec,
        test_play_update_view_position,
        bench_write_play_update_view_position,
        bench_read_play_update_view_position
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayUpdateViewDistance,
        PlayUpdateViewDistanceSpec,
        test_play_update_view_distance,
        bench_write_play_update_view_distance,
        bench_read_play_update_view_distance
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlaySpawnPosition,
        PlaySpawnPositionSpec,
        test_play_spawn_position,
        bench_write_play_spawn_position,
        bench_read_play_spawn_position
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayDisplayScoreboard,
        PlayDisplayScoreboardSpec,
        test_play_display_scoreboard,
        bench_write_play_display_scoreboard,
        bench_read_play_display_scoreboard
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayEntityMetadata,
        PlayEntityMetadataSpec,
        test_play_entity_metadata,
        bench_write_play_entity_metadata,
        bench_read_play_entity_metadata
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayAttachEntity,
        PlayAttachEntitySpec,
        test_play_attach_entity,
        bench_write_play_attach_entity,
        bench_read_play_attach_entity
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayEntityVelocity,
        PlayEntityVelocitySpec,
        test_play_entity_velocity,
        bench_write_play_entity_velocity,
        bench_read_play_entity_velocity
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayEntityEquipment,
        PlayEntityEquiptmentSpec,
        test_play_entity_equipment,
        bench_write_play_entity_equipment,
        bench_read_play_entity_equipment
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlaySetExperience,
        PlaySetExperienceSpec,
        test_play_set_experience,
        bench_write_play_set_experience,
        bench_read_play_set_experience
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayUpdatehealth,
        PlayUpdateHealthSpec,
        test_play_updatehealth,
        bench_write_play_updatehealth,
        bench_read_play_updatehealth
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayScoreboardObjective,
        PlayScoreboardObjectiveSpec,
        test_play_scoreboard_objective,
        bench_write_play_scoreboard_objective,
        bench_read_play_scoreboard_objective
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlaySetPassengers,
        PlaySetPassengersSpec,
        test_play_set_passengers,
        bench_write_play_set_passengers,
        bench_read_play_set_passengers
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayTeams,
        PlayTeamsSpec,
        test_play_teams,
        bench_write_play_teams,
        bench_read_play_teams
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayUpdateScore,
        PlayUpdateScoreSpec,
        test_play_update_score,
        bench_write_play_update_score,
        bench_read_play_update_score
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayTimeUpdate,
        PlayTimeUpdateSpec,
        test_play_time_update,
        bench_write_play_time_update,
        bench_read_play_time_update
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayTitle,
        PlayTitleSpec,
        test_play_title,
        bench_write_play_title,
        bench_read_play_title
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayEntitySoundEffect,
        PlayEntitySoundEffectSpec,
        test_play_entity_sound_effect,
        bench_write_play_entity_sound_effect,
        bench_read_play_entity_sound_effect
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlaySoundEffect,
        PlaySoundEffectSpec,
        test_play_sound_effect,
        bench_write_play_sound_effect,
        bench_read_play_sound_effect
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayStopSound,
        PlayStopSoundSpec,
        test_play_stop_sound,
        bench_write_play_stop_sound,
        bench_read_play_stop_sound
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayerPlayerListHeaderAndFooter,
        PlayPlayerListHeaderAndFooterSpec,
        test_player_player_list_header_and_footer,
        bench_write_player_player_list_header_and_footer,
        bench_read_player_player_list_header_and_footer
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayNbtQueryResponse,
        PlayNbtQueryResponseSpec,
        test_play_nbt_query_response,
        bench_write_play_nbt_query_response,
        bench_read_play_nbt_query_response
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayCollectItem,
        PlayCollectItemSpec,
        test_play_collect_item,
        bench_write_play_collect_item,
        bench_read_play_collect_item
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayEntityTeleport,
        PlayEntityTeleportSpec,
        test_play_entity_teleport,
        bench_write_play_entity_teleport,
        bench_read_play_entity_teleport
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayAdvancements,
        PlayAdvancementsSpec,
        test_play_advancements,
        bench_write_play_advancements,
        bench_read_play_advancements
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayEntityProperties,
        PlayEntityPropertiesSpec,
        test_play_entity_properties,
        bench_write_play_entity_properties,
        bench_read_play_entity_properties
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayEntityEffect,
        PlayEntityEffectSpec,
        test_play_entity_effect,
        bench_write_play_entity_effect,
        bench_read_play_entity_effect
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayDeclareRecipes,
        PlayDeclareRecipesSpec,
        test_play_declare_recipes,
        bench_write_play_declare_recipes,
        bench_read_play_declare_recipes
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayTags,
        PlayTagsSpec,
        test_play_tags,
        bench_write_play_tags,
        bench_read_play_tags
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayTeleportConfirm,
        PlayTeleportConfirmSpec,
        test_play_teleport_confirm,
        bench_write_play_teleport_confirm,
        bench_read_play_teleport_confirm
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayQueryBlockNbt,
        PlayQueryBlockNbtSpec,
        test_play_query_block_nbt,
        bench_write_play_query_block_nbt,
        bench_read_play_query_block_nbt
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayQueryEntityNbt,
        PlayQueryEntityNbtSpec,
        test_play_query_entity_nbt,
        bench_write_play_query_entity_nbt,
        bench_read_play_query_entity_nbt
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlaySetDifficulty,
        PlaySetDifficultySpec,
        test_play_set_difficulty,
        bench_write_play_set_difficulty,
        bench_read_play_set_difficulty
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayClientChatMessage,
        PlayClientChatMessageSpec,
        test_play_client_chat_message,
        bench_write_play_client_chat_message,
        bench_read_play_client_chat_message
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayClientStatus,
        PlayClientStatusSpec,
        test_play_client_status,
        bench_write_play_client_status,
        bench_read_play_client_status
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayClientSettings,
        PlayClientSettingsSpec,
        test_play_client_settings,
        bench_write_play_client_settings,
        bench_read_play_client_settings
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayClientTabComplete,
        PlayClientTabCompleteSpec,
        test_play_client_tab_complete,
        bench_write_play_client_tab_complete,
        bench_read_play_client_tab_complete
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayClientWindowConfirmation,
        PlayClientWindowConfirmationSpec,
        test_play_client_window_confirmation,
        bench_write_play_client_window_confirmation,
        bench_read_play_client_window_confirmation
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayClickWindowButton,
        PlayClickWindowButtonSpec,
        test_play_click_window_button,
        bench_write_play_click_window_button,
        bench_read_play_click_window_button
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayClickWindow,
        PlayClickWindowSpec,
        test_play_click_window,
        bench_write_play_click_window,
        bench_read_play_click_window
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayClientCloseWindow,
        PlayClientCloseWindowSpec,
        test_play_client_close_window,
        bench_write_play_client_close_window,
        bench_read_play_client_close_window
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayClientPluginMessage,
        PlayClientPluginMessageSpec,
        test_play_client_plugin_message,
        bench_write_play_client_plugin_message,
        bench_read_play_client_plugin_message
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayEditBook,
        PlayEditBookSpec,
        test_play_edit_book,
        bench_write_play_edit_book,
        bench_read_play_edit_book
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayInteractEntity,
        PlayInteractEntitySpec,
        test_play_interact_entity,
        bench_write_play_interact_entity,
        bench_read_play_interact_entity
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayGenerateStructure,
        PlayGenerateStructureSpec,
        test_play_generate_structure,
        bench_write_play_generate_structure,
        bench_read_play_generate_structure
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayClientKeepAlive,
        PlayClientKeepAliveSpec,
        test_play_client_keep_alive,
        bench_write_play_client_keep_alive,
        bench_read_play_client_keep_alive
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayLockDifficulty,
        PlayLockDifficultySpec,
        test_play_lock_difficulty,
        bench_write_play_lock_difficulty,
        bench_read_play_lock_difficulty
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayPlayerPosition,
        PlayPlayerPositionSpec,
        test_play_player_position,
        bench_write_play_player_position,
        bench_read_play_player_position
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayClientPlayerPositionAndRotation,
        PlayClientPlayerPositionAndRotationSpec,
        test_play_client_player_position_and_rotation,
        bench_write_play_client_player_position_and_rotation,
        bench_read_play_client_player_position_and_rotation
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayPlayerRotation,
        PlayPlayerRotationSpec,
        test_play_player_rotation,
        bench_write_play_player_rotation,
        bench_read_play_player_rotation
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayPlayerMovement,
        PlayPlayerMovementSpec,
        test_play_player_movement,
        bench_write_play_player_movement,
        bench_read_play_player_movement
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayClientVehicleMove,
        PlayClientVehicleMoveSpec,
        test_play_client_vehicle_move,
        bench_write_play_client_vehicle_move,
        bench_read_play_client_vehicle_move
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlaySteerBoat,
        PlaySteerBoatSpec,
        test_play_steer_boat,
        bench_write_play_steer_boat,
        bench_read_play_steer_boat
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayPickItem,
        PlayPickItemSpec,
        test_play_pick_item,
        bench_write_play_pick_item,
        bench_read_play_pick_item
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayCraftRecipeRequest,
        PlayCraftRecipeRequestSpec,
        test_play_craft_recipe_request,
        bench_write_play_craft_recipe_request,
        bench_read_play_craft_recipe_request
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayClientPlayerAbilities,
        PlayClientPlayerAbilitiesSpec,
        test_play_client_player_abilities,
        bench_write_play_client_player_abilities,
        bench_read_play_client_player_abilities
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayPlayerDigging,
        PlayPlayerDiggingSpec,
        test_play_player_digging,
        bench_write_play_player_digging,
        bench_read_play_player_digging
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayEntityAction,
        PlayEntityActionSpec,
        test_play_entity_action,
        bench_write_play_entity_action,
        bench_read_play_entity_action
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlaySteerVehicle,
        PlaySteerVehicleSpec,
        test_play_steer_vehicle,
        bench_write_play_steer_vehicle,
        bench_read_play_steer_vehicle
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlaySetDisplayedRecipe,
        PlaySetDisplayedRecipeSpec,
        test_play_set_displayed_recipe,
        bench_write_play_set_displayed_recipe,
        bench_read_play_set_displayed_recipe
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlaySetRecipeBookState,
        PlaySetRecipeBookStateSpec,
        test_play_set_recipe_book_state,
        bench_write_play_set_recipe_book_state,
        bench_read_play_set_recipe_book_state
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayNameItem,
        PlayNameItemSpec,
        test_play_name_item,
        bench_write_play_name_item,
        bench_read_play_name_item
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayResourcePackStatus,
        PlayResourcePackStatusSpec,
        test_play_resource_pack_status,
        bench_write_play_resource_pack_status,
        bench_read_play_resource_pack_status
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayAdvancementTab,
        PlayAdvancementTabSpec,
        test_play_advancement_tab,
        bench_write_play_advancement_tab,
        bench_read_play_advancement_tab
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlaySelectTrade,
        PlaySelectTradeSpec,
        test_play_select_trade,
        bench_write_play_select_trade,
        bench_read_play_select_trade
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlaySetBeaconEffect,
        PlaySetBeaconEffectSpec,
        test_play_set_beacon_effect,
        bench_write_play_set_beacon_effect,
        bench_read_play_set_beacon_effect
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayClientHeldItemChange,
        PlayClientHeldItemChangeSpec,
        test_play_client_held_item_change,
        bench_write_play_client_held_item_change,
        bench_read_play_client_held_item_change
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayUpdateCommandBlock,
        PlayUpdateCommandBlockSpec,
        test_play_update_command_block,
        bench_write_play_update_command_block,
        bench_read_play_update_command_block
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayUpdateCommandBlockMinecart,
        PlayUpdateCommandBlockMinecartSpec,
        test_play_update_command_block_minecart,
        bench_write_play_update_command_block_minecart,
        bench_read_play_update_command_block_minecart
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayUpdateJigsawBlock,
        PlayUpdateJigsawBlockSpec,
        test_play_update_jigsaw_block,
        bench_write_play_update_jigsaw_block,
        bench_read_play_update_jigsaw_block
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayCreativeInventoryAction,
        PlayCreativeInventoryActionSpec,
        test_play_creative_inventory_action,
        bench_write_play_creative_inventory_action,
        bench_read_play_creative_inventory_action
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayUpdateStructureBlock,
        PlayUpdateStructureBlockSpec,
        test_play_update_structure_block,
        bench_write_play_update_structure_block,
        bench_read_play_update_structure_block
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayUpdateSign,
        PlayUpdateSignSpec,
        test_play_update_sign,
        bench_write_play_update_sign,
        bench_read_play_update_sign
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayClientAnimation,
        PlayClientAnimationSpec,
        test_play_client_animation,
        bench_write_play_client_animation,
        bench_read_play_client_animation
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlaySpectate,
        PlaySpectateSpec,
        test_play_spectate,
        bench_write_play_spectate,
        bench_read_play_spectate
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayBlockPlacement,
        PlayBlockPlacementSpec,
        test_play_block_placement,
        bench_write_play_block_placement,
        bench_read_play_block_placement
    );

    packet_test_cases!(
        RawPacket759,
        Packet759,
        PlayUseItem,
        PlayUseItemSpec,
        test_play_use_item,
        bench_write_play_use_item,
        bench_read_play_use_item
    );

    // trust me, this is some cutting edge shit
    // I'm definitely not generating code using a unit test
    #[test]
    fn test_generate_test_cases() {
        Packet759::describe().packets.iter().map(move |packet| {
            let snake_case = to_snake_case(packet.name.clone());
            alloc::format!("packet_test_cases!(RawPacket759, Packet759, {}, {},\n        test_{}, bench_write_{}, bench_read_{});\n",
                           packet.name, packet.body_struct, snake_case, snake_case, snake_case).to_owned()
        }).for_each(move |line| {
            println!("{}", line)
        })
    }

    fn to_snake_case(camel: String) -> String {
        let mut parts = Vec::new();
        let mut buf = String::new();
        for c in camel.chars() {
            if !buf.is_empty() && char::is_uppercase(c) {
                parts.push(buf);
                buf = String::new();
            }

            buf.push(c.to_ascii_lowercase());
        }

        if !buf.is_empty() {
            parts.push(buf);
        }

        parts.join("_")
    }
}
