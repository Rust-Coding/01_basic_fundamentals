
pub fn trait_with_option () {

  let edad: Option<i32> = Some(23);
  let es_mayor = edad.es_mayor();
  println!("Es mayor: {}", es_mayor);

}


trait Licencia {
  fn es_mayor( &self ) -> bool;
}

impl Licencia for Option<i32> {
  fn es_mayor( &self ) -> bool {
    match self {
      Some(edad) => edad > &18,
      None => false,
    }
  }
}