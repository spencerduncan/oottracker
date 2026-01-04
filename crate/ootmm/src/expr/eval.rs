//! Expression evaluator.
//!
//! This module provides the core expression evaluator that ties together the lexer, parser,
//! and built-in functions to evaluate OoTMM condition expressions.

use crate::expr::builtins::{eval_can_use, eval_has};
use crate::expr::{parse, Expr, ParseError};
use thiserror::Error;

/// Error type for expression evaluation.
#[derive(Debug, Error)]
pub enum EvalError {
    /// Generic evaluation error.
    #[error("evaluation error: {0}")]
    Error(String),

    /// Unknown function called.
    #[error("unknown function: {0}")]
    UnknownFunction(String),

    /// Type error during evaluation.
    #[error("type error: expected {expected}, got {got}")]
    TypeError { expected: String, got: String },

    /// Unknown identifier encountered.
    #[error("unknown identifier: {0}")]
    UnknownIdent(String),

    /// Parse error when evaluating a string expression.
    #[error("parse error: {0}")]
    Parse(#[from] ParseError),
}

/// Context for expression evaluation.
///
/// Implement this trait to provide the game state that expressions are evaluated against.
pub trait EvalContext {
    /// Check if the player has at least `count` of the specified item.
    fn has_item(&self, item: &str, count: u32) -> bool;

    /// Check if a game event has occurred.
    fn event(&self, name: &str) -> bool;

    /// Get the value of a boolean setting. Returns `None` if the setting doesn't exist.
    ///
    /// This is used for `setting(name)` expressions that check if a setting is enabled.
    fn setting(&self, name: &str) -> Option<bool>;

    /// Check if a setting has a specific value.
    ///
    /// This is used for `setting(name, value)` expressions that check if a setting
    /// matches a specific value (e.g., `setting(openDungeonsOot, DC)`).
    ///
    /// Default implementation returns false for all queries.
    fn setting_value(&self, name: &str, value: &str) -> bool {
        let _ = (name, value);
        false
    }

    /// Check if a trick is enabled.
    fn trick(&self, name: &str) -> bool;

    /// Check if the player is currently Adult Link.
    fn is_adult(&self) -> bool;

    /// Check if the player is currently Child Link.
    fn is_child(&self) -> bool;

    // MM time-related methods

    /// Get the current MM time as a numeric value.
    /// Time is represented as minutes since the start of Day 1 at 6:00 AM.
    /// Each in-game day is 24 hours (1440 minutes), with the full 3-day cycle
    /// spanning 0 to 4319 (72 hours = 4320 minutes).
    fn mm_time(&self) -> u32;

    /// Check if it's currently daytime (6:00 AM to 6:00 PM).
    fn is_day(&self) -> bool {
        let time_in_day = self.mm_time() % 1440; // Minutes within current day
        time_in_day < 720 // 0-719 is day (6 AM to 6 PM)
    }

    /// Check if it's currently nighttime (6:00 PM to 6:00 AM).
    fn is_night(&self) -> bool {
        !self.is_day()
    }

    // ===== can_* helper macros =====
    // These compose existing has() and is_adult()/is_child() checks
    // to provide convenient helpers for common randomizer logic patterns.

    /// Check if the player has any explosive (bombs or bombchu).
    fn has_explosives(&self) -> bool {
        self.has_item("BOMBS", 1) || self.has_item("BOMB_BAG", 1) || self.has_item("BOMBCHU", 1)
    }

    /// Check if the player can use explosives to blast walls/obstacles.
    /// Requires having explosives (bombs or bombchu).
    fn can_blast(&self) -> bool {
        self.has_explosives()
    }

    /// Check if the player can use a ranged projectile.
    /// Includes bow (adult), slingshot (child), hookshot (adult), and boomerang (child).
    fn can_use_projectile(&self) -> bool {
        // Adult projectiles
        (self.is_adult()
            && (self.has_item("BOW", 1)
                || self.has_item("HOOKSHOT", 1)
                || self.has_item("LONGSHOT", 1)))
            // Child projectiles
            || (self.is_child()
                && (self.has_item("SLINGSHOT", 1) || self.has_item("BOOMERANG", 1)))
        // Any age projectiles (Deku Nuts can stun, but aren't truly projectiles for switches)
    }

    /// Check if the player can stun enemies.
    /// Includes deku nuts, boomerang (child), hookshot (adult), and various other items.
    fn can_stun(&self) -> bool {
        self.has_item("DEKU_NUT", 1)
            || self.has_item("DEKU_NUTS", 1)
            || (self.is_child() && self.has_item("BOOMERANG", 1))
            || (self.is_adult() && (self.has_item("HOOKSHOT", 1) || self.has_item("LONGSHOT", 1)))
    }

    /// Check if the player can use the hookshot.
    /// In OoT, hookshot is adult-only.
    fn can_hookshot(&self) -> bool {
        self.is_adult() && (self.has_item("HOOKSHOT", 1) || self.has_item("LONGSHOT", 1))
    }

    /// Check if the player can use the longshot.
    /// In OoT, longshot is adult-only.
    fn can_longshot(&self) -> bool {
        self.is_adult() && self.has_item("LONGSHOT", 1)
    }

    /// Check if the player can use the boomerang.
    /// In OoT, boomerang is child-only.
    fn can_boomerang(&self) -> bool {
        self.is_child() && self.has_item("BOOMERANG", 1)
    }

    /// Check if the player can use the megaton hammer.
    /// In OoT, hammer is adult-only.
    fn can_hammer(&self) -> bool {
        self.is_adult() && (self.has_item("HAMMER", 1) || self.has_item("MEGATON_HAMMER", 1))
    }

    /// Check if the player can smash things (rusty switches, etc.).
    /// In OoT this is the hammer. In MM this includes Goron pound.
    fn can_smash(&self) -> bool {
        self.can_hammer()
    }

    /// Check if the player can dive underwater.
    /// Requires having a scale (silver or golden).
    fn can_dive(&self) -> bool {
        self.has_item("SCALE", 1)
            || self.has_item("SILVER_SCALE", 1)
            || self.has_item("GOLDEN_SCALE", 1)
            || self.has_item("GOLD_SCALE", 1)
    }

    /// Check if the player can dive deeply underwater.
    /// Requires having the golden scale.
    fn can_dive_deep(&self) -> bool {
        self.has_item("GOLDEN_SCALE", 1) || self.has_item("GOLD_SCALE", 1)
    }

    /// Check if the player can play ocarina songs.
    /// Requires having any ocarina.
    fn can_play(&self) -> bool {
        self.has_item("OCARINA", 1)
            || self.has_item("OCARINA_FAIRY", 1)
            || self.has_item("OCARINA_TIME", 1)
            || self.has_item("OCARINA_OF_TIME", 1)
    }

    /// Check if the player can use a sword.
    /// Varies by age: child uses Kokiri Sword, adult uses Master/Biggoron Sword.
    fn can_use_sword(&self) -> bool {
        (self.is_child() && self.has_item("KOKIRI_SWORD", 1))
            || (self.is_adult()
                && (self.has_item("MASTER_SWORD", 1) || self.has_item("BIGGORON_SWORD", 1)))
    }

    /// Check if the player has fire-based attacks.
    /// Includes Din's Fire and Fire Arrows (with bow and magic).
    fn has_fire(&self) -> bool {
        (self.has_item("DINS_FIRE", 1) || self.has_item("DIN", 1))
            || (self.is_adult()
                && self.has_item("BOW", 1)
                && (self.has_item("FIRE_ARROWS", 1) || self.has_item("FIRE_ARROW", 1)))
    }

    /// Check if the player can light torches.
    /// Includes fire-based attacks, deku sticks, and fire arrows.
    fn can_light_torch(&self) -> bool {
        self.has_fire()
            || self.has_item("DEKU_STICK", 1)
            || self.has_item("DEKU_STICKS", 1)
            || self.has_item("STICKS", 1)
    }

    /// Check if the player can cut grass/bushes/signs/webs.
    /// Includes swords, deku sticks, boomerang (child), and other cutting implements.
    fn can_cut(&self) -> bool {
        self.can_use_sword()
            || self.has_item("DEKU_STICK", 1)
            || self.has_item("DEKU_STICKS", 1)
            || self.has_item("STICKS", 1)
            || self.can_boomerang()
            || self.can_hammer()
    }

