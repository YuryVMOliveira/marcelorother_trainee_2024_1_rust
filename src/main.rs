

//  JOGO DA ENTREGA
//
// O jogo consiste em fazer com que o carteiro ('&') leve a caixa ('@') até ó ponto desejado ('X') em um campo que será uma matriz 20x20, onde ('+') representa um espaço válido..
// Para desenvolver tal projeto vocês terão de usar/desenvolver as estruturas 'carteiro', 'caixa', e 'jogo'.
// Obs:
//   - O carteiro só pode andar um 'índice' por iteração
//   - Apliquem a ideia de Encapsulamento
//   - O código tera uma mapa de exemplo para o teste enquanto estiver em desenvolvimento
//   - No dia da apresentação o código será posto em prática com um código diferente 

mod carteiro;
mod caixa;
mod jogo;


use jogo::Jogo;
use std::io;
use rand::Rng;

fn gerar_coordenadas_unicas() -> Vec<(i32, i32)> {
    let mut rng = rand::thread_rng();
    let mut coordenadas = Vec::new();

    while coordenadas.len() < 3 {
        let x = rng.gen_range(0..20);
        let y = rng.gen_range(0..20);
        let nova_coordenada = (x, y);

        // Verificar se a nova coordenada já foi escolhida anteriormente
        if !coordenadas.contains(&nova_coordenada) {
            coordenadas.push(nova_coordenada);
        }
    }

    coordenadas
}

fn main() {
    let mut input = String::new();
    loop {
        println!("1-Escolher pontos inicial e final manualmente");
        println!("2-Fazer com mapa aleatorio");
        println!("3-sair");
        input.clear();
        io::stdin().read_line(&mut input).expect("Falha ao ler opcao");
        let mut x=input.trim().parse::<i32>().unwrap();
        match x{
            1 => {
                
                let mut input2 = String::new();
                println!("Digite os 6 numeros no intervalo [0,20[ separados por espaco");
                io::stdin().read_line(&mut input2).expect("Falha ao ler entrada");
                let numbers: Vec<i32> = input2.split_whitespace() .map(|s| s.trim().parse::<i32>().unwrap()) .collect();

                let (carteiro, caixa) = Jogo::cria_jogo(numbers[0], numbers[1], numbers[2], numbers[3]);
                let mut matriz = [['+'; 20]; 20];
                Jogo::joga(matriz, carteiro, caixa, numbers[4], numbers[5]);
            }
            2 =>{
                let coordenadas= gerar_coordenadas_unicas();
                let (carteiro, caixa) = Jogo::cria_jogo(coordenadas[0].0, coordenadas[0].1, coordenadas[1].0, coordenadas[1].1);
                let mut matriz = [['+'; 20]; 20];
                Jogo::joga(matriz, carteiro, caixa, coordenadas[2].0, coordenadas[2].1);
            }
            3 => {
                println!("Ate mais!");
                break},
            _ => println!("Opção inválida. Por favor, escolha 1, 2 ou 3."),
        }
    }
    
}

