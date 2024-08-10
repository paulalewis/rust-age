use rand::Rng;
use rand_chacha::ChaCha8Rng;

use crate::core::{reward::{Reward, ADVERSARIAL_DRAW, ADVERSARIAL_P1_LOSS, ADVERSARIAL_P1_WIN}, simulator::{LegalActions, Simulator}};

use super::{backgammon_action::BackgammonAction, backgammon_state::{BackgammonState, N_DIE_FACES, N_LOCATIONS}};

/*class BackgammonSimulator(private val random: Random = Random) : Simulator<BackgammonState, BackgammonAction> {

    override fun calculateLegalActions(state: BackgammonState): List<Set<BackgammonAction>> {
        val legalActions = listOf<MutableSet<BackgammonAction>>(mutableSetOf(), mutableSetOf())

        val rewards = calculateRewards(state)
        if (rewards[0] == 0) {
            val piece = if (state.agentTurn == 0) 1 else -1

            val dice = state.dice
            val values = if (dice[0] == dice[1]) {
                intArrayOf(dice[0] + 1)
            } else {
                intArrayOf(dice[0] + 1, dice[1] + 1)
            }

            val depth = if (dice[0] == dice[1]) 4 else 2

            val tempLegalActions = dfs(
                state.locations, ArrayList(),
                values, piece, depth, state.agentTurn
            )

            // only allow actions that are tied for using most moves
            var max = 0
            for ((moves) in tempLegalActions) {
                max = max(max, moves.size)
            }
            tempLegalActions
                .filter { it.moves.size == max }
                .forEach { legalActions[state.agentTurn].add(it) }
        }
        return legalActions
    }

    override fun stateTransition(state: BackgammonState, actions: Map<Int, BackgammonAction>): BackgammonState {
        val action = actions[state.agentTurn]
        val legalActions = calculateLegalActions(state)
        if (action === null || !legalActions[state.agentTurn].contains(action)) {
            throw IllegalArgumentException("Illegal action, $action, from state, $state")
        }

        val locations = state.locations

        for ((from, distance) in action.moves) {
            val piece: Byte = if (locations[from] > 0) 1 else -1
            val to = from + distance * piece
            if (to > 0 && to < BackgammonState.N_LOCATIONS - 1) {
                if (locations[to] * piece < 0) {
                    locations[to] = piece
                    if (piece > 0) {
                        locations[25] = (locations[25] - piece).toByte()
                    } else {
                        locations[0] = (locations[0] - piece).toByte()
                    }
                } else {
                    locations[to] = (locations[to] + piece).toByte()
                }
            }
            locations[from] = (locations[from] - piece).toByte()
        }
        val dice = byteArrayOf(
            random.nextInt(BackgammonState.N_DIE_FACES).toByte(),
            random.nextInt(BackgammonState.N_DIE_FACES).toByte()
        )
        return BackgammonState(locations, dice, nextPlayerTurnSequential(state.agentTurn, BackgammonState.N_PLAYERS))
    }

    companion object {

        private const val TURN_PLAYER_1 = 0

        private fun dfs(
            locations: ByteArray,
            moves: ArrayList<BackgammonMove>,
            values: IntArray,
            piece: Int,
            depth: Int,
            agentTurn: Int
        ): MutableList<BackgammonAction> {
            val legalActions = ArrayList<BackgammonAction>()
            val limit = if (piece > 0 && locations[0] > 0) 1 else BackgammonState.N_LOCATIONS
            val start = if (piece < 0 && locations[25] < 0) 25 else 0
            val moveOff = canMoveOff(locations, piece)

            for (i in start until limit) {
                if (locations[i] * piece >= 1) {
                    for (j in values.indices) {
                        if (canMove(i, values[j], moveOff, agentTurn, locations)) {
                            val move = BackgammonMove.valueOf(i, values[j])
                            if (moves.isEmpty() || move.compareTo(moves.last()) * piece >= 0) {
                                moves.add(move)
                                if (depth > 1) {
                                    locations[i] = (locations[i] - piece).toByte()
                                    val next = i + values[j] * piece
                                    if (next > 0 && next < BackgammonState.N_LOCATIONS - 1) {
                                        locations[next] = (locations[next] + piece).toByte()
                                    }
                                    var k = 0
                                    if (values.size == 2) {
                                        k = if (j == 0) 1 else 0
                                    }
                                    legalActions.addAll(
                                        dfs(
                                            locations, moves, intArrayOf(values[k]),
                                            piece, depth - 1, agentTurn
                                        )
                                    )
                                    if (next > 0 && next < BackgammonState.N_LOCATIONS - 1) {
                                        locations[next] = (locations[next] - piece).toByte()
                                    }
                                    locations[i] = (locations[i] + piece).toByte()
                                } else {
                                    legalActions.add(BackgammonAction(moves.toHashSet()))
                                }
                                moves.removeLast()
                            }
                        }
                    }
                }
            }
            if (legalActions.size == 0) {
                legalActions.add(BackgammonAction(moves.toHashSet()))
            }
            return legalActions
        }
    }
}*/

