use plotters::prelude::*;

// Constantes utilizadas para os cálculos na função sputinik
const P0: f64 = 0.322232431088;
const P1: f64 = 1.0;
const P2: f64 = 0.342242088547;
const P3: f64 = 0.204231210245e-1;
const P4: f64 = 0.453642210148e-4;

const Q0: f64 = 0.099348462606;
const Q1: f64 = 0.588581570495;
const Q2: f64 = 0.531103462366;
const Q3: f64 = 0.103537752850;
const Q4: f64 = 0.385607006340e-2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Média (m) e desvio padrão (s) para a distribuição normal
    let m = 5.0;
    let s = 2.0;

    // Quantidade de iterações para gerar os três histogramas
    let iteration_counts = [100, 10_000, 1_000_000];

    // Itera sobre cada valor em iteration_counts
    for &iterations in &iteration_counts {
        // Vetor para armazenar os números gerados
        let mut data = Vec::new();

        // Gera números aleatórios de acordo com a distribuição normal para cada iteração
        for _ in 0..iterations {
            let number = sputinik(m, s);
            data.push(number);
        }

        // Define o número de bins para o histograma
        let bin_count = 1_000;

        // Determina o valor mínimo e máximo dos dados gerados
        let min_value = *data
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let max_value = *data
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        // Calcula a largura de cada bin do histograma
        let bin_width = (max_value - min_value) / bin_count as f64;

        // Inicializa um vetor de frequências para cada bin
        let mut frequencies = vec![0; bin_count];

        // Distribui os valores gerados nos bins apropriados
        for &value in &data {
            let bin_index = ((value - min_value) / bin_width) as usize;
            if bin_index < bin_count {
                frequencies[bin_index] += 1;
            }
        }

        // Cria um nome de arquivo para salvar o histograma com base no número de iterações
        let filename = format!("histograma_{}.png", iterations);

        // Configura o backend para desenhar o gráfico em um arquivo PNG
        let root = BitMapBackend::new(&filename, (1280, 720)).into_drawing_area();
        root.fill(&WHITE)?;

        // Configura o gráfico com título, margens e áreas para rótulos
        let mut chart = ChartBuilder::on(&root)
            .caption(format!("Histograma com {} iterações", iterations), ("sans-serif", 25).into_font())
            .margin(10)
            .x_label_area_size(50)
            .y_label_area_size(50)
            .build_cartesian_2d(0..bin_count, 0..*frequencies.iter().max().unwrap())?;

        // Desenha a grade do gráfico
        chart.configure_mesh().draw()?;

        // Desenha as barras do histograma
        chart.draw_series(
            Histogram::vertical(&chart)
                .style(BLUE.mix(0.5).filled())
                .data(frequencies.iter().enumerate().map(|(i, &f)| (i, f))),
        )?;

        // Finaliza a renderização e salva o arquivo
        root.present()?;

        println!("Histograma salvo como '{}'", filename);
    }

    Ok(())
}

// Função que gera um número aleatório seguindo uma distribuição normal
fn sputinik(m: f64, s: f64) -> f64 {
    // Gera um número aleatório entre 0 e 1
    let u: f64 = rand::random::<f64>();
    let t: f64;

    // Calcula o valor de t baseado na transformação Box-Muller
    if u < 0.5 {
        t = (-2.0 * u.ln()).sqrt();
    } else {
        t = (-2.0 * (1.0 - u).ln()).sqrt();
    }

    // Calcula os valores p e q baseados em t usando as constantes definidas
    let p = P0 + t * (P1 + t * (P2 + t * (P3 + t * P4)));
    let q = Q0 + t * (Q1 + t * (Q2 + t * (Q3 + t * Q4)));

    // Calcula o valor z, que será utilizado para retornar o valor gerado pela distribuição normal
    let z: f64;
    if u < 0.5 {
        z = (p / q) - t;
    } else {
        z = t - (p / q);
    }

    // Retorna o número gerado ajustado pela média (m) e desvio padrão (s)
    m + s * z
}
