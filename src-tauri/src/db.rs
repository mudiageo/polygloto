use std::collections::HashMap;

pub trait Identifiable {
    fn id(&self) -> usize;
}

#[derive(Clone)]
pub struct Store<'a,T: Identifiable> {
  name: &'a str,
  data: HashMap<usize, T>,
}

impl<'a, T: Identifiable> Store<'a, T> where T: Identifiable {
 pub fn new(name: &'a str) -> Self{
  let data: HashMap<usize, T> = HashMap::new();
    Store { name, data }
  }
 pub fn insert(&mut self, new_data: T) -> (){
    self.data.insert(new_data.id(), new_data);
  //   new_data.clone()
  }
 pub fn update(&mut self, new_data: T) -> (){
    self.data.entry(new_data.id()).or_insert(new_data);
  //   new_data.clone()
  }
  pub fn get(&self, key: usize) -> &T {
    self.data.get(&key).unwrap()
  }
  pub fn get_all(&self) -> Vec<T>where T: Clone {
    self.data.clone().into_values().collect()
  }
}

pub fn create_store<T: Identifiable>(name: &str) -> Store<T> {
  Store::<T>::new(name)
}

