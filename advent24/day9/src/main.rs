#![allow(dead_code)]
use std::env;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::Read;
use std::ops::{Index, IndexMut};
use std::time::Instant;

use itertools::Itertools;

// 2333133121414131402

#[derive(Default, Debug, Clone)]
struct Block {
    file: bool,
    start: isize,
    end: isize,
    id: Option<usize>,
    searched: bool,
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for _ in self.start..self.end {
            write!(
                f,
                "{}",
                self.id
                    .map(|id| id.to_string())
                    .unwrap_or_else(|| ".".to_string())
            )?;
        }
        Ok(())
    }
}

impl Debug for BlockList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "total_length: {}", self.total_length)?;
        for block in self.blocks.iter() {
            writeln!(f, "{:?}", block)?;
        }
        Ok(())
    }
}

impl Block {
    fn len(&self) -> isize {
        self.end - self.start
    }
}

#[derive(Clone)]
struct BlockList {
    blocks: Vec<Block>,
    total_length: isize,
}

#[derive(Debug)]
enum CompressionError {
    EmptyBlock,
}

enum CompressionMethod {
    Simple,
    NonFragmented,
}

impl Index<usize> for BlockList {
    type Output = Block;

    fn index(&self, index: usize) -> &Self::Output {
        &self.blocks[index]
    }
}

impl IndexMut<usize> for BlockList {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.blocks[index]
    }
}

use std::ops::{Add, Div, Mul};
trait SumN: Add<Output = Self> + Div<Output = Self> + Mul<Output = Self> + From<u8> + Copy {}
impl SumN for usize {}
impl SumN for isize {}

fn sumn<T: SumN>(n: T) -> T {
    n * (n + T::from(1)) / T::from(2)
}

impl BlockList {
    fn default() -> Self {
        BlockList {
            blocks: Vec::new(),
            total_length: 0,
        }
    }

    fn check_integrity(&self) {
        for block in self.blocks.iter() {
            assert!(block.start <= block.end);
            if block.file {
                assert!(block.id.is_some());
            } else {
                assert!(block.id.is_none());
            }
        }
        for (first, second) in self.blocks.iter().tuple_windows() {
            // log::error!("{:?} {:?}", first.end, second.start);
            if first.end != second.start {
                log::error!("first: {}, second: {}", first, second);
                panic!("first.end != second.start");
            }
        }
    }

    #[inline]
    fn get_first_non_file(&self) -> usize {
        self.blocks
            .iter()
            .position(|block| !block.file)
            .expect("no non-file block found")
    }

    #[inline]
    fn get_first_non_file_n(&self, len: isize) -> Option<usize> {
        self.blocks
            .iter()
            .position(|block| !block.file && block.len() >= len)
    }

    #[inline]
    fn get_last_file(&self) -> usize {
        self.blocks
            .iter()
            .rposition(|block| block.file)
            .expect("no file block found")
    }

    fn get_last_unsearched_file(&self) -> Option<usize> {
        self.blocks
            .iter()
            .rposition(|block| block.file && !block.searched)
    }

    fn checksum(&self) -> usize {
        // 0099811188827773336446555566..............
        self.blocks
            .iter()
            .filter(|block| block.file)
            .map(|block| {
                let blen = block.len();
                let star = block.start;
                let sum = sumn(blen + star - 1) - sumn(star - 1);
                (sum as usize) * block.id.unwrap()
            })
            .sum()
    }

