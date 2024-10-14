// https://codeforces.com/group/QsHQUUet4f/contest/489465/problem/A
pub mod solution {

use std::collections::BTreeMap;

use crate::io::input::Input;
use crate::io::output::Output;
use crate::numbers::primes::factorize::Factorize;
use crate::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    let mut frequency_map: BTreeMap<u8, i64> = BTreeMap::new();
    for c in s {
        *frequency_map.entry(c).or_default() += 1;
    }
    let mut sorted_by_frequency = vec![];
    for (c, _) in &frequency_map {
        sorted_by_frequency.push(c);
    }
    sorted_by_frequency.sort_unstable_by(|a, b| frequency_map[b].cmp(&frequency_map[a]));

    for d in n.divisors() {
        let different_letters = d;
        let frequency_needed = n as i64 / d;

        let mut extra: Vec<usize> = vec![];
        let mut current_letter = *frequency_map.first_entry().unwrap().key();
        let mut last_letter_index = sorted_by_frequency.len() - 1;
        for (letter, &count) in &frequency_map {
            if count + extra.len() as i64 >= frequency_needed {
                // Change some from extra, potentially leaving some.
            } else {
                // We need to get from the right too
                let last_letter = sorted_by_frequency[last_letter_index];
                let last_count = frequency_map[last_letter];
                frequency_map[last_letter] += 2;
            }
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

}
pub mod collections {
pub mod bit_set {
use crate::collections::slice_ext::legacy_fill::LegacyFill;
use crate::numbers::num_traits::bit_ops::BitOps;
use std::ops::{BitAndAssign, BitOrAssign, Index};

const TRUE: bool = true;
const FALSE: bool = false;

#[derive(Clone, Eq, PartialEq)]
pub struct BitSet {
    data: Vec<u64>,
    len: usize,
}

impl BitSet {
    pub fn new(len: usize) -> Self {
        let data_len = if len == 0 {
            0
        } else {
            Self::index(len - 1) + 1
        };
        Self {
            data: vec![0; data_len],
            len,
        }
    }
    
    pub fn from_slice(len: usize, set: &[usize]) -> Self {
        let mut res = Self::new(len);
        for &i in set {
            res.set(i);
        }
        res
    }

    pub fn set(&mut self, at: usize) {
        assert!(at < self.len);
        self.data[Self::index(at)].set_bit(at & 63);
    }

    pub fn unset(&mut self, at: usize) {
        assert!(at < self.len);
        self.data[Self::index(at)].unset_bit(at & 63);
    }

    pub fn change(&mut self, at: usize, value: bool) {
        if value {
            self.set(at);
        } else {
            self.unset(at);
        }
    }

    pub fn flip(&mut self, at: usize) {
        self.change(at, !self[at]);
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn fill(&mut self, value: bool) {
        // 1.43
        self.data.legacy_fill(if value { std::u64::MAX } else { 0 })
    }

    pub fn is_superset(&self, other: &Self) -> bool {
        assert_eq!(self.len, other.len);
        for i in 0..self.data.len() {
            if self.data[i] & other.data[i] != other.data[i] {
                return false;
            }
        }
        true
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        other.is_superset(self)
    }

    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.into_iter()
    }

    fn index(at: usize) -> usize {
        at >> 6
    }

    pub fn count_ones(&self) -> usize {
        self.data.iter().map(|x| x.count_ones() as usize).sum()
    }
}

pub struct BitSetIter<'s> {
    at: usize,
    inside: usize,
    set: &'s BitSet,
}

impl<'s> Iterator for BitSetIter<'s> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.at < self.set.data.len()
            && (self.inside == 64 || (self.set.data[self.at] >> self.inside) == 0)
        {
            self.at += 1;
            self.inside = 0;
        }
        if self.at == self.set.data.len() {
            None
        } else {
            while !self.set.data[self.at].is_set(self.inside) {
                self.inside += 1;
            }
            let res = self.at * 64 + self.inside;
            if res < self.set.len {
                self.inside += 1;
                Some(res)
            } else {
                None
            }
        }
    }
}

impl<'a> IntoIterator for &'a BitSet {
    type Item = usize;
    type IntoIter = BitSetIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BitSetIter {
            at: 0,
            inside: 0,
            set: self,
        }
    }
}

impl BitOrAssign<&BitSet> for BitSet {
    fn bitor_assign(&mut self, rhs: &BitSet) {
        assert_eq!(self.len, rhs.len);
        for (i, &j) in self.data.iter_mut().zip(rhs.data.iter()) {
            *i |= j;
        }
    }
}

impl BitAndAssign<&BitSet> for BitSet {
    fn bitand_assign(&mut self, rhs: &BitSet) {
        assert_eq!(self.len, rhs.len);
        for (i, &j) in self.data.iter_mut().zip(rhs.data.iter()) {
            *i &= j;
        }
    }
}

impl Index<usize> for BitSet {
    type Output = bool;

    fn index(&self, at: usize) -> &Self::Output {
        assert!(at < self.len);
        if self.data[Self::index(at)].is_set(at & 63) {
            &TRUE
        } else {
            &FALSE
        }
    }
}

impl From<Vec<bool>> for BitSet {
    fn from(data: Vec<bool>) -> Self {
        let mut res = Self::new(data.len());
        for (i, &value) in data.iter().enumerate() {
            res.change(i, value);
        }
        res
    }
}
}
pub mod iter_ext {
pub mod collect {
pub trait IterCollect<T>: Iterator<Item = T> + Sized {
    fn collect_vec(self) -> Vec<T> {
        self.collect()
    }
}

impl<T, I: Iterator<Item = T> + Sized> IterCollect<T> for I {}
}
}
pub mod slice_ext {
pub mod indices {
use std::ops::Range;

pub trait Indices {
    fn indices(&self) -> Range<usize>;
}

impl<T> Indices for [T] {
    fn indices(&self) -> Range<usize> {
        0..self.len()
    }
}
}
pub mod legacy_fill {
// 1.50
pub trait LegacyFill<T> {
    fn legacy_fill(&mut self, val: T);
}

impl<T: Clone> LegacyFill<T> for [T] {
    fn legacy_fill(&mut self, val: T) {
        for el in self.iter_mut() {
            *el = val.clone();
        }
    }
}
}
}
pub mod vec_ext {
pub mod default {
pub fn default_vec<T: Default>(len: usize) -> Vec<T> {
    let mut v = Vec::with_capacity(len);
    for _ in 0..len {
        v.push(T::default());
    }
    v
}
}
pub mod sorted {
pub trait Sorted {
    fn sorted(self) -> Self;
}

impl<T: Ord> Sorted for Vec<T> {
    fn sorted(mut self) -> Self {
        self.sort();
        self
    }
}
}
}
}
pub mod io {
pub mod input {
use crate::collections::vec_ext::default::default_vec;
use std::io::Read;

pub struct Input<'s> {
    input: &'s mut dyn Read,
    buf: Vec<u8>,
    at: usize,
    buf_read: usize,
}

macro_rules! read_impl {
    ($t: ty, $read_name: ident, $read_vec_name: ident) => {
        pub fn $read_name(&mut self) -> $t {
            self.read()
        }

        pub fn $read_vec_name(&mut self, len: usize) -> Vec<$t> {
            self.read_vec(len)
        }
    };

    ($t: ty, $read_name: ident, $read_vec_name: ident, $read_pair_vec_name: ident) => {
        read_impl!($t, $read_name, $read_vec_name);

        pub fn $read_pair_vec_name(&mut self, len: usize) -> Vec<($t, $t)> {
            self.read_vec(len)
        }
    };
}

impl<'s> Input<'s> {
    const DEFAULT_BUF_SIZE: usize = 4096;

