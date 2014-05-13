//! Abstract Graph build on adjacency lists.

use std::iter::Iterator;

/**
* Representation of a Graph vertex.
*
* # Types parameters
* * K - The Vertex's Key type
* * L - The Vertex's Label type
* * V - The Edge's Value type
*/
#[deriving(Clone, Eq, Encodable, Decodable)]
pub struct Vertex<K, L, V> {
    key:                   K,
    label:                 Option<L>,
    edges:                 Option<Box<Edge<K, V>>>,
    next:                  Option<Box<Vertex<K, L, V>>>,
}

/// Iterator to iterate easily other all the vertex of a Graph.
// #[deriving(Clone)]
pub struct VertexIterator<'s, K, L, V> {
    head: &'s Option<Box<Vertex<K, L, V>>>,
}

impl<'s,
     K: ToStr + Ord + Eq + Clone,
     L: ToStr + Ord + Eq + Clone,
     V: ToStr + Ord + Eq + Clone>
     Iterator<(&'s K, Option<&'s L>)> for VertexIterator<'s, K, L, V> {

    /**
    * Get the next iterator of the Vertex list.
    *
    * # Return
    * Return an Option containing a tuple with the key of the Vertex and it value or None.
    */
    #[inline]
    fn next(&mut self) -> Option<(&'s K, Option<&'s L>)> {
        self.head.as_ref().map( |head| {
            self.head = &head.next;
            (&head.key, Some(head.label.get_ref()))
        })
    }
}

