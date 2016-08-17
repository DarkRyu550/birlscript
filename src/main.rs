mod parser;

/// Imprime mensagem de ajuda
fn print_help() {
    println!("Ta querendo ajuda, cumpade?");
    println!("O uso é o seguinte: birl [opções] [arquivo ou arquivos]");
    println!("Cê pode passar mais de um arquivo, só que apenas um pode ter a seção \"SHOW\", que \
              é");
    println!("o ponto de partida do teu programa.");
    println!("As opções são as seguintes:");
    println!("\t-a ou --ajuda-o-maluco-ta-doente       : Imprime essa mensagem de ajuda");
    println!("\t-v ou --vers[ã ou a]o-dessa-porra      : Imprime a versão do programa");
    println!("\t-e ou --ele-que-a-gente-quer [comando] : Imprime uma mensagem de ajuda para o \
              comando");
    println!("\t-j ou --jaula [nome]                   : Diz ao interpretador pra usar outro \
              ponto de partida. Padrão: SHOW");
}

/// Versão numérica
pub static BIRLSCRIPT_VERSION: &'static str = "0.1.3";
/// Release, como alfa, beta, etc
pub static BIRLSCRIPT_RELEASE: &'static str = "ALFA";

/// Imprime a mensagem de versão
fn print_version() {
    println!("Versão dessa porra, cumpade:");
    println!("Interpretador BIRLSCRIPT v{} - {}",
             BIRLSCRIPT_VERSION,
             BIRLSCRIPT_RELEASE);
    println!("Copyleft(ɔ) 2016 Rafael R Nakano - Nenhum direito reservado");
}

/// Coleção de parametros passados ao interpretador
enum Param {
    /// Pedido para printar versão
    PrintVersion,
    /// Pedido para printar ajuda
    PrintHelp,
    /// Pedido para printar ajuda com um comando
    CommandHelp(String),
    /// Pedido para modificar o ponto de partida
    CustomInit(String),
    /// Arquivo passado para interpretação
    InputFile(String),
}

/// Faz parsing dos comandos passados e retorna uma lista deles
fn get_params() -> Vec<Param> {
    use std::env;
    let mut ret: Vec<Param> = vec![];
    let mut params = env::args();
    if params.len() > 2 {
        loop {
            let p = match params.next() {
                Some(v) => v,
                None => break,
            };
            match p.as_str() {
                "-a" |
                "--ajuda-o-maluco-ta-doente" => ret.push(Param::PrintHelp),
                "-v" |
                "--versão-dessa-porra" |
                "--versao-dessa-porra" => ret.push(Param::PrintVersion),
                "-e" |
                "--ele-que-a-gente-quer" => {
                    let cmd = match params.next() {
                        Some(name) => name,
                        None => {
                            println!("Erro: a flag \"-e ou --ele-que-a-gente-quer\" espera um \
                                      valor.");
                            break;
                        }
                    };
                    ret.push(Param::CommandHelp(cmd));
                }
                "-j" | "--jaula" => {
                    let section = match params.next() {
                        Some(sect) => sect,
                        None => {
                            println!("Erro: a flag \"-j ou --jaula\" espera um valor.");
                            break;
                        }
                    };
                    ret.push(Param::CustomInit(section));
                }
                _ => ret.push(Param::InputFile(p)),
            }
        }
    }
    ret
}

/// Printa ajuda para um comando
fn command_help(_command: &str) {}

fn main() {
    let params = get_params();
    let mut files: Vec<String> = vec![];
    for p in params {
        match p {
            Param::PrintVersion => print_version(),
            Param::PrintHelp => print_help(),
            Param::CommandHelp(cmd) => command_help(&cmd),
            // TODO: Adicionar nas flags do interpretador a jaula customizada
            Param::CustomInit(_init) => {}
            Param::InputFile(file) => files.push(file),
        }
    }
    if files.len() > 0 {
        let mut units: Vec<parser::Unit> = vec![];
        for file in files {
            units.push(parser::parse(&file));
        }
    }
}
