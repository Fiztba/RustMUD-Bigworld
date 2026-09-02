//! Core scalar types and limit constants.
//!
//! Index types are 32 bits wide: vnums range 0..=MAX_VNUM and the top of the
//! range is the shared "none" sentinel for rooms, objects, mobs, and flags
//! alike. World files write these numbers as signed decimal, so the sentinel
//! is -1 on disk and to scripts, whatever the width in memory.

/// Index type for all virtual/real numbers.
pub type Idx = u32;

/// Highest vnum a world file or OLC will accept. The top of the range is the
/// nil sentinel, so every legal vnum stays distinct from it.
pub const MAX_VNUM: Idx = Idx::MAX - 1;

/// Vnum written to disk for "no such thing" by builds whose index type was an
/// unsigned short. Readers accept it alongside -1 so that worlds, player
/// files and rent files saved before the widening still load; writers emit
/// -1.
pub const LEGACY_NOWHERE: i64 = 65535;

/// Whether readers still honour LEGACY_NOWHERE. Leave this on to load a world
/// or player base that was saved before the widening. Turn it off in a world
/// that never was, and vnum 65535 becomes an ordinary vnum again.
pub const ACCEPT_LEGACY_NOWHERE: bool = true;

/// True for the numbers a data file may use to mean "no such thing". Files
/// are read as signed decimal, so the nil sentinel arrives as -1 whatever
/// width the index type has.
pub fn is_nil_vnum<T: Into<i64>>(n: T) -> bool {
    let n = n.into();
    n == -1 || (ACCEPT_LEGACY_NOWHERE && n == LEGACY_NOWHERE)
}

pub type RoomVnum = Idx;
pub type ObjVnum = Idx;
pub type MobVnum = Idx;
pub type ZoneVnum = Idx;
pub type ShopVnum = Idx;
pub type TrigVnum = Idx;
pub type QstVnum = Idx;

pub type RoomRnum = Idx;
pub type ObjRnum = Idx;
pub type MobRnum = Idx;
pub type ZoneRnum = Idx;
pub type ShopRnum = Idx;
pub type TrigRnum = Idx;
pub type QstRnum = Idx;

pub const NOWHERE: Idx = Idx::MAX;
pub const NOTHING: Idx = Idx::MAX;
pub const NOBODY: Idx = Idx::MAX;
pub const NOFLAG: Idx = Idx::MAX;

// OLC grants. A builder's zone is a zone vnum in olc_zone; the editors that
// are not tied to any zone are granted separately, as these bits in
// olc_grants, so no zone number has to be reserved to stand for them. They
// used to be zone numbers (999, 888, 666) above anything a 16-bit world could
// hold; a wider world can hold them, hence the split. The names are the ones
// Oasis has always used.
pub const AEDIT_PERMISSION: i32 = 1 << 0; // may edit socials
pub const HEDIT_PERMISSION: i32 = 1 << 1; // may edit help files
pub const ALL_PERMISSION: i32 = 1 << 2; // may edit every zone and use every editor
pub const NUM_OLC_GRANTS: usize = 3;
/// The grant names, in bit order; what set, stat and the player file use.
pub const OLC_GRANT_BITS: [&str; NUM_OLC_GRANTS] = ["aedit", "hedit", "all"];

/// The grants as sprintbit() spells them: "aedit hedit " (trailing space),
/// or "NOBITS " for none. This is the OlcG line of the player file.
pub fn olc_grant_names(grants: i32) -> String {
    let mut out = String::new();
    for (i, name) in OLC_GRANT_BITS.iter().enumerate() {
        if grants & (1 << i) != 0 {
            out.push_str(name);
            out.push(' ');
        }
    }
    if out.is_empty() {
        out.push_str("NOBITS ");
    }
    out
}

/// The inverse: grant names, whitespace-separated, to bits. Unknown words
/// are ignored, as search_block() ignores them.
pub fn parse_olc_grants(line: &[u8]) -> i32 {
    let mut grants = 0;
    for word in line.split(|b| b.is_ascii_whitespace()).filter(|w| !w.is_empty()) {
        if let Some(i) = OLC_GRANT_BITS.iter().position(|n| n.as_bytes().eq_ignore_ascii_case(word)) {
            grants |= 1 << i;
        }
    }
    grants
}

