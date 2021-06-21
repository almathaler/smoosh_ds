use std::collections::HashMap;

mod stk_env {
    /// Stk_envs are made up of Nodes. These nodes are similar to LinkedList
    /// nodes. They contain an Option<ref Node> [ref next] node, Option<ref Node>
    /// [ref prev] node, a [env] HashMap<K, V>, bool [cloned], and Option<i32>
    /// [cloned_depth]. If this Node is on a branch that's been cloned,
    /// [cloned_depth] guides the [smoosh] function all the way back to the
    /// head of the [stk_env], when it was first cloned.
    /// Invariant: [next] and [prev], if not None, are Nodes with the same <K, V>
    /// generics as [self]
    /// Invariant: if [cloned] is true, [depth] is not None
    struct Node<K, V> {
        next: Option<&Node<K, V>>,
        prev: Option<&Node<K, V>>,
        env: HashMap<K, V>,
        cloned: bool,
        depth: Option<u32>,
    }
    impl<K, V> Node<K, V> {
        fn new() -> Node<K, V> {
            Node {
                next: None, //hm what do we do when we clone, and a node suddenly has multiple [next]s?
                //I guess since clone returns a new Stk_env, the cloned-from node will still consider
                //its [next] to be None.
                prev: None,
                env: HashMap::new(),
                cloned: false,
                depth: None,
            }
        }
        //below will be used by Stk_env, user shouldn't touch Nodes
        fn contains(&self, k: K) -> bool {}
        fn get(&self, k: K) -> Option<V> {}
        fn set(&mut self, k: K, v: V) {}
    }
    /// A Stk_env contains its [head] Node and its [length]
    struct Stk_env<K, V> {
        head: Node,
        length: u32,
    }
    impl<K, V> Stk_env<K, V> {
        /// Returns a Stk_env with a [head] that's a Node with an empty [env]
        pub fn new() -> Stk_env<K, V> {
            Stk_env {
                head: Node::new(),
                length: 0,
            }
        }
        pub fn len(&self) -> u32 {}
        pub fn contains(&self, k: K) -> bool {}
        // Works as discussed -- if [head] doesn't contain [k], search all the way down until we find
        // some Node that contains [k], else if none exist, return None
        pub fn get(&self, k: K) -> Option<V> {}
        pub fn set(&mut self, k: K, v: V) {}

        //below - problem? too much cushioning w/ empty nodes? no not really, what's the point
        //of just copying the prev node? whole point of this DS is that you can
        /// Returns a new Stk_env, whose head is an empty node and whose [prev]
        /// is the head of the [Stk_env] we called clone from
        pub fn clone(&self) -> Stk_env<K, V> {}

        /// Returns this Stk_env with a modified head, if the head of this Stk_env
        /// was cloned from another. The new head contains the bindings from
        /// its cloned offspring.
        pub fn smoosh(&mut self) -> Self {}

        /// Returns all bindings that are different between the [head] of this Stk_env
        /// and the node right before it
        pub fn diff(&self) -> HashMap<K, V> {} //might not be best output type. Also this is "sieve" function
    }
}
