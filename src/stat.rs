#![deny(warnings)]

use serde::Serialize;
use serde::Serializer;
use std::collections::HashMap;
use sysinfo::{Pid, System};

// 步骤 1: 创建一个新的包装结构体
#[derive(Debug, Clone)]
struct SerializablePid(Pid);

// 步骤 2: 为包装结构体实现 Serialize trait
impl Serialize for SerializablePid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 将 Pid 转换为 u32 并序列化
        serializer.serialize_u32(self.0.as_u32())
    }
}

// 树节点结构
#[derive(Debug, Clone, Serialize)]
pub struct ProcessNode {
    // 步骤 3: 使用 SerializablePid 替代 Pid
    pid: SerializablePid,
    ppid: SerializablePid,
    name: String,
    cmd: Vec<String>,
    cpu_usage: f32,
    memory: u64,
    children: Vec<ProcessNode>,
}

impl ProcessNode {
    fn new(
        pid: Pid,
        ppid: Pid,
        name: String,
        cmd: Vec<String>,
        cpu_usage: f32,
        memory: u64,
    ) -> Self {
        ProcessNode {
            pid: SerializablePid(pid),
            ppid: SerializablePid(ppid),
            name,
            cmd,
            cpu_usage,
            memory,
            children: Vec::new(),
        }
    }
}

// 构建进程树
pub fn build_process_tree(system: &System, target_pid: Pid) -> Option<ProcessNode> {
    // 构建节点映射
    let mut nodes: HashMap<Pid, ProcessNode> = HashMap::new();
    let mut root_node: Option<ProcessNode> = None;

    // 创建所有节点
    for (pid, process) in system.processes() {
        let node = ProcessNode::new(
            *pid,
            process.parent().unwrap_or(Pid::from_u32(0)),
            process.name().to_string(),
            process.cmd().to_vec(),
            process.cpu_usage(),
            process.memory(),
        );
        nodes.insert(*pid, node);
    }

    // 构建父子关系
    let mut children_map: HashMap<Pid, Vec<Pid>> = HashMap::new();

    for (pid, process) in system.processes() {
        if let Some(parent_pid) = process.parent() {
            children_map.entry(parent_pid).or_default().push(*pid);
        }
    }

    // 寻找目标 PID 的根节点
    if let Some(root_pid) = nodes.get(&target_pid).map(|node| node.pid.0) {
        root_node = Some(nodes.remove(&root_pid)?);
        assemble_tree(root_node.as_mut().unwrap(), &nodes, &children_map);
    }

    root_node
}

// 递归组装树结构
fn assemble_tree(
    node: &mut ProcessNode,
    nodes: &HashMap<Pid, ProcessNode>,
    children_map: &HashMap<Pid, Vec<Pid>>,
) {
    if let Some(children) = children_map.get(&node.pid.0) {
        for child_pid in children {
            if let Some(mut child_node) = nodes.get(child_pid).cloned() {
                assemble_tree(&mut child_node, nodes, children_map);
                node.children.push(child_node);
            }
        }
    }
}
