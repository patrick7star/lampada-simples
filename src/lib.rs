
// biblioteca do programa:
mod constroi_simbolos;
use constroi_simbolos::arquivo_para_matriz;
pub mod palavras_filtro;

// bibliotecas externas:
pub extern crate pancurses;
use pancurses::*;

// biblioteca padrão do Rust:
use std::fmt::{Formatter, Debug, Display, Result as R};
use std::cmp::{Ordering, PartialOrd, PartialEq};


/* objeto auxiliar na configuração e manipulação
 * do objeto lâmpada.
 */

/* dimensão que marca a largura e altura do
 * objeto abstraído como uma imagem. 
 */
#[derive(Copy, Clone)]
pub struct Dimensao { pub largura:u16, pub altura:u16 }

impl Display for Dimensao {
   fn fmt(&self, formatador:&mut Formatter<'_>) -> R{
      return write!(formatador, "{0}x{1}", 
                     self.largura, self.altura);
   }
}

impl PartialEq for Dimensao {
   fn eq(&self, d:&Dimensao) -> bool {
      d.largura == self.largura &&
      d.altura == self.altura
   }
   fn ne(&self, d:&Dimensao) -> bool {
      return !self.eq(d);
   }
}

impl PartialOrd for Dimensao {
   fn partial_cmp(&self, _d:&Dimensao) -> Option<Ordering> { 
      // para encher linguiça...
      return Some(Ordering::Equal);
   }
   fn gt(&self, d:&Dimensao) -> bool {
      let minha_area = self.largura * self.altura;
      let argumento_area = d.largura * d.altura;
      return minha_area > argumento_area;
   }
   fn ge(&self, d:&Dimensao) -> bool {
      return self.gt(d) || self == d;
   }
   fn lt(&self, d:&Dimensao) -> bool {
      let minha_area = self.largura * self.altura;
      let argumento_area = d.largura * d.altura;
      return minha_area < argumento_area;
   }
   fn le(&self, d:&Dimensao) -> bool {
      return self.lt(d) || self == d;
   }
}


// ponto para marcar as posições na "Tela".
#[derive(Copy,Clone)]
pub struct Ponto { pub y:u16, pub x: u16 }

// com seus operadores importantes devidamente implementados.
impl Display for Ponto {
   fn fmt(&self, formatador:&mut Formatter<'_>) -> R{
      return write!(formatador, "linha={0} coluna={1}", 
                     self.y, self.x);
   }
}

impl Debug for Ponto {
   fn fmt(&self, formatador:&mut Formatter<'_>) -> R{
      return write!(formatador, "y={0} x={1}", self.x, self.y);
   }
}

impl PartialEq for Ponto {
   fn eq(&self, ponto:&Ponto) -> bool {
      self.y == ponto.y && self.x == ponto.x
   }
   fn ne(&self, ponto:&Ponto) -> bool {
      return !(self.eq(ponto));
   }
}


/* retorna todas as coordenadas que serão pintadas, 
 * ou mudadas, e representam abstratamente o 
 * interior da lâmpada. 
 */
fn computando_pixels_interno_ao_bulbo(matriz:&Vec<Vec<char>>, 
   dim:Dimensao) -> Vec<Ponto> 
{
   // se está dentro do bulbo.
   let mut dentro:bool = false;
   // espaços vázios dentro do bulbo.
   let mut pixels:Vec<Ponto> = Vec::new();

   // diminui um, pois o final é apenas a borda da làmpada.
   for i in 0..dim.altura-1 {
      for j in 0..dim.largura {
         /* se achar a linha de contorno mais a 
          esquerda, então começar a filtrar espaços
          vázios, quando encontrar o mais à 
          direita, desativar "modo filtro". */
         if matriz[i as usize][j as usize] == '#' 
            { dentro = !dentro; }
         /* só filtra se for um espaço em branco e,
          estiver entre as linhas de contornos. */
         if dentro && matriz[i as usize][j as usize].is_whitespace() 
            { pixels.push(Ponto{y:i, x:j}); }
      }
   }
   return pixels;
}


/* mesmo que a anterior, todos caractéres(pontos)
 * que serão mudados e pintados, e representam 
 * abstratamente o filete dentro do bulbo da lampada.
 */
fn computa_pixels_filete_do_bulbo(p:Ponto) -> Vec<Ponto> {
   // comprimento adiciona para levar em conta a posição.
   let l = p.x;
   let h = p.y;
   // criando pontos relativos aos pontos.
   vec![Ponto{x:7+l, y:4+h}, Ponto{x:7+l, y:5+h},
      Ponto{x:7+l, y:6+h}, Ponto{x:6+l, y:7+h},
      Ponto{x:8+l, y:7+h}]
}


/* o objeto lâmpada em sí, e todos seus atributos
 * e propriedades, o que ela guarda de informação.
 */
pub struct Lampada {
   // representação matricial dela.
   grade:Vec<Vec<char>>,
   /* posição dela na tela, referente ao
   * canto-superior-esquerdo(CSE). */
   posicao:Ponto,
   // sua dimensão como "figura", largura x altura.
   dimensao:Dimensao,
   // posições internas do bulbo a iluminar.
   pub pixels_interno_lampada:Vec<Ponto>,
   // coordenadas do filete(resistor) da lâmpada.
   pub pixels_filete:Vec<Ponto>,
   // estado da lampada.
   pub apagada:bool,
   // ascendida primeira vez.
   ligado_uma_vez:bool,
}