    /// Check if child Link can attack enemies.
    /// Includes Kokiri Sword, Deku Sticks, Slingshot, Boomerang, etc.
    fn can_child_attack(&self) -> bool {
        self.is_child()
            && (self.has_item("KOKIRI_SWORD", 1)
                || self.has_item("DEKU_STICK", 1)
                || self.has_item("DEKU_STICKS", 1)
                || self.has_item("STICKS", 1)
                || self.has_item("SLINGSHOT", 1)
                || self.has_item("BOOMERANG", 1)
                || self.has_item("BOMBS", 1)
                || self.has_item("BOMB_BAG", 1)
                || self.has_item("BOMBCHU", 1)
                || self.has_item("DINS_FIRE", 1)
                || self.has_item("DIN", 1))
    }

    /// Check if adult Link can attack enemies.
    /// Includes Master Sword, Biggoron Sword, Bow, Hookshot, Hammer, etc.
    fn can_adult_attack(&self) -> bool {
        self.is_adult()
            && (self.has_item("MASTER_SWORD", 1)
                || self.has_item("BIGGORON_SWORD", 1)
                || self.has_item("BOW", 1)
                || self.has_item("HOOKSHOT", 1)
                || self.has_item("LONGSHOT", 1)
                || self.has_item("HAMMER", 1)
                || self.has_item("MEGATON_HAMMER", 1)
                || self.has_item("BOMBS", 1)
                || self.has_item("BOMB_BAG", 1)
                || self.has_item("BOMBCHU", 1)
                || self.has_item("DINS_FIRE", 1)
                || self.has_item("DIN", 1))
    }

    /// Check if the player can deal damage to enemies.
    /// Combines child and adult attack checks.
    fn can_damage(&self) -> bool {
        self.can_child_attack() || self.can_adult_attack()
    }

    /// Check if the player has magic available.
    fn has_magic(&self) -> bool {
        self.has_item("MAGIC", 1)
            || self.has_item("MAGIC_METER", 1)
            || self.has_item("MAGIC_UPGRADE", 1)
    }

    /// Check if the player can use magic spells.
    /// Requires having magic and at least one spell.
    fn can_use_magic(&self) -> bool {
        self.has_magic()
            && (self.has_item("DINS_FIRE", 1)
                || self.has_item("DIN", 1)
                || self.has_item("FARORES_WIND", 1)
                || self.has_item("FARORE", 1)
                || self.has_item("NAYRUS_LOVE", 1)
                || self.has_item("NAYRU", 1))
    }

    /// Check if the player has any bow (adult only).
    fn can_use_bow(&self) -> bool {
        self.is_adult() && self.has_item("BOW", 1)
    }

    /// Check if the player has the slingshot (child only).
    fn can_use_slingshot(&self) -> bool {
        self.is_child() && self.has_item("SLINGSHOT", 1)
    }

    /// Check if the player can shoot something (bow or slingshot).
    fn can_shoot(&self) -> bool {
        self.can_use_bow() || self.can_use_slingshot()
    }

    /// Check if the player can access water temple areas (iron boots + zora tunic).
    fn can_dive_water_temple(&self) -> bool {
        self.is_adult()
            && (self.has_item("IRON_BOOTS", 1) || self.has_item("BOOTS_IRON", 1))
            && (self.has_item("ZORA_TUNIC", 1) || self.has_item("TUNIC_ZORA", 1))
    }

    /// Check if the player can use iron boots (adult only).
    fn can_use_iron_boots(&self) -> bool {
        self.is_adult() && (self.has_item("IRON_BOOTS", 1) || self.has_item("BOOTS_IRON", 1))
    }

    /// Check if the player can use hover boots (adult only).
    fn can_use_hover_boots(&self) -> bool {
        self.is_adult() && (self.has_item("HOVER_BOOTS", 1) || self.has_item("BOOTS_HOVER", 1))
    }

    /// Check if the player has strength upgrades for lifting/pushing.
    fn has_strength(&self) -> bool {
        self.has_item("STRENGTH", 1)
            || self.has_item("GORON_BRACELET", 1)
            || self.has_item("SILVER_GAUNTLETS", 1)
            || self.has_item("GOLDEN_GAUNTLETS", 1)
            || self.has_item("GOLD_GAUNTLETS", 1)
    }

    /// Check if the player can lift heavy objects (silver gauntlets or better).
    fn can_lift_heavy(&self) -> bool {
        self.has_item("SILVER_GAUNTLETS", 1)
            || self.has_item("GOLDEN_GAUNTLETS", 1)
            || self.has_item("GOLD_GAUNTLETS", 1)
    }

    /// Check if the player can lift the heaviest objects (golden gauntlets).
    fn can_lift_heaviest(&self) -> bool {
        self.has_item("GOLDEN_GAUNTLETS", 1) || self.has_item("GOLD_GAUNTLETS", 1)
    }

    // ===== MM-specific helpers =====
    // These are primarily useful for MM contexts but defined on the trait
    // with sensible defaults for OoT contexts.

    /// Check if the player has the Deku Mask (MM transformation).
    fn has_mask_deku(&self) -> bool {
        self.has_item("MASK_DEKU", 1)
    }

    /// Check if the player has the Goron Mask (MM transformation).
    fn has_mask_goron(&self) -> bool {
        self.has_item("MASK_GORON", 1)
    }

    /// Check if the player has the Zora Mask (MM transformation).
    fn has_mask_zora(&self) -> bool {
        self.has_item("MASK_ZORA", 1)
    }

    /// Check if the player has the Fierce Deity Mask (MM transformation).
    fn has_mask_fierce_deity(&self) -> bool {
        self.has_item("MASK_FIERCE_DEITY", 1)
    }

    /// Check if the player can use the powder keg (MM, requires Goron + license).
    fn can_use_powder_keg(&self) -> bool {
        self.has_mask_goron() && self.has_item("POWDER_KEG", 1)
    }

    /// Check if the player can swim fast (MM Zora mask).
    fn can_swim_fast(&self) -> bool {
        self.has_mask_zora()
    }

    /// Check if the player can roll fast (MM Goron mask).
    fn can_roll_fast(&self) -> bool {
        self.has_mask_goron()
    }

    /// Check if the player can fly (MM Deku mask + flower).
    fn can_fly_deku(&self) -> bool {
        self.has_mask_deku()
    }
}

/// Expression evaluator that ties together lexer, parser, and built-in functions.
///
/// The `Evaluator` provides the core evaluation logic for OoTMM condition expressions.
/// It recursively evaluates AST nodes against a provided context.
///
/// # Example
///
/// ```ignore
/// use ootmm::expr::{Evaluator, EvalContext};
///
/// let evaluator = Evaluator::new(&ctx);
/// let result = evaluator.eval_str("has(HOOKSHOT) && is_adult")?;
/// ```
pub struct Evaluator<'a, C: EvalContext> {
    ctx: &'a C,
}

impl<'a, C: EvalContext> Evaluator<'a, C> {
    /// Create a new evaluator with the given context.
    pub fn new(ctx: &'a C) -> Self {
        Self { ctx }
    }

    /// Parse and evaluate an expression string.
    ///
    /// This is a convenience method that combines parsing and evaluation.
    pub fn eval_str(&self, input: &str) -> Result<bool, EvalError> {
        let expr = parse(input)?;
        self.eval(&expr)
    }

    /// Evaluate an expression AST node.
    ///
    /// Recursively evaluates the expression tree against the context.
    pub fn eval(&self, expr: &Expr) -> Result<bool, EvalError> {
        match expr {
            Expr::Bool(b) => Ok(*b),

            Expr::Number(n) => {
                // Non-zero numbers are truthy
                Ok(*n != 0)
            }

            Expr::String(s) => {
                // Non-empty strings are truthy
                Ok(!s.is_empty())
            }

            Expr::Ident(name) => self.eval_ident(name),

            Expr::And(left, right) => {
                // Short-circuit AND: only evaluate right if left is true
                if self.eval(left)? {
                    self.eval(right)
                } else {
                    Ok(false)
                }
            }

            Expr::Or(left, right) => {
                // Short-circuit OR: only evaluate right if left is false
                if self.eval(left)? {
                    Ok(true)
                } else {
                    self.eval(right)
                }
            }

            Expr::Not(inner) => Ok(!self.eval(inner)?),

            Expr::Call { name, args } => self.eval_call(name, args),
        }
    }

