
use std::{io, sync::Mutex, thread};
use collections::HashMap;
use self::imp::*;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct ThreadStat {
  pub s_time: i64,
  pub u_time: i64
}

impl ThreadStat {
  #[inline]
  pub fn total_cpu_time(&self) -> f64 {
    cpu_total(self.s_time, self.u_time)
  }
}

#[inline]
fn cpu_total(sys_time: i64, user_time: i64) -> f64 {
  (sys_time + user_time) as f64 / ticks_per_second() as f64
}

#[cfg(target_os = "macos")]
#[allow(bad_style)]
mod imp {
  use std::{io, iter::FromIterator, mem::size_of, ptr::null_mut, slice};
  use libc::*;

  pub type Pid = mach_port_t;
  type task_inspect_t = mach_port_t;
  type thread_act_t = mach_port_t;
  type thread_act_array_t = *mut thread_act_t;

  extern "C" {
    fn task_threads(
      target_task: task_inspect_t,
      act_list: *mut thread_act_array_t,
      act_listCnt: *mut mach_msg_type_number_t
    ) -> kern_return_t;
  }

  const MICRO_SEC_PER_SEC: i64 = 1_000_000;

  #[derive(Default)]
  pub struct FullStat {
    pub stime: i64,
    pub utime: i64,
    pub command: String,
  }

  #[inline]
  pub fn ticks_per_second() -> i64 {
    MICRO_SEC_PER_SEC
  }

  #[inline]
  pub fn process_id() -> Pid {
    unsafe { mach_task_self_ }
  }

  #[inline]
  pub fn thread_id() -> Pid {
    unsafe { mach_thread_self() }
  }

  pub fn thread_ids<C: FromIterator<Pid>>(pid: Pid) -> io::Result<C> {
    unsafe {
      let mut act_list: thread_act_array_t = null_mut();
      let mut act_count: mach_msg_type_number_t = 0;
      let ret = task_threads(pid, &mut act_list, &mut act_count);
      if ret != KERN_SUCCESS {
        return Err(io::Error::from_raw_os_error(ret));
      }

      let pids = slice::from_raw_parts_mut(act_list, act_count as _)
        .iter()
        .copied()
        .collect();

      vm_deallocate(
        pid, 
        act_list as vm_address_t,
        size_of::<thread_act_t>() * act_count as usize
      );

      Ok(pids)
    }
  }

  pub fn full_thread_stat(_pid: Pid, tid: Pid) -> io::Result<FullStat> {
    unsafe {
      let flavor = THREAD_BASIC_INFO;
      let mut info = std::mem::zeroed::<thread_basic_info>();
      let mut thread_info_cnt = THREAD_BASIC_INFO_COUNT;

      let ret = thread_info(
        tid,
        flavor as task_flavor_t,
        (&mut info as *mut _) as thread_info_t,
        &mut thread_info_cnt
      );
      if ret != KERN_SUCCESS {
        return Err(io::Error::from_raw_os_error(ret));
      }

      Ok(FullStat {
        stime: info.system_time.seconds as i64 * 1_000_000 + info.system_time.microseconds as i64,
        utime: info.user_time.seconds as i64 * 1_000_000 + info.user_time.microseconds as i64,
        ..Default::default()
      })
    }
  }

  pub fn set_priority(_: i32) -> io::Result<()> {
    Ok(())
  }

  pub fn get_priority() -> io::Result<i32> {
    Ok(0)
  }
}

pub fn thread_stat(pid: Pid, tid: Pid) -> io::Result<ThreadStat> {
  let full_stat = full_thread_stat(pid, tid)?;
  Ok(ThreadStat {
    s_time: full_stat.stime,
    u_time: full_stat.utime
  })
}

pub fn current_thread_stat() -> io::Result<ThreadStat> {
  thread_stat(process_id(), thread_id())
}

pub trait StdThreadBuildWrapper {
  fn sapwn_wrapper<F, T>(self, f: F) -> io::Result<thread::JoinHandle<T>>
    where
      F: FnOnce() -> T,
      F: Send + 'static,
      T: Send + 'static;
}

lazy_static::lazy_static! {
  pub static ref THREAD_NAME_HASHMAP: Mutex<HashMap<Pid, String>> =
    Mutex::new(HashMap::default());
  pub static ref THREAD_START_HOOKS: Mutex<Vec<Box<dyn Fn() + Sync + Send>>> =
    Mutex::new(Vec::new());
}

pub fn hook_thread_start(f: Box<dyn Fn() + Sync + Send>) {
  THREAD_START_HOOKS.lock().unwrap().push(f);
}

pub(crate) fn call_thread_start_hooks() {
  for f in THREAD_START_HOOKS.lock().unwrap().iter() {
    f();
  }
}

pub(crate) fn add_thread_name_to_map() {
  if let Some(name) = std::thread::current().name() {
    let tid = thread_id();
    THREAD_NAME_HASHMAP
      .lock()
      .unwrap()
      .insert(tid, name.to_string());
    debug!("tid {} thread name is {}", tid, name);
  }
}

pub(crate) fn remove_thread_name_from_map() {
  let tid = thread_id();
  THREAD_NAME_HASHMAP.lock().unwrap().remove(&tid);
}