// implementação das ações(métodos) que a lâmpada
// realiza no seu objeto.
impl Lampada {
   pub fn monta_uma(acessa:bool) -> Lampada {
      // matriz com desenho "pixelado" nela formato retangular.
      let matriz_desenho = arquivo_para_matriz("lampada_molde.txt");
      // dimensões do desenho plotado na matriz.
      let l:u16 = matriz_desenho[0].len() as u16;
      let h:u16 = matriz_desenho.len() as u16;
      // pixels de elementos da figura.
      let pixels_fb = computa_pixels_filete_do_bulbo(Ponto{x:0, y:0});
      let pixels_ib = computando_pixels_interno_ao_bulbo(&matriz_desenho, Dimensao{largura:l, altura:h});

      // criando istância do objeto e movendo todos
      // objetos criado acima para ele.
      return Lampada {
         grade: matriz_desenho,
         apagada: !acessa,
         pixels_filete:pixels_fb, 
         pixels_interno_lampada:pixels_ib,
         dimensao: Dimensao{largura:l, altura:h},
         // centralizando a posição da lâmpada...
         posicao:Ponto{x:0, y:0},
         ligado_uma_vez:false
      };
   }
   pub fn desenha_na_tela(&self, tela:&Window) {
      // desenhando "pixel por pixel" na tela.
      for i in 0..self.dimensao.altura {
         for j in 0..self.dimensao.largura { 
            let ch = self.grade[i as usize][j as usize];
            if ch == '*' {
               tela.color_set(3);
               tela.attrset(A_BOLD);
            }
            else if ch == '#' {
               tela.attrset(A_BOLD);
               tela.color_set(4);
               //tela.mvaddch(i as i32, j as i32, ch);
            }
            else if ch == '|' || ch == '\\' || ch == '/' {
               tela.attrset(A_NORMAL);
               tela.color_set(3);
            }
            // agora escrevendo...
            tela.mvaddch(
               (self.posicao.y as i32) + i as i32, 
               (self.posicao.x as i32) + j as i32, ch
            );
         }
      }
      tela.refresh();
   }
   fn ascende_filete(&self, tela:&Window) {
      // desenhando "pixel por pixel" na tela.
      tela.attrset(A_BOLD);
      tela.color_set(1);

      for p in &self.pixels_filete {
         let (i,j) = (p.y,p.x);
         tela.mvaddch(
            //(self.posicao.y as i32) + (i as i32), 
            //(self.posicao.x as i32) + (j as i32), '%'
            i as i32, j as i32, '%'
         );
      }
      tela.refresh();
   }
   fn ascende_bulbo(&self, tela:&Window) {
      // desenhando "pixel por pixel" na tela.
      tela.attrset(A_BOLD);
      tela.color_set(2);
      for p in &self.pixels_interno_lampada {
         let (i,j) = (p.y,p.x);
         tela.mvaddch(
            //(self.posicao.y as i32) + i as i32, 
            //(self.posicao.x as i32) + j as i32, '@'
            i as i32, j as i32, '@'
         );
      }
      /* dependendo se foi a primeira vez ligada,
       * agiliza ou não. */
      if self.ligado_uma_vez { napms(80); }
      else { napms(800); }
      // refresca a tela.
      tela.refresh();
   }

   pub fn ascende(&mut self, tela:&Window) {
      self.ascende_filete(tela);
      self.ascende_bulbo(tela);
      self.apagada = false;
      // marca se é primeira vez ascessa, para agilizar nas próximas.
      if !self.ligado_uma_vez {
         self.ligado_uma_vez = true;
      }
   }
   
   pub fn desliga(&mut self, tela:&Window) {
      tela.clear();
      self.desenha_na_tela(tela);
      self.apagada = true;
   }

   pub fn centraliza(&mut self, tela:&Window) {
      // obtendo quantia de colunas da telinha.
      let largura_tela:u16 = tela.get_max_x() as u16;
      self.posicao.x = 6+(largura_tela/2) - self.dimensao.largura ;
      // atualizando grades também.
      for coord in &mut self.pixels_interno_lampada {
         coord.x += self.posicao.x;
      }
      for coord in &mut self.pixels_filete {
         coord.x += self.posicao.x;
      }
      // apaga tela.
      tela.erase();
      // refaz desenho, agora melhor posicionado.
      self.desenha_na_tela(tela);
   }
}


/* verifica se o ponto dado é um "píxel" da figura,
 * ou seja, localizado na mesma coordenada.
 */
pub fn toque_na_lampada(ponto:Ponto, lampada:&Lampada) -> bool {
   lampada.pixels_interno_lampada.contains(&ponto) ||
   lampada.pixels_filete.contains(&ponto)
}


/* enum de botão com todas opções disponíveis.
 * se tocou no texto que também representa botões.
 * a função parte de uma posição estática 
 * de tais textos/botões. 
 */
pub fn tocou_botoes(pt:&Ponto, tl:&Window) -> Option<Botao> {
   // obtém dimensão da tela.
   let lin = tl.get_max_y(); 
   
   // verificando se está na linha certa.
   if pt.y as i32 == lin-1 {
      // contado com três espaços entre as strings.
      if pt.x <= 18 
         { Some(Botao::Liga) }
      else if pt.x >= 22 && pt.x <= 41 
         { Some(Botao::Desliga) }
      else if pt.x >= 45 && pt.x <= 51 
         { Some(Botao::Sair) }
      else { None }
   }
   else { None }
}

// enum com ações a executar.
pub enum Botao { Sair, Liga, Desliga }