    pub fn new(input: &'s mut dyn Read) -> Self {
        Self {
            input,
            buf: default_vec(Self::DEFAULT_BUF_SIZE),
            at: 0,
            buf_read: 0,
        }
    }

    pub fn new_with_size(input: &'s mut dyn Read, buf_size: usize) -> Self {
        Self {
            input,
            buf: default_vec(buf_size),
            at: 0,
            buf_read: 0,
        }
    }

    pub fn get(&mut self) -> Option<u8> {
        if self.refill_buffer() {
            let res = self.buf[self.at];
            self.at += 1;
            if res == b'\r' {
                if self.refill_buffer() && self.buf[self.at] == b'\n' {
                    self.at += 1;
                }
                return Some(b'\n');
            }
            Some(res)
        } else {
            None
        }
    }

    pub fn peek(&mut self) -> Option<u8> {
        if self.refill_buffer() {
            let res = self.buf[self.at];
            Some(if res == b'\r' { b'\n' } else { res })
        } else {
            None
        }
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(b) = self.peek() {
            if !char::from(b).is_whitespace() {
                return;
            }
            self.get();
        }
    }

    pub fn next_token(&mut self) -> Option<Vec<u8>> {
        self.skip_whitespace();
        let mut res = Vec::new();
        while let Some(c) = self.get() {
            if char::from(c).is_whitespace() {
                break;
            }
            res.push(c);
        }
        if res.is_empty() {
            None
        } else {
            Some(res)
        }
    }

    //noinspection RsSelfConvention
    pub fn is_exhausted(&mut self) -> bool {
        self.peek().is_none()
    }

    pub fn read<T: Readable>(&mut self) -> T {
        T::read(self)
    }

    pub fn read_vec<T: Readable>(&mut self, size: usize) -> Vec<T> {
        let mut res = Vec::with_capacity(size);
        for _ in 0..size {
            res.push(self.read());
        }
        res
    }

    pub fn read_char(&mut self) -> char {
        self.skip_whitespace();
        self.get().unwrap().into()
    }

    read_impl!(u32, read_unsigned, read_unsigned_vec);
    read_impl!(u64, read_u64, read_u64_vec);
    read_impl!(usize, read_size, read_size_vec, read_size_pair_vec);
    read_impl!(i32, read_int, read_int_vec, read_int_pair_vec);
    read_impl!(i64, read_long, read_long_vec, read_long_pair_vec);
    read_impl!(i128, read_i128, read_i128_vec);

    fn refill_buffer(&mut self) -> bool {
        if self.at == self.buf_read {
            self.at = 0;
            self.buf_read = self.input.read(&mut self.buf).unwrap();
            self.buf_read != 0
        } else {
            true
        }
    }
}

pub trait Readable {
    fn read(input: &mut Input) -> Self;
}

impl Readable for char {
    fn read(input: &mut Input) -> Self {
        input.read_char()
    }
}

impl<T: Readable> Readable for Vec<T> {
    fn read(input: &mut Input) -> Self {
        let size = input.read();
        input.read_vec(size)
    }
}

macro_rules! read_integer {
    ($($t:ident)+) => {$(
        impl Readable for $t {
            fn read(input: &mut Input) -> Self {
                input.skip_whitespace();
                let mut c = input.get().unwrap();
                let sgn = match c {
                    b'-' => {
                        c = input.get().unwrap();
                        true
                    }
                    b'+' => {
                        c = input.get().unwrap();
                        false
                    }
                    _ => false,
                };
                let mut res = 0;
                loop {
                    assert!(c.is_ascii_digit());
                    res *= 10;
                    let d = (c - b'0') as $t;
                    if sgn {
                        res -= d;
                    } else {
                        res += d;
                    }
                    match input.get() {
                        None => break,
                        Some(ch) => {
                            if ch.is_ascii_whitespace() {
                                break;
                            } else {
                                c = ch;
                            }
                        }
                    }
                }
                res
            }
        }
    )+};
}

read_integer!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

macro_rules! tuple_readable {
    ($($name:ident)+) => {
        impl<$($name: Readable), +> Readable for ($($name,)+) {
            fn read(input: &mut Input) -> Self {
                ($($name::read(input),)+)
            }
        }
    }
}

tuple_readable! {T}
tuple_readable! {T U}
tuple_readable! {T U V}
tuple_readable! {T U V X}
tuple_readable! {T U V X Y}
tuple_readable! {T U V X Y Z}
tuple_readable! {T U V X Y Z A}
tuple_readable! {T U V X Y Z A B}
tuple_readable! {T U V X Y Z A B C}
tuple_readable! {T U V X Y Z A B C D}
tuple_readable! {T U V X Y Z A B C D E}
tuple_readable! {T U V X Y Z A B C D E F}

impl Read for Input<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.at == self.buf_read {
            self.input.read(buf)
        } else {
            let mut i = 0;
            while i < buf.len() && self.at < self.buf_read {
                buf[i] = self.buf[self.at];
                i += 1;
                self.at += 1;
            }
            Ok(i)
        }
    }
}
}
pub mod output {
use crate::collections::vec_ext::default::default_vec;
use std::io::{stderr, Stderr, Write};

#[derive(Copy, Clone)]
pub enum BoolOutput {
    YesNo,
    YesNoCaps,
    PossibleImpossible,
    Custom(&'static str, &'static str),
}

impl BoolOutput {
    pub fn output(&self, output: &mut Output, val: bool) {
        (if val { self.yes() } else { self.no() }).write(output);
    }

    fn yes(&self) -> &str {
        match self {
            BoolOutput::YesNo => "Yes",
            BoolOutput::YesNoCaps => "YES",
            BoolOutput::PossibleImpossible => "Possible",
            BoolOutput::Custom(yes, _) => yes,
        }
    }

    fn no(&self) -> &str {
        match self {
            BoolOutput::YesNo => "No",
            BoolOutput::YesNoCaps => "NO",
            BoolOutput::PossibleImpossible => "Impossible",
            BoolOutput::Custom(_, no) => no,
        }
    }
}

pub struct Output<'s> {
    output: &'s mut dyn Write,
    buf: Vec<u8>,
    at: usize,
    auto_flush: bool,
    bool_output: BoolOutput,
}

impl<'s> Output<'s> {
    const DEFAULT_BUF_SIZE: usize = 4096;

    pub fn new(output: &'s mut dyn Write) -> Self {
        Self {
            output,
            buf: default_vec(Self::DEFAULT_BUF_SIZE),
            at: 0,
            auto_flush: false,
            bool_output: BoolOutput::YesNoCaps,
        }
    }

    pub fn new_with_auto_flush(output: &'s mut dyn Write) -> Self {
        Self {
            output,
            buf: default_vec(Self::DEFAULT_BUF_SIZE),
            at: 0,
            auto_flush: true,
            bool_output: BoolOutput::YesNoCaps,
        }
    }

    pub fn flush(&mut self) {
        if self.at != 0 {
            self.output.write_all(&self.buf[..self.at]).unwrap();
            self.output.flush().unwrap();
            self.at = 0;
        }
    }

    pub fn print<T: Writable>(&mut self, s: T) {
        s.write(self);
        self.maybe_flush();
    }

    pub fn print_line<T: Writable>(&mut self, s: T) {
        self.print(s);
        self.put(b'\n');
        self.maybe_flush();
    }

    pub fn put(&mut self, b: u8) {
        self.buf[self.at] = b;
        self.at += 1;
        if self.at == self.buf.len() {
            self.flush();
        }
    }

    pub fn maybe_flush(&mut self) {
        if self.auto_flush {
            self.flush();
        }
    }

    pub fn print_per_line<T: Writable>(&mut self, arg: &[T]) {
        for i in arg {
            i.write(self);
            self.put(b'\n');
        }
    }

