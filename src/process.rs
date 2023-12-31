// process.rs
// Process struct as the process control block
// Author: Hank Bao

pub struct Process {
    pid: u32,
    io_interval: u32,
    io_length: u32,
    workload: u32,
    work_done: u32,
    start_time: u32,
    next_schedule_time: u32,
    turnaround_time: u32,
    response_time: u32,
    allotment: u32,
    state: ProcessState,
}

#[allow(dead_code)]
impl Process {
    pub fn new(
        pid: u32,
        io_interval: u32,
        io_length: u32,
        workload: u32,
        arrival_time: u32,
    ) -> Process {
        Process {
            pid,
            io_interval,
            io_length,
            workload,
            work_done: 0,
            start_time: arrival_time,
            next_schedule_time: arrival_time,
            turnaround_time: 0,
            response_time: 0,
            allotment: 0,
            state: ProcessState::Ready,
        }
    }

    pub fn pid(&self) -> u32 {
        self.pid
    }

    pub fn io_interval(&self) -> u32 {
        self.io_interval
    }

    pub fn io_length(&self) -> u32 {
        self.io_length
    }

    pub fn workload(&self) -> u32 {
        self.workload
    }

    pub fn work_done(&self) -> u32 {
        self.work_done
    }

    pub fn start_time(&self) -> u32 {
        self.start_time
    }

    pub fn next_schedule_time(&self) -> u32 {
        self.next_schedule_time
    }

    pub fn turnaround_time(&self) -> u32 {
        self.turnaround_time
    }

    pub fn response_time(&self) -> u32 {
        self.response_time
    }

    pub fn set_allotment(&mut self, allotment: u32) {
        self.allotment = allotment;
    }

    pub fn allotment(&self) -> u32 {
        self.allotment
    }

    pub fn is_blocked(&self) -> bool {
        match self.state {
            ProcessState::Blocked => true,
            _ => false,
        }
    }

    pub fn is_finished(&self) -> bool {
        match self.state {
            ProcessState::Finished => true,
            _ => false,
        }
    }

    pub fn run(&mut self, quantum: u32, at: u32, queue: usize) -> u32 {
        // record the response time
        if self.response_time == 0 {
            assert!(at >= self.start_time);
            self.response_time = at - self.start_time;
        }

        match self.state {
            ProcessState::Ready => self.run_from_ready(quantum, at, queue),
            ProcessState::Running => self.run_from_running(quantum, at, queue),
            ProcessState::Blocked => self.run_from_blocked(quantum, at, queue),
            ProcessState::Finished => panic!("Run a finished process {}.", self.pid),
        }
    }

    fn run_from_ready(&mut self, quantum: u32, at: u32, queue: usize) -> u32 {
        self.state = ProcessState::Running;
        println!("[{}:<{}>] Process {} start running.", at, queue, self.pid);

        self.run_from_running(quantum, at, queue)
    }

    fn run_from_running(&mut self, quantum: u32, at: u32, queue: usize) -> u32 {
        assert_eq!(self.state, ProcessState::Running);
        assert!(self.allotment > 0);

        let run_time: u32; // actual run time
        let work_left = self.workload - self.work_done; // work left

        if self.io_interval > 0 {
            // Check if the process is going to do I/O before the quantum is up
            let work_before_io = self.io_interval - (self.work_done % self.io_interval);
            if work_before_io < work_left && work_before_io <= quantum {
                run_time = work_before_io;
                self.work_done += run_time;
                self.next_schedule_time = at + self.io_length;
                self.state = ProcessState::Blocked;
            } else if work_left <= quantum {
                run_time = work_left;
                self.work_done += run_time;
                self.next_schedule_time = u32::MAX;
                self.turnaround_time = at - self.start_time + run_time;
                self.state = ProcessState::Finished;
                assert!(self.workload == self.work_done);
            } else {
                run_time = quantum;
                self.work_done += run_time;
                self.next_schedule_time = at + quantum;
            }
        } else {
            if work_left <= quantum {
                run_time = work_left;
                self.work_done += run_time;
                self.next_schedule_time = u32::MAX;
                self.turnaround_time = at - self.start_time + run_time;
                self.state = ProcessState::Finished;
                assert!(self.workload == self.work_done);
            } else {
                run_time = quantum;
                self.work_done += run_time;
                self.next_schedule_time = at + quantum;
            }
        }

        assert!(run_time > 0);

        // Update allotment
        if run_time < self.allotment {
            self.allotment -= run_time;
        } else {
            self.allotment = 0;
        }

        // Print status
        match self.state {
            ProcessState::Running => {
                println!(
                    "[{}:<{}>] Process {} has run for {}.",
                    at + run_time,
                    queue,
                    self.pid,
                    run_time
                )
            }
            ProcessState::Blocked => println!(
                "[{}:<{}>] Process {} has run for {}, then blocked. It will perform I/O for {}",
                at + run_time,
                queue,
                self.pid,
                run_time,
                self.io_length
            ),
            ProcessState::Finished => println!(
                "[{}:<{}>] Process {} has run for {}, then finished.",
                at + run_time,
                queue,
                self.pid,
                run_time
            ),
            _ => panic!("Process {} is in an invalid state.", self.pid),
        }

        run_time
    }

    fn run_from_blocked(&mut self, quantum: u32, at: u32, queue: usize) -> u32 {
        self.state = ProcessState::Running;
        println!(
            "[{}:<{}>] Process {} resume running from I/O.",
            at, queue, self.pid
        );

        self.run_from_running(quantum, at, queue)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum ProcessState {
    Ready,
    Running,
    Blocked,
    Finished,
}
