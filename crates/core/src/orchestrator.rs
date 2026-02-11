use crate::health::wait_ready;
use crate::state::{write_state, RunningService, State};
use crate::types::{Service, WorkspaceConfig};
use anyhow::{bail, Context, Result};
use std::collections::{HashMap, VecDeque};
use std::fs::{create_dir_all, File};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use sysinfo::{Pid, ProcessesToUpdate, System};

fn topo_order(services: &[Service]) -> Result<Vec<String>> {
    let mut indeg: HashMap<String, usize> = HashMap::new();
    let mut adj: HashMap<String, Vec<String>> = HashMap::new();
    for s in services {
        indeg.entry(s.name.clone()).or_default();
    }
    for s in services {
        for d in s.depends_on.clone().unwrap_or_default() {
            indeg.entry(s.name.clone()).and_modify(|e| *e += 1);
            adj.entry(d).or_default().push(s.name.clone());
        }
    }
    let mut q: VecDeque<String> = indeg
        .iter()
        .filter(|(_, &v)| v == 0)
        .map(|(k, _)| k.clone())
        .collect();
    let mut res = Vec::new();
    let mut indeg_mut = indeg.clone();
    while let Some(u) = q.pop_front() {
        res.push(u.clone());
        if let Some(neigh) = adj.get(&u) {
            for v in neigh {
                if let Some(e) = indeg_mut.get_mut(v) {
                    *e -= 1;
                    if *e == 0 {
                        q.push_back(v.clone());
                    }
                }
            }
        }
    }
    if res.len() != indeg.len() {
        bail!("cycle in dependencies")
    }
    Ok(res)
}

fn spawn_service(base_dir: &Path, logs_dir: &Path, s: &Service) -> Result<RunningService> {
    let out_path = logs_dir.join(format!("{}.out.log", s.name));
    let err_path = logs_dir.join(format!("{}.err.log", s.name));
    let out_file = File::options().create(true).append(true).open(&out_path)?;
    let err_file = File::options().create(true).append(true).open(&err_path)?;
    let mut cmd = if cfg!(windows) {
        let mut c = Command::new("cmd");
        c.arg("/C").arg(&s.command);
        c
    } else {
        let mut c = Command::new("sh");
        c.arg("-c").arg(&s.command);
        c
    };
    if let Some(cwd) = &s.cwd {
        let p = base_dir.join(cwd);
        cmd.current_dir(p);
    }
    if let Some(env) = &s.env {
        for (k, v) in env {
            cmd.env(k, v);
        }
    }
    cmd.stdout(Stdio::from(out_file));
    cmd.stderr(Stdio::from(err_file));
    let child: Child = cmd.spawn().context("spawn")?;
    let pid = child.id() as i32;
    Ok(RunningService {
        name: s.name.clone(),
        pid,
        stdout_log: out_path,
        stderr_log: err_path,
    })
}

pub fn up(
    cfg: &WorkspaceConfig,
    base_dir: impl AsRef<Path>,
    state_path: impl AsRef<Path>,
) -> Result<State> {
    let base = base_dir.as_ref();
    let logs_dir = base.join("logs");
    create_dir_all(&logs_dir)?;
    let order = topo_order(&cfg.services)?;
    let mut by_name: HashMap<String, &Service> = HashMap::new();
    for s in &cfg.services {
        by_name.insert(s.name.clone(), s);
    }
    let mut running: Vec<RunningService> = Vec::new();
    for name in order {
        let s = by_name.get(&name).unwrap();
        let rs = spawn_service(base, &logs_dir, s)?;
        if let Some(hc) = &s.health_check {
            let _ = wait_ready(hc);
        }
        running.push(rs);
    }
    let st = State { services: running };
    write_state(state_path, &st)?;
    Ok(st)
}

pub fn down(state_path: impl AsRef<Path>) -> Result<()> {
    let p = state_path.as_ref();
    let st = crate::state::read_state(p)?;
    if st.is_none() {
        return Ok(());
    }
    let st = st.unwrap();
    let mut sys = System::new();
    sys.refresh_processes(ProcessesToUpdate::All, true);
    for s in st.services {
        if let Some(proc_) = sys.process(Pid::from_u32(s.pid as u32)) {
            let _ = proc_.kill();
        }
    }
    std::fs::remove_file(p).ok();
    Ok(())
}

pub fn status(state_path: impl AsRef<Path>) -> Result<Vec<(String, i32, bool)>> {
    let st = crate::state::read_state(state_path)?;
    let mut sys = System::new();
    sys.refresh_processes(ProcessesToUpdate::All, true);
    let mut res = Vec::new();
    if let Some(st) = st {
        for s in st.services {
            let alive = sys.process(Pid::from_u32(s.pid as u32)).is_some();
            res.push((s.name, s.pid, alive));
        }
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Service;

    fn make_service(name: &str, depends_on: Vec<&str>) -> Service {
        Service {
            name: name.to_string(),
            command: "echo".to_string(),
            cwd: None,
            env: None,
            depends_on: Some(depends_on.into_iter().map(|s| s.to_string()).collect()),
            health_check: None,
        }
    }

    #[test]
    fn test_topo_order_basic() {
        let s1 = make_service("db", vec![]);
        let s2 = make_service("backend", vec!["db"]);
        let s3 = make_service("frontend", vec!["backend"]);
        let services = vec![s1, s2, s3];

        let order = topo_order(&services).unwrap();
        assert_eq!(
            order,
            vec![
                "db".to_string(),
                "backend".to_string(),
                "frontend".to_string()
            ]
        );
    }

    #[test]
    fn test_topo_order_independent() {
        let s1 = make_service("a", vec![]);
        let s2 = make_service("b", vec![]);
        let services = vec![s1, s2];
        let order = topo_order(&services).unwrap();
        assert_eq!(order.len(), 2);
        assert!(order.contains(&"a".to_string()));
        assert!(order.contains(&"b".to_string()));
    }

    #[test]
    fn test_topo_order_cycle() {
        let s1 = make_service("a", vec!["b"]);
        let s2 = make_service("b", vec!["a"]);
        let services = vec![s1, s2];
        let res = topo_order(&services);
        assert!(res.is_err());
    }
}
