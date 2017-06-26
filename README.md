## Premissa

A fim de estudos com Rust e com uma brincadeira iniciado quanto a real maior utilidade de entre amigos sobre o site dos Correios, o serviço postal brasileiro, decidimos iniciar um projeto com a pesquisa de pacotes mais simples.

### Instalação

Instale Rust em seu computador:

```sh
curl https://sh.rustup.rs -sSf | sh
```

source: https://www.rust-lang.org/en-US/install.html

Coloque em seu arquivo de profile o seguinte `alias`:

```sh
alias cargo-home='source $HOME/.cargo/env'
```

Isso apontara a variavel HOME para o binario de Cargo, o gerenciador de dependencias do projeto.

Instale o *openssl* em seu computador com seu gerenciador de pacotes (brew, yaourt, pacman, aptitude).

