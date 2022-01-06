## Calculadora simples em Rust

Esta é uma calculadora relativamente simples que escrevi em Rust. Faz parte de um processo autoavaliativo já que não tenho como pagar cursos online de programação.

A ideia era de ter operações muito simples, mas percebi que não usaria todo o conhecimento que adquiri até o capítulo 7 do livro* ("A linguagem de programação Rust").

### Como utilizar

Primeiramente, clone este repositório ou baixe um pacote zip e extraia em algum lugar.

#### Clonando com git

`$ git clone https://github.com/cledsupper/calculadora-rust.git`

#### Testando

`$ cargo run`

Cada token deve ser inserido em uma linha (↵).

Tokens são: qualquer número (123.45), qualquer operador (+, -, *, /, ^), abre parêntese [(], fecha parêntese [), =].

Para finalizar o cálculo, usa-se o token de fecha parêntese ('=' e ')' significam a mesma coisa).

## Referências

(*): "A Linguagem de Programação Rust" por Steve Klabnik e Carol Nichols. Disponível em: https://livro.rustbr.org/


Esta calculadora é bugada e pode realizar cálculos incorretos. Ela é apenas um teste de conhecimentos na linguagem Rust.

by cleds.upper
