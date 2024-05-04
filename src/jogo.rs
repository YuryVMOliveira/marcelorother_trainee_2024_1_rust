use crate::carteiro::Carteiro;
use crate::caixa::Caixa;
use std::thread;
use std::time::Duration;

pub struct Jogo {}

impl Jogo {
    pub fn new() -> Self {
        Jogo {}
    }

    pub fn cria_jogo(cx: i32, cy: i32, caixax: i32, caixay: i32) -> (Carteiro, Caixa) {
        let carteiro = Carteiro::new(cx, cy);
        let caixa = Caixa::new(caixax, caixay);
        (carteiro, caixa)
    }

    pub fn joga(mut matriz: [[char; 20]; 20], mut carteiro: Carteiro, caixa: Caixa,  px:i32, py:i32) {
        let mut atual_x = carteiro.get_pos_x();
        let mut atual_y = carteiro.get_pos_y();
        matriz[carteiro.get_pos_x() as usize][carteiro.get_pos_y() as usize] = '&';
        matriz[caixa.get_pos_x() as usize][caixa.get_pos_y() as usize] = '@';
        matriz[px as usize][py as usize] = '#'; 
        thread::sleep(Duration::from_secs(1));
        Self::imprimir_matriz(&matriz);
        thread::sleep(Duration::from_secs(1));
        // Movimento em direção à caixa
        while atual_x != caixa.get_pos_x() {
            let mut novo_x = atual_x;


            matriz[carteiro.get_pos_x() as usize][carteiro.get_pos_y() as usize] = '+';
            if atual_x < caixa.get_pos_x() {
                novo_x += 1; 
            } else if atual_x > caixa.get_pos_x() {
                novo_x -= 1; 
            }
            if novo_x<20&&novo_x>=0{
                carteiro.mover_para(novo_x, atual_y);
                atual_x=novo_x;

            }else {
                break; 
            }
            matriz[carteiro.get_pos_x() as usize][carteiro.get_pos_y() as usize] = '&';
            matriz[px as usize][py as usize] = '#';
            Self::imprimir_matriz(&matriz);
            
             
        }
        while atual_y!=caixa.get_pos_y() {
            let mut novo_y: i32 = atual_y;


            matriz[carteiro.get_pos_x() as usize][carteiro.get_pos_y() as usize] = '+';
            if atual_y < caixa.get_pos_y() {
                novo_y += 1; 
            } else if atual_y > caixa.get_pos_y() {
                novo_y -= 1; 
            }
            if novo_y<20 && novo_y>=0{
                carteiro.mover_para(atual_x, novo_y);
                atual_y=novo_y;

            }else {
                break; 
            }
            matriz[carteiro.get_pos_x() as usize][carteiro.get_pos_y() as usize] = '&'; 
            Self::imprimir_matriz(&matriz);
            matriz[px as usize][py as usize] = '#';
        }
        println!("Carteiro pegou a caixa!");
        println!("");

        thread::sleep(Duration::from_secs(1));
        matriz[carteiro.get_pos_x() as usize][carteiro.get_pos_y() as usize] = '&';
        Jogo::imprimir_matriz(&matriz);

        matriz[px as usize][py as usize] = '#'; 


        // Movimento em direção ao ponto final
        while atual_x != px {
            let mut novo_x = atual_x;
            matriz[carteiro.get_pos_x() as usize][carteiro.get_pos_y() as usize] = '+';
            if atual_x < px {
                novo_x += 1; 
            } else if atual_x > px {
                novo_x -= 1; 
            }
            if novo_x<20 && novo_x>=0{
                carteiro.mover_para(novo_x, atual_y);
                atual_x=novo_x;

            }else {
                break; 
            }
            matriz[carteiro.get_pos_x() as usize][carteiro.get_pos_y() as usize] = '&';
            Jogo::imprimir_matriz(&matriz);
        }
        while (atual_y) != (py) {
            let mut novo_y = atual_y;
            matriz[carteiro.get_pos_x() as usize][carteiro.get_pos_y() as usize] = '+';
            if atual_y < py {
                novo_y += 1; 
            } else if atual_x > py {
                novo_y -= 1; 
            }
            if novo_y<20&&novo_y>=0{
                carteiro.mover_para(atual_x, novo_y);
                atual_y=novo_y;

            }else {
                break; 
            }
            matriz[carteiro.get_pos_x() as usize][carteiro.get_pos_y() as usize] = '&';
            Jogo::imprimir_matriz(&matriz);
        }
        
        matriz[carteiro.get_pos_x() as usize][carteiro.get_pos_y() as usize] = 'X';
        Jogo::imprimir_matriz(&matriz);
        println!("Caixa entregue no ponto final!");
        println!("");

    }

    pub fn imprimir_matriz(matriz: &[[char; 20]; 20]) {
        for linha in matriz {
            for &elemento in linha {
                print!("{} ", elemento);
            }
            println!();
        }
        println!();
        thread::sleep(Duration::from_secs(1));
    }
}
