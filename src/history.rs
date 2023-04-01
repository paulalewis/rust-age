/**
 * Keeps track of state transition history.
 */
struct History<S, A> {
    nodes: [HistoryNode<S, A>],

/* 
    companion object {
        fun <S : State<S>, A : Action<A>> create(initialState: S): History<S, A> {
            val nodes = mutableListOf<Node<S, A>>()
            nodes.add(Node(initialState, emptyMap()))
            return History(nodes)
        }
    }
*/
}

struct HistoryNode<S, A> {
    state: S,
    actions: Map<Int, A>,
}

impl History {
    fn clear(&self) {
        self.nodes.clear
    }
    /**
     * Add the next state and the actions taken by each agent
     * to arrive at that state.
     * @param state the current state
     * @param actions the actions taken by each player to end up in the current state
     */
    fn add(&self, state: S, actions: Map<Int, A>) {
        self.nodes.add(HistoryNode(state, actions))
    }
}