use crate::protocol::command::Command;
use crate::protocol::{Protocol, Protocols, Step};
use crate::tracer::stdio_redirecting::Redirector;
use crate::tracer::SyscallMock;
use crate::R;
use libc::user_regs_struct;
use nix::unistd::Pid;
use yaml_rust::Yaml;

pub struct Recorder {
    protocol: Protocol,
}

impl Default for Recorder {
    fn default() -> Recorder {
        Recorder {
            protocol: Protocol::new(vec![]),
        }
    }
}

impl SyscallMock for Recorder {
    type Result = Yaml;

    fn handle_execve_enter(
        &mut self,
        _pid: Pid,
        _registers: &user_regs_struct,
        executable: Vec<u8>,
        arguments: Vec<Vec<u8>>,
    ) -> R<()> {
        self.protocol.steps.push_back(Step::new(Command {
            executable,
            arguments,
        }));
        Ok(())
    }

    fn handle_end(mut self, exitcode: i32, _redirector: &Redirector) -> R<Yaml> {
        if exitcode != 0 {
            self.protocol.exitcode = Some(exitcode);
        }
        Ok(Protocols::new(vec![self.protocol]).serialize())
    }
}