impl<K: ToStr + Ord + Eq + Clone,
     L: ToStr + Ord + Eq + Clone,
     V: ToStr + Ord + Eq + Clone>
     Vertex<K, L, V> {

    /**
    * Create a new Vertex with a Key.
    *
    * # Arguments
    * * key - The key to represent the Vertex
    *
    * # Return
    * A new Vertex.
    */
    pub fn new(key: K) -> Vertex<K, L, V> {
        Vertex {
            key:    key,
            label:  None,
            edges:  None,
            next:   None
        }
    }

    /**
    * Create a new Vertex with a Key and an optional Label.
    *
    * # Arguments
    * * key - The key to represent the Vertex
    * * label - The label to attach to the Vertex
    *
    * # Return
    * A new Vertex.
    */
    pub fn new_with_opt(key: K,
                        label: Option<L>)
                        -> Vertex<K, L, V> {
        Vertex {
            key:    key,
            label:  label,
            edges:  None,
            next:   None
        }
    }

    /**
    * Create a new Vertex with a Key and an array of Edges.
    *
    * Warning : The validity of the edges is not certfied.
    *
    * # Arguments
    * * key - The key to represent the Vertex
    * * edges- The array of edges of the Vertex
    *
    * # Return
    * A new Vertex.
    */
    pub fn new_with_edges(key: K,
                          mut edges: ~[Box<Edge<K, V>>])
                          -> Vertex<K, L, V> {
        let tmp_edges: Option<Box<Edge<K, V>>> = edges.shift();
        let mut vertex = Vertex {
            key:    key,
            label:  None,
            edges:  tmp_edges,
            next:   None
        };
        for i in edges.move_iter() {
            VertexUtils::add_edge(&mut vertex.edges, i);
        }
        vertex
    }

    /**
    * Create a new Vertex with a Key and a Label.
    *
    * # Arguments
    * * key - The key to represent the Vertex
    * * label - The label to attach to the Vertex
    *
    * # Return
    * A new Vertex.
    */
    pub fn new_with_label(key: K,
                          label: L)
                          -> Vertex<K, L, V> {
        Vertex {
            key:    key,
            label:  Some(label),
            edges:  None,
            next:   None,
        }
    }

    /**
    * Create a new Vertex with a Key a Label, and an array of Edges.
    *
    * # Arguments
    * * key - The key to represent the Vertex
    * * label - The label to attach to the Vertex
    * * edges - The array of edges to attach to the Vertex
    *
    * # Return
    * A new Vertex.
    */
    pub fn new_with_label_edges(key: K,
                                label: L,
                                mut edges: ~[Box<Edge<K, V>>])
                                -> Vertex<K, L, V> {
        let tmp_edges: Option<Box<Edge<K, V>>> = edges.shift();
        let mut vertex = Vertex {
            key:    key,
            label:  Some(label),
            edges:  tmp_edges,
            next:   None
        };
        for i in edges.move_iter() {
            VertexUtils::add_edge(&mut vertex.edges, i);
        }
        vertex
    }

    /**
    * Get the label of a Vertex.
    *
    * # Return
    * The immuable value of the Vertex.
    */
    pub fn get_label<'r>(&'r self) -> Option<&'r L> {
        match self.label {
            Some(ref l) =>  Some(l),
            None    => None
        }
    }

    /**
    * Set the label of the Vertex.
    *
    * # Argument
    * * new_label - The new label for the Vertex.
    */
    pub fn set_label(&mut self, new_label: L) {
        self.label = Some(new_label)
    }

    /**
    * Remove the label of the Vertex.
    */
    pub fn remove_label(&mut self) -> () {
        self.label = None
    }

    /**
    * Get an Edge iterator.
    *
    * # Return
    * An iterator to iterate over the Edges of the Vertex.
    */
    pub fn edges_iter<'r>(&'r self) -> EdgeIterator<'r, K, V> {
        EdgeIterator {
            head: &self.edges
        }
    }

    /**
    * The Edge exist in the Vertex.
    *
    * # Arguments
    * * key - The key of the which identify the second Vertex of the Edge
    *
    * # Return
    * true if the Edge exist, false otherwise.
    */
    pub fn edge_exist(&self,
                      key: &K)
                      -> bool {
        match VertexUtils::get_edge_imm(&self.edges, key) {
            Some(_) => true,
            None    => false
        }
    }

    /**
    * Add an Edge to the Vertex with an optional value.
    *
    * # Arguments
    * * to_key - The second Vertex of the Edge
    * * cost - The optional value to attach to the Edge
    *
    * # Return
    * true if the Edge is successfuly added, false otherwise.
    */
    pub fn add_edge_opt_v(&mut self,
                         to_key: K,
                         value: Option<V>)
                         -> bool {
        if !self.edge_exist(&to_key) {
            match value {
                Some(v) => VertexUtils::add_edge(&mut self.edges,
                                                 box Edge::new_with_value(to_key,
                                                                       v)),
                None    => VertexUtils::add_edge(&mut self.edges,
                                                 box Edge::new(to_key))
            }
            true
        } else {
            false
        }
    }

    /**
    * Add an Edge to the Vertex with a value.
    *
    * # Arguments
    * * to_key - The second Vertex of the Edge
    * * cost - The value to attach to the Edge
    *
    * # Return
    * true if the Edge is successfuly added, false otherwise.
    */
    pub fn add_edge_v(&mut self,
                      to_key: K,
                      value: V)
                      -> bool {
        if !self.edge_exist(&to_key) {
            VertexUtils::add_edge(&mut self.edges,
                                  box Edge::new_with_value(to_key, value));
            true
        } else {
            false
        }
    }

    /**
    * Add an Edge to the Vertex without value to attach.
    *
    * # Arguments
    * * to_key - The second Vertex of the Edge
    *
    * # Return
    * true if the Edge is successfuly added, false otherwise.
    */
    pub fn add_edge(&mut self,
                    to_key: K)
                    -> bool {
        if !self.edge_exist(&to_key) {
            VertexUtils::add_edge(&mut self.edges, box Edge::new(to_key));
            true
        } else {
            false
        }
    }

    /**
    * Remove an Edge attached to a Vertex.
    *
    * # Arguments
    * * to_key - The key of the second Vertex of the Edge
    *
    * # Return
    * true if the Edge is successfuly added, false otherwise.
    */
    pub fn remove_edge(&mut self,
                       to_key: K)
                       -> bool {
        if self.edge_exist(&to_key) {
            VertexUtils::remove_edge(&mut self.edges, to_key);
            true
        } else {
            false
        }
    }

    /**
    * Set an Edge value with an optional value.
    *
    * # Arguments
    * * to_key - The key of the second Vertex of the Edge
    * * new_value - The optional value to set to the Edge
    *
    * # Return
    * true if the value is set successfully, false otherwise.
    */
    pub fn set_edge_value_opt(&mut self,
                              to_key: K,
                              new_value: Option<V>)
                              -> bool {
        if self.edge_exist(&to_key) {
            VertexUtils::update_edge_value(&mut self.edges, new_value, to_key);
            true
        } else {
            false
        }
    }

    /**
    * Set the value of an Edge.
    *
    * # Arguments
    * * to_key - The key of the second Vertex of the Edge
    * * new_value - The value to set to the Edge
    *
    * # Return
    * true if the value is set successfully, false otherwise.
    */
    pub fn set_edge_value(&mut self,
                          to_key: K,
                          new_value: V)
                          -> bool {
        if self.edge_exist(&to_key) {
            VertexUtils::update_edge_value(&mut self.edges,
                                           Some(new_value), to_key);
            true
        } else {
            false
        }
    }

    /**
    * Remove the value of an Edge.
    *
    * # Arguments
    * * to_key - The key of the second Vertex of the Edge
    *
    * # Return
    * true if the value is removed successfully, false otherwise.
    */
    pub fn remove_edge_value(&mut self,
                             to_key: K)
                             -> bool {
        if self.edge_exist(&to_key) {
            VertexUtils::update_edge_value(&mut self.edges, None, to_key);
            true
        } else {
            false
        }
    }
}

