pub mod es;

mod data;

use data::*;
use std::cmp::Ordering;

pub struct Calculadora {
    entrada: es::Entrada,
}

impl Calculadora {
    pub fn nova(stdin: std::io::Stdin) -> Calculadora {
        Calculadora { entrada: es::Entrada::nova(stdin), }
    }

    pub fn sessao(&mut self) -> f32 {
        let mut estado = Estado::novo();
        let mut buf: String;
        let mut res: f32 = 0.0;
        loop {
            if let Some(texto) = self.entrada.ler() {
                buf = texto.trim().to_string();
            }
            else { break; }

            res = self.iterar(buf.as_str(), &mut estado);
            match estado.op {
                Operador::Retorno => { break; }
                _ => (),
            };
        }
        res
    }

    fn iterar(&mut self, buf: &str, estado: &mut Estado) -> f32 {
        let mut val = 0.0_f32;
        /* if estado.op < Operador::Multiplicacao { val = 0.0_f32; }
        else { val = 1.0; } */
        let mut nan = true;
        let mut act_buf = buf;

        if buf == "=" { act_buf = ")"; }

        let op = match act_buf {
            "+" => Operador::Soma,
            "-" => Operador::Diferenca,
            "*" => Operador::Multiplicacao,
            "/" => Operador::Divisao,
            "^" => Operador::Potencia,
            "(" => Operador::Chamada,
            ")" => {
                let mut descidas = 0;
                if estado.op2 > Operador::Iniciar {
                    descidas = 2;
                }
                else if estado.op1 > Operador::Iniciar {
                    descidas = 1;
                }
                if estado.op >= Operador::Multiplicacao {
                    self.opera(estado, Operador::Iniciar);
                    if (descidas-1) > 0 {
                        self.opera(estado, Operador::Iniciar);
                    }
                }
                self.calcula(estado, 0.0);
                estado.op = Operador::Retorno;
                val = estado.num_1();
                estado.op
            }
            _ => {
                val = match buf.parse() {
                    Err(_) => {
                        estado.op = Operador::Retorno;
                        0.0
                    }
                    Ok(num) => {
                        nan = false;
                        num
                    }
                };
                estado.op
            }
        };

        if nan {
            if op == Operador::Retorno { return val; }
            self.opera(estado, op);
        }
        else { self.calcula(estado, val); }
        val
    }

    fn opera(&mut self, estado: &mut Estado, mut op: Operador) {
        match op.partial_cmp(&estado.op).expect("ERRO INTERNO") {
            Ordering::Less => {
                if estado.op == Operador::Potencia {
                    estado.salva(estado.num_1().powf(estado.num_0()));
                    estado.desce();
                }
                else if estado.op == Operador::Multiplicacao {
                    estado.salva(estado.num_1() * estado.num_0());
                    estado.desce();
                }
                return;
            }
            Ordering::Greater => {
                if op == Operador::Chamada {
                    if estado.op < Operador::Multiplicacao {
                        estado.memoriza();
                        estado.salva(self.sessao());
                    }
                    else {
                        estado.salva(self.sessao());
                        self.calcula(estado, 1.0);
                    }
                    op = estado.op;
                }
                else {
                    if estado.op != Operador::Iniciar { estado.sobe(); }
                    if op == Operador::Multiplicacao {
                        estado.memoriza();
                        estado.salva(1.0);
                    }
                    if op == Operador::Potencia {
                        estado.memoriza();
                        estado.salva(1.0);
                    }
                }
            }
            _ => (),
        }
        estado.op = op;
    }

    fn calcula(&mut self, estado: &mut Estado, val: f32) {
        match estado.op {
            Operador::Iniciar => {
                estado.memoriza();
                estado.salva(val);
            }
            Operador::Soma => {
                estado.salva(estado.num_1() + estado.num_0());
                estado.memoriza();
                estado.salva(val);
            }
            Operador::Diferenca => {
                estado.salva(estado.num_1() + estado.num_0());
                estado.memoriza();
                estado.salva(-val);
            }
            Operador::Multiplicacao => {
                estado.salva(estado.num_1() * estado.num_0());
                estado.memoriza();
                estado.salva(val);
            }
            Operador::Divisao => {
                estado.salva(estado.num_1() * estado.num_0());
                estado.memoriza();
                estado.salva(1.0/val);
            }
            Operador::Potencia => {
                estado.salva(estado.num_1().powf(estado.num_0()));
                estado.memoriza();
                estado.salva(val);
            }
            _ => {
                estado.op = Operador::Retorno;
            }
        }
    }
}