    pub fn print_iter<T: Writable, I: Iterator<Item = T>>(&mut self, iter: I) {
        let mut first = true;
        for e in iter {
            if first {
                first = false;
            } else {
                self.put(b' ');
            }
            e.write(self);
        }
    }

    pub fn print_iter_ref<'a, T: 'a + Writable, I: Iterator<Item = &'a T>>(&mut self, iter: I) {
        let mut first = true;
        for e in iter {
            if first {
                first = false;
            } else {
                self.put(b' ');
            }
            e.write(self);
        }
    }

    pub fn set_bool_output(&mut self, bool_output: BoolOutput) {
        self.bool_output = bool_output;
    }
}

impl Write for Output<'_> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut start = 0usize;
        let mut rem = buf.len();
        while rem > 0 {
            let len = (self.buf.len() - self.at).min(rem);
            self.buf[self.at..self.at + len].copy_from_slice(&buf[start..start + len]);
            self.at += len;
            if self.at == self.buf.len() {
                self.flush();
            }
            start += len;
            rem -= len;
        }
        self.maybe_flush();
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.flush();
        Ok(())
    }
}

pub trait Writable {
    fn write(&self, output: &mut Output);
}

impl Writable for &str {
    fn write(&self, output: &mut Output) {
        output.write_all(self.as_bytes()).unwrap();
    }
}

impl Writable for String {
    fn write(&self, output: &mut Output) {
        output.write_all(self.as_bytes()).unwrap();
    }
}

impl Writable for char {
    fn write(&self, output: &mut Output) {
        output.put(*self as u8);
    }
}

impl<T: Writable> Writable for [T] {
    fn write(&self, output: &mut Output) {
        output.print_iter_ref(self.iter());
    }
}

impl<T: Writable, const N: usize> Writable for [T; N] {
    fn write(&self, output: &mut Output) {
        output.print_iter_ref(self.iter());
    }
}

impl<T: Writable> Writable for &T {
    fn write(&self, output: &mut Output) {
        T::write(self, output)
    }
}

impl<T: Writable> Writable for Vec<T> {
    fn write(&self, output: &mut Output) {
        self.as_slice().write(output);
    }
}

impl Writable for () {
    fn write(&self, _output: &mut Output) {}
}

macro_rules! write_to_string {
    ($($t:ident)+) => {$(
        impl Writable for $t {
            fn write(&self, output: &mut Output) {
                self.to_string().write(output);
            }
        }
    )+};
}

write_to_string!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

macro_rules! tuple_writable {
    ($name0:ident $($name:ident: $id:tt )*) => {
        impl<$name0: Writable, $($name: Writable,)*> Writable for ($name0, $($name,)*) {
            fn write(&self, out: &mut Output) {
                self.0.write(out);
                $(
                out.put(b' ');
                self.$id.write(out);
                )*
            }
        }
    }
}

tuple_writable! {T}
tuple_writable! {T U:1}
tuple_writable! {T U:1 V:2}
tuple_writable! {T U:1 V:2 X:3}
tuple_writable! {T U:1 V:2 X:3 Y:4}
tuple_writable! {T U:1 V:2 X:3 Y:4 Z:5}
tuple_writable! {T U:1 V:2 X:3 Y:4 Z:5 A:6}
tuple_writable! {T U:1 V:2 X:3 Y:4 Z:5 A:6 B:7}

impl<T: Writable> Writable for Option<T> {
    fn write(&self, output: &mut Output) {
        match self {
            None => (-1).write(output),
            Some(t) => t.write(output),
        }
    }
}

impl Writable for bool {
    fn write(&self, output: &mut Output) {
        let bool_output = output.bool_output;
        bool_output.output(output, *self)
    }
}

static mut ERR: Option<Stderr> = None;
pub fn err() -> Output<'static> {
    unsafe {
        if ERR.is_none() {
            ERR = Some(stderr());
        }
        Output::new_with_auto_flush(ERR.as_mut().unwrap())
    }
}
}
}
pub mod misc {
pub mod random {
use crate::collections::slice_ext::indices::Indices;
use std::time::SystemTime;

const NN: usize = 312;
const MM: usize = 156;
const MATRIX_A: u64 = 0xB5026F5AA96619E9;
const UM: u64 = 0xFFFFFFFF80000000;
const LM: u64 = 0x7FFFFFFF;
const F: u64 = 6364136223846793005;
const MAG01: [u64; 2] = [0, MATRIX_A];

pub struct Random {
    mt: [u64; NN],
    index: usize,
}

impl Random {
    pub fn new(seed: u64) -> Self {
        let mut res = Self {
            mt: [0u64; NN],
            index: NN,
        };
        res.mt[0] = seed;
        for i in 1..NN {
            res.mt[i] = F
                .wrapping_mul(res.mt[i - 1] ^ (res.mt[i - 1] >> 62))
                .wrapping_add(i as u64);
        }
        res
    }

    pub fn gen(&mut self) -> u64 {
        if self.index == NN {
            for i in 0..(NN - MM) {
                let x = (self.mt[i] & UM) | (self.mt[i + 1] & LM);
                self.mt[i] = self.mt[i + MM] ^ (x >> 1) ^ MAG01[(x & 1) as usize];
            }
            for i in (NN - MM)..(NN - 1) {
                let x = (self.mt[i] & UM) | (self.mt[i + 1] & LM);
                self.mt[i] = self.mt[i + MM - NN] ^ (x >> 1) ^ MAG01[(x & 1) as usize];
            }
            let x = (self.mt[NN - 1] & UM) | (self.mt[0] & LM);
            self.mt[NN - 1] = self.mt[MM - 1] ^ (x >> 1) ^ MAG01[(x & 1) as usize];
            self.index = 0;
        }
        let mut x = self.mt[self.index];
        self.index += 1;
        x ^= (x >> 29) & 0x5555555555555555;
        x ^= (x << 17) & 0x71D67FFFEDA60000;
        x ^= (x << 37) & 0xFFF7EEE000000000;
        x ^= x >> 43;
        x
    }

    pub fn next(&mut self, n: u64) -> u64 {
        self.gen() % n
    }

