/*!
   Faz um programa simples, apenas para uma 
estreia minha no GitHub.
   Ele simplesmente acende uma lâmpada se dado
um clique, e desliga se ele estiver acessa.
*/

// biblioteca do programa:
use lampada_simples::palavras_filtro::FormadorPalavra;
use lampada_simples::pancurses::*;
use lampada_simples::{tocou_botoes, 
                     toque_na_lampada, 
                     Lampada, Ponto,
                     Botao};

// biblioteca padrão do Rust:
use std::time::Instant;
use std::env::args;


// rodando o programa.
fn main() {
   // criando instância do objeto e a tela.
   let mut lamp = Lampada::monta_uma(false);   
   let tela = initscr();

   // configurando "pancurses"...
   start_color();
   noecho();  
   curs_set(0); // tira cursor.
   tela.keypad(true);
   // habilitando o mouse no ncurses...
   let mut null:u32 = 10;
   let resultado = mousemask(BUTTON1_CLICKED, &mut null);

   // paletas de cores.
   init_pair(1, COLOR_RED, COLOR_BLACK);
   init_pair(2, COLOR_YELLOW, COLOR_BLACK);
   init_pair(3, COLOR_WHITE, COLOR_BLACK);
   init_pair(4, COLOR_CYAN, COLOR_BLACK);
   init_pair(5, COLOR_GREEN, COLOR_BLACK);
   // verde bem claro.
   //init_color(8, 20, 252, 20);

   // desenhando tela.
   lamp.desenha_na_tela(&tela);
   // centralizando...
   lamp.centraliza(&tela);

   // comando dando estado da lâmpada.
   match args().skip(1).next() {
      Some(s) => {
         if s == "claro"
            { lamp.ascende(&tela); }
         else if s == "escuro"
            { lamp.desliga(&tela); }
         tela.refresh();
      },
      None => (),
   };
   
   // filtrador de comandos inseridos:
   let mut comando_ligar = FormadorPalavra::novo("claro");
   let mut comando_desligar = FormadorPalavra::novo("escuro");
   
   // marcando tempo.
   let tempo = Instant::now();
   loop { 
      /* barra de status com instruções e informação
       * do estado atual da lâmpada.  */
      barra_status(&tela, &lamp);

      // atual comando(letra) ou sequência(palavra inteira).
      let acao = tela.getch();
      
      match getmouse() {
         /* se ocorrer, então pega o evento de clique do mouse */
         Ok(evento) => {
            // coordenadas e um Ponto dela.
            let (x, y):(i32,i32) = (evento.x, evento.y);
            let ponto = Ponto { y: y as u16, x: x as u16 }; 
            
            /* o interruptor funciona também se
             * clicar nos textos apresentando as 
             * opções; assim como a opção sair.  */
            match tocou_botoes(&ponto, &tela) {
               Some(Botao::Sair) => { break },
               Some(Botao::Liga) =>
                  { lamp.ascende(&tela); },
               Some(Botao::Desliga) =>
                  { lamp.desliga(&tela); },
               None => { 
                  // interruptor básico de lampada.
                  if toque_na_lampada(ponto, &lamp) {
                     if lamp.apagada 
                        { lamp.ascende(&tela); }
                     else 
                        { lamp.desliga(&tela); }
                  }
                  else {
                     // info do erro cometido.
                     tela.mvaddstr(0, 0, format!("linha={} coluna={}",y, x));
                     tela.mvaddstr(1,0, "tem que clicar no bulbo! Ou,");
                     tela.mvaddstr(2,0, "clique no texto para executar"); 
                  }
               }
            };
            
         },
         // se não for, então é um evento do teclado. 
         Err(_) => {
            match acao  {
               // interruptor básico:
               // "HOME", liga lâmpada.
               Some(Input::KeyHome) => { 
                  lamp.ascende(&tela); 
                  comando_desligar.reseta(); 
               },
               // "END", do seu lado, apaga lâmpada.
               Some(Input::KeyEnd) => {
                  lamp.desliga(&tela); 
                  comando_ligar.reseta(); 
               },

               Some(Input::Character(ch)) => {
                  // para abandonar o programa.
                  if ch == 'S' { break }
                  
                  // pega sequência de caractéres, e decodifica.
                  comando_ligar.add_novo_ch(&ch); 
                  comando_desligar.add_novo_ch(&ch);

                  // executando a ação...
                  if comando_ligar.palavra_esta_formada() { 
                     lamp.ascende(&tela); 
                     comando_desligar.reseta(); 
                  }
                  else if comando_desligar.palavra_esta_formada() { 
                     lamp.desliga(&tela); 
                     comando_ligar.reseta();
                  }
                     
               },
               None => { 
                  // atualiza a cada dez segundos.
                  if tempo.elapsed().as_secs() % 10 == 0
                     { lamp.desenha_na_tela(&tela); }
               },
               _ => ()
            };
         },
      };

      tela.refresh();
   }
   // termina "interface-gráfica".   
   endwin();
   println!("estado da ativação do mouse: {}", resultado);

   println!("argumentos passados:{:#?}",args());
}


fn barra_status(tela:&Window, lampada:&Lampada) {
   let linha = tela.get_max_y() - 1; 
   tela.attrset(A_BOLD);
   
   // colore apenas se estiver apagada.
   if lampada.apagada {
      tela.attrset(A_BOLD);
      tela.color_set(5);
      tela.mvaddstr(linha, 0, "ON<Home ou \"claro\">");
   } 
   else {
      tela.attrset(A_NORMAL);
      tela.color_set(3);
      tela.mvaddstr(linha, 0, "ON<Home ou \"claro\">");
   }
   
   // colore apenas se estiver ascessa.
   if !lampada.apagada {
      tela.attrset(A_BOLD);
      tela.color_set(COLOR_RED);
      tela.addstr("   OFF<End ou \"escuro\">");
   } else {
      tela.color_set(3);
      tela.attrset(A_NORMAL);
      tela.addstr("   OFF<End ou \"escuro\">");
   }

   tela.attrset(A_BOLD);
   tela.color_set(3);
   tela.addstr("   sair<S>");
}

