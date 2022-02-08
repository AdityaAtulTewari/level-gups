use clap::Parser;
use std::mem;
use std::alloc::{alloc,Layout};
use libc;
use errno::errno;
use std::slice;
use std::ops::{BitXorAssign,Shl,Shr};


#[cfg(all(feature = "broadwell", feature = "m5"))]
compile_error!("Feature broadwell and m5 should not be enabled at the same time.");

#[cfg(feature = "broadwell")]
#[link(name="perf_broadwell", kind = "static")]
mod perf
{
  #[repr(C)]
  #[derive(Copy,Clone)]
  pub struct PassAround
  {
    pub fd0 : i64,
    pub ids : *mut u64,
  }
  extern "C"
  {
    pub fn create_counters () -> PassAround;
    pub fn reset_counters (pa0 : PassAround) -> ();
    pub fn start_counters (pa0 : PassAround) -> ();
    pub fn stop_counters (pa0 : PassAround) -> ();
    pub fn print_counters (pa0 : PassAround) -> ();
  }
}

#[cfg(feature = "m5")]
#[link(name="m5", kind = "static")]
mod perf
{
  #[repr(C)]
  #[derive(Copy,Clone)]
  pub struct PassAround {}
  extern "C"
  {
    fn m5_exit(ns_delay : u64) -> ();
    fn m5_reset_stats(ns_delay : u64, ns_period : u64) -> ();
  }
  pub fn create_counters () -> PassAround {PassAround {}}
  pub fn reset_counters (_pa0 : PassAround) -> () {}
  pub fn start_counters (_pa0 : PassAround) -> () {m5_reset_stats(0,0);}
  pub fn stop_counters (_pa0 : PassAround) -> () {m5_exit(0);}
  pub fn print_counters (_pa0 : PassAround) -> () {}
}

#[cfg(not(any(feature = "m5", feature = "broadwell")))]
mod perf
{
  #[repr(C)]
  #[derive(Copy,Clone)]
  pub struct PassAround {}
  pub fn create_counters () -> PassAround {PassAround{}}
  pub fn reset_counters (_pa0 : PassAround) -> () {}
  pub fn start_counters (_pa0 : PassAround) -> () {}
  pub fn stop_counters (_pa0 : PassAround) -> () {}
  pub fn print_counters (_pa0 : PassAround) -> () {}
}

use perf::{PassAround,create_counters,reset_counters,start_counters,stop_counters,print_counters};


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args
{
    /// page table level to target
    #[clap(short, long, default_value_t = 0)]
    tgt_level : u8,

    /// Number of threads to use
    #[clap(long, default_value_t = 1)]
    n_threads : u8,

    /// Number of updates
    #[clap(short, long, default_value_t = 1000000000)]
    n_updates : u64,

    /// Number of choices
    #[clap(short, long, default_value_t = 512)]
    choices_n : usize,

    /// Don't Collect stats
    #[clap(long)]
    nmsr_stats : bool,

    /// Don't disable THP
    #[clap(short, long)]
    disabl_thp : bool,

    #[clap(short, long)]
    access_sin : bool,
}

struct Prand<T : Shl<u8, Output=T> + Shr<u8, Output=T> + BitXorAssign + Copy>
{
  x : T,
  y : T,
  z : T,
  w : T,
}

impl<T : Shl<u8, Output=T> + Shr<u8, Output=T> + BitXorAssign + Copy> Prand<T>
{
  fn simplerand(&mut self) -> T
  {
    let mut t : T  = self.x;
    t ^= t << 11;
    t ^= t >> 8;
    self.x = self.y;
    self.y = self.z;
    self.z = self.w;
    self.w ^= self.w >> 19;
    self.w ^= t;
    return self.w;
  }
}

fn random_placem(buf : &mut [usize], ind_value : usize, num : usize) -> ()
{
  let mut rng = Prand::<usize>{ x : 1, y : 4, z : 7, w : 13};
  let c_ind_value = ind_value/mem::size_of::<usize>();
  for i in 0..num
  {
    for j in 0..(4096/mem::size_of::<usize>())
    {
      buf[i*c_ind_value+j] = rng.simplerand();
    }
  }
}