    pub fn next_bounds(&mut self, f: u64, t: u64) -> u64 {
        f + self.next(t - f + 1)
    }
}

static mut RAND: Option<Random> = None;

pub fn random() -> &'static mut Random {
    unsafe {
        if RAND.is_none() {
            RAND = Some(Random::new(
                (SystemTime::UNIX_EPOCH.elapsed().unwrap().as_nanos() & 0xFFFFFFFFFFFFFFFF) as u64,
            ));
        }
        RAND.as_mut().unwrap()
    }
}

pub trait Shuffle {
    fn shuffle(&mut self);
}

impl<T> Shuffle for [T] {
    fn shuffle(&mut self) {
        for i in self.indices() {
            let at = (random().gen() % ((i + 1) as u64)) as usize;
            self.swap(i, at);
        }
    }
}
}
pub mod recursive_function {
use std::marker::PhantomData;

macro_rules! recursive_function {
    ($name: ident, $trait: ident, ($($type: ident $arg: ident,)*)) => {
        pub trait $trait<$($type, )*Output> {
            fn call(&mut self, $($arg: $type,)*) -> Output;
        }

        pub struct $name<F, $($type, )*Output>
        where
            F: FnMut(&mut dyn $trait<$($type, )*Output>, $($type, )*) -> Output,
        {
            f: std::cell::UnsafeCell<F>,
            $($arg: PhantomData<$type>,
            )*
            phantom_output: PhantomData<Output>,
        }

        impl<F, $($type, )*Output> $name<F, $($type, )*Output>
        where
            F: FnMut(&mut dyn $trait<$($type, )*Output>, $($type, )*) -> Output,
        {
            pub fn new(f: F) -> Self {
                Self {
                    f: std::cell::UnsafeCell::new(f),
                    $($arg: Default::default(),
                    )*
                    phantom_output: Default::default(),
                }
            }
        }

        impl<F, $($type, )*Output> $trait<$($type, )*Output> for $name<F, $($type, )*Output>
        where
            F: FnMut(&mut dyn $trait<$($type, )*Output>, $($type, )*) -> Output,
        {
            fn call(&mut self, $($arg: $type,)*) -> Output {
                unsafe { (*self.f.get())(self, $($arg, )*) }
            }
        }
    }
}

recursive_function!(RecursiveFunction0, Callable0, ());
recursive_function!(RecursiveFunction, Callable, (Arg arg,));
recursive_function!(RecursiveFunction2, Callable2, (Arg1 arg1, Arg2 arg2,));
recursive_function!(RecursiveFunction3, Callable3, (Arg1 arg1, Arg2 arg2, Arg3 arg3,));
recursive_function!(RecursiveFunction4, Callable4, (Arg1 arg1, Arg2 arg2, Arg3 arg3, Arg4 arg4,));
recursive_function!(RecursiveFunction5, Callable5, (Arg1 arg1, Arg2 arg2, Arg3 arg3, Arg4 arg4, Arg5 arg5,));
recursive_function!(RecursiveFunction6, Callable6, (Arg1 arg1, Arg2 arg2, Arg3 arg3, Arg4 arg4, Arg5 arg5, Arg6 arg6,));
recursive_function!(RecursiveFunction7, Callable7, (Arg1 arg1, Arg2 arg2, Arg3 arg3, Arg4 arg4, Arg5 arg5, Arg6 arg6, Arg7 arg7,));
recursive_function!(RecursiveFunction8, Callable8, (Arg1 arg1, Arg2 arg2, Arg3 arg3, Arg4 arg4, Arg5 arg5, Arg6 arg6, Arg7 arg7, Arg8 arg8,));
recursive_function!(RecursiveFunction9, Callable9, (Arg1 arg1, Arg2 arg2, Arg3 arg3, Arg4 arg4, Arg5 arg5, Arg6 arg6, Arg7 arg7, Arg8 arg8, Arg9 arg9,));
}
pub mod value {
use std::hash::Hash;

pub trait Value<T>: Copy + Eq + Hash {
    fn val() -> T;
}

pub trait ConstValue<T>: Value<T> {
    const VAL: T;
}

impl<T, V: ConstValue<T>> Value<T> for V {
    fn val() -> T {
        Self::VAL
    }
}

#[macro_export]
macro_rules! value {
    ($name: ident: $t: ty = $val: expr) => {
        #[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Default)]
        pub struct $name {}

        impl $crate::misc::value::ConstValue<$t> for $name {
            const VAL: $t = $val;
        }
    };
}

pub trait DynamicValue<T>: Value<T> {
    //noinspection RsSelfConvention
    fn set_val(t: T);
}

#[macro_export]
macro_rules! dynamic_value {
    ($name: ident: $t: ty) => {
        static mut VAL: Option<$t> = None;

        #[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
        struct $name {}

        impl $crate::misc::value::DynamicValue<$t> for $name {
            fn set_val(t: $t) {
                unsafe {
                    VAL = Some(t);
                }
            }
        }

        impl $crate::misc::value::Value<$t> for $name {
            fn val() -> $t {
                unsafe { VAL.unwrap() }
            }
        }
    };
    ($name: ident: $t: ty = $val: expr) => {
        dynamic_value!($name: $t);

        $name::set_val($val);
    };
}
}
pub mod when {
#[macro_export]
macro_rules! when {
    {$($cond: expr => $then: expr,)*} => {
        match () {
            $(_ if $cond => $then,)*
            _ => unreachable!(),
        }
    };
    {$($cond: expr => $then: expr,)* else $(=>)? $else: expr,} => {
        match () {
            $(_ if $cond => $then,)*
            _ => $else,
        }
    };
}
}
}
pub mod numbers {
pub mod gcd {
use crate::numbers::num_traits::add_sub::AddSub;
use crate::numbers::num_traits::mul_div_rem::{MulDivRem, Multable};
use crate::numbers::num_traits::wideable::Wideable;
use crate::numbers::num_traits::zero_one::ZeroOne;
use std::mem::swap;

pub fn extended_gcd<T: Copy + ZeroOne + AddSub + MulDivRem + Wideable + PartialEq>(
    a: T,
    b: T,
) -> (T, T::W, T::W)
where
    T::W: Copy + ZeroOne + AddSub + Multable,
{
    if a == T::zero() {
        (b, T::W::zero(), T::W::one())
    } else {
        let (d, y, mut x) = extended_gcd(b % a, a);
        x -= T::W::from(b / a) * y;
        (d, x, y)
    }
}

pub fn gcd<T: Copy + ZeroOne + MulDivRem + PartialEq>(mut a: T, mut b: T) -> T {
    while b != T::zero() {
        a %= b;
        swap(&mut a, &mut b);
    }
    a
}

pub fn lcm<T: Copy + ZeroOne + MulDivRem + PartialEq>(a: T, b: T) -> T {
    (a / gcd(a, b)) * b
}
}
pub mod mod_int {
use crate::io::input::{Input, Readable};
use crate::io::output::{Output, Writable};
use crate::misc::value::Value;
use crate::numbers::gcd::extended_gcd;
use crate::numbers::num_traits::add_sub::AddSub;
use crate::numbers::num_traits::as_index::AsIndex;
use crate::numbers::num_traits::from_u8::FromU8;
use crate::numbers::num_traits::invertable::Invertable;
use crate::numbers::num_traits::mul_div_rem::{MulDiv, MulDivRem};
use crate::numbers::num_traits::wideable::Wideable;
use crate::numbers::num_traits::zero_one::ZeroOne;
use crate::{value, when};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait BaseModInt:
    AddSub + MulDiv + Neg<Output = Self> + Copy + ZeroOne + PartialEq + Invertable<Output = Self>
{
    type W: AddSub + MulDivRem + Copy + ZeroOne + From<Self::T>;
    type T: AddSub + MulDivRem + Copy + PartialEq + ZeroOne + Wideable<W = Self::W> + Ord;

    fn from(v: Self::T) -> Self;
    fn module() -> Self::T;
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct ModInt<T, V: Value<T>> {
    n: T,
    phantom: PhantomData<V>,
}

impl<T: Copy, V: Value<T>> ModInt<T, V> {
    pub fn val(&self) -> T {
        self.n
    }
}

impl<T: AddSub + Copy + ZeroOne + Ord, V: Value<T>> ModInt<T, V> {
    unsafe fn unchecked_new(n: T) -> Self {
        debug_assert!(n >= T::zero() && n < V::val());
        Self {
            n,
            phantom: Default::default(),
        }
    }

    unsafe fn maybe_subtract_mod(mut n: T) -> T {
        debug_assert!(n < V::val() + V::val() && n >= T::zero());
        if n >= V::val() {
            n -= V::val();
        }
        n
    }
}

impl<T: AddSub + Copy + ZeroOne + Ord + MulDivRem, V: Value<T>> ModInt<T, V> {
    pub fn new(n: T) -> Self {
        unsafe { Self::unchecked_new(Self::maybe_subtract_mod(n % (V::val()) + V::val())) }
    }
}

impl<T: Copy + ZeroOne + AddSub + MulDivRem + Wideable + PartialEq + Ord + Hash, V: Value<T>>
    ModInt<T, V>
where
    T::W: Copy + ZeroOne + AddSub + MulDivRem,
{
    pub fn log(&self, alpha: Self) -> T {
        let mut base = HashMap::new();
        let mut exp = T::zero();
        let mut pow = Self::one();
        let mut inv = *self;
        let alpha_inv = alpha.inv().unwrap();
        while exp * exp < Self::module() {
            if inv == Self::one() {
                return exp;
            }
            base.insert(inv, exp);
            exp += T::one();
            pow *= alpha;
            inv *= alpha_inv;
        }
        let step = pow;
        let mut i = T::one();
        loop {
            if let Some(b) = base.get(&pow) {
                break exp * i + *b;
            }
            pow *= step;
            i += T::one();
        }
    }
}

impl<T: Wideable + AddSub + Copy + ZeroOne + Ord, V: Value<T>> ModInt<T, V>
where
    T::W: MulDivRem,
{
    pub fn new_from_wide(n: T::W) -> Self {
        unsafe {
            Self::unchecked_new(Self::maybe_subtract_mod(
                T::downcast(n % (V::val()).into()) + V::val(),
            ))
        }
    }
}

impl<T: Copy + ZeroOne + AddSub + MulDivRem + Wideable + PartialEq + Ord, V: Value<T>> Invertable
    for ModInt<T, V>
where
    T::W: Copy + ZeroOne + AddSub + MulDivRem,
{
    type Output = Self;

    fn inv(&self) -> Option<Self> {
        let (g, x, _) = extended_gcd(self.n, V::val());
        if g != T::one() {
            None
        } else {
            Some(Self::new_from_wide(x))
        }
    }
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord, V: Value<T>> BaseModInt
    for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    type W = T::W;
    type T = T;

    fn from(v: Self::T) -> Self {
        Self::new(v)
    }

    fn module() -> T {
        V::val()
    }
}

impl<T: AddSub + Copy + ZeroOne + Ord + MulDivRem, V: Value<T>> From<T> for ModInt<T, V> {
    fn from(n: T) -> Self {
        Self::new(n)
    }
}

impl<T: AddSub + Copy + ZeroOne + Ord, V: Value<T>> AddAssign for ModInt<T, V> {
    fn add_assign(&mut self, rhs: Self) {
        self.n = unsafe { Self::maybe_subtract_mod(self.n + rhs.n) };
    }
}

impl<T: AddSub + Copy + ZeroOne + Ord, V: Value<T>> Add for ModInt<T, V> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T: AddSub + Copy + ZeroOne + Ord, V: Value<T>> SubAssign for ModInt<T, V> {
    fn sub_assign(&mut self, rhs: Self) {
        self.n = unsafe { Self::maybe_subtract_mod(self.n + V::val() - rhs.n) };
    }
}

impl<T: AddSub + Copy + ZeroOne + Ord, V: Value<T>> Sub for ModInt<T, V> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<T: AddSub + MulDivRem + Copy + Wideable + ZeroOne + Ord, V: Value<T>> MulAssign
    for ModInt<T, V>
where
    T::W: MulDivRem + Copy,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.n = T::downcast(T::W::from(self.n) * T::W::from(rhs.n) % T::W::from(V::val()));
    }
}

impl<T: AddSub + MulDivRem + Copy + Wideable + ZeroOne + Ord, V: Value<T>> Mul for ModInt<T, V>
where
    T::W: MulDivRem + Copy,
{
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord, V: Value<T>> DivAssign
    for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    #[allow(clippy::suspicious_op_assign_impl)]
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs.inv().unwrap();
    }
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord, V: Value<T>> Div
    for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        self /= rhs;
        self
    }
}