    fn moveer(&mut self, dest: usize, source: usize) {
        assert!(!self[dest].file, "cannot move into file");
        assert!(self[source].file, "cannot move empty space");
        assert_ne!(dest, source, "cannot replace the same block");
        use std::cmp::Ordering;

        match self[dest].len().cmp(&self[source].len()) {
            Ordering::Equal => {
                // If it fits, replace it
                self[dest] = Block {
                    start: self[dest].start,
                    end: self[dest].end,
                    ..self[source]
                };
                self[source] = Block {
                    file: false,
                    id: None,
                    ..self[source]
                };
            }
            Ordering::Greater => {
                // There is some space left in self[i] after replacing it with new_block
                let dest_end = self[dest].end;
                self[dest] = Block {
                    start: self[dest].start,
                    end: self[dest].start + self[source].len(),
                    ..self[source]
                };
                self.blocks.insert(
                    dest + 1,
                    Block {
                        file: false,
                        start: self[dest].end,
                        end: dest_end,
                        id: None,
                        searched: false,
                    },
                );
                log::trace!("source block: {:?}", self.blocks[source + 1]);
                self[source + 1].file = false;
                self[source + 1].id = None;
            }
            Ordering::Less => {
                // There is not enough space left, so the source is split up and the last chunk is
                // moved to the destination
                self[dest].file = self[source].file;
                self[dest].id = self[source].id;
                self[source].end -= self[dest].len();
                self.blocks.insert(
                    source + 1,
                    Block {
                        file: false,
                        id: None,
                        start: self[source].end,
                        end: self[source].end + self[dest].len(),
                        searched: false,
                    },
                );
            }
        }
        // clean up
        if let Some(chunk) = self.blocks.last_chunk::<2>() {
            log::trace!("last_chunk: {:?}", chunk);
            if !chunk[0].file && !chunk[1].file {
                log::trace!("merging last two blocks");
                let end = self.blocks.pop().unwrap().end;
                self.blocks.last_mut().unwrap().end = end;
            }
        }
        self.check_integrity();
    }

    fn replace_i_th_block(&mut self, dest: usize, source: usize) {
        assert!(!self[dest].file, "cannot move into file");
        assert!(self[source].file, "cannot move empty space");
        assert_ne!(dest, source, "cannot replace the same block");
        use std::cmp::Ordering;

        match self[dest].len().cmp(&self[source].len()) {
            Ordering::Equal => {
                // If it fits, replace it
                self[dest] = Block {
                    start: self[dest].start,
                    end: self[dest].end,
                    ..self[source]
                };
                self[source] = Block {
                    file: false,
                    id: None,
                    ..self[source]
                };
            }
            Ordering::Greater => {
                // There is some space left in self[i] after replacing it with new_block
                let dest_end = self[dest].end;
                self[dest] = Block {
                    start: self[dest].start,
                    end: self[dest].start + self[source].len(),
                    ..self[source]
                };
                self.blocks.insert(
                    dest + 1,
                    Block {
                        file: false,
                        start: self[dest].end,
                        end: dest_end,
                        id: None,
                        searched: false,
                    },
                );
                log::trace!("source block: {:?}", self.blocks[source + 1]);
                self[source + 1].file = false;
                self[source + 1].id = None;
            }
            Ordering::Less => {
                // There is not enough space left, so the source is split up and the last chunk is
                // moved to the destination
                self[dest].file = self[source].file;
                self[dest].id = self[source].id;
                self[source].end -= self[dest].len();
                self.blocks.insert(
                    source + 1,
                    Block {
                        file: false,
                        id: None,
                        start: self[source].end,
                        end: self[source].end + self[dest].len(),
                        searched: false,
                    },
                );
            }
        }
        // clean up
        if let Some(chunk) = self.blocks.last_chunk::<2>() {
            log::trace!("last_chunk: {:?}", chunk);
            if !chunk[0].file && !chunk[1].file {
                log::trace!("merging last two blocks");
                let end = self.blocks.pop().unwrap().end;
                self.blocks.last_mut().unwrap().end = end;
            }
        }
        self.check_integrity();
    }

    fn compress_step(&mut self) -> Result<(), CompressionError> {
        log::debug!("compress step {}", self);
        let first_non_file = self.get_first_non_file();
        let last_file = self.get_last_file();
        log::trace!("first_non_file: {}", first_non_file);
        log::trace!("last_file: {}", last_file);
        if first_non_file == self.blocks.len() - 1 {
            return Err(CompressionError::EmptyBlock);
        }
        self.replace_i_th_block(first_non_file, last_file);
        Ok(())
    }

