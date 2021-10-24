#[derive(Debug)]
pub enum GambleAction {
    Bet(usize),
    RequestBalance,
}
#[derive(Debug)]
pub struct GambleF {
    /// Code that controls the behavior of the [`GambleF`] process, guarranteed to have an equal amount of `'['`s and `']'`s by [`GambleF::new()`].
    code: Vec<char>,
    /// Memory for the [`GambleF`] process.
    memory: [usize; 256],
    /// Where the [`GambleF`] process is currently looking at in its [`memory`](GambleF::memory).
    mem_pointer: u8,
    /// Where the [`GambleF`] process is at in its [`code`](GambleF::code),
    /// generally will move once per call to [`self.tick()`](GambleF::tick),
    /// except for [`branch`](GambleF::branch) (`'['`) and [`loop_back`](GambleF::loop_back) (`']'`).
    /// The [character](std::char) this points to in the [`GambleF`] process's [`code`](GambleF::code)
    /// determines what method [`self.tick()`](GambleF::tick) will call.
    code_pointer: usize,
}

impl GambleF {
    /// Returns a new [`GambleF`] struct with the provided code
    pub fn new(code: &str) -> Self {
        // Check that '[' and ']' occur the same amount of times
        let code: Vec<char> = code.chars().collect();
        if code.iter().filter(|&c| *c == '[').count() != code.iter().filter(|&c| *c == ']').count()
        {
            panic!("Unequal amounts of \'[\' and \']\'");
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
    /// Sets [`mem_pointer`](GambleF::mem_pointer) to 0 if it equals [`u8::MAX`] (aka 255), otherwise adds 1 to [`mem_pointer`](GambleF::mem_pointer)
    fn right(&mut self) -> Option<GambleAction> {
        if self.mem_pointer == u8::MAX {
            self.mem_pointer = 0;
        } else {
            self.mem_pointer += 1;
        }
        None
    }
    /// Sets [`mem_pointer`](GambleF::mem_pointer) to [`u8::MAX`] (aka 255) if it equals 0, otherwise subtracts 1 from `mem_pointer`
    fn left(&mut self) -> Option<GambleAction> {
        if self.mem_pointer == 0 {
            self.mem_pointer = 255;
        } else {
            self.mem_pointer -= 1;
        }
        None
    }

    /// Sets the value at [`memory`](GambleF::memory)\[[`mem_pointer`](GambleF::mem_pointer)\] to [`u8::MAX`] (aka 255) if it equals 0, otherwise adds 1 to the value at `mem_pointer`
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
            match self.char_at_code_pointer() {
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
            match self.char_at_code_pointer() {
                Some(']') => indent += 1,
                Some('[') => indent -= 1,
                None => panic!("code pointer got out of bounds!"),
                _ => (),
            }
        }
        None
    }

    fn char_at_code_pointer(&self) -> Option<&char> {
        self.code.get(self.code_pointer as usize)
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
        let result = match self.char_at_code_pointer() {
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