impl<T: AddSub + Copy + ZeroOne + Ord, V: Value<T>> Neg for ModInt<T, V> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.n = unsafe { Self::maybe_subtract_mod(V::val() - self.n) };
        self
    }
}

impl<T: Display, V: Value<T>> Display for ModInt<T, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <T as Display>::fmt(&self.n, f)
    }
}

impl<T: AddSub + Copy + ZeroOne + Ord + MulDivRem + Readable, V: Value<T>> Readable
    for ModInt<T, V>
{
    fn read(input: &mut Input) -> Self {
        Self::new(T::read(input))
    }
}

impl<T: Writable, V: Value<T>> Writable for ModInt<T, V> {
    fn write(&self, output: &mut Output) {
        self.n.write(output);
    }
}

impl<T: ZeroOne + MulDivRem + AddSub + Copy + Ord, V: Value<T>> ZeroOne for ModInt<T, V> {
    fn zero() -> Self {
        unsafe { Self::unchecked_new(T::zero()) }
    }

    fn one() -> Self {
        Self::new(T::one())
    }
}

impl<T, V: Value<T>> Wideable for ModInt<T, V> {
    type W = Self;

    fn downcast(w: Self::W) -> Self {
        w
    }
}

impl<T: AddSub + Copy + ZeroOne + Ord + MulDivRem + FromU8, V: Value<T>> FromU8 for ModInt<T, V> {
    fn from_u8(n: u8) -> Self {
        Self::new(T::from_u8(n))
    }
}

impl<
        T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord + Display + FromU8,
        V: Value<T>,
    > std::fmt::Debug for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let max = T::from_u8(100);
        when! {
            self.n <= max => write!(f, "{}", self.n),
            self.n >= V::val() - max => write!(f, "{}", self.n - V::val()),
            else => {
                let mut denominator = T::one();
                while denominator < max {
                    let mut num = T::one();
                    while num < max {
                        if Self::new(num) / Self::new(denominator) == *self {
                            return write!(f, "{}/{}", num, denominator);
                        }
                        if -Self::new(num) / Self::new(denominator) == *self {
                            return write!(f, "-{}/{}", num, denominator);
                        }
                        num += T::one();
                    }
                    denominator += T::one();
                }
                write!(f, "(?? {} ??)", self.n)
            },
        }
    }
}

impl<T: AddSub + Copy + ZeroOne + Ord + MulDivRem + AsIndex, V: Value<T>> AsIndex for ModInt<T, V> {
    fn from_index(idx: usize) -> Self {
        Self::new(T::from_index(idx))
    }

    fn to_index(self) -> usize {
        self.n.to_index()
    }
}

value!(Val7: i32 = 1_000_000_007);
pub type ModInt7 = ModInt<i32, Val7>;

value!(Val9: i32 = 1_000_000_009);
pub type ModInt9 = ModInt<i32, Val9>;