fn run_benchmark(buf : &mut [usize], ind_value : usize, num : usize, times : u64, single_walk : bool, nmsr_stats : bool) -> ()
{
  let mut rng = Prand::<usize>{ x : 1, y : 4, z : 7, w : 13};
  let c_ind_value = ind_value/mem::size_of::<usize>();
  let mask = num - 1;
  let pa : PassAround = unsafe {create_counters()};
  let start_measure = if nmsr_stats {|_pa : PassAround| {}} else {|pa : PassAround| {unsafe{reset_counters(pa); start_counters(pa);}}};
  let stop_print_me = if nmsr_stats {|_pa : PassAround| {}} else {|pa : PassAround| {unsafe{stop_counters(pa); print_counters(pa);}}};
  let benchmark = if !single_walk
  {
    |buf : &mut[usize], i : usize, c_ind_value : usize, rng  : &mut Prand<usize>| -> ()
    {for j in 0..(4096/mem::size_of::<usize>())
      {buf[i * c_ind_value + j] ^= rng.simplerand();}}
  }
  else {|buf : &mut[usize], i : usize, c_ind_value : usize, rng : &mut Prand<usize>| -> () {buf[i * c_ind_value] ^= rng.simplerand()}};

  start_measure(pa);
  for _ in 0..times
  {
    let i : usize = rng.simplerand() & mask;
    benchmark(buf, i, c_ind_value, &mut rng);
  }
  stop_print_me(pa);
}

unsafe fn alloc_aligned<T>(layout : Layout) -> Result<&'static mut [T], String>
{
  let mmap : *mut u8 = alloc(layout);
  if mmap.is_null()
  {
    return Err(format!("Layout of size: {} and alignment: {}", layout.size(), layout.align()));
  }
  let buf : &'static mut [T] = slice::from_raw_parts_mut(mmap as *mut T, layout.size()/mem::size_of::<T>());
  return Ok(buf);

}

fn main()
{
  let args = Args::parse();

  if args.choices_n == 0
  {
    panic!("There are zero choices");
  }
  if (args.choices_n & (args.choices_n - 1)) != 0
  {
    panic!("The choices_n is not a power of 2");
  }

  if !args.disabl_thp
  {
    let res : libc::c_int;
    let val : libc::c_ulong = 1;
    let nval: libc::c_ulong = 0;
    unsafe
    {
      res = libc::prctl(libc::PR_SET_THP_DISABLE, val,nval,nval,nval);
    }
    if res != 0
    {
      let e = errno();
      println!("res was {}", res);
      println!("prctl had Error {}: {}", e.0, e);
      println!("prctl called with: {}", libc::PR_SET_THP_DISABLE);
      panic!("Unable to disable THP");
    }
  }


  // We will make an assumption based upon the size of a pointer how many levels this has (2 or 4)
  let pg_sz : u8 = if mem::size_of::<*const usize>() == 8 { 4 } else { 2 };

  if args.tgt_level >= pg_sz
  {
    panic!("The target level you have selected is out of bounds.");
  }
  let num = args.choices_n; // - 1;
  let ind_value = 4096 * (if pg_sz == 2 {usize::pow(1024, args.tgt_level as u32)} else {usize::pow(1 << 9, args.tgt_level as u32)});
  let sz_to_alloc = num * ind_value; //+ 8;
  let r_layout = Layout::from_size_align(sz_to_alloc, ind_value);
  let layout : Layout;
  match r_layout
  {
    Ok(lay) => {layout = lay;}
    Err(e) => {println!("We tried a layout with sz_to_alloc {} and ind_value {} to get the Error: {}", sz_to_alloc, ind_value, e); panic!("Align setup failed");}
  }

  let mmap : Result<&'static mut [usize], String>;
  unsafe
  {
    mmap = alloc_aligned(layout);
  }

  match mmap
  {
    Ok(buf) =>
    {
      random_placem(buf, ind_value, args.choices_n);
      run_benchmark(buf, ind_value, args.choices_n, args.n_updates, args.access_sin, args.nmsr_stats);
    }
    Err(e) => {println!("We tried to allocate {} and got Error: {}", sz_to_alloc, e); panic!("Setup failed");}
  }
}
