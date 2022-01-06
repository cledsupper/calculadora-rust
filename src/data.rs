use std::cmp::Ordering;

#[derive(Copy, Clone)]
pub enum Operador {
    Iniciar,
    Soma,
    Diferenca,
    Multiplicacao,
    Divisao,
    Potencia,
    Chamada,
    Retorno,
}

impl Operador {
    pub fn ordem(&self) -> i8 {
        match self {
            Operador::Retorno => -1,
            Operador::Iniciar => 0,
            Operador::Soma => 1,
            Operador::Diferenca => 1,
            Operador::Multiplicacao => 2,
            Operador::Divisao => 2,
            Operador::Potencia => 3,
            Operador::Chamada => 4,
        }
    }
}

impl PartialEq<Operador> for Operador {
    fn eq(&self, outro: &Operador) -> bool {
        self.ordem() == outro.ordem()
    }
    fn ne(&self, outro: &Operador) -> bool {
        self.ordem() != outro.ordem()
    }
}

impl PartialOrd<Operador> for Operador {
    fn partial_cmp(&self, outro: &Operador) -> Option<Ordering> {
        let (a, b) = (self.ordem(), outro.ordem());

        if a < b {
            return Some(Ordering::Less);
        } else if a == b {
            return Some(Ordering::Equal);
        }

        Some(Ordering::Greater)
    }

    fn lt(&self, outro: &Operador) -> bool {
        self.ordem() < outro.ordem()
    }
    fn le(&self, outro: &Operador) -> bool {
        self.ordem() <= outro.ordem()
    }
    fn gt(&self, outro: &Operador) -> bool {
        self.ordem() > outro.ordem()
    }
    fn ge(&self, outro: &Operador) -> bool {
        self.ordem() >= outro.ordem()
    }
}

pub struct Estado {
    n: (f32, f32),
    n1: (f32, f32),
    n2: (f32, f32),
    pub op: Operador,
    pub op1: Operador,
    pub op2: Operador,
}

impl Estado {
    pub fn novo() -> Estado {
        Estado {
            n: (0.0, 0.0),
            n1: (0.0, 0.0),
            n2: (0.0, 0.0),
            op: Operador::Iniciar,
            op1: Operador::Iniciar,
            op2: Operador::Iniciar,
        }
    }

    pub fn salva(&mut self, num: f32) {
        self.n.0 = num;
    }

    pub fn memoriza(&mut self) {
        self.n.1 = self.n.0;
    }

    pub fn num_0(&self) -> f32 {
        self.n.0
    }
    pub fn num_1(&self) -> f32 {
        self.n.1
    }

    pub fn sobe(&mut self) {
        self.n2 = self.n1;
        self.n1 = self.n;
        self.n = (self.n.0, 0.0);

        self.op2 = self.op1;
        self.op1 = self.op;
        self.op = Operador::Iniciar;
    }
    pub fn desce(&mut self) {
        if self.op1 == Operador::Iniciar {
          self.n.1 = 0.0;
          return
        }

        self.n = (self.n.0, self.n1.1);
        self.n1 = self.n2;
        self.n2 = (0.0, 0.0);

        self.op = self.op1;
        self.op1 = self.op2;
        self.op2 = Operador::Iniciar;
    }
}