    /// Evaluate an identifier.
    ///
    /// Special identifiers are handled directly:
    /// - `is_adult`: true if the player is Adult Link
    /// - `is_child`: true if the player is Child Link
    /// - `is_human`: always true (player can always be in human form in MM tracker)
    /// - `is_day`: true if it's daytime (6 AM - 6 PM)
    /// - `is_night`: true if it's nighttime (6 PM - 6 AM)
    /// - `true`: always true
    /// - `false`: always false
    /// - can_* helpers: various helper macros
    fn eval_ident(&self, name: &str) -> Result<bool, EvalError> {
        match name {
            // Core identifiers
            "is_adult" => Ok(self.ctx.is_adult()),
            "is_child" => Ok(self.ctx.is_child()),
            "is_human" => Ok(true),
            "is_day" => Ok(self.ctx.is_day()),
            "is_night" => Ok(self.ctx.is_night()),
            "true" => Ok(true),
            "false" => Ok(false),

            // can_* helper macros (can be used without parentheses)
            "has_explosives" => Ok(self.ctx.has_explosives()),
            "can_blast" => Ok(self.ctx.can_blast()),
            "can_use_projectile" => Ok(self.ctx.can_use_projectile()),
            "can_stun" => Ok(self.ctx.can_stun()),
            "can_hookshot" => Ok(self.ctx.can_hookshot()),
            "can_longshot" => Ok(self.ctx.can_longshot()),
            "can_boomerang" => Ok(self.ctx.can_boomerang()),
            "can_hammer" => Ok(self.ctx.can_hammer()),
            "can_smash" => Ok(self.ctx.can_smash()),
            "can_dive" => Ok(self.ctx.can_dive()),
            "can_dive_deep" => Ok(self.ctx.can_dive_deep()),
            "can_play" => Ok(self.ctx.can_play()),
            "can_use_sword" => Ok(self.ctx.can_use_sword()),
            "has_fire" => Ok(self.ctx.has_fire()),
            "can_light_torch" => Ok(self.ctx.can_light_torch()),
            "can_cut" => Ok(self.ctx.can_cut()),
            "can_child_attack" => Ok(self.ctx.can_child_attack()),
            "can_adult_attack" => Ok(self.ctx.can_adult_attack()),
            "can_damage" => Ok(self.ctx.can_damage()),
            "has_magic" => Ok(self.ctx.has_magic()),
            "can_use_magic" => Ok(self.ctx.can_use_magic()),
            "can_use_bow" => Ok(self.ctx.can_use_bow()),
            "can_use_slingshot" => Ok(self.ctx.can_use_slingshot()),
            "can_shoot" => Ok(self.ctx.can_shoot()),
            "can_dive_water_temple" => Ok(self.ctx.can_dive_water_temple()),
            "can_use_iron_boots" => Ok(self.ctx.can_use_iron_boots()),
            "can_use_hover_boots" => Ok(self.ctx.can_use_hover_boots()),
            "has_strength" => Ok(self.ctx.has_strength()),
            "can_lift_heavy" => Ok(self.ctx.can_lift_heavy()),
            "can_lift_heaviest" => Ok(self.ctx.can_lift_heaviest()),
            // MM-specific helpers
            "has_mask_deku" => Ok(self.ctx.has_mask_deku()),
            "has_mask_goron" => Ok(self.ctx.has_mask_goron()),
            "has_mask_zora" => Ok(self.ctx.has_mask_zora()),
            "has_mask_fierce_deity" => Ok(self.ctx.has_mask_fierce_deity()),
            "can_use_powder_keg" => Ok(self.ctx.can_use_powder_keg()),
            "can_swim_fast" => Ok(self.ctx.can_swim_fast()),
            "can_roll_fast" => Ok(self.ctx.can_roll_fast()),
            "can_fly_deku" => Ok(self.ctx.can_fly_deku()),

            _ => {
                // Check if it's an event or setting
                if self.ctx.event(name) {
                    return Ok(true);
                }
                if let Some(val) = self.ctx.setting(name) {
                    return Ok(val);
                }
                // Unknown identifier - could be an item check shorthand
                // For now, treat unknown identifiers as events that haven't occurred
                Ok(false)
            }
        }
    }

    /// Evaluate a function call.
    fn eval_call(&self, name: &str, args: &[Expr]) -> Result<bool, EvalError> {
        match name {
            "has" => eval_has(args, self.ctx),
            "can_use" => eval_can_use(args, self.ctx),
            "event" => self.eval_event(args),
            "setting" => self.eval_setting(args),
            "trick" => self.eval_trick(args),
            "cond" => self.eval_cond(args),
            // can_* helper macros - no arguments
            "has_explosives" => {
                self.eval_no_args(args, "has_explosives", || self.ctx.has_explosives())
            }
            "can_blast" => self.eval_no_args(args, "can_blast", || self.ctx.can_blast()),
            "can_use_projectile" => {
                self.eval_no_args(args, "can_use_projectile", || self.ctx.can_use_projectile())
            }
            "can_stun" => self.eval_no_args(args, "can_stun", || self.ctx.can_stun()),
            "can_hookshot" => self.eval_no_args(args, "can_hookshot", || self.ctx.can_hookshot()),
            "can_longshot" => self.eval_no_args(args, "can_longshot", || self.ctx.can_longshot()),
            "can_boomerang" => {
                self.eval_no_args(args, "can_boomerang", || self.ctx.can_boomerang())
            }
            "can_hammer" => self.eval_no_args(args, "can_hammer", || self.ctx.can_hammer()),
            "can_smash" => self.eval_no_args(args, "can_smash", || self.ctx.can_smash()),
            "can_dive" => self.eval_no_args(args, "can_dive", || self.ctx.can_dive()),
            "can_dive_deep" => {
                self.eval_no_args(args, "can_dive_deep", || self.ctx.can_dive_deep())
            }
            "can_play" => self.eval_no_args(args, "can_play", || self.ctx.can_play()),
            "can_use_sword" => {
                self.eval_no_args(args, "can_use_sword", || self.ctx.can_use_sword())
            }
            "has_fire" => self.eval_no_args(args, "has_fire", || self.ctx.has_fire()),
            "can_light_torch" => {
                self.eval_no_args(args, "can_light_torch", || self.ctx.can_light_torch())
            }
            "can_cut" => self.eval_no_args(args, "can_cut", || self.ctx.can_cut()),
            "can_child_attack" => {
                self.eval_no_args(args, "can_child_attack", || self.ctx.can_child_attack())
            }
            "can_adult_attack" => {
                self.eval_no_args(args, "can_adult_attack", || self.ctx.can_adult_attack())
            }
            "can_damage" => self.eval_no_args(args, "can_damage", || self.ctx.can_damage()),
            "has_magic" => self.eval_no_args(args, "has_magic", || self.ctx.has_magic()),
            "can_use_magic" => {
                self.eval_no_args(args, "can_use_magic", || self.ctx.can_use_magic())
            }
            "can_use_bow" => self.eval_no_args(args, "can_use_bow", || self.ctx.can_use_bow()),
            "can_use_slingshot" => {
                self.eval_no_args(args, "can_use_slingshot", || self.ctx.can_use_slingshot())
            }
            "can_shoot" => self.eval_no_args(args, "can_shoot", || self.ctx.can_shoot()),
            "can_dive_water_temple" => self.eval_no_args(args, "can_dive_water_temple", || {
                self.ctx.can_dive_water_temple()
            }),
            "can_use_iron_boots" => {
                self.eval_no_args(args, "can_use_iron_boots", || self.ctx.can_use_iron_boots())
            }
            "can_use_hover_boots" => self.eval_no_args(args, "can_use_hover_boots", || {
                self.ctx.can_use_hover_boots()
            }),
            "has_strength" => self.eval_no_args(args, "has_strength", || self.ctx.has_strength()),
            "can_lift_heavy" => {
                self.eval_no_args(args, "can_lift_heavy", || self.ctx.can_lift_heavy())
            }
            "can_lift_heaviest" => {
                self.eval_no_args(args, "can_lift_heaviest", || self.ctx.can_lift_heaviest())
            }
            // MM-specific helpers
            "has_mask_deku" => {
                self.eval_no_args(args, "has_mask_deku", || self.ctx.has_mask_deku())
            }
            "has_mask_goron" => {
                self.eval_no_args(args, "has_mask_goron", || self.ctx.has_mask_goron())
            }
            "has_mask_zora" => {
                self.eval_no_args(args, "has_mask_zora", || self.ctx.has_mask_zora())
            }
            "has_mask_fierce_deity" => self.eval_no_args(args, "has_mask_fierce_deity", || {
                self.ctx.has_mask_fierce_deity()
            }),
            "can_use_powder_keg" => {
                self.eval_no_args(args, "can_use_powder_keg", || self.ctx.can_use_powder_keg())
            }
            "can_swim_fast" => {
                self.eval_no_args(args, "can_swim_fast", || self.ctx.can_swim_fast())
            }
            "can_roll_fast" => {
                self.eval_no_args(args, "can_roll_fast", || self.ctx.can_roll_fast())
            }
            "can_fly_deku" => self.eval_no_args(args, "can_fly_deku", || self.ctx.can_fly_deku()),
            _ => Err(EvalError::UnknownFunction(name.to_string())),
        }
    }

