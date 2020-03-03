pub trait NodeTraits<T> {
    // required/default helper functions
    fn print_traversal(&self);
    fn count_leaves(&self) -> usize;
    fn get_depth_vec(&self) -> Vec<(T, usize)>;
    fn calc_depth(&self, dep: usize, vec: &mut Vec<(T, usize)>);
    // fn calc_depth(&self, dep: usize, map: &mut HashMap<T, usize>);
    // fn get_depth_map(&self) -> HashMap<T, usize>;

    // required getters for node properties
    fn value(&self) -> Option<T>;

    // setters for node properties
    fn set_value(&self, value: T);
}

pub trait TreeBase<T> {
    fn height(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
    fn contains(&self, value: T) -> bool;
    fn count_leaves(&self) -> usize;
    fn insert_node(&mut self, value: T);
    fn delete_node(&mut self, value: T);
    fn print(&self);
    fn get_by_depth(&self) -> Vec<(T, usize)>;
}