value!(ValF: i32 = 998_244_353);
pub type ModIntF = ModInt<i32, ValF>;
}
pub mod num_traits {
pub mod add_sub {
use std::ops::{Add, AddAssign, Sub, SubAssign};

pub trait Addable: Add<Output = Self> + AddAssign + Copy {}
impl<T: Add<Output = Self> + AddAssign + Copy> Addable for T {}

pub trait AddSub: Addable + Sub<Output = Self> + SubAssign {}
impl<T: Addable + Sub<Output = Self> + SubAssign> AddSub for T {}
}
pub mod as_index {
pub trait AsIndex {
    fn from_index(idx: usize) -> Self;
    fn to_index(self) -> usize;
}

macro_rules! from_index_impl {
    ($($t: ident)+) => {$(
        impl AsIndex for $t {
            fn from_index(idx: usize) -> Self {
                idx as $t
            }

            fn to_index(self) -> usize {
                self as usize
            }
        }
    )+};
}

from_index_impl!(i128 i64 i32 i16 i8 isize u128 u64 u32 u16 u8 usize);
}
pub mod bit_ops {
use crate::numbers::num_traits::zero_one::ZeroOne;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, RangeInclusive, Shl,};
use std::ops::{ShlAssign, Shr, ShrAssign};

pub trait BitOps:
    Copy
    + BitAnd<Output = Self>
    + BitAndAssign
    + BitOr<Output = Self>
    + BitOrAssign
    + BitXor<Output = Self>
    + BitXorAssign
    + Not<Output = Self>
    + Shl<usize, Output = Self>
    + ShlAssign<usize>
    + Shr<usize, Output = Self>
    + ShrAssign<usize>
    + ZeroOne
    + PartialEq
{
    fn bit(at: usize) -> Self {
        Self::one() << at
    }

    fn is_set(&self, at: usize) -> bool {
        (*self >> at & Self::one()) == Self::one()
    }

    fn set_bit(&mut self, at: usize) {
        *self |= Self::bit(at)
    }

    fn unset_bit(&mut self, at: usize) {
        *self &= !Self::bit(at)
    }

    #[must_use]
    fn with_bit(mut self, at: usize) -> Self {
        self.set_bit(at);
        self
    }

    #[must_use]
    fn without_bit(mut self, at: usize) -> Self {
        self.unset_bit(at);
        self
    }

    fn flip_bit(&mut self, at: usize) {
        *self ^= Self::bit(at)
    }

    fn all_bits(n: usize) -> Self {
        let mut res = Self::zero();
        for i in 0..n {
            res.set_bit(i);
        }
        res
    }

    fn iter_all(n: usize) -> RangeInclusive<Self> {
        Self::zero()..=Self::all_bits(n)
    }
}

impl<
        T: Copy
            + BitAnd<Output = Self>
            + BitAndAssign
            + BitOr<Output = Self>
            + BitOrAssign
            + BitXor<Output = Self>
            + BitXorAssign
            + Not<Output = Self>
            + Shl<usize, Output = Self>
            + ShlAssign<usize>
            + Shr<usize, Output = Self>
            + ShrAssign<usize>
            + ZeroOne
            + PartialEq,
    > BitOps for T
{
}

pub trait Bits: BitOps {
    fn bits() -> u32;
}

macro_rules! bits_integer_impl {
    ($($t: ident $bits: expr),+) => {$(
        impl Bits for $t {
            fn bits() -> u32 {
                $bits
            }
        }
    )+};
}

bits_integer_impl!(i128 128, i64 64, i32 32, i16 16, i8 8, isize 64, u128 128, u64 64, u32 32, u16 16, u8 8, usize 64);
}
pub mod from_u8 {
pub trait FromU8 {
    fn from_u8(val: u8) -> Self;
}

macro_rules! from_u8_impl {
    ($($t: ident)+) => {$(
        impl FromU8 for $t {
            fn from_u8(val: u8) -> Self {
                val as $t
            }
        }
    )+};
}

from_u8_impl!(i128 i64 i32 i16 i8 isize u128 u64 u32 u16 u8 usize);
}
pub mod invertable {
pub trait Invertable: Sized {
    type Output;

    fn inv(&self) -> Option<Self>;
}
}
pub mod mul_div_rem {
use std::ops::{Div, DivAssign, Mul, MulAssign, Rem, RemAssign};

pub trait Multable: Mul<Output = Self> + MulAssign + Copy {}
impl<T: Mul<Output = Self> + MulAssign + Copy> Multable for T {}

pub trait MulDiv: Multable + Div<Output = Self> + DivAssign {}
impl<T: Multable + Div<Output = Self> + DivAssign> MulDiv for T {}

pub trait MulDivRem: MulDiv + Rem<Output = Self> + RemAssign {}
impl<T: MulDiv + Rem<Output = Self> + RemAssign> MulDivRem for T {}
}
pub mod primitive {
pub trait Primitive<T>: Copy {
    fn to(self) -> T;
}

macro_rules! primitive_one {
    ($t: ident, $($u: ident)+) => {$(
        impl Primitive<$u> for $t {
            fn to(self) -> $u {
                self as $u
            }
        }
    )+};
}

macro_rules! primitive {
    ($($t: ident)+) => {$(
        primitive_one!($t, u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
    )+}
}

primitive!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
}
pub mod wideable {
use std::convert::From;

pub trait Wideable: Sized {
    type W: From<Self>;

    fn downcast(w: Self::W) -> Self;
}

macro_rules! wideable_impl {
    ($($t: ident $w: ident),+) => {$(
        impl Wideable for $t {
            type W = $w;

            fn downcast(w: Self::W) -> Self {
                w as $t
            }
        }
    )+};
}

wideable_impl!(i128 i128, i64 i128, i32 i64, i16 i32, i8 i16, isize isize, u128 u128, u64 u128, u32 u64, u16 u32, u8 u16, usize usize);
}
pub mod zero_one {
pub trait ZeroOne {
    fn zero() -> Self;
    fn one() -> Self;
}

macro_rules! zero_one_integer_impl {
    ($($t: ident)+) => {$(
        impl ZeroOne for $t {
            fn zero() -> Self {
                0
            }

            fn one() -> Self {
                1
            }
        }
    )+};
}

zero_one_integer_impl!(i128 i64 i32 i16 i8 isize u128 u64 u32 u16 u8 usize);
}
}
pub mod number_ext {
use crate::numbers::num_traits::add_sub::AddSub;
use crate::numbers::num_traits::from_u8::FromU8;
use crate::numbers::num_traits::mul_div_rem::{MulDiv, MulDivRem, Multable};
use crate::numbers::num_traits::zero_one::ZeroOne;
use crate::when;
use std::ops::Mul;

pub trait Power {
    #[must_use]
    fn power<T: ZeroOne + PartialEq + MulDivRem + AddSub + Copy>(&self, exp: T) -> Self;
}

impl<S: ZeroOne + Copy + Multable> Power for S {
    fn power<T: ZeroOne + PartialEq + MulDivRem + AddSub + Copy>(&self, exp: T) -> Self {
        when! {
            exp == T::zero() => S::one(),
            exp % (T::one() + T::one()) == T::zero() => {
                let res = self.power(exp / (T::one() + T::one()));
                res * res
            },
            else => self.power(exp - T::one()) * (*self),
        }
    }
}

pub trait NumDigs {
    fn num_digs(&self) -> usize;
}

impl<S: ZeroOne + FromU8 + MulDiv + Copy + PartialEq> NumDigs for S {
    fn num_digs(&self) -> usize {
        let mut copy = *self;
        let ten = S::from_u8(10);
        let mut res = 0;
        while copy != S::zero() {
            copy /= ten;
            res += 1;
        }
        res
    }
}

pub trait Square {
    fn square(self) -> Self;
}

impl<T: Mul<Output = T> + Copy> Square for T {
    fn square(self) -> Self {
        self * self
    }
}
}
pub mod primes {
pub mod factorize {
use crate::collections::vec_ext::sorted::Sorted;
use crate::misc::recursive_function::{Callable2, RecursiveFunction2};
use crate::numbers::num_traits::as_index::AsIndex;
use crate::numbers::num_traits::primitive::Primitive;
use crate::numbers::primes::prime::find_divisor;
use crate::numbers::primes::sieve::divisor_table;
use std::cmp::Ordering;
use std::ops::Mul;

pub trait Factorize {
    fn prime_divisors(self) -> Vec<(i64, usize)>;
    fn divisors(self) -> Vec<i64>;
    fn max_power(self, p: Self) -> usize;
}

impl<T: Primitive<i64>> Factorize for T {
    fn prime_divisors(self) -> Vec<(i64, usize)> {
        let n = self.to();
        assert!(n >= 1);
        if n == 1 {
            return Vec::new();
        }
        let d = if n > 100 {
            find_divisor(n)
        } else {
            let mut res = n;
            let mut i = 2;
            while i * i <= n {
                if n % i == 0 {
                    res = i;
                    break;
                }
                i += 1;
            }
            res
        };
        if d == n {
            return vec![(d, 1)];
        }
        let left = d.prime_divisors();
        let right = (n / d).prime_divisors();
        let mut res = Vec::new();
        let mut i = 0;
        let mut j = 0;
        while i < left.len() && j < right.len() {
            match left[i].0.cmp(&right[j].0) {
                Ordering::Less => {
                    res.push(left[i]);
                    i += 1;
                }
                Ordering::Equal => {
                    res.push((left[i].0, left[i].1 + right[j].1));
                    i += 1;
                    j += 1;
                }
                Ordering::Greater => {
                    res.push(right[j]);
                    j += 1;
                }
            }
        }
        res.extend_from_slice(&left[i..]);
        res.extend_from_slice(&right[j..]);
        res
    }