mod VertexUtils {
    use super::{Edge};

    pub fn update_edge_value<K: Eq, V>(edge: &mut Option<Box<Edge<K, V>>>,
                                       value: Option<V>,
                                       to_key: K) -> () {
        match *edge {
            Some(ref mut e) => {
                if e.to_key == to_key {
                    e.value = value;
                } else {
                    update_edge_value(&mut e.next, value, to_key)
                }
            },
            None => {}
        }
    }

    pub fn remove_edge<K: Eq, V>(edge: &mut Option<Box<Edge<K, V>>>,
                                 to_key: K) {
        match *edge {
            Some(ref mut e) => {
                if e.next.get_ref().to_key == to_key {
                    if e.next.get_ref().next.is_some() {
                        e.next = Some(e.next.take_unwrap().next.take_unwrap());
                    } else {
                        e.next = None;
                    }

                } else {
                    remove_edge(&mut e.next, to_key)
                }
            },
            None => {}
        }
    }

    pub fn get_edge_imm<'r, K: Eq, V>(edge: &'r Option<Box<Edge<K, V>>>,
                                      to_key: &K)
                                      -> Option<&'r Box<Edge<K, V>>> {
        match *edge {
            Some(ref e) => {
                if e.to_key == *to_key {
                    edge.as_ref()
                } else {
                    get_edge_imm(&e.next, to_key)
                }
            },
            None => None
        }
    }

    pub fn add_edge<K: ToStr + Ord + Eq + Clone,
                    V: ToStr + Ord + Eq + Clone>
                    (edge: &mut Option<Box<Edge<K, V>>>,
                    new_edge: Box<Edge<K, V>>) {
        match *edge {
            Some(ref mut e) => add_edge(&mut e.next, new_edge),
            None => {*edge = Some(new_edge)}
        }
    }
}

/**
* The represantation of an Edge.
*
* # Types parameters
* * K - The Vertex's Key type
* * V - The Edge's Value type
*/
#[deriving(Clone, Eq, Encodable, Decodable)]
pub struct Edge<K, V> {
    value:             Option<V>,
    to_key:            K,
    next:              Option<Box<Edge<K, V>>>
}

/// An Iterator to iterate othe the Edge of a Vertex
// #[deriving(Clone)]
pub struct EdgeIterator<'s, K, V> {
    head: &'s Option<Box<Edge<K, V>>>,
}