// Directions
pub const NORTH: usize = 0;
pub const EAST: usize = 1;
pub const SOUTH: usize = 2;
pub const WEST: usize = 3;
pub const UP: usize = 4;
pub const DOWN: usize = 5;
pub const NORTHWEST: usize = 6;
pub const NORTHEAST: usize = 7;
pub const SOUTHEAST: usize = 8;
pub const SOUTHWEST: usize = 9;
pub const NUM_OF_DIRS: usize = 10;

// Immortal levels
pub const LVL_IMPL: u8 = 34;
pub const LVL_GRGOD: u8 = 33;
pub const LVL_GOD: u8 = 32;
pub const LVL_IMMORT: u8 = 31;
pub const LVL_BUILDER: u8 = LVL_IMMORT;

// Positions
pub const POS_DEAD: u8 = 0;
pub const POS_MORTALLYW: u8 = 1;
pub const POS_INCAP: u8 = 2;
pub const POS_STUNNED: u8 = 3;
pub const POS_SLEEPING: u8 = 4;
pub const POS_RESTING: u8 = 5;
pub const POS_SITTING: u8 = 6;
pub const POS_FIGHTING: u8 = 7;
pub const POS_STANDING: u8 = 8;
pub const NUM_POSITIONS: usize = 9;

// Sex
pub const SEX_NEUTRAL: u8 = 0;
pub const SEX_MALE: u8 = 1;
pub const SEX_FEMALE: u8 = 2;
pub const NUM_GENDERS: usize = 3;

// PC classes. CLASS_UNDEFINED is -1.
pub const CLASS_UNDEFINED: i8 = -1;
pub const CLASS_MAGIC_USER: i8 = 0;
pub const CLASS_CLERIC: i8 = 1;
pub const CLASS_THIEF: i8 = 2;
pub const CLASS_WARRIOR: i8 = 3;
pub const NUM_CLASSES: usize = 4;

// Equipment wear positions
pub const WEAR_LIGHT: usize = 0;
pub const WEAR_FINGER_R: usize = 1;
pub const WEAR_FINGER_L: usize = 2;
pub const WEAR_NECK_1: usize = 3;
pub const WEAR_NECK_2: usize = 4;
pub const WEAR_BODY: usize = 5;
pub const WEAR_HEAD: usize = 6;
pub const WEAR_LEGS: usize = 7;
pub const WEAR_FEET: usize = 8;
pub const WEAR_HANDS: usize = 9;
pub const WEAR_ARMS: usize = 10;
pub const WEAR_SHIELD: usize = 11;
pub const WEAR_ABOUT: usize = 12;
pub const WEAR_WAIST: usize = 13;
pub const WEAR_WRIST_R: usize = 14;
pub const WEAR_WRIST_L: usize = 15;
pub const WEAR_WIELD: usize = 16;
pub const WEAR_HOLD: usize = 17;
pub const NUM_WEARS: usize = 18;

// Pulse engine. One pulse = 100ms; constants are in
// pulses. PULSE_DG_SCRIPT lives with the script engine.
pub const PASSES_PER_SEC: u64 = 10;
pub const PULSE_ZONE: u64 = 10 * PASSES_PER_SEC;
pub const PULSE_MOBILE: u64 = 10 * PASSES_PER_SEC;
pub const PULSE_VIOLENCE: u64 = 2 * PASSES_PER_SEC;
pub const PULSE_AUTOSAVE: u64 = 60 * PASSES_PER_SEC;
pub const PULSE_IDLEPWD: u64 = 15 * PASSES_PER_SEC;
pub const PULSE_SANITY: u64 = 30 * PASSES_PER_SEC;
pub const PULSE_USAGE: u64 = 5 * 60 * PASSES_PER_SEC;
pub const PULSE_TIMESAVE: u64 = 30 * 60 * PASSES_PER_SEC;
pub const PULSE_DG_SCRIPT: u64 = 13 * PASSES_PER_SEC;

