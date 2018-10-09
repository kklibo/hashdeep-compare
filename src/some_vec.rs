use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub struct SomeVec<T> {
    v: Vec<T>,
    _prevent_struct_literals: (),
}

impl<T> SomeVec<T> {

    pub fn from_vec(v: Vec<T>) -> Option<SomeVec<T>> {

        match v.is_empty() {
            true => None,
            false => Some(SomeVec{v, _prevent_struct_literals: ()}),
        }
    }

    pub fn from_first_value(value: T) -> SomeVec<T> {
        SomeVec{v: vec!{value}, _prevent_struct_literals: ()}
    }

    pub fn from_values(value1: T, value2: T) -> SomeVec<T> {
        SomeVec{v: vec!{value1, value2}, _prevent_struct_literals: ()}
    }

    pub fn len(&self) -> usize {
        self.v.len()
    }

    pub fn first(&self) -> &T {
        &self.v[0]
    }

    pub fn at(&self, index: usize) -> &T {
        &self.v[index]
    }

    pub fn push(&mut self, value: T) {
        self.v.push(value);
    }

    pub fn inner_ref(&self) -> &Vec<T> {
        &self.v
    }

    pub fn sort_by<F>(&mut self, compare: F) where
        F: FnMut(&T, &T) -> Ordering,
    {
        self.v.sort_by(compare);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_vec_and_len_test() {
        assert_eq!(SomeVec::<i32>::from_vec(vec![]), None);
        {
            let v = SomeVec::<i32>::from_vec(vec![1]).unwrap();
            assert_eq!(v.len(), 1);
            assert_eq!(*v.inner_ref(), vec![1]);
            assert_eq!(*v.first(), 1);
        }
        {
            let v = SomeVec::<i32>::from_vec(vec![1,2,3]).unwrap();
            assert_eq!(v.len(), 3);
            assert_eq!(*v.inner_ref(), vec![1,2,3]);
            assert_eq!(*v.first(), 1);
        }
    }

    #[test]
    fn from_first_value_test() {
        {
            let v = SomeVec::<i32>::from_first_value(1);
            assert_eq!(v.len(), 1);
            assert_eq!(*v.inner_ref(), vec![1]);
            assert_eq!(*v.first(), 1);
        }
        {
            let v = SomeVec::<()>::from_first_value(());
            assert_eq!(v.len(), 1);
            assert_eq!(*v.inner_ref(), vec![()]);
            assert_eq!(*v.first(), ());
        }
        {
            let v = SomeVec::<String>::from_first_value("1".to_owned());
            assert_eq!(v.len(), 1);
            assert_eq!(*v.inner_ref(), vec!["1".to_owned()]);
            assert_eq!(*v.first(), "1".to_owned());
        }
    }

    #[test]
    fn from_values_test() {
        {
            let v = SomeVec::<()>::from_values((),());
            assert_eq!(v.len(), 2);
            assert_eq!(*v.inner_ref(), vec![(),()]);
            assert_eq!(*v.first(), ());
        }
        {
            let v = SomeVec::<i32>::from_values(1,2);
            assert_eq!(v.len(), 2);
            assert_eq!(*v.inner_ref(), vec![1,2]);
            assert_eq!(*v.first(), 1);
        }
    }

    /*
    //commented out for now: generates test output noise
    #[test]
    #[should_panic(expected = "index out of bounds: the len is 1 but the index is 1")]
    fn at_out_of_bounds_test() {
        SomeVec::<i32>::from_vec(vec![1]).unwrap().at(1);
    }*/

    #[test]
    fn at_test() {
        assert_eq!(*SomeVec::<i32>::from_vec(vec![1,2,3]).unwrap().at(1), 2);
        assert_eq!(*SomeVec::<String>::from_vec(vec!["1".to_owned()]).unwrap().at(0), "1");
    }

    #[test]
    fn push_test() {
        let mut v = SomeVec::<String>::from_vec(vec!["1".to_owned()]).unwrap();
        assert_eq!(v.len(), 1);
        assert_eq!(*v.first(), "1".to_owned());
        assert_eq!(*v.at(0), "1");

        v.push("2".to_owned());
        assert_eq!(v.len(), 2);
        assert_eq!(*v.first(), "1".to_owned());
        assert_eq!(*v.at(0), "1");
        assert_eq!(*v.at(1), "2");
    }

    #[test]
    fn sort_by_test() {
        {
            let mut v = SomeVec::<u32>::from_vec(vec!{1}).unwrap();
            v.sort_by(|x,y| x.cmp(y));
            assert_eq!(v.inner_ref(), &vec!{1});
        }
        {
            let mut v = SomeVec::<u32>::from_vec(vec!{2, 1}).unwrap();
            v.sort_by(|x,y| x.cmp(y));
            assert_eq!(v.inner_ref(), &vec!{1, 2});
        }
        {
            let mut v = SomeVec::<u32>::from_vec(vec!{2, 3, 1}).unwrap();
            v.sort_by(|x,y| x.cmp(y));
            assert_eq!(v.inner_ref(), &vec!{1,2,3});
        }
    }
}