    /// Evaluate a no-argument helper function.
    fn eval_no_args<F>(&self, args: &[Expr], name: &str, f: F) -> Result<bool, EvalError>
    where
        F: FnOnce() -> bool,
    {
        if !args.is_empty() {
            return Err(EvalError::Error(format!(
                "{}() expects 0 arguments, got {}",
                name,
                args.len()
            )));
        }
        Ok(f())
    }

    /// Evaluate the `event` built-in function.
    ///
    /// Checks if a game event has occurred.
    ///
    /// # Syntax
    /// - `event(EVENT_NAME)` - true if the event has occurred
    fn eval_event(&self, args: &[Expr]) -> Result<bool, EvalError> {
        if args.len() != 1 {
            return Err(EvalError::Error(format!(
                "event() expects 1 argument, got {}",
                args.len()
            )));
        }

        let event_name = self.extract_name(&args[0])?;
        Ok(self.ctx.event(&event_name))
    }

    /// Evaluate the `setting` built-in function.
    ///
    /// Checks the value of a game setting.
    ///
    /// # Syntax
    /// - `setting(SETTING_NAME)` - true if the boolean setting is enabled
    /// - `setting(SETTING_NAME, VALUE)` - true if the setting has the specified value
    fn eval_setting(&self, args: &[Expr]) -> Result<bool, EvalError> {
        match args.len() {
            1 => {
                // Boolean setting check: setting(name)
                let setting_name = self.extract_name(&args[0])?;
                Ok(self.ctx.setting(&setting_name).unwrap_or(false))
            }
            2 => {
                // Value setting check: setting(name, value)
                let setting_name = self.extract_name(&args[0])?;
                let setting_value = self.extract_name(&args[1])?;
                Ok(self.ctx.setting_value(&setting_name, &setting_value))
            }
            _ => Err(EvalError::Error(format!(
                "setting() expects 1 or 2 arguments, got {}",
                args.len()
            ))),
        }
    }

    /// Evaluate the `trick` built-in function.
    ///
    /// Checks if a trick is enabled.
    ///
    /// # Syntax
    /// - `trick(TRICK_NAME)` - true if the trick is enabled
    fn eval_trick(&self, args: &[Expr]) -> Result<bool, EvalError> {
        if args.len() != 1 {
            return Err(EvalError::Error(format!(
                "trick() expects 1 argument, got {}",
                args.len()
            )));
        }

        let trick_name = self.extract_name(&args[0])?;
        Ok(self.ctx.trick(&trick_name))
    }

    /// Evaluate the `cond` built-in function.
    ///
    /// Conditional expression: `cond(condition, then_value, else_value)`
    ///
    /// # Syntax
    /// - `cond(test, if_true, if_false)` - returns if_true if test is true, else if_false
    fn eval_cond(&self, args: &[Expr]) -> Result<bool, EvalError> {
        if args.len() != 3 {
            return Err(EvalError::Error(format!(
                "cond() expects 3 arguments, got {}",
                args.len()
            )));
        }

        if self.eval(&args[0])? {
            self.eval(&args[1])
        } else {
            self.eval(&args[2])
        }
    }

    /// Extract a name (identifier or string) from an expression.
    fn extract_name(&self, expr: &Expr) -> Result<String, EvalError> {
        match expr {
            Expr::Ident(name) => Ok(name.clone()),
            Expr::String(s) => Ok(s.clone()),
            _ => Err(EvalError::TypeError {
                expected: "identifier or string".to_string(),
                got: format!("{:?}", expr),
            }),
        }
    }
}

/// Evaluate an expression against a context.
///
/// This is a convenience function that creates an `Evaluator` and evaluates the expression.
pub fn eval(expr: &Expr, ctx: &impl EvalContext) -> Result<bool, EvalError> {
    let evaluator = Evaluator::new(ctx);
    evaluator.eval(expr)
}