// Mud calendar. 75 real seconds per mud hour; 35-day months;
// 17-month years.
/// Real-world spans — rent expiry and played-time math.
pub const SECS_PER_REAL_MIN: i64 = 60;
pub const SECS_PER_REAL_HOUR: i64 = 60 * SECS_PER_REAL_MIN;
pub const SECS_PER_REAL_DAY: i64 = 24 * SECS_PER_REAL_HOUR;

pub const SECS_PER_MUD_HOUR: u64 = 75;
pub const SECS_PER_MUD_DAY: u64 = 24 * SECS_PER_MUD_HOUR;
pub const SECS_PER_MUD_MONTH: u64 = 35 * SECS_PER_MUD_DAY;
pub const SECS_PER_MUD_YEAR: u64 = 17 * SECS_PER_MUD_MONTH;

// String/buffer limits. These are byte limits and are
// player-visible via truncation behavior — do not "fix".
pub const MAX_SOCK_BUF: usize = 24 * 1024;
pub const MAX_PROMPT_LENGTH: usize = 96;
pub const GARBAGE_SPACE: usize = 32;
pub const SMALL_BUFSIZE: usize = 1024;
pub const LARGE_BUFSIZE: usize = MAX_SOCK_BUF - GARBAGE_SPACE - MAX_PROMPT_LENGTH;
pub const MAX_STRING_LENGTH: usize = 49152;
pub const MAX_INPUT_LENGTH: usize = 512;
pub const MAX_RAW_INPUT_LENGTH: usize = 12 * 1024;
pub const MAX_MESSAGES: usize = 60;
pub const MAX_NAME_LENGTH: usize = 20;
pub const MAX_PWD_LENGTH: usize = 30;
pub const MAX_TITLE_LENGTH: usize = 80;
pub const HOST_LENGTH: usize = 40;
pub const BANNED_SITE_LENGTH: usize = 50;
pub const PLR_DESC_LENGTH: usize = 4096;
pub const MAX_SKILLS: usize = 200;
pub const MAX_AFFECT: usize = 32;
pub const MAX_OBJ_AFFECT: usize = 6;
pub const MAX_NOTE_LENGTH: usize = 4000;
pub const MAX_HELP_KEYWORDS: usize = 256;
pub const MAX_COMPLETED_QUESTS: usize = 1024;
pub const MAX_CMD_LENGTH: usize = 16384;
pub const MAX_BAG_ROWS: usize = 5;

pub const MAX_GOLD: i32 = 2_140_000_000;
pub const MAX_BANK: i32 = 2_140_000_000;

/// Spell/skill numbers. Stage 5 brings the full table; these are
/// referenced earlier (poisoned food/drink, pick lock).
pub const SPELL_POISON: i16 = 33;
pub const SKILL_PICK_LOCK: i16 = 135;

/// Connection states. Values are stable — they gate
/// nanny dispatch and appear in `users` output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ConState {
    Playing = 0,
    Close = 1,
    GetName = 2,
    NameCnfrm = 3,
    Password = 4,
    Newpasswd = 5,
    Cnfpasswd = 6,
    Qsex = 7,
    Qclass = 8,
    Rmotd = 9,
    Menu = 10,
    PlrDesc = 11,
    ChpwdGetold = 12,
    ChpwdGetnew = 13,
    ChpwdVrfy = 14,
    Delcnf1 = 15,
    Delcnf2 = 16,
    Disconnect = 17,
    Oedit = 18,
    Redit = 19,
    Zedit = 20,
    Medit = 21,
    Sedit = 22,
    Tedit = 23,
    Cedit = 24,
    Aedit = 25,
    Trigedit = 26,
    Hedit = 27,
    Qedit = 28,
    Prefedit = 29,
    Ibtedit = 30,
    Msgedit = 31,
    GetProtocol = 32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sentinels_are_idx_max() {
        assert_eq!(NOWHERE, u32::MAX);
        assert_eq!(MAX_VNUM, u32::MAX - 1);
        assert!(is_nil_vnum(-1) && is_nil_vnum(65535) && !is_nil_vnum(65534));
        assert_eq!(LARGE_BUFSIZE, 24448);
    }

    #[test]
    fn calendar_math() {
        assert_eq!(SECS_PER_MUD_DAY, 1800);
        assert_eq!(SECS_PER_MUD_YEAR, 75 * 24 * 35 * 17);
    }
}
