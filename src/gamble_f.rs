#[derive(Debug)]
pub enum GambleAction {
    Bet(usize),
    RequestBalance,
}
#[derive(Debug)]
pub struct GambleF {
    code: Vec<char>,
    memory: [usize; 256],
    mem_pointer: u8,
    code_pointer: usize,
}

impl GambleF {
    pub fn new(code: Vec<char>) -> Self {
        // Check that '[' and ']' occur the same amount of times
        if code.iter().filter(|&c| *c == '[').count() != code.iter().filter(|&c| *c == ']').count()
        {
            panic!("Unequal amounts of '[' and ']'");
        }
        Self {
            code,
            memory: [0; 256],
            mem_pointer: 0,
            code_pointer: 0,
        }
    }
}

impl GambleF {
    /// Command to be executed when the `char` at `code[code_pointer]` == `>`
    /// Adds 1 to `mem_pointer`
    fn right(&mut self) -> Option<GambleAction> {
        if self.mem_pointer == 255 {
            self.mem_pointer = 0;
        } else {
            self.mem_pointer += 1;
        }
        None
    }
    // <
    fn left(&mut self) -> Option<GambleAction> {
        if self.mem_pointer == 0 {
            self.mem_pointer = 255;
        } else {
            self.mem_pointer -= 1;
        }
        None
    }

    // +
    fn inc(&mut self) -> Option<GambleAction> {
        if self.memory[self.mem_pointer as usize] == usize::MAX {
            self.memory[self.mem_pointer as usize] = 0;
        } else {
            self.memory[self.mem_pointer as usize] += 1;
        }
        None
    }
    // -
    fn dec(&mut self) -> Option<GambleAction> {
        if self.memory[self.mem_pointer as usize] == 0 {
            self.memory[self.mem_pointer as usize] = usize::MAX;
        } else {
            self.memory[self.mem_pointer as usize] -= 1;
        }
        None
    }

    // .
    fn gamble(&self) -> GambleAction {
        GambleAction::Bet(self.memory[self.mem_pointer as usize])
    }

    // ,
    fn request_balance() -> GambleAction {
        GambleAction::RequestBalance
    }

    // [
    /// # Panics
    /// Panics if the code pointer manages to get out of bounds, which should never happen,
    /// (end of code should be handled by tick function)
    fn branch(&mut self) -> Option<GambleAction> {
        if self.memory[self.mem_pointer as usize] != 0 {
            return None;
        }
        let mut indent = 1;
        while indent > 0 {
            self.code_pointer += 1;
            match self.code.get(self.code_pointer as usize) {
                Some('[') => indent += 1,
                Some(']') => indent -= 1,
                None => panic!("code pointer got out of bounds!"),
                _ => (),
            }
        }
        None
    }
    // ]
    fn loop_back(&mut self) -> Option<GambleAction> {
        let mut indent = 1;
        while indent > 0 {
            self.code_pointer -= 1;
            match self.code.get(self.code_pointer as usize) {
                Some(']') => indent += 1,
                Some('[') => indent -= 1,
                None => panic!("code pointer got out of bounds!"),
                _ => (),
            }
        }
        None
    }
}

impl GambleF {
    /// # Panics
    /// Will panic if the code pointer somehow gets longer than the code
    pub fn tick(&mut self) -> Option<GambleAction> {
        // We loop at the end btw
        if self.code_pointer == self.code.len() {
            self.code_pointer = 0;
        }
        let result = match self.code.get(self.code_pointer) {
            Some('<') => self.left(),
            Some('+') => self.inc(),
            Some('-') => self.dec(),
            Some('[') => self.branch(),
            Some(']') => self.loop_back(),
            Some('.') => Some(self.gamble()),
            Some(',') => Some(GambleF::request_balance()),
            Some('>') => self.right(),
            Some(_) => None,
            None => panic!("code pointer out of bounds!"),
        };
        self.code_pointer += 1;
        result
    }
}

//more interfacey stuff
impl GambleF {
    pub fn gamble_result(&mut self, result: bool) {
        self.memory[self.mem_pointer as usize] = if result { 1 } else { 0 };
    }

    pub fn balance(&mut self, balance: usize) {
        self.memory[self.mem_pointer as usize] = balance;
    }

    pub fn code_string(&self) -> String {
        self.code.iter().copied().collect::<String>()
    }
    pub fn get_code_pointer(&self) -> usize {
        self.code_pointer
    }
}