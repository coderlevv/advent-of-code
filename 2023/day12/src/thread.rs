#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlockType {
    Block,
    Gap
}

#[derive(Debug, Copy, Clone)]
pub struct Block {
    pub block_type: BlockType,
    pub n: usize
}

impl Block {
    pub fn new(block_type: BlockType, n: usize) -> Self {
        Self {
            block_type,
            n
        }
    }
}

#[derive(Debug, Clone)]
pub struct Thread {
    pub pat: Vec<char>,
    pub seq: Vec<Block>,
    pub arr: Option<u64>
}

impl Thread {
    pub fn from_vec(vec: Vec<u32>, pat: Vec<char>) -> Self {
        let mut seq: Vec<Block> = vec![Block::new(BlockType::Gap, 0)];
        for (i, n) in vec.iter().enumerate() {
            seq.push(Block::new(BlockType::Block, *n as usize));
            match i == vec.len()-1 {
                false => seq.push(Block::new(BlockType::Gap, 1)),
                true => seq.push(Block::new(BlockType::Gap, 0))
            }
        }
        Thread {
            pat,
            seq,
            arr: None
        }
    }

    pub fn len(&self) -> usize {
        self.seq.iter().map(|b| b.n).sum()
    }

    pub fn is_consistent(&self) -> bool {
        if self.len() > self.pat.len() {
            return false;
        }
        let mut pos = 0;
        for b in &self.seq {
            if pos + b.n > self.pat.len() {
                return false;
            }
            for k in 0..b.n {
                if b.block_type == BlockType::Gap && self.pat[pos + k] == '#' { 
                    return false;
                }
                if b.block_type == BlockType::Block && self.pat[pos + k] == '.' {
                    return false;
                }
            }
            pos += b.n;
        }
        if pos + 1 < self.pat.len() {
            for k in pos..self.pat.len() {
                if self.pat[k] == '#' {
                    return false;
                }
            }
        }
        true
    }

    pub fn output(&self) -> String {
        let mut s: String = String::new();
        for b in &self.seq {
            for _ in 0..b.n {
                match b.block_type {
                    BlockType::Block => s.push('#'),
                    BlockType::Gap => s.push('.')
                }
            }
        }
        if s.len() < self.pat.len() {
            while s.len() < self.pat.len() {
                s.push('.');
            }
        }
        s
    }
}