    fn divisors(self) -> Vec<i64> {
        let pd = self.prime_divisors();
        let mut res = Vec::new();
        let mut rec = RecursiveFunction2::new(|f, mut d: i64, step: usize| {
            if step == pd.len() {
                res.push(d);
            } else {
                let (p, e) = pd[step];
                for i in 0..=e {
                    f.call(d, step + 1);
                    if i < e {
                        d *= p;
                    }
                }
            }
        });
        rec.call(1, 0);
        res.sorted()
    }

    fn max_power(self, p: Self) -> usize {
        let mut res = 0;
        let mut cur = self.to();
        assert!(cur >= 1);
        let p = p.to();
        assert!(p >= 2);
        while cur % p == 0 {
            cur /= p;
            res += 1;
        }
        res
    }
}

pub fn all_divisors<T: AsIndex + PartialEq + Copy + Mul<Output = T> + Ord>(
    n: usize,
    sorted: bool,
) -> Vec<Vec<T>> {
    let d: Vec<T> = divisor_table(n);
    let mut res = Vec::with_capacity(n);
    if n > 0 {
        res.push(Vec::new());
    }
    if n > 1 {
        res.push(vec![T::from_index(1)]);
    }
    for (i, p) in d.into_iter().enumerate().skip(2) {
        let mut q = 0;
        let mut c = i;
        while c % p.to_index() == 0 {
            c /= p.to_index();
            q += 1;
        }
        let mut cur = Vec::with_capacity(res[c].len() * (q + 1));
        let mut by = T::from_index(1);
        for j in 0..=q {
            cur.extend(res[c].iter().map(|&x| x * by));
            if j != q {
                by = by * p;
            }
        }
        if sorted {
            cur.sort();
        }
        res.push(cur);
    }
    res
}
}
pub mod prime {
use crate::misc::random::random;
use crate::misc::value::DynamicValue;
use crate::numbers::gcd::gcd;
use crate::numbers::mod_int::ModInt;
use crate::numbers::num_traits::primitive::Primitive;
use crate::numbers::num_traits::zero_one::ZeroOne;
use crate::numbers::number_ext::Power;
use crate::{dynamic_value, when};

pub fn is_prime(n: impl Primitive<i64>) -> bool {
    let n = n.to();
    if n <= 1 {
        return false;
    }
    let mut s = 0;
    let mut d = n - 1;
    while d % 2 == 0 {
        s += 1;
        d >>= 1;
    }
    if s == 0 {
        return n == 2;
    }
    dynamic_value!(IsPrimeModule: i64 = n);
    type Mod = ModInt<i64, IsPrimeModule>;
    for _ in 0..20 {
        let a = Mod::new(random().next(n as u64) as i64);
        if a == Mod::zero() {
            continue;
        }
        if a.power(d) == Mod::one() {
            continue;
        }
        let mut dd = d;
        let mut good = true;
        for _ in 0..s {
            if a.power(dd) + Mod::one() == Mod::zero() {
                good = false;
                break;
            }
            dd *= 2;
        }
        if good {
            return false;
        }
    }
    true
}

pub fn next_prime(mut n: i64) -> i64 {
    if n <= 2 {
        return 2;
    }
    n += 1 - (n & 1);
    while !is_prime(n) {
        n += 2;
    }
    n
}

fn brent(n: i64, x0: i64, c: i64) -> i64 {
    dynamic_value!(ModVal: i64 = n);
    type Mod = ModInt<i64, ModVal>;
    let mut x = Mod::new(x0);
    let c = Mod::new(c);
    let mut g = 1;
    let mut q = Mod::one();
    let mut xs = Mod::zero();
    let mut y = Mod::zero();
    let m = 128i64;
    let mut l = 1;
    while g == 1 {
        y = x;
        for _ in 1..l {
            x = x * x + c;
        }
        let mut k = 0;
        while k < l && g == 1 {
            xs = x;
            for _ in 0..m.min(l - k) {
                x = x * x + c;
                q *= y - x;
            }
            g = gcd(q.val(), n);
            k += m;
        }
        l *= 2;
    }
    if g == n {
        loop {
            xs = xs * xs + c;
            g = gcd((xs - y).val(), n);
            if g != 1 {
                break;
            }
        }
    }
    g
}

pub fn find_divisor(n: i64) -> i64 {
    when! {
        n == 1 => 1,
        n % 2 == 0 => 2,
        is_prime(n) => n,
        else => {
            loop {
                let res = brent(
                    n,
                    random().next_bounds(2, n as u64 - 1) as i64,
                    random().next_bounds(1, n as u64 - 1) as i64,
                );
                if res != n {
                    return res;
                }
            }
        },
    }
}
}
pub mod sieve {
use crate::collections::bit_set::BitSet;
use crate::collections::iter_ext::collect::IterCollect;
use crate::numbers::num_traits::as_index::AsIndex;

pub fn primality_table(n: usize) -> BitSet {
    let mut res = BitSet::new(n);
    res.fill(true);
    if n > 0 {
        res.unset(0);
    }
    if n > 1 {
        res.unset(1);
    }
    let mut i = 2;
    while i * i < n {
        if res[i] {
            for j in ((i * i)..n).step_by(i) {
                res.unset(j);
            }
        }
        i += 1;
    }
    res
}

pub fn primes<T: AsIndex>(n: usize) -> Vec<T> {
    primality_table(n)
        .into_iter()
        .map(|i| T::from_index(i))
        .collect_vec()
}

pub fn divisor_table<T: AsIndex + PartialEq>(n: usize) -> Vec<T> {
    let mut res = (0..n).map(|i| T::from_index(i)).collect_vec();
    let mut i = 2;
    while i * i < n {
        if res[i] == T::from_index(i) {
            for j in ((i * i)..n).step_by(i) {
                res[j] = T::from_index(i);
            }
        }
        i += 1;
    }
    res
}
}
}
}
pub mod string {
pub mod str {
use crate::collections::iter_ext::collect::IterCollect;
use crate::io::input::{Input, Readable};
use crate::io::output::{Output, Writable};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::iter::{Copied, FromIterator};
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Deref, DerefMut, Index, IndexMut};
use std::slice::{Iter, IterMut, SliceIndex};
use std::vec::IntoIter;

pub enum Str<'s> {
    Extendable(Vec<u8>, PhantomData<&'s [u8]>),
    Owned(Box<[u8]>, PhantomData<&'s [u8]>),
    Ref(&'s [u8]),
}

impl Default for Str<'static> {
    fn default() -> Self {
        Self::new()
    }
}

impl Str<'static> {
    pub fn new() -> Self {
        Str::Extendable(Vec::new(), PhantomData)
    }

    pub fn with_capacity(cap: usize) -> Self {
        Str::Extendable(Vec::with_capacity(cap), PhantomData)
    }
}

impl<'s> Str<'s> {
    pub fn push(&mut self, c: u8) {
        self.transform_to_extendable();
        self.as_extendable().push(c)
    }

