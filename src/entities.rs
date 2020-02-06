
use serde::Deserialize;

use serde_json::value::Value;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Root {
    #[serde(rename = "itemTemplates")]
    pub item_templates: Vec<Template>,
    #[serde(rename = "timestampMs")]
    pub timestamp_ms: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Template {
    #[serde(rename = "templateId")]
    pub template_id: String,
    #[serde(rename = "avatarCustomization")]
    pub avatar_customization: Option<AvatarCustomization>,
    #[serde(rename = "backgroundModeSettings")]
    pub background_mode_settings: Option<BackgroundModeSettings>,
    #[serde(rename = "badgeSettings")]
    pub badge_settings: Option<BadgeSettings>,
    #[serde(rename = "battleHubBadgeSettings")]
    pub battle_hub_badge_settings : Option<BattleHubBadgeSettings>,
    #[serde(rename = "battleHubOrderSettings")]
    pub battle_hub_order_settings: Option<BattleHubOrderSettings>,
    #[serde(rename = "battleSettings")]
    pub battle_settings: Option<BattleSettings>,
    #[serde(rename = "belugaPokemonWhitelist")]
    pub beluga_pokemon_whitelist: Option<BelugaPokemonWhitelist>,
    #[serde(rename = "buddyActivitySettings")]
    pub buddy_activity_settings: Option<BuddyActivitySettings>,
    #[serde(rename = "buddyActivityCategorySettings")]
    pub buddy_activity_category_settings: Option<BuddyActivityCategorySettings>,
    #[serde(rename = "buddyEmotionLevelSettings")]
    pub buddy_emotion_level_settings: Option<BuddyEmotionLevelSettings>,
    #[serde(rename = "buddyEncounterCameoSettings")]
    pub buddy_encounter_cameo_settings: Option<BuddyEncounterCameoSettings>,
    #[serde(rename = "buddyHungerSettings")]
    pub buddy_hunger_settings: Option<BuddyHungerSettings>,
    #[serde(rename = "buddyInteractionSettings")]
    pub buddy_interaction_settings: Option<BuddyInteractionSettings>,
    #[serde(rename = "buddyLevelSettings")]
    pub buddy_level_settings: Option<BuddyLevelSettings>,
    #[serde(rename = "buddySwapSettings")]
    pub buddy_swap_settings: Option<BuddySwapSettings>,
    #[serde(rename = "buddyWalkSettings")]
    pub buddy_walk_settings: Option<BuddyWalkSettings>,
    #[serde(rename = "invasionNpcDisplaySettings")]
    pub invasion_npc_display_settings: Option<InvasionNpcDisplaySettings>,
    #[serde(rename = "combatCompetitiveSeasonSettings")]
    pub combat_competitive_season_settings: Option<CombatCompetitiveSeasonSettings>,
    #[serde(rename = "combatLeague")]
    pub combat_league: Option<CombatLeague>,
    #[serde(rename = "combatLeagueSettings")]
    pub combat_league_settings: Option<CombatLeagueSettings>,
    #[serde(rename = "combatType")]
    pub combat_type: Option<CombatType>,
    #[serde(rename = "combatRankingProtoSettings")]
    pub combat_ranking_proto_settings: Option<CombatRankingProtoSettings>,
    #[serde(rename = "combatSettings")]
    pub combat_settings: Option<CombatSettings>,
    #[serde(rename = "combatStatStageSettings")]
    pub combat_stat_stage_settings: Option<CombatStatStageSettings>,
    #[serde(rename = "combatMove")]
    pub combat_move: Option<CombatMove>,
    #[serde(rename = "encounterSettings")]
    pub encounter_settings: Option<EncounterSettings>,
    #[serde(rename = "exRaidSettings")]
    pub ex_raid_settings: Option<ExRaidSettings>,
    #[serde(rename = "formSettings")]
    pub form_settings: Option<FormSettings>,
    #[serde(rename = "friendshipMilestoneSettings")]
    pub friendship_milestone_settings: Option<FriendshipMilestoneSettings>,
    #[serde(rename = "gymBadgeSettings")]
    pub gym_badge_settings: Option<GymBadgeSettings>,
    #[serde(rename = "gymLevel")]
    pub gym_level: Option<GymLevel>,
    #[serde(rename = "iapCategoryDisplay")]
    pub iap_category_display: Option<IapCategoryDisplay>,
    #[serde(rename = "iapSettings")]
    pub iap_settings: Option<IapSettings>,
    #[serde(rename = "pokestopInvasionAvailabilitySettings")]
    pub pokestop_invasion_availability_settings: Option<PokestopInvasionAvailabilitySettings>,
    #[serde(rename = "itemSettings")]
    pub item_settings: Option<ItemSettings>,
    #[serde(rename = "loadingScreenSettings")]
    pub loading_screen_settings: Option<LoadingScreenSettings>,
    #[serde(rename = "limitedPurchaseSkuSettings")]
    pub limited_purchase_sku_settings: Option<LimitedPurchaseSkuSettings>,
    #[serde(rename = "luckyPokemonSettings")]
    pub lucky_pokemon_settings: Option<LuckyPokemonSettings>,
    #[serde(rename = "onboardingV2Settings")]
    pub onboarding_v2_settings: Option<OnboardingV2Settings>,
    #[serde(rename = "partyRecommendationSettings")]
    pub party_recommendation_settings: Option<PartyRecommendationSettings>,
    #[serde(rename = "platypusRolloutSettings")]
    pub platypus_rollout_settings: Option<PlatypusRolloutSettings>,
    #[serde(rename = "playerLevel")]
    pub player_level: Option<PlayerLevel>,
    #[serde(rename = "pokecoinPurchaseDisplayGmt")]
    pub pokecoin_purchase_display_gmt: Option<PokecoinPurchaseDisplayGmt>,
    #[serde(rename = "pokemonScaleSettings")]
    pub pokemon_scale_settings: Option<PokemonScaleSettings>,
    #[serde(rename = "typeEffective")]
    pub type_effective: Option<TypeEffective>,
    #[serde(rename = "pokemonUpgrades")]
    pub pokemon_upgrades: Option<PokemonUpgrades>,
    #[serde(rename = "questSettings")]
    pub quest_settings: Option<QuestSettings>,
    #[serde(rename = "smeargleMovesSettings")]
    pub smeargle_moves_settings: Option<SmeargleMovesSettings>,
    #[serde(rename = "genderSettings")]
    pub gender_settings: Option<GenderSettings>,
    #[serde(rename = "combatNpcTrainer")]
    pub combat_npc_trainer: Option<CombatNpcTrainer>,
    #[serde(rename = "combatNpcPersonality")]
    pub combat_npc_personality: Option<CombatNpcPersonality>,
    #[serde(rename = "pokemonSettings")]
    pub pokemon_settings: Option<PokemonSettings>,
    #[serde(rename = "vsSeekerClientSettings")]
    pub vs_seeker_client_settings: Option<VsSeekerClientSettings>,
    #[serde(rename = "vsSeekerLootProto")]
    pub vs_seeker_loot_proto: Option<VsSeekerLootProto>,
    #[serde(rename = "vsSeekerPokemonRewards")]
    pub vs_seeker_pokemon_rewards: Option<VsSeekerPokemonRewards>,
    #[serde(rename = "moveSettings")]
    pub move_settings: Option<MoveSettings>,
    #[serde(rename = "weatherAffinities")]
    pub weather_affinities: Option<WeatherAffinities>,
    #[serde(rename = "weatherBonusSettings")]
    pub weather_bonus_settings: Option<WeatherBonusSettings>,
    #[serde(rename = "adventureSyncV2Gmt")]
    pub adventure_sync_v2_gmt: Option<AdventureSyncV2Gmt>,
    #[serde(rename = "iapItemDisplay")]
    pub iap_item_display: Option<IapItemDisplay>,
    #[serde(rename = "camera")]
    pub camera: Option<Camera>,
    #[serde(rename = "moveSequenceSettings")]
    pub move_sequence_settings: Option<MoveSequenceSettings>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AvatarCustomization {
    pub enabled: Option<bool>,
    #[serde(rename = "avatarType")]
    pub avatar_type: Option<String>,
    pub slot: Vec<String>,
    #[serde(rename = "bundleName")]
    pub bundle_name: Option<String>,
    #[serde(rename = "assetName")]
    pub asset_name: String,
    #[serde(rename = "groupName")]
    pub group_name: String,
    #[serde(rename = "sortOrder")]
    pub sort_order: u16,
    #[serde(rename = "unlockType")]
    pub unlock_type: String,
    #[serde(rename = "unlockBadgeType")]
    pub unlock_badge_type: Option<String>,
    #[serde(rename = "iapSku")]
    pub iap_sku: Option<String>,
    #[serde(rename = "unlockBadgeLevel")]
    pub unlock_badge_level: Option<u16>,
    #[serde(rename = "iconName")]
    pub icon_name: Option<String>,
    #[serde(rename = "unlockPlayerLevel")]
    pub unlock_player_level: Option<u8>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BackgroundModeSettings {
    #[serde(rename = "weeklyFitnessGoalLevel1DistanceKm")]
    pub weekly_fitness_goal_level1_distance_km: f32,
    #[serde(rename = "weeklyFitnessGoalLevel2DistanceKm")]
    pub weekly_fitness_goal_level2_distance_km: f32,
    #[serde(rename = "weeklyFitnessGoalLevel3DistanceKm")]
    pub weekly_fitness_goal_level3_distance_km: f32,
    #[serde(rename = "weeklyFitnessGoalLevel4DistanceKm")]
    pub weekly_fitness_goal_level4_distance_km: f32,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BadgeSettings {
    #[serde(rename = "badgeType")]
    pub badge_type: String,
    #[serde(rename = "badgeRank")]
    pub badge_rank: u8,
    pub targets: Vec<u32>,
    #[serde(rename = "eventBadge")]
    pub event_badge: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BattleHubBadgeSettings {
    #[serde(rename = "combatHubDisplayedBadges")]
    pub combat_hub_displayed_badges: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BattleHubOrderSettings {
    section: Vec<Section>,
    #[serde(rename = "sectionGroup")]
    section_group: Vec<SectionGroup>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Section {
    #[serde(rename = "mainSection")]
    main_section: String,
    subsection: Vec<String>
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SectionGroup {
    section: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BattleSettings {
    #[serde(rename = "retargetSeconds")]
    pub retarget_seconds: f32,
    #[serde(rename = "enemyAttackInterval")]
    pub enemy_attack_interval: f32,
    #[serde(rename = "attackServerInterval")]
    pub attack_server_interval: f32,
    #[serde(rename = "roundDurationSeconds")]
    pub round_duration_seconds: f32,
    #[serde(rename = "bonusTimePerAllySeconds")]
    pub bonus_time_per_ally_seconds: f32,
    #[serde(rename = "maximumAttackersPerBattle")]
    pub maximum_attackers_per_battle: u8,
    #[serde(rename = "sameTypeAttackBonusMultiplier")]
    pub same_type_attack_bonus_multiplier: f32,
    #[serde(rename = "maximumEnergy")]
    pub maximum_energy: u8,
    #[serde(rename = "energyDeltaPerHealthLost")]
    pub energy_delta_per_health_lost: f32,
    #[serde(rename = "dodgeDurationMs")]
    pub dodge_duration_ms: u16,
    #[serde(rename = "minimumPlayerLevel")]
    pub minimum_player_level: u8,
    #[serde(rename = "swapDurationMs")]
    pub swap_duration_ms: u16,
    #[serde(rename = "dodgeDamageReductionPercent")]
    pub dodge_damage_reduction_percent: f32,
    #[serde(rename = "minimumRaidPlayerLevel")]
    pub minimum_raid_player_level: u8,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BelugaPokemonWhitelist {
    #[serde(rename = "maxAllowedPokemonPokedexNumber")]
    pub max_allowed_pokemon_pokedex_number: u8,
    #[serde(rename = "additionalPokemonAllowed")]
    pub additional_pokemon_allowed: Vec<String>,
    #[serde(rename = "costumesAllowed")]
    pub costumes_allowed: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuddyActivitySettings {
    pub activity: String,
    #[serde(rename = "activityCategory")]
    pub activity_category: String,
    #[serde(rename = "maxTimesPerDay")]
    pub max_times_per_day: u8,
    #[serde(rename = "numPointsPerAction")]
    pub num_points_per_action: u8,
    #[serde(rename = "numEmotionPointsPerAction")]
    pub num_emotion_points_per_action: u8,
    #[serde(rename = "emotionCooldownDurationMs")]
    pub emotion_cooldown_duration_ms: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuddyActivityCategorySettings {
    #[serde(rename = "activityCategory")]
    pub activity_category: String,
    #[serde(rename = "maxPointsPerDay")]
    pub max_points_per_day: u8,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuddyEmotionLevelSettings {
    #[serde(rename = "emotionLevel")]
    pub emotion_level: String,
    #[serde(rename = "minEmotionPointsRequired")]
    pub min_emotion_points_required: Option<u8>,
    #[serde(rename = "emotionAnimation")]
    pub emotion_animation: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuddyEncounterCameoSettings {
    #[serde(rename = "buddyWildEncounterCameoChancePercent")]
    pub buddy_wild_encounter_cameo_chance_percent: f32,
    #[serde(rename = "buddyQuestEncounterCameoChancePercent")]
    pub buddy_quest_encounter_cameo_chance_percent: f32,
    #[serde(rename = "buddyRaidEncounterCameoChancePercent")]
    pub buddy_raid_encounter_cameo_chance_percent: f32,
    #[serde(rename = "buddyInvasionEncounterCameoChancePercent")]
    pub buddy_invasion_encounter_cameo_chance_percent: f32,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuddyHungerSettings {
    #[serde(rename = "numHungerPointsRequiredForFull")]
    pub num_hunger_points_required_for_full: u8,
    #[serde(rename = "decayPointsPerBucket")]
    pub decay_points_per_bucket: u8,
    #[serde(rename = "millisecondsPerBucket")]
    pub milliseconds_per_bucket: String,
    #[serde(rename = "cooldownDurationMs")]
    pub cooldown_duration_ms: String,
    #[serde(rename = "decayDurationAfterFullMs")]
    pub decay_duration_after_full_ms: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuddyInteractionSettings {
    #[serde(rename = "feedItemWhitelist")]
    pub feed_item_whitelist: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuddyLevelSettings {
    pub level: String,
    #[serde(rename = "minNonCumulativePointsRequired")]
    pub min_non_cumulative_points_required: Option<u16>,
    #[serde(rename = "unlockedTraits")]
    pub unlocked_traits: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuddySwapSettings {
    #[serde(rename = "maxSwapsPerDay")]
    pub max_swaps_per_day: u8,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuddyWalkSettings {
    #[serde(rename = "kmRequiredPerAffectionPoint")]
    pub km_required_per_affection_point: f32,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InvasionNpcDisplaySettings {
    #[serde(rename = "trainerName")]
    pub trainer_name: String,
    pub avatar: Avatar,
    #[serde(rename = "trainerTitle")]
    pub trainer_title: String,
    #[serde(rename = "trainerQuote")]
    pub trainer_quote: String,
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
    #[serde(rename = "backdropImageBundle")]
    pub backdrop_image_bundle: Option<String>,
    #[serde(rename = "modelName")]
    pub model_name: String,
    #[serde(rename = "tutorialOnLossString")]
    pub tutorial_on_loss_string: Option<String>,
    #[serde(rename = "isMale")]
    pub is_male: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CombatCompetitiveSeasonSettings {
    #[serde(rename = "seasonEndTimeTimestamp")]
    pub season_end_time_timestamp: Vec<String>,
    #[serde(rename = "ratingAdjustmentPercentage")]
    pub rating_adjustment_percentage: f32,
    #[serde(rename = "rankingAdjustmentPercentage")]
    pub ranking_adjustment_percentage: f32,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Avatar {
    pub avatar: u8,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CombatLeague {
    pub title: String,
    pub enabled: bool,
    #[serde(rename = "unlockCondition")]
    pub unlock_condition: Option<Vec<PokemonCondition>>,
    #[serde(rename = "pokemonCondition")]
    pub pokemon_condition: Vec<PokemonCondition>,
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
    #[serde(rename = "pokemonCount")]
    pub pokemon_count: u8,
    #[serde(rename = "bannedPokemon")]
    pub banned_pokemon: Vec<String>,
    #[serde(rename = "badgeType")]
    pub badge_type: String,
    #[serde(rename = "battlePartyCombatLeagueTemplateId")]
    pub battle_party_combat_league_template_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PokemonCondition {
    pub r#type: String,
    #[serde(rename = "minPokemonCount")]
    pub min_pokemon_count: Option<u8>,
    #[serde(rename = "withPokemonCpLimit")]
    pub with_pokemon_cp_limit: Option<Limit>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Limit {
    #[serde(rename = "minCp")]
    pub min_cp: Option<u32>,
    #[serde(rename = "maxCp")]
    pub max_cp: u32,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CombatLeagueSettings {
    #[serde(rename = "combatLeagueTemplateId")]
    pub combat_league_template_id: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CombatType {
    pub r#type: String,
    #[serde(rename = "niceLevelThreshold")]
    pub nice_level_threshold: f32,
    #[serde(rename = "greatLevelThreshold")]
    pub great_level_threshold: f32,
    #[serde(rename = "excellentLevelThreshold")]
    pub excellent_level_threshold: f32,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CombatRankingProtoSettings {
    #[serde(rename = "rankLevel")]
    pub rank_level: Vec<RankLevel>,
    #[serde(rename = "requiredForRewards")]
    pub required_for_rewards: RankLevel,
    #[serde(rename = "minRankToDisplayRating")]
    pub min_rank_to_display_rating: u8,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RankLevel {
    #[serde(rename = "rankLevel")]
    pub rank_level: u8,
    #[serde(rename = "additionalTotalBattlesRequired")]
    pub additional_total_battles_required: Option<u8>,
    #[serde(rename = "additionalWinsRequired")]
    pub additional_wins_required: Option<u8>,
    #[serde(rename = "minRatingRequired")]
    pub min_rating_required: Option<u16>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CombatSettings {
    #[serde(rename = "roundDurationSeconds")]
    pub round_duration_seconds: f32,
    #[serde(rename = "turnDurationSeconds")]
    pub turn_duration_seconds: f32,
    #[serde(rename = "minigameDurationSeconds")]
    pub minigame_duration_seconds: f32,
    #[serde(rename = "sameTypeAttackBonusMultiplier")]
    pub same_type_attack_bonus_multiplier: f32,
    #[serde(rename = "fastAttackBonusMultiplier")]
    pub fast_attack_bonus_multiplier: f32,
    #[serde(rename = "chargeAttackBonusMultiplier")]
    pub charge_attack_bonus_multiplier: f32,
    #[serde(rename = "defenseBonusMultiplier")]
    pub defense_bonus_multiplier: f32,
    #[serde(rename = "minigameBonusBaseMultiplier")]
    pub minigame_bonus_base_multiplier: f32,
    #[serde(rename = "minigameBonusVariableMultiplier")]
    pub minigame_bonus_variable_multiplier: f32,
    #[serde(rename = "maxEnergy")]
    pub max_energy: u8,
    #[serde(rename = "defenderMinigameMultiplier")]
    pub defender_minigame_multiplier: f32,
    #[serde(rename = "changePokemonDurationSeconds")]
    pub change_pokemon_duration_seconds: f32,
    #[serde(rename = "minigameSubmitScoreDurationSeconds")]
    pub minigame_submit_score_duration_seconds: f32,
    #[serde(rename = "quickSwapCooldownDurationSeconds")]
    pub quick_swap_cooldown_duration_seconds: f32,
    #[serde(rename = "chargeScoreBase")]
    pub charge_score_base: f32,
    #[serde(rename = "chargeScoreNice")]
    pub charge_score_nice: f32,
    #[serde(rename = "chargeScoreGreat")]
    pub charge_score_great: f32,
    #[serde(rename = "chargeScoreExcellent")]
    pub charge_score_excellent: f32,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CombatStatStageSettings {
    #[serde(rename = "minimumStatStage")]
    pub minimum_stat_stage: i8,
    #[serde(rename = "maximumStatStage")]
    pub maximum_stat_stage: i8,
    #[serde(rename = "attackBuffMultiplier")]
    pub attack_buff_multiplier: Vec<f32>,
    #[serde(rename = "defenseBuffMultiplier")]
    pub defense_buff_multiplier: Vec<f32>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CombatMove {
    #[serde(rename = "uniqueId")]
    pub unique_id: String,
    pub r#type: String,
    pub power: Option<f32>,
    #[serde(rename = "vfxName")]
    pub vfx_name: String,
    #[serde(rename = "energyDelta")]
    pub energy_delta: Option<i8>,
    #[serde(rename = "durationTurns")]
    pub duration_turns: Option<u8>,
    pub buffs: Option<Buffs>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Buffs {
    #[serde(rename = "attackerAttackStatStageChange")]
    pub attacker_attack_stat_stage_change: Option<i8>,
    #[serde(rename = "attackerDefenseStatStageChange")]
    pub attacker_defense_stat_stage_change: Option<i8>,
    #[serde(rename = "targetAttackStatStageChange")]
    pub target_attack_stat_stage_change: Option<i8>,
    #[serde(rename = "targetDefenseStatStageChange")]
    pub target_defense_stat_stage_change: Option<i8>,
    #[serde(rename = "buffActivationChance")]
    pub buff_activation_chance: f32,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EncounterSettings {
    #[serde(rename = "spinBonusThreshold")]
    pub spin_bonus_threshold: f32,
    #[serde(rename = "excellentThrowThreshold")]
    pub excellent_throw_threshold: f32,
    #[serde(rename = "greatThrowThreshold")]
    pub great_throw_threshold: f32,
    #[serde(rename = "niceThrowThreshold")]
    pub nice_throw_threshold: f32,
    #[serde(rename = "milestoneThreshold")]
    pub milestone_threshold: u8,
    #[serde(rename = "arPlusModeEnabled")]
    pub ar_plus_mode_enabled: bool,
    #[serde(rename = "arCloseProximityThreshold")]
    pub ar_close_proximity_threshold: f32,
    #[serde(rename = "arLowAwarenessThreshold")]
    pub ar_low_awareness_threshold: f32,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExRaidSettings {
    #[serde(rename = "minimumExRaidShareLevel")]
    pub minimum_ex_raid_share_level: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FormSettings {
    pub pokemon: String,
    pub forms: Option<Vec<PokemonForm>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PokemonForm {
    pub form: String,
    #[serde(rename = "assetBundleValue")]
    pub asset_bundle_value: Option<u8>,
    #[serde(rename = "assetBundleSuffix")]
    pub asset_bundle_suffix: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FriendshipMilestoneSettings {
    #[serde(rename = "minPointsToReach")]
    pub min_points_to_reach: Option<u8>,
    #[serde(rename = "milestoneXpReward")]
    pub milestone_xp_reward: u32,
    #[serde(rename = "attackBonusPercentage")]
    pub attack_bonus_percentage: f32,
    #[serde(rename = "raidBallBonus")]
    pub raid_ball_bonus: Option<u8>,
    #[serde(rename = "unlockedTrading")]
    pub unlocked_trading: Vec<String>,
    #[serde(rename = "tradingDiscount")]
    pub trading_discount: Option<f32>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GymBadgeSettings {
    pub target: Vec<u32>,
    #[serde(rename = "battleWinningScorePerDefenderCp")]
    pub battle_winning_score_per_defender_cp: f32,
    #[serde(rename = "gymDefendingScorePerMinute")]
    pub gym_defending_score_per_minute: f32,
    #[serde(rename = "berryFeedingScore")]
    pub berry_feeding_score: u8,
    #[serde(rename = "pokemonDeployScore")]
    pub pokemon_deploy_score: u8,
    #[serde(rename = "raidBattleWinningScore")]
    pub raid_battle_winning_score: u16,
    #[serde(rename = "loseAllBattlesScore")]
    pub lose_all_battles_score: u8,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GymLevel {
    #[serde(rename = "requiredExperience")]
    pub required_experience: Vec<u32>,
    #[serde(rename = "leaderSlots")]
    pub leader_slots: Vec<u8>,
    #[serde(rename = "trainerSlots")]
    pub trainer_slots: Vec<u8>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IapCategoryDisplay {
    pub category: String,
    #[serde(rename = "sortOrder")]
    pub sort_order: u8,
    #[serde(rename = "bannerEnabled")]
    pub banner_enabled: Option<bool>,
    #[serde(rename = "imageUrl")]
    pub image_url: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "bannerTitle")]
    pub banner_title: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IapSettings {
    #[serde(rename = "dailyDefenderBonusPerPokemon")]
    pub daily_defender_bonus_per_pokemon: Vec<u16>,
    #[serde(rename = "dailyDefenderBonusMaxDefenders")]
    pub daily_defender_bonus_max_defenders: u8,
    #[serde(rename = "dailyDefenderBonusCurrency")]
    pub daily_defender_bonus_currency: Vec<String>,
    #[serde(rename = "minTimeBetweenClaimsMs")]
    pub min_time_between_claims_ms: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PokestopInvasionAvailabilitySettings {
    #[serde(rename = "availabilityStartMinute")]
    pub availability_start_minute: String,
    #[serde(rename = "availabilityEndMinute")]
    pub availability_end_minute: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ItemSettings {
    #[serde(rename = "itemId")]
    pub item_id: String,
    #[serde(rename = "itemType")]
    pub item_type: String,
    pub category: String,
    #[serde(rename = "dropTrainerLevel")]
    pub drop_trainer_level: Option<u8>,
    pub food: Option<Food>,
    pub potion: Option<Potion>,
    pub revive: Option<Potion>,
    #[serde(rename = "eggIncubator")]
    pub egg_incubator: Option<EggIncubator>,
    #[serde(rename = "inventoryUpgrade")]
    pub inventory_upgrade: Option<InventoryUpgrade>,
    #[serde(rename = "xpBoost")]
    pub xp_boost: Option<Boost>,
    #[serde(rename = "stardustBoost")]
    pub stardust_boost: Option<Boost>,
    #[serde(rename = "incidentTicket")]
    pub incident_ticket: Option<IncidentTicket>,
    #[serde(rename = "globalEventTicket")]
    pub global_event_ticket: Option<GlobalEventTicket>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IncidentTicket {
    #[serde(rename = "ignoreFullInventory")]
    pub ignore_full_inventory: Option<bool>,
    #[serde(rename = "upgradeRequirementCount")]
    pub upgrade_requirement_count: Option<u8>,
    #[serde(rename = "upgradedItem")]
    pub upgraded_item: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalEventTicket {
    #[serde(rename = "eventBadge")]
    pub event_badge: String,
    #[serde(rename = "grantBadgeBeforeEventStartMs")]
    pub grant_badge_before_event_start_ms: String,
    #[serde(rename = "eventStartTime")]
    pub event_start_time: String,
    #[serde(rename = "eventEndTime")]
    pub event_end_time: String,
    #[serde(rename = "clientEventStartTimeUtcMs")]
    pub client_event_start_time_utc_ms: String,
    #[serde(rename = "clientEventEndTimeUtcMs")]
    pub client_event_end_time_utc_ms: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Food {
    #[serde(rename = "itemEffect")]
    pub item_effect: Option<Vec<String>>,
    #[serde(rename = "itemEffectPercent")]
    pub item_effect_percent: Option<Vec<f32>>,
    #[serde(rename = "growthPercent")]
    pub growth_percent: Option<f32>,
    #[serde(rename = "berryMultiplier")]
    pub berry_multiplier: Option<f32>,
    #[serde(rename = "remoteBerryMultiplier")]
    pub remote_berry_multiplier: Option<f32>,
    #[serde(rename = "numBuddyAffectionPoints")]
    pub num_buddy_affection_points: Option<u8>,
    #[serde(rename = "mapDurationMs")]
    pub map_duration_ms: Option<String>,
    #[serde(rename = "activeDurationMs")]
    pub active_duration_ms: Option<String>,
    #[serde(rename = "numBuddyHungerPoints")]
    pub num_buddy_hunger_points: Option<u8>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Potion {
    #[serde(rename = "staAmount")]
    pub sta_amount: Option<u8>,
    #[serde(rename = "staPercent")]
    pub sta_percent: Option<f32>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EggIncubator {
    #[serde(rename = "incubatorType")]
    pub incubator_type: String,
    pub uses: Option<u8>,
    #[serde(rename = "distanceMultiplier")]
    pub distance_multiplier: f32,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InventoryUpgrade {
    #[serde(rename = "additionalStorage")]
    pub additional_storage: u8,
    #[serde(rename = "upgradeType")]
    pub upgrade_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Boost {
    #[serde(rename = "xpMultiplier")]
    pub xp_multiplier: Option<f32>,
    #[serde(rename = "stardustMultiplier")]
    pub stardust_multiplier: Option<f32>,
    #[serde(rename = "boostDurationMs")]
    pub boost_duration_ms: u32,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LoadingScreenSettings {
    pub url: String,
    #[serde(rename = "displayAfterTimestampMs")]
    pub display_after_timestamp_ms: String,
    #[serde(rename = "colorSettings")]
    pub color_settings: ColorSettings,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ColorSettings {
    pub warning_text: String,
    pub progress_background: String,
    pub progress_bar_left: String,
    pub progress_bar_right: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LimitedPurchaseSkuSettings {
    #[serde(rename = "purchaseLimit")]
    pub purchase_limit: u8,
    pub version: u8,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LuckyPokemonSettings {
    #[serde(rename = "powerUpStardustDiscountPercent")]
    pub power_up_stardust_discount_percent: f32,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OnboardingV2Settings {
    #[serde(rename = "pokedexId")]
    pub pokedex_id: Vec<String>,
    #[serde(rename = "eggKmUntilHatch")]
    pub egg_km_until_hatch: u8,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PartyRecommendationSettings {
    pub mode: String,
    pub variance: f32,
    #[serde(rename = "thirdMoveWeight")]
    pub third_move_weight: f32,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlatypusRolloutSettings {
    #[serde(rename = "buddyV2MinPlayerLevel")]
    pub buddy_v2_min_player_level: u8,
    #[serde(rename = "buddyMultiplayerMinPlayerLevel")]
    pub buddy_multiplayer_min_player_level: u8,
    #[serde(rename = "wallabySettings")]
    pub wallaby_settings: Value,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlayerLevel {
    #[serde(rename = "rankNum")]
    pub rank_num: Vec<u8>,
    #[serde(rename = "requiredExperience")]
    pub required_experience: Vec<u32>,
    #[serde(rename = "cpMultiplier")]
    pub cp_multiplier: Vec<f64>,
    #[serde(rename = "maxEggPlayerLevel")]
    pub max_egg_player_level: u8,
    #[serde(rename = "maxEncounterPlayerLevel")]
    pub max_encounter_player_level: u8,
    #[serde(rename = "maxQuestEncounterPlayerLevel")]
    pub max_quest_encounter_player_level: u8,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PokecoinPurchaseDisplayGmt {
    #[serde(rename = "featureEnabled")]
    pub feature_enabled: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PokemonScaleSettings {
    #[serde(rename = "pokemonScaleMode")]
    pub pokemon_scale_mode: Option<String>,
    #[serde(rename = "minHeight")]
    pub min_height: f32,
    #[serde(rename = "maxHeight")]
    pub max_height: f32,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TypeEffective {
    #[serde(rename = "attackScalar")]
    pub attack_scalar: Vec<f64>,
    #[serde(rename = "attackType")]
    pub attack_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PokemonUpgrades {
    #[serde(rename = "upgradesPerLevel")]
    pub upgrades_per_level: u8,
    #[serde(rename = "allowedLevelsAbovePlayer")]
    pub allowed_levels_above_player: u8,
    #[serde(rename = "candyCost")]
    pub candy_cost: Vec<u8>,
    #[serde(rename = "stardustCost")]
    pub stardust_cost: Vec<u32>,
    #[serde(rename = "shadowStardustMultiplier")]
    pub shadow_stardust_multiplier: f32,
    #[serde(rename = "shadowCandyMultiplier")]
    pub shadow_candy_multiplier: f32,
    #[serde(rename = "purifiedStardustMultiplier")]
    pub purified_stardust_multiplier: f32,
    #[serde(rename = "purifiedCandyMultiplier")]
    pub purified_candy_multiplier: f32,
    #[serde(rename = "maxNormalUpgradeLevel")]
    pub max_normal_upgrade_level: u8,
    #[serde(rename = "defaultCpBoostAdditionalLevel")]
    pub default_cp_boost_additional_level: u8,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QuestSettings {
    #[serde(rename = "questType")]
    pub quest_type: String,
    #[serde(rename = "dailyQuest")]
    pub daily_quest: DailyQuest,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DailyQuest {
    #[serde(rename = "bucketsPerDay")]
    pub buckets_per_day: u8,
    #[serde(rename = "streakLength")]
    pub streak_length: u8,
    #[serde(rename = "bonusMultiplier")]
    pub bonus_multiplier: Option<f32>,
    #[serde(rename = "streakBonusMultiplier")]
    pub streak_bonus_multiplier: Option<f32>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SmeargleMovesSettings {
    #[serde(rename = "quickMoves")]
    pub quick_moves: Vec<String>,
    #[serde(rename = "cinematicMoves")]
    pub cinematic_moves: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GenderSettings {
    pub pokemon: String,
    pub gender: Gender
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Gender {
    #[serde(rename = "malePercent")]
    pub male_percent: Option<f32>,
    #[serde(rename = "femalePercent")]
    pub female_percent: Option<f32>,
    #[serde(rename = "genderlessPercent")]
    pub genderless_percent: Option<f32>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CombatNpcTrainer {
    #[serde(rename = "trainerName")]
    pub trainer_name: String,
    #[serde(rename = "combatLeagueTemplateId")]
    pub combat_league_template_id: String,
    #[serde(rename = "combatPersonalityId")]
    pub combat_personality_id: String,
    pub avatar: Avatar,
    #[serde(rename = "availablePokemon")]
    pub available_pokemon: Vec<AvailablePokemon>,
    #[serde(rename = "trainerTitle")]
    pub trainer_title: String,
    #[serde(rename = "trainerQuote")]
    pub trainer_quote: String,
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
    #[serde(rename = "backdropImageBundle")]
    pub backdrop_image_bundle: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AvailablePokemon {
    #[serde(rename = "pokemonId")]
    pub pokemon_id: Option<String>,
    #[serde(rename = "pokemonType")]
    pub pokemon_type: Option<String>,
    #[serde(rename = "pokemonDisplay")]
    pub pokemon_display: Option<PokemonForm>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CombatNpcPersonality {
    #[serde(rename = "personalityName")]
    pub personality_name: String,
    #[serde(rename = "superEffectiveChance")]
    pub super_effective_chance: f32,
    #[serde(rename = "specialChance")]
    pub special_chance: f32,
    #[serde(rename = "defensiveMinimumScore")]
    pub defensive_minimum_score: Option<f32>,
    #[serde(rename = "defensiveMaximumScore")]
    pub defensive_maximum_score: Option<f32>,
    #[serde(rename = "offensiveMinimumScore")]
    pub offensive_minimum_score: f32,
    #[serde(rename = "offensiveMaximumScore")]
    pub offensive_maximum_score: f32,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PokemonSettings {
    #[serde(rename = "pokemonId")]
    pub pokemon_id: String,
    #[serde(rename = "modelScale")]
    pub model_scale: f32,
    pub r#type: String,
    pub type2: Option<String>,
    pub camera: CameraSetting,
    pub encounter: Encounter,
    pub stats: Stats,
    #[serde(rename = "quickMoves")]
    pub quick_moves: Option<Vec<String>>,
    #[serde(rename = "cinematicMoves")]
    pub cinematic_moves: Option<Vec<String>>,
    #[serde(rename = "animationTime")]
    pub animation_time: Vec<f32>,
    #[serde(rename = "evolutionIds")]
    pub evolution_ids: Option<Vec<String>>,
    #[serde(rename = "evolutionPips")]
    pub evolution_pips: u8,
    pub rarity: Option<String>,
    #[serde(rename = "pokedexHeightM")]
    pub pokedex_height_m: f32,
    #[serde(rename = "pokedexWeightKg")]
    pub pokedex_weight_kg: f32,
    #[serde(rename = "parentPokemonId")]
    pub parent_pokemon_id: Option<String>,
    #[serde(rename = "heightStdDev")]
    pub height_std_dev: f32,
    #[serde(rename = "weightStdDev")]
    pub weight_std_dev: f32,
    #[serde(rename = "familyId")]
    pub family_id: String,
    #[serde(rename = "candyToEvolve")]
    pub candy_to_evolve: Option<u16>,
    #[serde(rename = "kmBuddyDistance")]
    pub km_buddy_distance: f32,
    #[serde(rename = "buddySize")]
    pub buddy_size: Option<String>,
    #[serde(rename = "modelHeight")]
    pub model_height: f32,
    #[serde(rename = "evolutionBranch")]
    pub evolution_branch: Option<Vec<EvolutionBranch>>,
    #[serde(rename = "modelScaleV2")]
    pub model_scale_v2: f32,
    pub form: Option<String>,
    #[serde(rename = "buddyOffsetMale")]
    pub buddy_offset_male: Vec<f32>,
    #[serde(rename = "buddyOffsetFemale")]
    pub buddy_offset_female: Vec<f32>,
    #[serde(rename = "buddyScale")]
    pub buddy_scale: f32,
    #[serde(rename = "buddyPortraitOffset")]
    pub buddy_portrait_offset: Option<Vec<f32>>,
    #[serde(rename = "thirdMove")]
    pub third_move: ThirdMove,
    #[serde(rename = "isTransferable")]
    pub is_transferable: Option<bool>,
    #[serde(rename = "isDeployable")]
    pub is_deployable: Option<bool>,
    #[serde(rename = "combatShoulderCameraAngle")]
    pub combat_shoulder_camera_angle: Option<Vec<f32>>,
    #[serde(rename = "combatDefaultCameraAngle")]
    pub combat_default_camera_angle: Option<Vec<f32>>,
    #[serde(rename = "combatPlayerFocusCameraAngle")]
    pub combat_player_focus_camera_angle: Option<Vec<f32>>,
    pub shadow: Option<Shadow>,
    #[serde(rename = "combatPlayerPokemonPositionOffset")]
    pub combat_player_pokemon_position_offset: Option<Vec<f32>>,
    #[serde(rename = "combatOpponentFocusCameraAngle")]
    pub combat_opponent_focus_camera_angle: Option<Vec<f32>>,
    #[serde(rename = "buddyGroupNumber")]
    pub buddy_group_number: u8,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CameraSetting {
    #[serde(rename = "diskRadiusM")]
    pub disk_radius_m: f32,
    #[serde(rename = "cylinderRadiusM")]
    pub cylinder_radius_m: f32,
    #[serde(rename = "cylinderHeightM")]
    pub cylinder_height_m: f32,
    #[serde(rename = "cylinderGroundM")]
    pub cylinder_ground_m: Option<f32>,
    #[serde(rename = "shoulderModeScale")]
    pub shoulder_mode_scale: Option<f32>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Encounter {
    #[serde(rename = "baseCaptureRate")]
    pub base_capture_rate: Option<f32>,
    #[serde(rename = "baseFleeRate")]
    pub base_flee_rate: Option<f32>,
    #[serde(rename = "collisionRadiusM")]
    pub collision_radius_m: f32,
    #[serde(rename = "collisionHeightM")]
    pub collision_height_m: f32,
    #[serde(rename = "collisionHeadRadiusM")]
    pub collision_head_radius_m: f32,
    #[serde(rename = "movementType")]
    pub movement_type: Option<String>,
    #[serde(rename = "movementTimerS")]
    pub movement_timer_s: f32,
    #[serde(rename = "jumpTimeS")]
    pub jump_time_s: Option<f32>,
    #[serde(rename = "attackTimerS")]
    pub attack_timer_s: f32,
    #[serde(rename = "bonusCandyCaptureReward")]
    pub bonus_candy_capture_reward: Option<u8>,
    #[serde(rename = "bonusStardustCaptureReward")]
    pub bonus_stardust_capture_reward: Option<u16>,
    #[serde(rename = "attackProbability")]
    pub attack_probability: Option<f32>,
    #[serde(rename = "dodgeProbability")]
    pub dodge_probability: Option<f32>,
    #[serde(rename = "dodgeDurationS")]
    pub dodge_duration_s: f32,
    #[serde(rename = "dodgeDistance")]
    pub dodge_distance: f32,
    #[serde(rename = "cameraDistance")]
    pub camera_distance: f32,
    #[serde(rename = "minPokemonActionFrequencyS")]
    pub min_pokemon_action_frequency_s: f32,
    #[serde(rename = "maxPokemonActionFrequencyS")]
    pub max_pokemon_action_frequency_s: f32,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Stats {
    #[serde(rename = "baseStamina")]
    pub base_stamina: u16,
    #[serde(rename = "baseAttack")]
    pub base_attack: u16,
    #[serde(rename = "baseDefense")]
    pub base_defense: u16,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvolutionBranch {
    pub evolution: Option<String>,
    #[serde(rename = "evolutionItemRequirement")]
    pub evolution_item_requirement: Option<String>,
    #[serde(rename = "candyCost")]
    pub candy_cost: Option<u16>,
    #[serde(rename = "kmBuddyDistanceRequirement")]
    pub km_buddy_distance_requirement: Option<f32>,
    #[serde(rename = "mustBeBuddy")]
    pub must_be_buddy: Option<bool>,
    #[serde(rename = "onlyDaytime")]
    pub only_daytime: Option<bool>,
    #[serde(rename = "onlyNighttime")]
    pub only_nighttime: Option<bool>,
    pub priority: Option<u8>,
    pub form: Option<String>,
    #[serde(rename = "noCandyCostViaTrade")]
    pub no_candy_cost_via_trade: Option<bool>,
    #[serde(rename = "lureItemRequirement")]
    pub lure_item_requirement: Option<String>,
    #[serde(rename = "genderRequirement")]
    pub gender_requirement: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ThirdMove {
    #[serde(rename = "stardustToUnlock")]
    pub stardust_to_unlock: Option<u32>,
    #[serde(rename = "candyToUnlock")]
    pub candy_to_unlock: u32,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Shadow {
    #[serde(rename = "purificationStardustNeeded")]
    pub purification_stardust_needed: u16,
    #[serde(rename = "purificationCandyNeeded")]
    pub purification_candy_needed: u8,
    #[serde(rename = "purifiedChargeMove")]
    pub purified_charge_move: String,
    #[serde(rename = "shadowChargeMove")]
    pub shadow_charge_move: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VsSeekerClientSettings {
    #[serde(rename = "allowedVsSeekerLeagueTemplateId")]
    pub allowed_vs_seeker_league_template_id: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VsSeekerLootProto {
    #[serde(rename = "rankLevel")]
    pub rank_level: u8,
    pub reward: Vec<Reward>,
    #[serde(rename = "rewardTrack")]
    pub reward_track: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Reward {
    pub item: Option<Item>,
    #[serde(rename = "itemLootTable")]
    pub item_loot_table: Option<bool>,
    #[serde(rename = "pokemonReward")]
    pub pokemon_reward: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Item {
    pub count: u16,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VsSeekerPokemonRewards {
    #[serde(rename = "availablePokemon")]
    pub available_pokemon: Vec<PokemonReward>,
    #[serde(rename = "rewardTrack")]
    pub reward_track: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PokemonReward {
    pub pokemon: Option<AvailablePokemon>,
    #[serde(rename = "guaranteedLimitedPokemonReward")]
    pub guaranteed_limited_pokemon_reward: Option<GuaranteedLimitedPokemonReward>,
    #[serde(rename = "unlockedAtRank")]
    pub unlocked_at_rank: u8,
    #[serde(rename = "attackIvOverride")]
    pub attack_iv_override: IvOverride,
    #[serde(rename = "defenseIvOverride")]
    pub defense_iv_override: IvOverride,
    #[serde(rename = "staminaIvOverride")]
    pub stamina_iv_override: IvOverride,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GuaranteedLimitedPokemonReward {
    pub pokemon: AvailablePokemon,
    pub identifier: String,
    #[serde(rename = "lifetimeMaxCount")]
    pub lifetime_max_count: u8,
    #[serde(rename = "rewardTrack")]
    pub reward_track: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IvOverride {
    pub range: Range,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Range {
    pub min: String,
    pub max: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MoveSettings {
    #[serde(rename = "movementId")]
    pub movement_id: String,
    #[serde(rename = "animationId")]
    pub animation_id: u8,
    #[serde(rename = "pokemonType")]
    pub pokemon_type: String,
    pub power: Option<f32>,
    #[serde(rename = "accuracyChance")]
    pub accuracy_chance: f32,
    #[serde(rename = "criticalChance")]
    pub critical_chance: Option<f32>,
    #[serde(rename = "healScalar")]
    pub heal_scalar: Option<f32>,
    #[serde(rename = "staminaLossScalar")]
    pub stamina_loss_scalar: Option<f32>,
    #[serde(rename = "trainerLevelMin")]
    pub trainer_level_min: u8,
    #[serde(rename = "trainerLevelMax")]
    pub trainer_level_max: u8,
    #[serde(rename = "vfxName")]
    pub vfx_name: String,
    #[serde(rename = "durationMs")]
    pub duration_ms: u32,
    #[serde(rename = "damageWindowStartMs")]
    pub damage_window_start_ms: u32,
    #[serde(rename = "damageWindowEndMs")]
    pub damage_window_end_ms: u32,
    #[serde(rename = "energyDelta")]
    pub energy_delta: Option<i8>,
    #[serde(rename = "isLocked")]
    pub is_locked: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WeatherAffinities {
    #[serde(rename = "weatherCondition")]
    pub weather_condition: String,
    #[serde(rename = "pokemonType")]
    pub pokemon_type: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WeatherBonusSettings {
    #[serde(rename = "cpBaseLevelBonus")]
    pub cp_base_level_bonus: u8,
    #[serde(rename = "guaranteedIndividualValues")]
    pub guaranteed_individual_values: u8,
    #[serde(rename = "stardustBonusMultiplier")]
    pub stardust_bonus_multiplier: f32,
    #[serde(rename = "attackBonusMultiplier")]
    pub attack_bonus_multiplier: f32,
    #[serde(rename = "raidEncounterCpBaseLevelBonus")]
    pub raid_encounter_cp_base_level_bonus: u8,
    #[serde(rename = "raidEncounterGuaranteedIndividualValues")]
    pub raid_encounter_guaranteed_individual_values: u8,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdventureSyncV2Gmt {
    #[serde(rename = "featureEnabled")]
    pub feature_enabled: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IapItemDisplay {
    pub sku: String,
    pub category: String,
    #[serde(rename = "skuEnableTime")]
    pub sku_enable_time: Option<String>,
    #[serde(rename = "skuDisableTime")]
    pub sku_disable_time: Option<String>,
    #[serde(rename = "skuEnableTimeUtcMs")]
    pub sku_enable_time_utc_ms: Option<String>,
    #[serde(rename = "skuDisableTimeUtcMs")]
    pub sku_disable_time_utc_ms: Option<String>,
    #[serde(rename = "sortOrder")]
    pub sort_order: Option<u8>,
    pub sale: Option<bool>,
    pub hidden: Option<bool>,
    #[serde(rename = "spriteId")]
    pub sprite_id: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Camera {
    #[serde(rename = "nextCamera")]
    pub next_camera: Option<String>,
    pub interpolation: Vec<String>,
    #[serde(rename = "targetType")]
    pub target_type: Vec<String>,
    #[serde(rename = "easeInSpeed")]
    pub ease_in_speed: Vec<f32>,
    #[serde(rename = "easeOutSpeed")]
    pub ease_out_speed: Vec<f32>,
    #[serde(rename = "durationSeconds")]
    pub duration_seconds: Vec<f32>,
    #[serde(rename = "waitSeconds")]
    pub wait_seconds: Vec<f32>,
    #[serde(rename = "transitionSeconds")]
    pub transition_seconds: Vec<f32>,
    #[serde(rename = "angleDegree")]
    pub angle_degree: Vec<f32>,
    #[serde(rename = "angleOffsetDegree")]
    pub angle_offset_degree: Vec<f32>,
    #[serde(rename = "pitchDegree")]
    pub pitch_degree: Vec<f32>,
    #[serde(rename = "pitchOffsetDegree")]
    pub pitch_offset_degree: Vec<f32>,
    #[serde(rename = "rollDegree")]
    pub roll_degree: Vec<f32>,
    #[serde(rename = "distanceMeters")]
    pub distance_meters: Vec<f32>,
    #[serde(rename = "heightPercent")]
    pub height_percent: Vec<f32>,
    #[serde(rename = "vertCtrRatio")]
    pub vert_ctr_ratio: Vec<f32>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MoveSequenceSettings {
    pub sequence: Vec<String>,
}

// impl CombatMove {
//     // Damage Per Turn (power / duration) only for fast moves
//     pub fn get_dpt(&self) -> f32 {
//         self.power.unwrap_or_else(|| 0.0) / ((self.duration_turns.unwrap_or_else(|| 0) + 1) as f32)
//     }

//     // Energy Per Turn (energy / duration) only for fast moves
//     pub fn get_ept(&self) -> f32 {
//         (self.energy_delta.unwrap_or_else(|| 0) as f32) / ((self.duration_turns.unwrap_or_else(|| 0) + 1) as f32)
//     }

//     // Damage Per Energy (power / energy) only for charged moves
//     pub fn get_dpe(&self) -> f32 {
//         (self.energy_delta.unwrap_or_else(|| 0) as f32) / ((self.duration_turns.unwrap_or_else(|| 0) + 1) as f32)
//     }
// }
