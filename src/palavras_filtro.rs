//use std::time::{Duration, Instant};

pub struct FormadorPalavra<'a> {
   // palavra que tem que ser formada.
   pub palavra:&'a str,
   // se já foi formado tal palavra.
   formado:bool,
   // string onde é formada.
   string:String,
   // tempo na formação.
   //tempo_inicial:Instant,
}

impl <'a> FormadorPalavra<'a> {
   /// cria um novo formador.
   pub fn novo(p:&str) -> FormadorPalavra {
      FormadorPalavra {
         palavra: p,
         formado:false,
         string:String::from("")
      }
   }
   /// verifica se a palavra foi formada.
   pub fn palavra_esta_formada(&mut self) -> bool {
      // verificando se a string's são iguais.
      let valor = {
         let tp = self.palavra.len(); 
         let ts = self.string.len();

         if tp == ts      
            { self.palavra == self.string.as_str() }
         else if ts > tp
            { self.string.contains(self.palavra) }
         else { false }
            
      };
      if valor { self.formado = true; self.string.clear(); }
      // retornando status...
      return valor;
   }

   /// adiciona um novo caractére válido.
   /// retorna o status sobre a operação de inserção.
   pub fn add_novo_ch(&mut self, ch:&char) -> bool {
      // tem que ser ascii tal novo character.
      if ch.is_ascii() && self.palavra.contains(*ch) {
         self.string.push(*ch);
         // adição um sucesso.
         return true;
      }
      // adicionação fracossou!
      return false;
   }

   /// reseta palavra formada para uma nova rodada.
   pub fn reseta(&mut self) {
      self.formado = false;
      self.string.clear();
   }
}