/// Parse and evaluate an expression string against a context.
///
/// This is a convenience function that combines parsing and evaluation.
pub fn eval_str(input: &str, ctx: &impl EvalContext) -> Result<bool, EvalError> {
    let evaluator = Evaluator::new(ctx);
    evaluator.eval_str(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    /// Mock context for testing the evaluator.
    struct MockContext {
        items: HashMap<String, u32>,
        events: HashSet<String>,
        settings: HashMap<String, bool>,
        /// Settings with values: key is "settingName:value", value is true if that combination is valid
        setting_values: HashSet<String>,
        tricks: HashSet<String>,
        is_adult: bool,
    }

    impl MockContext {
        fn new() -> Self {
            Self {
                items: HashMap::new(),
                events: HashSet::new(),
                settings: HashMap::new(),
                setting_values: HashSet::new(),
                tricks: HashSet::new(),
                is_adult: true,
            }
        }

        fn with_item(mut self, item: &str, count: u32) -> Self {
            self.items.insert(item.to_uppercase(), count);
            self
        }

        fn with_event(mut self, event: &str) -> Self {
            self.events.insert(event.to_string());
            self
        }

        fn with_setting(mut self, setting: &str, value: bool) -> Self {
            self.settings.insert(setting.to_string(), value);
            self
        }

        /// Sets a setting to have a specific value (for 2-argument setting checks).
        fn with_setting_value(mut self, setting: &str, value: &str) -> Self {
            self.setting_values.insert(format!("{}:{}", setting, value));
            self
        }

        fn with_trick(mut self, trick: &str) -> Self {
            self.tricks.insert(trick.to_string());
            self
        }

        fn with_child_age(mut self) -> Self {
            self.is_adult = false;
            self
        }

        fn with_adult_age(mut self) -> Self {
            self.is_adult = true;
            self
        }
    }

    impl EvalContext for MockContext {
        fn has_item(&self, item: &str, count: u32) -> bool {
            self.items
                .get(&item.to_uppercase())
                .map(|&c| c >= count)
                .unwrap_or(false)
        }

        fn event(&self, name: &str) -> bool {
            self.events.contains(name)
        }

        fn setting(&self, name: &str) -> Option<bool> {
            self.settings.get(name).copied()
        }

        fn setting_value(&self, name: &str, value: &str) -> bool {
            self.setting_values.contains(&format!("{}:{}", name, value))
        }

        fn trick(&self, name: &str) -> bool {
            self.tricks.contains(name)
        }

        fn is_adult(&self) -> bool {
            self.is_adult
        }

        fn is_child(&self) -> bool {
            !self.is_adult
        }

        fn mm_time(&self) -> u32 {
            0 // Default to time 0 for tests
        }
    }

    // --- Boolean literal tests ---

    #[test]
    fn test_eval_true() {
        let ctx = MockContext::new();
        assert!(eval_str("true", &ctx).unwrap());
    }

    #[test]
    fn test_eval_false() {
        let ctx = MockContext::new();
        assert!(!eval_str("false", &ctx).unwrap());
    }

    // --- Identifier tests ---

    #[test]
    fn test_eval_is_adult() {
        let ctx = MockContext::new().with_adult_age();
        assert!(eval_str("is_adult", &ctx).unwrap());

        let ctx = MockContext::new().with_child_age();
        assert!(!eval_str("is_adult", &ctx).unwrap());
    }

    #[test]
    fn test_eval_is_child() {
        let ctx = MockContext::new().with_child_age();
        assert!(eval_str("is_child", &ctx).unwrap());

        let ctx = MockContext::new().with_adult_age();
        assert!(!eval_str("is_child", &ctx).unwrap());
    }

    #[test]
    fn test_eval_is_human() {
        // is_human always returns true for MM tracker purposes
        let ctx = MockContext::new();
        assert!(eval_str("is_human", &ctx).unwrap());

        // Should work in combination with other conditions
        let ctx = MockContext::new().with_item("HOOKSHOT", 1);
        assert!(eval_str("is_human && has(HOOKSHOT)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_unknown_ident() {
        let ctx = MockContext::new();
        // Unknown identifiers default to false
        assert!(!eval_str("unknown_thing", &ctx).unwrap());
    }

    // --- Logical operator tests ---

    #[test]
    fn test_eval_and_true_true() {
        let ctx = MockContext::new();
        assert!(eval_str("true && true", &ctx).unwrap());
    }

    #[test]
    fn test_eval_and_true_false() {
        let ctx = MockContext::new();
        assert!(!eval_str("true && false", &ctx).unwrap());
    }

    #[test]
    fn test_eval_and_false_true() {
        let ctx = MockContext::new();
        assert!(!eval_str("false && true", &ctx).unwrap());
    }

    #[test]
    fn test_eval_and_false_false() {
        let ctx = MockContext::new();
        assert!(!eval_str("false && false", &ctx).unwrap());
    }

    #[test]
    fn test_eval_or_true_true() {
        let ctx = MockContext::new();
        assert!(eval_str("true || true", &ctx).unwrap());
    }

    #[test]
    fn test_eval_or_true_false() {
        let ctx = MockContext::new();
        assert!(eval_str("true || false", &ctx).unwrap());
    }

    #[test]
    fn test_eval_or_false_true() {
        let ctx = MockContext::new();
        assert!(eval_str("false || true", &ctx).unwrap());
    }

    #[test]
    fn test_eval_or_false_false() {
        let ctx = MockContext::new();
        assert!(!eval_str("false || false", &ctx).unwrap());
    }

    #[test]
    fn test_eval_not_true() {
        let ctx = MockContext::new();
        assert!(!eval_str("!true", &ctx).unwrap());
    }

    #[test]
    fn test_eval_not_false() {
        let ctx = MockContext::new();
        assert!(eval_str("!false", &ctx).unwrap());
    }

    #[test]
    fn test_eval_double_not() {
        let ctx = MockContext::new();
        assert!(eval_str("!!true", &ctx).unwrap());
        assert!(!eval_str("!!false", &ctx).unwrap());
    }

    // --- Short-circuit evaluation tests ---

    #[test]
    fn test_and_short_circuit() {
        // When left side is false, right side should not be evaluated
        let ctx = MockContext::new();
        // This would fail if unknown_func was evaluated, but it shouldn't be
        // because false && anything = false
        let expr = parse("false && unknown_func()").unwrap();
        assert!(!eval(&expr, &ctx).unwrap());
    }

    #[test]
    fn test_or_short_circuit() {
        // When left side is true, right side should not be evaluated
        let ctx = MockContext::new();
        // This would fail if unknown_func was evaluated, but it shouldn't be
        // because true || anything = true
        let expr = parse("true || unknown_func()").unwrap();
        assert!(eval(&expr, &ctx).unwrap());
    }

    // --- has() function tests ---

    #[test]
    fn test_eval_has_item() {
        let ctx = MockContext::new().with_item("HOOKSHOT", 1);
        assert!(eval_str("has(HOOKSHOT)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_has_missing_item() {
        let ctx = MockContext::new();
        assert!(!eval_str("has(HOOKSHOT)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_has_with_count() {
        let ctx = MockContext::new().with_item("BOMBS", 20);
        assert!(eval_str("has(BOMBS, 10)", &ctx).unwrap());
        assert!(eval_str("has(BOMBS, 20)", &ctx).unwrap());
        assert!(!eval_str("has(BOMBS, 30)", &ctx).unwrap());
    }

    // --- can_use() function tests ---

    #[test]
    fn test_eval_can_use_adult_item_as_adult() {
        let ctx = MockContext::new().with_item("HOOKSHOT", 1).with_adult_age();
        assert!(eval_str("can_use(HOOKSHOT)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_can_use_adult_item_as_child() {
        let ctx = MockContext::new().with_item("HOOKSHOT", 1).with_child_age();
        assert!(!eval_str("can_use(HOOKSHOT)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_can_use_child_item_as_child() {
        let ctx = MockContext::new()
            .with_item("BOOMERANG", 1)
            .with_child_age();
        assert!(eval_str("can_use(BOOMERANG)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_can_use_child_item_as_adult() {
        let ctx = MockContext::new()
            .with_item("BOOMERANG", 1)
            .with_adult_age();
        assert!(!eval_str("can_use(BOOMERANG)", &ctx).unwrap());
    }

    // --- event() function tests ---

    #[test]
    fn test_eval_event_occurred() {
        let ctx = MockContext::new().with_event("MIDO_MOVED");
        assert!(eval_str("event(MIDO_MOVED)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_event_not_occurred() {
        let ctx = MockContext::new();
        assert!(!eval_str("event(MIDO_MOVED)", &ctx).unwrap());
    }

    // --- setting() function tests ---

    #[test]
    fn test_eval_setting_enabled() {
        let ctx = MockContext::new().with_setting("skip_child_zelda", true);
        assert!(eval_str("setting(skip_child_zelda)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_setting_disabled() {
        let ctx = MockContext::new().with_setting("skip_child_zelda", false);
        assert!(!eval_str("setting(skip_child_zelda)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_setting_missing() {
        let ctx = MockContext::new();
        assert!(!eval_str("setting(nonexistent)", &ctx).unwrap());
    }

    // --- trick() function tests ---

    #[test]
    fn test_eval_trick_enabled() {
        let ctx = MockContext::new().with_trick("hover_boost");
        assert!(eval_str("trick(hover_boost)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_trick_disabled() {
        let ctx = MockContext::new();
        assert!(!eval_str("trick(hover_boost)", &ctx).unwrap());
    }

    // --- cond() function tests ---

    #[test]
    fn test_eval_cond_true() {
        let ctx = MockContext::new();
        assert!(eval_str("cond(true, true, false)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_cond_false() {
        let ctx = MockContext::new();
        assert!(!eval_str("cond(false, true, false)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_cond_with_expressions() {
        let ctx = MockContext::new()
            .with_item("HOOKSHOT", 1)
            .with_event("BOSS_DEFEATED");
        assert!(eval_str("cond(has(HOOKSHOT), event(BOSS_DEFEATED), false)", &ctx).unwrap());
    }

    // --- Complex expression tests ---

    #[test]
    fn test_eval_complex_and_or() {
        let ctx = MockContext::new()
            .with_item("HOOKSHOT", 1)
            .with_item("BOW", 1);

        assert!(eval_str("has(HOOKSHOT) && has(BOW)", &ctx).unwrap());
        assert!(eval_str("has(HOOKSHOT) || has(BOMBS)", &ctx).unwrap());
        assert!(!eval_str("has(HOOKSHOT) && has(BOMBS)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_complex_with_not() {
        let ctx = MockContext::new().with_item("HOOKSHOT", 1);

        assert!(eval_str("!has(BOMBS)", &ctx).unwrap());
        assert!(!eval_str("!has(HOOKSHOT)", &ctx).unwrap());
        assert!(eval_str("has(HOOKSHOT) && !has(BOMBS)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_nested_parentheses() {
        let ctx = MockContext::new()
            .with_item("HOOKSHOT", 1)
            .with_event("MIDO_MOVED");

        assert!(eval_str("(has(HOOKSHOT) && event(MIDO_MOVED))", &ctx).unwrap());
        assert!(eval_str("((has(HOOKSHOT)))", &ctx).unwrap());
    }

    #[test]
    fn test_eval_real_world_expression() {
        // A realistic OoTMM logic expression
        let ctx = MockContext::new()
            .with_item("HOOKSHOT", 1)
            .with_adult_age()
            .with_event("FOREST_TEMPLE_CLEAR");

        assert!(eval_str(
            "is_adult && has(HOOKSHOT) && event(FOREST_TEMPLE_CLEAR)",
            &ctx
        )
        .unwrap());
    }

    #[test]
    fn test_eval_or_with_setting() {
        let ctx = MockContext::new()
            .with_setting("skip_child_zelda", true)
            .with_event("MET_ZELDA");

        assert!(eval_str("event(MET_ZELDA) || setting(skip_child_zelda)", &ctx).unwrap());

        let ctx2 = MockContext::new().with_setting("skip_child_zelda", true);
        assert!(eval_str("event(MET_ZELDA) || setting(skip_child_zelda)", &ctx2).unwrap());

        let ctx3 = MockContext::new();
        assert!(!eval_str("event(MET_ZELDA) || setting(skip_child_zelda)", &ctx3).unwrap());
    }

    // --- Error handling tests ---

    #[test]
    fn test_eval_unknown_function() {
        let ctx = MockContext::new();
        let result = eval_str("unknown_func()", &ctx);
        assert!(matches!(result, Err(EvalError::UnknownFunction(_))));
    }

    #[test]
    fn test_eval_event_wrong_args() {
        let ctx = MockContext::new();
        let result = eval_str("event()", &ctx);
        assert!(result.is_err());

        let result = eval_str("event(a, b)", &ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_eval_setting_wrong_args() {
        let ctx = MockContext::new();
        let result = eval_str("setting()", &ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_eval_trick_wrong_args() {
        let ctx = MockContext::new();
        let result = eval_str("trick()", &ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_eval_cond_wrong_args() {
        let ctx = MockContext::new();
        let result = eval_str("cond(true, false)", &ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_eval_parse_error() {
        let ctx = MockContext::new();
        let result = eval_str("has(HOOKSHOT &&", &ctx);
        assert!(matches!(result, Err(EvalError::Parse(_))));
    }

    // --- Evaluator struct tests ---

    #[test]
    fn test_evaluator_new() {
        let ctx = MockContext::new();
        let evaluator = Evaluator::new(&ctx);
        assert!(evaluator.eval_str("true").unwrap());
    }

    #[test]
    fn test_evaluator_eval_expr() {
        let ctx = MockContext::new().with_item("HOOKSHOT", 1);
        let evaluator = Evaluator::new(&ctx);
        let expr = parse("has(HOOKSHOT)").unwrap();
        assert!(evaluator.eval(&expr).unwrap());
    }

    // --- Number and String truthiness tests ---

    #[test]
    fn test_eval_number_truthiness() {
        let ctx = MockContext::new();
        let evaluator = Evaluator::new(&ctx);

        // Non-zero numbers are truthy
        assert!(evaluator.eval(&Expr::Number(1)).unwrap());
        assert!(evaluator.eval(&Expr::Number(42)).unwrap());
        assert!(evaluator.eval(&Expr::Number(-1)).unwrap());

        // Zero is falsy
        assert!(!evaluator.eval(&Expr::Number(0)).unwrap());
    }

    #[test]
    fn test_eval_string_truthiness() {
        let ctx = MockContext::new();
        let evaluator = Evaluator::new(&ctx);

        // Non-empty strings are truthy
        assert!(evaluator.eval(&Expr::String("hello".to_string())).unwrap());
        assert!(evaluator.eval(&Expr::String(" ".to_string())).unwrap());

        // Empty string is falsy
        assert!(!evaluator.eval(&Expr::String("".to_string())).unwrap());
    }

    // --- Integration tests ---

    #[test]
    fn test_full_evaluation_pipeline() {
        // Test the complete pipeline: string -> parse -> eval
        let ctx = MockContext::new()
            .with_item("HOOKSHOT", 1)
            .with_item("BOW", 1)
            .with_adult_age()
            .with_event("FOREST_TEMPLE_CLEAR")
            .with_setting("shuffle_songs", true);

        let expressions = vec![
            ("true", true),
            ("false", false),
            ("is_adult", true),
            ("is_child", false),
            ("has(HOOKSHOT)", true),
            ("has(BOMBS)", false),
            ("can_use(HOOKSHOT)", true),
            ("event(FOREST_TEMPLE_CLEAR)", true),
            ("setting(shuffle_songs)", true),
            ("has(HOOKSHOT) && has(BOW) && is_adult", true),
            ("event(FOREST_TEMPLE_CLEAR) || has(BOMBS)", true),
            ("!(has(BOMBS))", true),
        ];

        for (expr_str, expected) in expressions {
            let result = eval_str(expr_str, &ctx).unwrap();
            assert_eq!(result, expected, "Expression '{}' failed", expr_str);
        }
    }

    // --- Two-argument setting() tests ---

    #[test]
    fn test_eval_setting_two_args_matches() {
        let ctx = MockContext::new()
            .with_setting_value("openDungeonsOot", "DC")
            .with_setting_value("openDungeonsOot", "BotW");

        assert!(eval_str("setting(openDungeonsOot, DC)", &ctx).unwrap());
        assert!(eval_str("setting(openDungeonsOot, BotW)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_setting_two_args_no_match() {
        let ctx = MockContext::new().with_setting_value("openDungeonsOot", "DC");

        assert!(!eval_str("setting(openDungeonsOot, Shadow)", &ctx).unwrap());
        assert!(!eval_str("setting(openDungeonsMm, ST)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_setting_two_args_enum_value() {
        let ctx = MockContext::new()
            .with_setting_value("ganonBossKey", "removed")
            .with_setting_value("ageChange", "none");

        assert!(eval_str("setting(ganonBossKey, removed)", &ctx).unwrap());
        assert!(!eval_str("setting(ganonBossKey, custom)", &ctx).unwrap());
        assert!(eval_str("setting(ageChange, none)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_setting_two_args_in_complex_expr() {
        let ctx = MockContext::new()
            .with_setting_value("openDungeonsOot", "DC")
            .with_setting_value("beneathWell", "open")
            .with_item("HOOKSHOT", 1);

        // Test setting in AND expression
        assert!(eval_str("setting(openDungeonsOot, DC) && has(HOOKSHOT)", &ctx).unwrap());
        assert!(!eval_str("setting(openDungeonsOot, Shadow) && has(HOOKSHOT)", &ctx).unwrap());

        // Test setting in OR expression
        assert!(eval_str("setting(beneathWell, open) || has(BOMBS)", &ctx).unwrap());

        // Test negated setting
        assert!(eval_str("!setting(openDungeonsOot, Shadow)", &ctx).unwrap());
        assert!(!eval_str("!setting(openDungeonsOot, DC)", &ctx).unwrap());
    }

    #[test]
    fn test_eval_setting_two_args_like_world_data() {
        // Test expressions that match patterns from real world data files
        let ctx = MockContext::new()
            .with_setting_value("climbMostSurfacesOot", "off")
            .with_setting_value("hookshotAnywhereOot", "off")
            .with_setting_value("ageChange", "none");

        // Pattern: !setting(climbMostSurfacesOot, off)
        assert!(!eval_str("!setting(climbMostSurfacesOot, off)", &ctx).unwrap());

        // Pattern: !setting(hookshotAnywhereOot, off) && !setting(ageChange, none)
        assert!(!eval_str(
            "!setting(hookshotAnywhereOot, off) && !setting(ageChange, none)",
            &ctx
        )
        .unwrap());
    }

    #[test]
    fn test_eval_setting_mixed_one_and_two_args() {
        let ctx = MockContext::new()
            .with_setting("agelessBoots", true)
            .with_setting_value("openDungeonsOot", "DC");

        // Mix of 1-arg and 2-arg settings
        assert!(eval_str(
            "setting(agelessBoots) && setting(openDungeonsOot, DC)",
            &ctx
        )
        .unwrap());
        assert!(!eval_str(
            "setting(agelessBoots) && setting(openDungeonsOot, Shadow)",
            &ctx
        )
        .unwrap());
    }

    #[test]
    fn test_eval_setting_three_args_error() {
        let ctx = MockContext::new();
        let result = eval_str("setting(a, b, c)", &ctx);
        assert!(result.is_err());
    }

    // ===== can_* helper macro tests =====

    // --- has_explosives / can_blast tests ---

    #[test]
    fn test_has_explosives_with_bombs() {
        let ctx = MockContext::new().with_item("BOMBS", 1);
        assert!(eval_str("has_explosives", &ctx).unwrap());
        assert!(eval_str("has_explosives()", &ctx).unwrap());
        assert!(eval_str("can_blast", &ctx).unwrap());
        assert!(eval_str("can_blast()", &ctx).unwrap());
    }

    #[test]
    fn test_has_explosives_with_bombchu() {
        let ctx = MockContext::new().with_item("BOMBCHU", 1);
        assert!(eval_str("has_explosives", &ctx).unwrap());
        assert!(eval_str("can_blast", &ctx).unwrap());
    }

    #[test]
    fn test_has_explosives_none() {
        let ctx = MockContext::new();
        assert!(!eval_str("has_explosives", &ctx).unwrap());
        assert!(!eval_str("can_blast", &ctx).unwrap());
    }

    // --- can_use_projectile tests ---

    #[test]
    fn test_can_use_projectile_adult_bow() {
        let ctx = MockContext::new().with_item("BOW", 1).with_adult_age();
        assert!(eval_str("can_use_projectile", &ctx).unwrap());
        assert!(eval_str("can_use_projectile()", &ctx).unwrap());
    }

    #[test]
    fn test_can_use_projectile_adult_hookshot() {
        let ctx = MockContext::new().with_item("HOOKSHOT", 1).with_adult_age();
        assert!(eval_str("can_use_projectile", &ctx).unwrap());
    }

    #[test]
    fn test_can_use_projectile_child_slingshot() {
        let ctx = MockContext::new()
            .with_item("SLINGSHOT", 1)
            .with_child_age();
        assert!(eval_str("can_use_projectile", &ctx).unwrap());
    }

    #[test]
    fn test_can_use_projectile_child_boomerang() {
        let ctx = MockContext::new()
            .with_item("BOOMERANG", 1)
            .with_child_age();
        assert!(eval_str("can_use_projectile", &ctx).unwrap());
    }

    #[test]
    fn test_can_use_projectile_adult_with_child_items() {
        // Adult with child-only items cannot use them
        let ctx = MockContext::new()
            .with_item("SLINGSHOT", 1)
            .with_item("BOOMERANG", 1)
            .with_adult_age();
        assert!(!eval_str("can_use_projectile", &ctx).unwrap());
    }

    #[test]
    fn test_can_use_projectile_child_with_adult_items() {
        // Child with adult-only items cannot use them
        let ctx = MockContext::new()
            .with_item("BOW", 1)
            .with_item("HOOKSHOT", 1)
            .with_child_age();
        assert!(!eval_str("can_use_projectile", &ctx).unwrap());
    }

    // --- can_hookshot tests ---

    #[test]
    fn test_can_hookshot_adult() {
        let ctx = MockContext::new().with_item("HOOKSHOT", 1).with_adult_age();
        assert!(eval_str("can_hookshot", &ctx).unwrap());
        assert!(eval_str("can_hookshot()", &ctx).unwrap());
    }

    #[test]
    fn test_can_hookshot_adult_with_longshot() {
        let ctx = MockContext::new().with_item("LONGSHOT", 1).with_adult_age();
        assert!(eval_str("can_hookshot", &ctx).unwrap());
    }

    #[test]
    fn test_can_hookshot_child() {
        // Child cannot use hookshot
        let ctx = MockContext::new().with_item("HOOKSHOT", 1).with_child_age();
        assert!(!eval_str("can_hookshot", &ctx).unwrap());
    }

    #[test]
    fn test_can_hookshot_no_item() {
        let ctx = MockContext::new().with_adult_age();
        assert!(!eval_str("can_hookshot", &ctx).unwrap());
    }

    // --- can_longshot tests ---

    #[test]
    fn test_can_longshot_adult() {
        let ctx = MockContext::new().with_item("LONGSHOT", 1).with_adult_age();
        assert!(eval_str("can_longshot", &ctx).unwrap());
        assert!(eval_str("can_longshot()", &ctx).unwrap());
    }

    #[test]
    fn test_can_longshot_adult_with_hookshot_only() {
        // Having hookshot doesn't mean you have longshot
        let ctx = MockContext::new().with_item("HOOKSHOT", 1).with_adult_age();
        assert!(!eval_str("can_longshot", &ctx).unwrap());
    }

    // --- can_boomerang tests ---

    #[test]
    fn test_can_boomerang_child() {
        let ctx = MockContext::new()
            .with_item("BOOMERANG", 1)
            .with_child_age();
        assert!(eval_str("can_boomerang", &ctx).unwrap());
        assert!(eval_str("can_boomerang()", &ctx).unwrap());
    }

    #[test]
    fn test_can_boomerang_adult() {
        // Adult cannot use boomerang
        let ctx = MockContext::new()
            .with_item("BOOMERANG", 1)
            .with_adult_age();
        assert!(!eval_str("can_boomerang", &ctx).unwrap());
    }

    // --- can_hammer tests ---

    #[test]
    fn test_can_hammer_adult() {
        let ctx = MockContext::new().with_item("HAMMER", 1).with_adult_age();
        assert!(eval_str("can_hammer", &ctx).unwrap());
        assert!(eval_str("can_hammer()", &ctx).unwrap());
        assert!(eval_str("can_smash", &ctx).unwrap());
    }

    #[test]
    fn test_can_hammer_adult_megaton() {
        let ctx = MockContext::new()
            .with_item("MEGATON_HAMMER", 1)
            .with_adult_age();
        assert!(eval_str("can_hammer", &ctx).unwrap());
    }

    #[test]
    fn test_can_hammer_child() {
        // Child cannot use hammer
        let ctx = MockContext::new().with_item("HAMMER", 1).with_child_age();
        assert!(!eval_str("can_hammer", &ctx).unwrap());
    }

    // --- can_dive tests ---

    #[test]
    fn test_can_dive_with_scale() {
        let ctx = MockContext::new().with_item("SCALE", 1);
        assert!(eval_str("can_dive", &ctx).unwrap());
        assert!(eval_str("can_dive()", &ctx).unwrap());
    }

    #[test]
    fn test_can_dive_with_silver_scale() {
        let ctx = MockContext::new().with_item("SILVER_SCALE", 1);
        assert!(eval_str("can_dive", &ctx).unwrap());
    }

    #[test]
    fn test_can_dive_with_golden_scale() {
        let ctx = MockContext::new().with_item("GOLDEN_SCALE", 1);
        assert!(eval_str("can_dive", &ctx).unwrap());
        assert!(eval_str("can_dive_deep", &ctx).unwrap());
    }

    #[test]
    fn test_can_dive_deep_requires_golden() {
        let ctx = MockContext::new().with_item("SILVER_SCALE", 1);
        assert!(eval_str("can_dive", &ctx).unwrap());
        assert!(!eval_str("can_dive_deep", &ctx).unwrap());
    }

    #[test]
    fn test_can_dive_none() {
        let ctx = MockContext::new();
        assert!(!eval_str("can_dive", &ctx).unwrap());
    }

    // --- can_play tests ---

    #[test]
    fn test_can_play_with_ocarina() {
        let ctx = MockContext::new().with_item("OCARINA", 1);
        assert!(eval_str("can_play", &ctx).unwrap());
        assert!(eval_str("can_play()", &ctx).unwrap());
    }

    #[test]
    fn test_can_play_with_fairy_ocarina() {
        let ctx = MockContext::new().with_item("OCARINA_FAIRY", 1);
        assert!(eval_str("can_play", &ctx).unwrap());
    }

    #[test]
    fn test_can_play_none() {
        let ctx = MockContext::new();
        assert!(!eval_str("can_play", &ctx).unwrap());
    }

    // --- can_use_sword tests ---

    #[test]
    fn test_can_use_sword_child() {
        let ctx = MockContext::new()
            .with_item("KOKIRI_SWORD", 1)
            .with_child_age();
        assert!(eval_str("can_use_sword", &ctx).unwrap());
        assert!(eval_str("can_use_sword()", &ctx).unwrap());
    }

    #[test]
    fn test_can_use_sword_adult() {
        let ctx = MockContext::new()
            .with_item("MASTER_SWORD", 1)
            .with_adult_age();
        assert!(eval_str("can_use_sword", &ctx).unwrap());
    }

    #[test]
    fn test_can_use_sword_wrong_age() {
        // Adult with Kokiri Sword can't use it
        let ctx = MockContext::new()
            .with_item("KOKIRI_SWORD", 1)
            .with_adult_age();
        assert!(!eval_str("can_use_sword", &ctx).unwrap());

        // Child with Master Sword can't use it
        let ctx2 = MockContext::new()
            .with_item("MASTER_SWORD", 1)
            .with_child_age();
        assert!(!eval_str("can_use_sword", &ctx2).unwrap());
    }

    // --- has_fire tests ---

    #[test]
    fn test_has_fire_dins() {
        let ctx = MockContext::new().with_item("DINS_FIRE", 1);
        assert!(eval_str("has_fire", &ctx).unwrap());
        assert!(eval_str("has_fire()", &ctx).unwrap());
    }

    #[test]
    fn test_has_fire_arrows() {
        let ctx = MockContext::new()
            .with_item("BOW", 1)
            .with_item("FIRE_ARROWS", 1)
            .with_adult_age();
        assert!(eval_str("has_fire", &ctx).unwrap());
    }

    #[test]
    fn test_has_fire_none() {
        let ctx = MockContext::new();
        assert!(!eval_str("has_fire", &ctx).unwrap());
    }

    // --- can_stun tests ---

    #[test]
    fn test_can_stun_with_nuts() {
        let ctx = MockContext::new().with_item("DEKU_NUT", 1);
        assert!(eval_str("can_stun", &ctx).unwrap());
        assert!(eval_str("can_stun()", &ctx).unwrap());
    }

    #[test]
    fn test_can_stun_with_boomerang_child() {
        let ctx = MockContext::new()
            .with_item("BOOMERANG", 1)
            .with_child_age();
        assert!(eval_str("can_stun", &ctx).unwrap());
    }

    #[test]
    fn test_can_stun_with_hookshot_adult() {
        let ctx = MockContext::new().with_item("HOOKSHOT", 1).with_adult_age();
        assert!(eval_str("can_stun", &ctx).unwrap());
    }

    // --- can_child_attack / can_adult_attack / can_damage tests ---

    #[test]
    fn test_can_child_attack() {
        let ctx = MockContext::new()
            .with_item("KOKIRI_SWORD", 1)
            .with_child_age();
        assert!(eval_str("can_child_attack", &ctx).unwrap());
        assert!(eval_str("can_child_attack()", &ctx).unwrap());
        assert!(eval_str("can_damage", &ctx).unwrap());
    }

    #[test]
    fn test_can_child_attack_with_stick() {
        let ctx = MockContext::new()
            .with_item("DEKU_STICK", 1)
            .with_child_age();
        assert!(eval_str("can_child_attack", &ctx).unwrap());
    }

    #[test]
    fn test_can_adult_attack() {
        let ctx = MockContext::new()
            .with_item("MASTER_SWORD", 1)
            .with_adult_age();
        assert!(eval_str("can_adult_attack", &ctx).unwrap());
        assert!(eval_str("can_adult_attack()", &ctx).unwrap());
        assert!(eval_str("can_damage", &ctx).unwrap());
    }

    #[test]
    fn test_can_damage_requires_attack() {
        let ctx = MockContext::new().with_adult_age();
        assert!(!eval_str("can_damage", &ctx).unwrap());
    }

    // --- has_strength / can_lift tests ---

    #[test]
    fn test_has_strength() {
        let ctx = MockContext::new().with_item("GORON_BRACELET", 1);
        assert!(eval_str("has_strength", &ctx).unwrap());
        assert!(eval_str("has_strength()", &ctx).unwrap());
    }

    #[test]
    fn test_can_lift_heavy() {
        let ctx = MockContext::new().with_item("SILVER_GAUNTLETS", 1);
        assert!(eval_str("has_strength", &ctx).unwrap());
        assert!(eval_str("can_lift_heavy", &ctx).unwrap());
        assert!(eval_str("can_lift_heavy()", &ctx).unwrap());
    }

    #[test]
    fn test_can_lift_heaviest() {
        let ctx = MockContext::new().with_item("GOLDEN_GAUNTLETS", 1);
        assert!(eval_str("has_strength", &ctx).unwrap());
        assert!(eval_str("can_lift_heavy", &ctx).unwrap());
        assert!(eval_str("can_lift_heaviest", &ctx).unwrap());
        assert!(eval_str("can_lift_heaviest()", &ctx).unwrap());
    }

    #[test]
    fn test_can_lift_heavy_not_enough() {
        let ctx = MockContext::new().with_item("GORON_BRACELET", 1);
        assert!(eval_str("has_strength", &ctx).unwrap());
        assert!(!eval_str("can_lift_heavy", &ctx).unwrap());
    }

    // --- can_use_bow / can_use_slingshot / can_shoot tests ---

    #[test]
    fn test_can_use_bow() {
        let ctx = MockContext::new().with_item("BOW", 1).with_adult_age();
        assert!(eval_str("can_use_bow", &ctx).unwrap());
        assert!(eval_str("can_use_bow()", &ctx).unwrap());
        assert!(eval_str("can_shoot", &ctx).unwrap());
    }

    #[test]
    fn test_can_use_slingshot() {
        let ctx = MockContext::new()
            .with_item("SLINGSHOT", 1)
            .with_child_age();
        assert!(eval_str("can_use_slingshot", &ctx).unwrap());
        assert!(eval_str("can_use_slingshot()", &ctx).unwrap());
        assert!(eval_str("can_shoot", &ctx).unwrap());
    }

    #[test]
    fn test_can_shoot_none() {
        let ctx = MockContext::new();
        assert!(!eval_str("can_shoot", &ctx).unwrap());
    }

    // --- can_use_iron_boots / can_use_hover_boots tests ---

    #[test]
    fn test_can_use_iron_boots() {
        let ctx = MockContext::new()
            .with_item("IRON_BOOTS", 1)
            .with_adult_age();
        assert!(eval_str("can_use_iron_boots", &ctx).unwrap());
        assert!(eval_str("can_use_iron_boots()", &ctx).unwrap());
    }

    #[test]
    fn test_can_use_hover_boots() {
        let ctx = MockContext::new()
            .with_item("HOVER_BOOTS", 1)
            .with_adult_age();
        assert!(eval_str("can_use_hover_boots", &ctx).unwrap());
        assert!(eval_str("can_use_hover_boots()", &ctx).unwrap());
    }

    #[test]
    fn test_can_use_boots_child() {
        // Child cannot use adult boots
        let ctx = MockContext::new()
            .with_item("IRON_BOOTS", 1)
            .with_item("HOVER_BOOTS", 1)
            .with_child_age();
        assert!(!eval_str("can_use_iron_boots", &ctx).unwrap());
        assert!(!eval_str("can_use_hover_boots", &ctx).unwrap());
    }

    // --- can_dive_water_temple tests ---

    #[test]
    fn test_can_dive_water_temple() {
        let ctx = MockContext::new()
            .with_item("IRON_BOOTS", 1)
            .with_item("ZORA_TUNIC", 1)
            .with_adult_age();
        assert!(eval_str("can_dive_water_temple", &ctx).unwrap());
        assert!(eval_str("can_dive_water_temple()", &ctx).unwrap());
    }

    #[test]
    fn test_can_dive_water_temple_missing_tunic() {
        let ctx = MockContext::new()
            .with_item("IRON_BOOTS", 1)
            .with_adult_age();
        assert!(!eval_str("can_dive_water_temple", &ctx).unwrap());
    }

    #[test]
    fn test_can_dive_water_temple_missing_boots() {
        let ctx = MockContext::new()
            .with_item("ZORA_TUNIC", 1)
            .with_adult_age();
        assert!(!eval_str("can_dive_water_temple", &ctx).unwrap());
    }

    // --- MM mask helpers tests ---

    #[test]
    fn test_has_mask_deku() {
        let ctx = MockContext::new().with_item("MASK_DEKU", 1);
        assert!(eval_str("has_mask_deku", &ctx).unwrap());
        assert!(eval_str("has_mask_deku()", &ctx).unwrap());
        assert!(eval_str("can_fly_deku", &ctx).unwrap());
    }

    #[test]
    fn test_has_mask_goron() {
        let ctx = MockContext::new().with_item("MASK_GORON", 1);
        assert!(eval_str("has_mask_goron", &ctx).unwrap());
        assert!(eval_str("has_mask_goron()", &ctx).unwrap());
        assert!(eval_str("can_roll_fast", &ctx).unwrap());
    }

    #[test]
    fn test_has_mask_zora() {
        let ctx = MockContext::new().with_item("MASK_ZORA", 1);
        assert!(eval_str("has_mask_zora", &ctx).unwrap());
        assert!(eval_str("has_mask_zora()", &ctx).unwrap());
        assert!(eval_str("can_swim_fast", &ctx).unwrap());
    }

    #[test]
    fn test_has_mask_fierce_deity() {
        let ctx = MockContext::new().with_item("MASK_FIERCE_DEITY", 1);
        assert!(eval_str("has_mask_fierce_deity", &ctx).unwrap());
        assert!(eval_str("has_mask_fierce_deity()", &ctx).unwrap());
    }

    #[test]
    fn test_can_use_powder_keg() {
        let ctx = MockContext::new()
            .with_item("MASK_GORON", 1)
            .with_item("POWDER_KEG", 1);
        assert!(eval_str("can_use_powder_keg", &ctx).unwrap());
        assert!(eval_str("can_use_powder_keg()", &ctx).unwrap());
    }

    #[test]
    fn test_can_use_powder_keg_no_goron() {
        let ctx = MockContext::new().with_item("POWDER_KEG", 1);
        assert!(!eval_str("can_use_powder_keg", &ctx).unwrap());
    }

    // --- Helper with wrong arg count error tests ---

    #[test]
    fn test_helper_wrong_arg_count() {
        let ctx = MockContext::new();
        let result = eval_str("can_blast(BOMBS)", &ctx);
        assert!(result.is_err());
    }

    // --- Complex expressions with helpers ---

    #[test]
    fn test_helper_in_complex_expression() {
        let ctx = MockContext::new()
            .with_item("HOOKSHOT", 1)
            .with_item("BOMBS", 1)
            .with_item("SCALE", 1)
            .with_adult_age();

        assert!(eval_str("can_hookshot && can_blast && can_dive", &ctx).unwrap());
        assert!(eval_str("can_hookshot() && can_blast() && can_dive()", &ctx).unwrap());
        assert!(eval_str("(can_hookshot || can_boomerang) && can_blast", &ctx).unwrap());
    }

    #[test]
    fn test_helper_combined_with_has_and_events() {
        let ctx = MockContext::new()
            .with_item("HOOKSHOT", 1)
            .with_item("BOW", 1)
            .with_event("FOREST_TEMPLE_CLEAR")
            .with_adult_age();

        assert!(eval_str(
            "is_adult && can_hookshot && has(BOW) && event(FOREST_TEMPLE_CLEAR)",
            &ctx
        )
        .unwrap());
    }

    // --- is_day / is_night tests ---

    #[test]
    fn test_is_day_identifier() {
        let ctx = MockContext::new(); // Default time is 0 (daytime)
        assert!(eval_str("is_day", &ctx).unwrap());
        assert!(!eval_str("is_night", &ctx).unwrap());
    }
}