impl<'s,
     K: ToStr + Ord + Eq + Clone,
     V: ToStr + Ord + Eq + Clone>
     Iterator<(&'s K, Option<&'s V>)> for EdgeIterator<'s, K, V> {
    /**
    * Get the next iterator of the Edge.
    *
    * # Return
    * Return an Option containing a tuple with the key of the Vertex and it
    * cost or None.
    */
    #[inline]
    fn next(&mut self) -> Option<(&'s K, Option<&'s V>)> {
        self.head.as_ref().map( |head| {
            self.head = &head.next;
            (&head.to_key, Some(head.value.get_ref()))
        })
    }
}

impl<K: ToStr + Ord + Eq + Clone,
     V: ToStr + Ord + Eq + Clone>
     Edge<K, V> {

    /**
    * Create a new edge between two Vertex.
    *
    * # Arguments
    * * to_key - The key which represent the second Vertex of the Edge
    *
    * # Return
    * A new Edge.
    */
    pub fn new(to_key: K) -> Edge<K, V> {
        Edge {
            value:  None,
            to_key: to_key,
            next:   None
        }
    }

    /**
    * Create a new edge between two Vertex, with an optional value attached to.
    *
    * # Arguments
    * * to_key - The key which represent the second Vertex of the Edge
    * * value - The optional value to attach to the Edge
    *
    * # Return
    * A new Edge.
    */
    pub fn new_with_opt(to_key: K,
                        value: Option<V>)
                        -> Edge<K, V> {
        Edge {
            value:  value,
            to_key: to_key,
            next:   None
        }
    }

    /**
    * Create a new edge between two Vertex, with a value attached to.
    *
    * # Arguments
    * * to_key - The key which represent the second Vertex of the Edge
    * * value - The value to attach to the Edge
    *
    * # Return
    * A new Edge.
    */
    pub fn new_with_value(to_key: K,
                           value: V)
                           -> Edge<K, V> {
        Edge {
            value:   Some(value),
            to_key: to_key,
            next:   None
        }
    }
}

/**
* Representation of the Graph.
*
* # Types parameters
* * K - The Vertex's Key type
* * L - The Vertex's Label type
* * V - The Edge's Value type
*/
#[deriving(Clone, Eq, Encodable, Decodable)]
pub struct Graph<K, L, V> {
    vertices:      Option<Box<Vertex<K, L, V>>>,
    len:           uint,
    directed:      bool
}