    fn compress(&mut self, compression_method: CompressionMethod) {
        match compression_method {
            CompressionMethod::Simple => while self.compress_step().is_ok() {},
            CompressionMethod::NonFragmented => {
                while let Some(unsearched) = self.get_last_unsearched_file() {
                    let b = &self[unsearched];
                    let blen = b.len();
                    let bid = b.id;
                    self[unsearched].searched = true;
                    if let Some(fitting_slot) = self.get_first_non_file_n(blen) {
                        log::trace!("{}", self);
                        if fitting_slot > unsearched {
                            continue;
                        }
                        let fitting_slot_len = self[fitting_slot].len();
                        // Fill the fitting_slot
                        self[fitting_slot] = Block {
                            start: self[fitting_slot].start,
                            end: self[fitting_slot].start + blen,
                            id: bid,
                            file: true,
                            searched: true,
                        };
                        // Delete the file
                        self[unsearched].file = false;
                        self[unsearched].id = None;
                        // Include an empty slot
                        if fitting_slot_len > blen {
                            self.blocks.insert(
                                fitting_slot + 1,
                                Block {
                                    start: self[fitting_slot].end,
                                    end: self[fitting_slot].end + fitting_slot_len - blen,
                                    id: None,
                                    file: false,
                                    searched: true,
                                },
                            );
                        }
                    } else {
                        continue;
                    }
                }
                self.check_integrity();
            }
        }
    }

    fn read_from_string(s: &str) -> Self {
        let mut file = true;
        let mut start: isize = 0;
        let mut id: usize = 0;
        let mut result = BlockList::default();
        for (i, c) in s.chars().enumerate() {
            if c == '\n' {
                continue;
            }
            let n: isize = c.to_string().parse().unwrap();
            let end = start + n;
            let new_block = Block {
                file,
                id: if file { Some(id) } else { None },
                start,
                end,
                searched: false,
            };
            start = end;
            if file {
                id += 1;
            }
            file = !file;
            log::trace!(
                "i: {}, c: {}, start {}, end {}",
                i,
                c,
                new_block.start,
                new_block.end
            );
            if new_block.len() > 0 {
                result.blocks.push(new_block);
            }
        }
        result.check_integrity();
        log::info!("finished read");
        result
    }

    fn read_from_file(file_name: &str) -> Self {
        let s: &str = &read_file(file_name).unwrap_or("12345".to_string());
        BlockList::read_from_string(s)
    }
}

impl Display for BlockList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for block in self.blocks.iter() {
            for _ in block.start..block.end {
                write!(
                    f,
                    "{}",
                    block
                        .id
                        .map(|id| id.to_string())
                        .unwrap_or_else(|| ".".to_string())
                )?;
            }
        }
        Ok(())
    }
}

fn read_file(file_name: &str) -> std::io::Result<String> {
    log::info!("Attempting to read file: {}", file_name);
    let mut file = File::open(file_name).map_err(|e| {
        log::error!("Failed to open file: {}", e);
        e
    })?;
    log::info!("Successfully opened file: {}", file_name);
    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        log::error!("Failed to read file: {}", e);
        return Err(e);
    }
    log::info!("Successfully read file: {}", file_name);
    Ok(contents)
}

fn main() {
    let start = Instant::now();
    env_logger::builder()
        .format_source_path(true)
        .format_timestamp(None)
        .format_target(false)
        .format_module_path(false)
        .init();
    let args: Vec<String> = env::args().collect();
    let mut block_list: BlockList = {
        if args.len() != 2 {
            log::warn!("Usage: {} <filename>", args[0]);
            BlockList::read_from_file("kjk")
        } else {
            BlockList::read_from_file(&args[1])
        }
    };
    let mut second = block_list.clone();
    // log::debug!("BlockList: {:?}", block_list);
    log::debug!("BlockList: {}", block_list);
    block_list.compress(CompressionMethod::Simple);
    log::debug!("BlockList: {}", block_list);
    log::info!("checksum: {}", block_list.checksum());
    second.compress(CompressionMethod::NonFragmented);
    log::debug!("BlockList: {}", second);
    log::info!("checksum: {}", second.checksum());

    log::info!("time elapsed: {:?}", start.elapsed());
}
