    use rand::Rng;
    use regex::Regex;
    #[macro_use]
    extern crate text_io;

    struct Banco{
        numConta: u32,
        tipo: char,
        dono: String,
        saldo: f64,
        status: bool,
        passWord: String,
        opcao: char,
    }

    impl Banco {
        // Método para abrir uma nova conta
        fn informacoes(n: u32, t: char, d: String, m: f64, s: bool, p: String) -> Banco {
            Banco{
                numConta: n,
                tipo: t,
                dono: d,
                saldo: m,
                status: s,
                passWord: p,
                opcao: '0', // Inicializa com opção padrão
            }
        }

        fn Depositar(&mut self){
            println!("Digite quanto dinheiro você quer depositar:");
            let deposito: f64 = read!();
        if  deposito <= 0.0{
            print!("Não foi possivel realizar esse deposito");
        }
        else {
            self.saldo += deposito;
            println!("Deposito realizado com sucesso. Novo saldo: {}", self.saldo)
        }
        }

        fn Sacar(&mut self){
            println!("Digite quanto dinheiro você quer sacar:");
            let saque: f64 = read!();
            if saque > self.saldo{
                print!("Não foi possivel sacar mais do que você tem!");
            }
            else if saque <= 0.0{
                print!("Não foi possivel realizar esse saque");
            }
            else {
                self.saldo -= saque;
                println!("Saque realizado com sucesso. Novo saldo: {}", self.saldo)
            }
        }

        fn Taxa(&mut self){

            let taxa: f64 = if self.tipo == 'C' {
                12.50
            } else {
                20.00
            };

            if self.saldo < taxa{
                println!("Não foi possível, saldo insuficiente!😟")
            }
            else {
                self.saldo -= taxa;
                println!("Taxa paga com sucesso!😄")
            }
        }

        fn FecharConta(&mut self){
            println!("Tem certeza que quer fechar a conta?😶\nDigite 'SIM' se sim, senão digite 'N'!");
            let desicao: String = read!();

            if desicao == "SIM"{
                self.status = false;
                println!("Conta fechada com sucesso!🙃")
            }
            else {
                println!("Operação cancelada.😉")
            }
        }
    }

    fn VerificarSenha(senha: &str) -> bool{
        let maiuscula: Regex = Regex::new(r"[A-Z]").unwrap(); //Em situações onde você tem certeza de que a operação sempre retornará um resultado válido
        let numero: Regex = Regex::new(r"\d").unwrap();
        let special: Regex = Regex::new(r#"[!@#$%^&*()_+\-\[\]{};':,.<>?]"#).unwrap();
        //let special: Regex = Regex::new(r#"[!@#$%^&*()_+\-\[\]{};':,.<>?]£¢|°¬¨³²¹º´`"#).unwrap();
        
        maiuscula.is_match(senha) && numero.is_match(senha) && special.is_match(senha)
    }

    fn abrirConta() -> Banco{
        let mut saldo: f64 = 0.0;
        let mut status_conta: bool = false;

        println!("Olá você vai querer abrir uma conta?");
        println!("Digite S para sim ou N para não!");
        let opcao: char = read!();

        if opcao == 'S' || opcao == 's' {
            let mut tipoC: char = '0';
            loop {
                println!("Qual tipo de conta você gostaria? P para Poupança ou C para Corrente");
                let tipoC: char = read!();

                if tipoC != 'C' && tipoC != 'P'{
                    print!("Escolha inconsistente! Selecione 'C' para Conta Corrente ou 'P' para Poupança?")
                }
                else {
                    {break;}
                }
            }

            if tipoC == 'C'{
                saldo += 50.0; 
                status_conta = true;
            }
            if tipoC == 'P'{
                saldo += 150.0;
                status_conta = true;
            }

            println!("Digite seu nome completo:");
            let nome: String = read!();
            let numero_conta: u32 = rand::thread_rng().gen_range(1..11); //10

            println!("Digite sua senha com no mínimo 8 carateres com números e pelo menos uma especial!");
            let mut senha: String = read!();

            loop {
                if  VerificarSenha(&senha) == false{
                    println!("A senha não atingiou os criterios");
                    senha = read!()
                }
                else {
                    {break;}
                }
            }

            // Criar e retornar uma nova instância de Banco com as informações fornecidas
            return Banco::informacoes(numero_conta, tipoC, nome, saldo, status_conta, senha)
        }
        else {
            println!("Ok atendimento encerrado!");
            return  Banco::informacoes(0, '¨', String::new(), 0.0, false, String::new());
        }
    }

    pub fn menu(banco: &mut Banco){

        loop {
            print!("1. Depositar\n2. Sacar\n3. Pagar Taxa\n4. Fechar Conta\n");
            print!("Digite o número correspondente à opção que você deseja:");
            let mut option: u8 = read!();

           if banco.status == true {
            match option {
                1 => banco.Depositar(),
                2 => banco.Sacar(),
                3 => banco.Taxa(),
                4 => banco.FecharConta(),
                _ => println!("Opção inválida!"),
            }
           }
           else {
            {break;}
            }
        }
    }

    fn main() {
        let mut banco = abrirConta();
        menu(&mut banco);
    }


    // Melhor #[macro_use]
    // extern crate text_io;

    // fn main() {
    //     let num: i32 = read!();
    //     println!("{}", num);
    // }



    // use std::io;

    // mod ex1;
    // use  ex1::mensagem;
    // fn main() {
        
    //     println!("Digite um número inteiro:");
    //     let mut input = String::new();
        
    //     io::stdin().read_line(&mut input).expect("Falha ao ler linha");

    //     let num: i32 = input.trim().parse().expect("Por favor, digite um número inteiro!");
    //     println!("Você digitou o número: {}", num);

    //     println!("Digite uma palavra:");
    //     input.clear(); // Limpa o conteúdo anterior da variável
        
    //     io::stdin().read_line(&mut input).expect("Falha ao ler linha");

    //     let palavra = input.trim();
    //     println!("Você digitou a palavra: {}", palavra);
        
    //     mensagem();
    // }