impl<K: ToStr + Ord + Eq + Clone,
     L: ToStr + Ord + Eq + Clone,
     V: ToStr + Ord + Eq + Clone>
     Graph<K, L, V> {

    /**
    * Create a new empty Graph.
    *
    * # Return
    * A new empty graph.
    */
    pub fn new() -> Graph<K, L, V> {
        Graph {
            vertices:   None,
            len:        0,
            directed:   true
        }
    }

    /**
    * Create a new Graph with an vector of Vertex.
    *
    * Warning: The validity of the vector of vertex is not certified.
    *
    * # Arguments
    * * vertices - The vector of Vertiex to attach to the Graph
    *
    * # Return
    * A new graph with initialized with vertices.
    */
    pub fn new_with_vertices(mut vertices: ~[Box<Vertex<K, L, V>>]) -> Graph<K, L, V> {
        let tmp_vertice: Option<Box<Vertex<K, L, V>>> = vertices.shift();
        let mut graph = Graph {
            vertices:   tmp_vertice,
            len:        0,
            directed:   true
        };
        for i in vertices.move_iter() {
            GraphUtils::add_vertex(&mut graph.vertices, i);
        }
        graph
    }

    /**
    * Is the Graph directed or not.
    *
    * # Return
    * true if the graph is directed, false otherwise.
    */
    pub fn is_directed(&self) -> bool {
        self.directed
    }

    /**
    * Add a Vertex to the Graph with a Key and an optional Label.
    *
    * Check if the vertex already exist.
    *
    * # Arguments
    * * key - The key of the new Vertex
    * * label - The option Label to attach to the graph
    *
    * # Return
    * true if the Vertex is successfully added, false otherwise.
    */
    pub fn add_vertex_opt_l(&mut self,
                            key: K,
                            label: Option<L>)
                            -> bool {
        if !self.vertex_exist(&key) {
            match label {
                Some(l) => GraphUtils::add_vertex(&mut self.vertices,
                                                  box Vertex::new_with_label(key,
                                                                          l)),
                None    => GraphUtils::add_vertex(&mut self.vertices,
                                                  box Vertex::new(key))
            }
            self.len += 1;
            true
        } else {
            false
        }
    }

    /**
    * Add a Vertex to the Graph with a Key and a Label.
    *
    * Check if the vertex already exist.
    *
    * # Arguments
    * * key - The key of the new Vertex
    * * label - The Label to attach to the graph
    *
    * # Return
    * true if the Vertex is successfully added, false otherwise.
    */
    pub fn add_vertex_l(&mut self,
                            key: K,
                            label: L)
                            -> bool {
        if !self.vertex_exist(&key) {
            GraphUtils::add_vertex(&mut self.vertices,
                                   box Vertex::new_with_label(key, label));
            self.len += 1;
            true
        } else {
            false
        }
    }

    /**
    * Add a Vertex to the Graph with a Key.
    *
    * Check if the vertex already exist.
    *
    * # Arguments
    * * key - The key of the new Vertex
    *
    * # Return
    * true if the Vertex is successfully added, false otherwise.
    */
    pub fn add_vertex(&mut self,
                      key: K)
                      -> bool {
        if !self.vertex_exist(&key) {
            GraphUtils::add_vertex(&mut self.vertices, box Vertex::new(key));
            self.len += 1;
            true
        } else {
            false
        }
    }

    /**
    * Get an Option to an immutable reference to a vertex.
    *
    * # Arguments
    * * vertex_key - The key of the vertex to return
    *
    * # Return
    * Some(vertex) if it exist, None otherwise.
    */
    pub fn get_vertex<'r>(&'r self,
                          vertex_key: K)
                          -> Option<&'r Box<Vertex<K, L, V>>> {
        GraphUtils::get_vertex_imm(&self.vertices, &vertex_key)
    }

    /**
    * Get an Option to a mutable reference to a vertex.
    *
    * If you want to update the Vertex data you should prefer to update it
    * directly from graph function. See:
    *
    * * set_vertex_label_opt, set_vertex_label, remove_vertex_label.
    * * add_edge_opt_v, add_edge_v, add_edge.
    * * set_edge_value_opt, set_edge_value, remove_edge_value.
    *
    * # Arguments
    * * vertex_key - The key of the vertex to return
    *
    * # Return
    * Some(vertex) if it exist, None otherwise.
    */
    pub fn get_vertex_mut<'r>(&'r mut self,
                              vertex_key: K)
                              -> Option<&'r mut Box<Vertex<K, L, V>>> {
        GraphUtils::get_vertex_mut(&mut self.vertices, vertex_key)
    }

    /**
    * Iterate over the vertices of the Graph.
    *
    * # Return
    * An immutable iterator to the vertices of the Graph.
    */
    pub fn vertices_iter<'r>(&'r self) -> VertexIterator<'r, K, L, V> {
        VertexIterator {
            head: &self.vertices
        }
    }

    /**
    * Set the label of a Vertex with an optional label.
    *
    * # Arguments
    * * vertex_key - The key of the vertex to set
    * * new_label - The optional label to set to the Vertex
    *
    * # Return
    * true if the label is successfully set, false otherwise.
    */
    pub fn set_vertex_label_opt(&mut self,
                                vertex_key: K,
                                new_label: Option<L>)
                                -> bool {
        if self.vertex_exist(&vertex_key) {
            GraphUtils::update_vertex_label(&mut self.vertices,
                                            new_label, vertex_key);
            true
        } else {
            false
        }
    }

    /**
    * Set the label of a Vertex.
    *
    * # Arguments
    * * vertex_key - The key of the vertex to set
    * * new_label - The new label of the Vertex
    *
    * # Return
    * true if the label is successfully set, false otherwise.
    */
    pub fn set_vertex_label(&mut self,
                            vertex_key: K,
                            new_label: L)
                            -> bool {
        if self.vertex_exist(&vertex_key) {
            GraphUtils::update_vertex_label(&mut self.vertices,
                                            Some(new_label),
                                            vertex_key);
            true
        } else {
            false
        }
    }

    /**
    * Remove the label of a Vertex
    *
    * # Arguments
    * * vertex_key - The key of the vertex
    *
    * # Return
    * true if the label is successfully set, false otherwise
    */
    pub fn remove_vertex_label(&mut self,
                               vertex_key: K)
                               -> bool {
        if self.vertex_exist(&vertex_key) {
            GraphUtils::update_vertex_label(&mut self.vertices,
                                            None,
                                            vertex_key);
            true
        } else {
            false
        }
    }

    /**
    * Get the label of a Vertex.
    *
    * # Arguments
    * * vertex_key - The key of the vertex
    *
    * # Return
    * Some(label) if there is a label, None otherwise.
    */
    pub fn get_vertex_label<'r>(&'r self,
                            vertex_key: K)
                            -> Option<&'r L> {
        if self.vertex_exist(&vertex_key) {
            GraphUtils::get_vertex_imm(&self.vertices,
                                       &vertex_key).unwrap().get_label()
        } else {
            None
        }
    }

    /**
    * Add an Edge to the graph with an optional value attached to.
    *
    * # Arguments
    * * from_key - The Key of the first Vertex of the Edge
    * * to_key - The Kye of the second Vertex of the Edge
    * * value - The optional value to attach to the Edge
    *
    * # Return
    * true if the edge is successfully added, false otherwise.
    */
    pub fn add_edge_opt_v(&mut self,
                         from_key: K,
                         to_key: K,
                         value: Option<V>)
                         -> bool {
        if self.vertex_exist(&from_key) &&
           self.vertex_exist(&to_key) {
            GraphUtils::get_vertex_mut(&mut self.vertices,
                                       from_key).unwrap().add_edge_opt_v(to_key,
                                                                         value)
        } else {
            false
        }
    }

    /**
    * Add an Edge to the graph with a value attached to.
    *
    * # Arguments
    * * from_key - The Key of the first Vertex of the Edge
    * * to_key - The Kye of the second Vertex of the Edge
    * * value - The value to attach to the Edge
    *
    * # Return
    * true if the edge is successfully added, false otherwise.
    */
    pub fn add_edge_v(&mut self,
                      from_key: K,
                      to_key: K,
                      value: V)
                      -> bool {
        if self.vertex_exist(&from_key) &&
           self.vertex_exist(&to_key) {
            GraphUtils::get_vertex_mut(&mut self.vertices,
                                       from_key).unwrap().add_edge_v(to_key,
                                                                     value)
        } else {
            false
        }
    }

    /**
    * Add an Edge to the graph.
    *
    * The both Vertex should exist.
    *
    * # Arguments
    * * from_key - The Key of the first Vertex of the Edge
    * * to_key - The Kye of the second Vertex of the Edge
    *
    * # Return
    * true if the edge is successfully added, false otherwise.
    */
    pub fn add_edge(&mut self,
                    from_key: K,
                    to_key: K)
                    -> bool {
        if self.vertex_exist(&from_key) &&
           self.vertex_exist(&to_key) {
            GraphUtils::get_vertex_mut(&mut self.vertices,
                                       from_key).unwrap().add_edge(to_key)
        } else {
            false
        }
    }

    /**
    * Set an optional value to an Edge.
    *
    * The both Vertex should exist.
    *
    * # Arguments
    * * from_key - The first Key of the Edge
    * * to_key - The second Key of the Edge
    * * new_value - The optional value to set to the Edge
    *
    * # Return
    * true if the value is successfully set, false otherwise
    */
    pub fn set_edge_value_opt(&mut self,
                             from_key: K,
                             to_key: K,
                             new_value: Option<V>)
                             -> bool {
        if self.vertex_exist(&from_key) &&
           self.vertex_exist(&to_key) {
            GraphUtils::get_vertex_mut(&mut self.vertices,
                                       from_key).unwrap().set_edge_value_opt(to_key,
                                                                             new_value)
        } else {
            false
        }
    }

    /**
    * Set a value of an Edge.
    *
    * The both Vertex should exist.
    *
    * # Arguments
    * * from_key - The first Key of the Edge
    * * to_key - The second Key of the Edge
    * * new_value - The value to set to the Edge
    *
    * # Return
    * true if the value is successfully set, false otherwise
    */
    pub fn set_edge_value(&mut self,
                          from_key: K,
                          to_key: K,
                          new_value: V)
                          -> bool {
        if self.vertex_exist(&from_key) &&
           self.vertex_exist(&to_key) {
            GraphUtils::get_vertex_mut(&mut self.vertices,
                                       from_key).unwrap().set_edge_value(to_key,
                                                                         new_value)
        } else {
            false
        }
    }

    /**
    * Remove a value of an Edge.
    *
    * The both Vertex should exist.
    *
    * # Arguments
    * * from_key - The first Key of the Edge
    * * to_key - The second Key of the Edge
    *
    * # Return
    * true if the value is successfully removed, false otherwise
    */
    pub fn remove_edge_value(&mut self,
                             from_key: K,
                             to_key: K)
                             -> bool {
        if self.vertex_exist(&from_key) &&
           self.vertex_exist(&to_key) {
            GraphUtils::get_vertex_mut(&mut self.vertices,
                                       from_key).unwrap().remove_edge_value(to_key)
        } else {
            false
        }
    }

    /**
    * Check if a Vertex exist or not.
    *
    * # Arguments
    * * vertex_key - The key of the Vertex to check
    *
    * # Return
    * true if the Vertex exist, false otherwise.
    */
    pub fn vertex_exist(&self,
                        vertex_key: &K)
                        -> bool {
        match GraphUtils::get_vertex_imm(&self.vertices, vertex_key) {
            Some(_) => true,
            None    => false
        }
    }

    /**
    * Check if an Edge exist or not.
    *
    * # Arguments
    * * from_key - The key of the first Vertex of the Edge to check
    * * to_key - The key of the second Vertex of the Edge to check
    *
    * # Return
    * true if the Edge exist, false otherwise.
    */
    pub fn edge_exist(&self,
                      from_key: K,
                      to_key: K)
                      -> bool {
        match GraphUtils::get_vertex_imm(&self.vertices, &from_key) {
            Some(v) => v.edge_exist(&to_key),
            None    => false
        }
    }

    /**
    * Check if two Vertex are adjacent.
    *
    * The two Vertex should exist in the Graph.
    *
    * # Arguments
    * * from_key - The key of the first Vertex of the Edge
    * * to_key - The key of the second Vertex of the Edge
    *
    * # Return
    * true if the two Vertex are adjacent, false otherwise.
    */
    pub fn adjacent(&self,
                    from_key: K,
                    to_key: K)
                    -> bool {
        if self.vertex_exist(&from_key) &&
           self.vertex_exist(&to_key) {
            GraphUtils::get_vertex_imm(&self.vertices,
                                       &from_key).unwrap().edge_exist(&to_key)
        } else {
            false
        }
    }

    /**
    * Remove an Edge.
    *
    * The both Vertex should exist.
    *
    * # Arguments
    * * from_key - The first Key of the Edge
    * * to_key - The second Key of the Edge
    *
    * # Return
    * true if the edge is successfully removed, false otherwise
    */
    pub fn remove_edge(&mut self,
                       from_key: K,
                       to_key: K)
                       -> bool {
        if self.vertex_exist(&from_key) &&
           self.vertex_exist(&to_key) {
            GraphUtils::get_vertex_mut(&mut self.vertices,
                                       from_key).unwrap().remove_edge(to_key)
        } else {
            false
        }
    }

    /**
    * Remove a Vertex.
    *
    * The Vertex should exist.
    *
    * # Arguments
    * * vertex_key - The first Key of the Edge
    *
    * # Return
    * true if the Vertex is successfully removed, false otherwise
    */
    pub fn remove_vertex(&mut self,
                         vertex_key: K)
                         -> bool {
        if self.vertex_exist(&vertex_key) {
            GraphUtils::remove_vertex(&mut self.vertices, vertex_key.clone());
            GraphUtils::remove_edge_to(&mut self.vertices, vertex_key);
            true
        } else {
            false
        }
    }
}

