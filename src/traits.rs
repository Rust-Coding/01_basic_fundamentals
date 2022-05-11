
// Trait = rasgo / interface
pub fn traits () {

  let julio = Humano;
  let humano = julio.hablar();
  println!("{}", humano);

  let pelusa = Gato;
  let gato = pelusa.hablar();
  println!("hablar {}", gato);
  println!("idioma {}", Gato::idioma());
  
}

struct Humano;
struct Gato;

trait Hablar {
  
  fn hablar(&self) -> String;

  // Default implementation for struct has to be implemented
  fn idioma() -> String {
    String::from("Español") 
  }

}

impl Hablar for Humano {

  fn hablar(&self) -> String {
    "Hola".to_string()
  }

}

impl Hablar for Gato {

  fn hablar(&self) -> String {
    "Miau".to_string()
  }

  fn idioma() -> String {
    "Gatuno".to_string()
  }

}