/// Classic game of [Backgammon](https://en.wikipedia.org/wiki/Backgammon)
pub struct BackgammonSimulator {
    random: ChaCha8Rng,
}

impl Simulator<BackgammonState, BackgammonAction> for BackgammonSimulator {
    fn generate_initial_state(&mut self) -> BackgammonState {
        let locations = [0, 2, 0, 0, 0, 0, -5, 0, -3, 0, 0, 0, 5, -5, 0, 0, 0, 3, 0, 5, 0, 0, 0, 0, -2, 0];
        let p1_turn: bool = self.random.gen_bool(0.5);
        let dice = [
            match self.random.gen_range(0..15) {
                0 => 1,
                1 | 2 => 2,
                3 | 4 | 5 => 3,
                6 | 7 | 8 | 9 => 4,
                _ => 5,
            } as u8,
            self.random.gen_range(0..N_DIE_FACES) as u8,
        ];
        BackgammonState { locations, dice, p1_turn }
    }

    fn calculate_rewards(&mut self, state: &BackgammonState) -> Vec<Reward> {
        let mut pos = false;
        let mut neg = false;
        for i in 0..N_LOCATIONS {
            if !pos && state.locations[i] > 0 {
                pos = true;
            } else if !neg && state.locations[i] < 0 {
                neg = true;
            }
        }
        if !pos {
            ADVERSARIAL_P1_WIN.to_vec()
        } else if !neg {
            ADVERSARIAL_P1_LOSS.to_vec()
        } else {
            ADVERSARIAL_DRAW.to_vec()
        }
    }
    
    fn calculate_legal_actions(&mut self, state: &BackgammonState) -> Vec<LegalActions<BackgammonAction>> {
        todo!()
    }
    
    fn state_transition(&mut self, state: &BackgammonState, actions: &Vec<Option<BackgammonAction>>) -> BackgammonState {
        todo!()
    }
    
    fn number_of_players(&mut self) -> usize { 2 }
}

impl BackgammonSimulator {
    fn can_move(
        location: u8,
        distance: u8,
        move_off: bool,
        p1_turn: bool,
        locations: [i8; N_LOCATIONS],
    ) -> bool {
        if p1_turn {
            let next = location + distance;
            next < N_LOCATIONS as u8 - 1 && locations[next as usize] >= -1 ||
                move_off && next >= N_LOCATIONS as u8 - 1
        } else {
            let next = location - distance;
            next > 0 && locations[next as usize] <= 1 || move_off && next <= 0
        }
    }
    /// Checks if a player can start moving pieces off of the board.
    /// Return true if legal to move off board.
    fn can_move_off(locations: &[i8], piece: i8) -> bool {
        if piece > 0 {
            for i in 0..18 {
                if locations[i] > 0 {
                    return false;
                }
            }
        } else {
            for i in 7..N_LOCATIONS {
                if locations[i] < 0 {
                    return false;
                }
            }
        }
        true
    }
}