impl<K, L, V> Container for Graph<K, L, V> {
    fn len(&self) -> uint {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<K: ToStr + Ord + Eq + Clone,
     L: ToStr + Ord + Eq + Clone,
     V: ToStr + Ord + Eq + Clone>
     Mutable for Graph<K, L, V> {
    /// Clear the Graph, removing all Vertices and edges
    fn clear(&mut self) {
        self.vertices = None;
        self.len = 0;
    }
}

mod GraphUtils {
    use super::{Vertex};

    pub fn remove_vertex<K: Eq, L, V>(vertex: &mut Option<Box<Vertex<K, L, V>>>,
                                      key: K) -> () {
        match *vertex {
            Some(ref mut v) => {
                if v.next.get_ref().key == key {
                    if v.next.get_ref().next.is_some() {
                        v.next = Some(v.next.take_unwrap().next.take_unwrap());
                    } else {
                        v.next = None;
                    }

                } else {
                    remove_vertex(&mut v.next, key)
                }
            },
            None => {}
        }
    }

    pub fn remove_edge_to<K: ToStr + Ord + Eq + Clone,
                          L: ToStr + Ord + Eq + Clone,
                          V: ToStr + Ord + Eq + Clone>
                          (vertex: &mut Option<Box<Vertex<K, L, V>>>, key: K) -> () {
        match *vertex {
            Some(ref mut v) => {
                if v.edge_exist(&key) {
                    v.remove_edge(key.clone());
                }
                remove_edge_to(&mut v.next, key);
            },
            None => {}
        }
    }

    pub fn get_vertex_mut<'r, K: Eq, L, V>(vertex: &'r mut Option<Box<Vertex<K, L, V>>>,
                                           key: K) -> Option<&'r mut Box<Vertex<K, L, V>>> {
        match *vertex {
            Some(ref mut v) => {
                if v.key == key {
                    Some(v)
                } else {
                    get_vertex_mut(&mut v.next, key)
                }
            },
            None => None
        }
    }