    pub fn as_slice(&self) -> &[u8] {
        match self {
            Str::Extendable(s, _) => s.as_ref(),
            Str::Owned(s, _) => s.as_ref(),
            Str::Ref(s) => s,
        }
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> Copied<Iter<u8>> {
        match self {
            Str::Extendable(v, _) => v.iter(),
            Str::Owned(v, _) => v.iter(),
            Str::Ref(v) => v.iter(),
        }
        .copied()
    }

    pub fn iter_mut(&mut self) -> IterMut<u8> {
        self.transform_to_owned();
        self.as_mut_slice().iter_mut()
    }

    pub fn sort(&mut self) {
        self.transform_to_owned();
        self.as_mut_slice().sort_unstable();
    }

    pub fn into_owned(mut self) -> Str<'static> {
        self.transform_to_owned();
        match self {
            Str::Extendable(v, _) => Str::Extendable(v, PhantomData),
            Str::Owned(v, _) => Str::Owned(v, PhantomData),
            _ => unreachable!(),
        }
    }

    fn transform_to_extendable(&mut self) {
        match self {
            Str::Extendable(_, _) => {}
            Str::Owned(_, _) => {
                let mut fake = Str::new();
                std::mem::swap(self, &mut fake);
                if let Str::Owned(s, _) = fake {
                    *self = Str::Extendable(s.to_vec(), PhantomData)
                }
            }
            Str::Ref(s) => *self = Str::Extendable(s.to_vec(), PhantomData),
        }
    }

    fn as_extendable(&mut self) -> &mut Vec<u8> {
        match self {
            Str::Extendable(s, _) => s,
            _ => panic!("unreachable"),
        }
    }

    fn transform_to_owned(&mut self) {
        if let Str::Ref(s) = self {
            *self = Str::Owned(s.to_vec().into_boxed_slice(), PhantomData)
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        match self {
            Str::Extendable(s, _) => s.as_mut_slice(),
            Str::Owned(s, _) => s.as_mut(),
            _ => panic!("unreachable"),
        }
    }

    pub fn into_string(self) -> String {
        match self {
            Str::Extendable(v, _) => unsafe { String::from_utf8_unchecked(v) },
            Str::Owned(v, _) => unsafe { String::from_utf8_unchecked(v.into_vec()) },
            Str::Ref(v) => String::from_utf8_lossy(v).into_owned(),
        }
    }

    pub fn reverse(&mut self) {
        self.as_mut_slice().reverse();
    }

    pub fn trim(&self) -> &[u8] {
        let mut start = 0;
        let mut end = self.len();
        while start < end && (self[start] as char).is_whitespace() {
            start += 1;
        }
        while start < end && (self[end - 1] as char).is_whitespace() {
            end -= 1;
        }
        &self[start..end]
    }
}

impl<'s> IntoIterator for Str<'s> {
    type Item = u8;
    type IntoIter = IntoIter<u8>;

    #[allow(clippy::unnecessary_to_owned)]
    fn into_iter(self) -> Self::IntoIter {
        match self {
            Str::Extendable(v, _) => v.into_iter(),
            Str::Owned(v, _) => v.into_vec().into_iter(),
            Str::Ref(v) => v.to_vec().into_iter(),
        }
    }
}

impl From<String> for Str<'static> {
    fn from(s: String) -> Self {
        Str::Extendable(s.into(), PhantomData)
    }
}

impl<'s> From<&'s str> for Str<'s> {
    fn from(s: &'s str) -> Self {
        Str::Ref(s.as_bytes())
    }
}

impl From<Vec<u8>> for Str<'static> {
    fn from(s: Vec<u8>) -> Self {
        Str::Extendable(s, PhantomData)
    }
}

impl<'s> From<&'s [u8]> for Str<'s> {
    fn from(s: &'s [u8]) -> Self {
        Str::Ref(s)
    }
}

impl<'s> From<&'s String> for Str<'s> {
    fn from(s: &'s String) -> Self {
        Str::Ref(s.as_bytes())
    }
}

impl<'s> From<&'s Vec<u8>> for Str<'s> {
    fn from(s: &'s Vec<u8>) -> Self {
        Str::Ref(s.as_slice())
    }
}

impl From<u8> for Str<'static> {
    fn from(c: u8) -> Self {
        Str::Owned(Box::new([c]), PhantomData)
    }
}

impl<R: SliceIndex<[u8]>> Index<R> for Str<'_> {
    type Output = R::Output;

    fn index(&self, index: R) -> &Self::Output {
        self.as_slice().index(index)
    }
}

impl<R: SliceIndex<[u8]>> IndexMut<R> for Str<'_> {
    fn index_mut(&mut self, index: R) -> &mut Self::Output {
        self.transform_to_owned();
        self.as_mut_slice().index_mut(index)
    }
}

impl Clone for Str<'_> {
    fn clone(&self) -> Self {
        match self {
            Str::Extendable(s, _) => s.clone().into(),
            Str::Owned(s, _) => s.to_vec().into(),
            Str::Ref(s) => Str::Ref(s),
        }
    }
}

impl<'r, 's, S: Into<Str<'r>>> AddAssign<S> for Str<'s> {
    fn add_assign(&mut self, rhs: S) {
        self.transform_to_extendable();
        self.as_extendable()
            .extend_from_slice(rhs.into().as_slice());
    }
}

impl<'r, 's, S: Into<Str<'r>>> Add<S> for Str<'s> {
    type Output = Str<'s>;

    fn add(mut self, rhs: S) -> Self::Output {
        self += rhs;
        self
    }
}

impl Readable for Str<'static> {
    fn read(input: &mut Input) -> Self {
        input.next_token().unwrap().into()
    }
}

impl Writable for Str<'_> {
    fn write(&self, output: &mut Output) {
        for c in self.as_slice() {
            output.put(*c);
        }
        output.maybe_flush();
    }
}

impl Display for Str<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <String as Display>::fmt(&String::from_utf8(self.as_slice().to_vec()).unwrap(), f)
    }
}

impl Hash for Str<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

impl<'r> PartialEq<Str<'r>> for Str<'_> {
    fn eq(&self, other: &Str<'r>) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}

impl Eq for Str<'_> {}

impl<'r> PartialOrd<Str<'r>> for Str<'_> {
    fn partial_cmp(&self, other: &Str<'r>) -> Option<Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}

impl Ord for Str<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl FromIterator<u8> for Str<'static> {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        Self::Extendable(iter.into_iter().collect_vec(), Default::default())
    }
}

impl<'r> FromIterator<&'r u8> for Str<'static> {
    fn from_iter<T: IntoIterator<Item = &'r u8>>(iter: T) -> Self {
        Self::Extendable(iter.into_iter().cloned().collect_vec(), Default::default())
    }
}

impl Deref for Str<'_> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl DerefMut for Str<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

pub trait StrReader {
    fn read_str(&mut self) -> Str<'static>;
    fn read_str_vec(&mut self, n: usize) -> Vec<Str<'static>>;
    fn read_line(&mut self) -> Str<'static>;
}

impl StrReader for Input<'_> {
    fn read_str(&mut self) -> Str<'static> {
        self.read()
    }

    fn read_str_vec(&mut self, n: usize) -> Vec<Str<'static>> {
        self.read_vec(n)
    }

    fn read_line(&mut self) -> Str<'static> {
        let mut res = Str::new();
        while let Some(c) = self.get() {
            if c == b'\n' {
                break;
            }
            res.push(c);
        }
        res
    }
}
}
}
fn main() {
    let mut sin = std::io::stdin();
    let input = if false {
        crate::io::input::Input::new_with_size(&mut sin, 1)
    } else {
        crate::io::input::Input::new(&mut sin)
    };

    let mut stdout = std::io::stdout();
    let output = if false {
        crate::io::output::Output::new_with_auto_flush(&mut stdout)
    } else {
        crate::io::output::Output::new(&mut stdout)
    };

    crate::solution::run(input, output);
}

