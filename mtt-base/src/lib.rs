use borsh::{BorshDeserialize, BorshSerialize};
use race_api::event::BridgeEvent;
use race_api::prelude::*;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, Default, PartialEq, Eq)]
pub struct MttTablePlayer {
    pub id: u64,
    pub chips: u64,
    pub table_position: usize,
}

impl MttTablePlayer {
    pub fn new(id: u64, chips: u64, table_position: usize) -> Self {
        Self {
            id,
            chips,
            table_position,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Default, Debug)]
pub struct InitTableData {
    pub table_id: u8,
    pub table_size: u8,
}

#[derive(Default, Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
pub struct MttTable {
    pub hand_id: usize,
    pub btn: usize,
    pub sb: u64,
    pub bb: u64,
    pub players: Vec<MttTablePlayer>,
    pub next_game_start: u64,
}

#[derive(Default, Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
pub struct MttTableCheckpoint {
    pub hand_id: usize,
    pub btn: usize,
    pub sb: u64,
    pub bb: u64,
    pub next_game_start: u64,
    pub players: Vec<MttTablePlayer>,
}

impl MttTableCheckpoint {
    pub fn new(table: &MttTable) -> Self {
        MttTableCheckpoint {
            btn: table.btn,
            sb: table.sb,
            bb: table.bb,
            next_game_start: table.next_game_start,
            hand_id: table.hand_id,
            players: table.players.clone(),
        }
    }
}

// impl From<&MttTable> for MttTableCheckpoint {
//     fn from(value: &MttTable) -> Self {
//         Self { sb: value.sb, bb: value.bb, btn: value.btn }
//     }
// }

impl MttTable {
    pub fn new(checkpoint: &MttTableCheckpoint, players: Vec<MttTablePlayer>) -> Self {
        Self {
            sb: checkpoint.sb,
            bb: checkpoint.bb,
            btn: checkpoint.btn,
            players,
            next_game_start: checkpoint.next_game_start,
            hand_id: checkpoint.hand_id,
        }
    }

    pub fn add_player(&mut self, player: &mut MttTablePlayer) {
        let mut table_position = 0;
        for i in 0.. {
            if self
                .players
                .iter()
                .find(|p| p.table_position == i)
                .is_none()
            {
                table_position = i;
                break;
            }
        }
        self.players.push(MttTablePlayer {
            id: player.id,
            chips: player.chips,
            table_position,
        });
        // Update relocated player's table position as well
        player.table_position = table_position;
    }
}

/// Holdem specific bridge events for interaction with the `mtt` crate.  Transactor will pass
/// through such events to the mtt handler.  Also see [`race_api::event::Event::Bridge`].
#[derive(Debug, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
pub enum HoldemBridgeEvent {
    /// Start game with specified SB and BB.
    /// The `moved_players` indicates those should be removed before next hand.
    StartGame {
        sb: u64,
        bb: u64,
        moved_players: Vec<u64>,
    },
    /// Add players to current game.
    Relocate { players: Vec<MttTablePlayer> },
    /// Close table, all players should be removed from this game.
    /// Additionally, the game can be closed.
    CloseTable,
    GameResult {
        hand_id: usize,
        table_id: u8,
        settles: Vec<Settle>,
        table: MttTable,
    },
}

impl BridgeEvent for HoldemBridgeEvent {}

#[cfg(test)]
mod tests {
    use crate::{InitTableData, MttTable};
    use borsh::BorshDeserialize;

    #[test]
    fn test_parse_mtt_init_table_data() -> anyhow::Result<()> {
        let data = [
            207, 128, 234, 18, 142, 1, 0, 0, 0, 225, 245, 5, 0, 0, 0, 0, 2, 32, 161, 7, 0, 0, 0, 0,
            0, 96, 234, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 50, 30, 20, 0,
        ];
        let st = InitTableData::try_from_slice(&data)?;
        println!("{:?}", st);
        Ok(())
    }
}