    pub fn get_vertex_imm<'r, K: Eq, L, V>(vertex: &'r Option<Box<Vertex<K, L, V>>>,
                                           key: &K) -> Option<&'r Box<Vertex<K, L, V>>> {
        match *vertex {
            Some(ref v) => {
                if v.key == *key {
                    vertex.as_ref()
                } else {
                    get_vertex_imm(&v.next, key)
                }
            },
            None => None
        }
    }

    pub fn update_vertex_label<K: Eq, L, V>(vertex: &mut Option<Box<Vertex<K, L, V>>>,
                                            label: Option<L>,
                                            key: K) -> () {
        match *vertex {
            Some(ref mut v) => {
                if v.key == key {
                    v.label = label;
                } else {
                    update_vertex_label(&mut v.next, label, key)
                }
            },
            None => {}
        }
    }

    pub fn add_vertex<K: ToStr + Ord + Eq + Clone,
                      L: ToStr + Ord + Eq + Clone,
                      V: ToStr + Ord + Eq + Clone>
                      (vertex: &mut Option<Box<Vertex<K, L, V>>>,
                      new_vertex: Box<Vertex<K, L, V>>) -> () {
        match *vertex {
            Some(ref mut v) => add_vertex(&mut v.next, new_vertex),
            None            => *vertex = Some(new_vertex)
        }
    }
}

