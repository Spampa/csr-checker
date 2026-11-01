use std::cmp::Reverse;
use std::collections::HashSet;

use crate::graph::{Chain, Graph};

pub trait Printer {
    fn print_graph(graph: &Graph, cycles: &[Chain]);
}

/// Draws the graph as an arc diagram: nodes in a row, forward edges
/// (u < v) as arcs above the nodes, backward edges as arcs below.
/// Edges belonging to a cycle are highlighted in red.
pub struct GraphPrinter {}

impl Printer for GraphPrinter {
    fn print_graph(graph: &Graph, cycles: &[Chain]) {
        print!("{}", render(graph, cycles));
    }
}

const GAP: usize = 3;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const CYCLE: &str = "\x1b[1;31m";
const PALETTE: [&str; 5] = ["\x1b[36m", "\x1b[32m", "\x1b[33m", "\x1b[34m", "\x1b[35m"];

const UP: u8 = 1;
const DOWN: u8 = 2;
const LEFT: u8 = 4;
const RIGHT: u8 = 8;

struct Arc {
    from: usize,
    to: usize,
    level: usize,
    color: &'static str,
}

impl Arc {
    fn lo(&self) -> usize {
        self.from.min(self.to)
    }

    fn hi(&self) -> usize {
        self.from.max(self.to)
    }
}

#[derive(Clone, Copy, Default)]
struct Cell {
    mask: u8,
    ch: Option<char>,
    color: Option<&'static str>,
}

struct Canvas {
    cells: Vec<Vec<Cell>>,
}

impl Canvas {
    fn new(width: usize, height: usize) -> Self {
        Self { cells: vec![vec![Cell::default(); width]; height] }
    }

    /// Adds line directions to a cell, so crossing lines merge into `┼`.
    fn line(&mut self, row: usize, col: usize, mask: u8, color: &'static str) {
        let cell = &mut self.cells[row][col];
        cell.mask |= mask;
        cell.color = Some(color);
    }

    fn put(&mut self, row: usize, col: usize, ch: char, color: Option<&'static str>) {
        let cell = &mut self.cells[row][col];
        cell.ch = Some(ch);
        cell.color = color;
    }

    fn text(&mut self, row: usize, col: usize, s: &str, color: Option<&'static str>) {
        for (i, ch) in s.chars().enumerate() {
            self.put(row, col + i, ch, color);
        }
    }

    fn render(&self) -> String {
        let mut out = String::new();
        for row in &self.cells {
            let chars: Vec<(char, Option<&str>)> = row
                .iter()
                .map(|c| (c.ch.or(mask_char(c.mask)).unwrap_or(' '), c.color))
                .collect();
            let end = chars.iter().rposition(|(ch, _)| *ch != ' ').map_or(0, |i| i + 1);

            let mut current = None;
            for &(ch, color) in &chars[..end] {
                let color = if ch == ' ' { current } else { color };
                if color != current {
                    out.push_str(RESET);
                    if let Some(c) = color {
                        out.push_str(c);
                    }
                    current = color;
                }
                out.push(ch);
            }
            if current.is_some() {
                out.push_str(RESET);
            }
            out.push('\n');
        }
        out
    }
}

fn mask_char(mask: u8) -> Option<char> {
    let ch = match mask {
        m if m == UP | DOWN => '│',
        m if m == LEFT | RIGHT => '─',
        m if m == DOWN | RIGHT => '┌',
        m if m == DOWN | LEFT => '┐',
        m if m == UP | RIGHT => '└',
        m if m == UP | LEFT => '┘',
        m if m == UP | DOWN | RIGHT => '├',
        m if m == UP | DOWN | LEFT => '┤',
        m if m == DOWN | LEFT | RIGHT => '┬',
        m if m == UP | LEFT | RIGHT => '┴',
        m if m == UP | DOWN | LEFT | RIGHT => '┼',
        _ => return None,
    };
    Some(ch)
}

/// Assigns each arc the lowest level where it doesn't overlap another arc,
/// shortest arcs first so nested arcs stay inside longer ones.
/// Returns the number of levels used.
fn assign_levels(arcs: &mut [Arc]) -> usize {
    let mut order: Vec<usize> = (0..arcs.len()).collect();
    order.sort_by_key(|&i| arcs[i].hi() - arcs[i].lo());

    let mut levels: Vec<Vec<(usize, usize)>> = vec![];
    for i in order {
        let (lo, hi) = (arcs[i].lo(), arcs[i].hi());
        let free = levels
            .iter()
            .position(|spans| spans.iter().all(|&(l, h)| l.max(lo) >= h.min(hi)));
        let level = match free {
            Some(level) => level,
            None => {
                levels.push(vec![]);
                levels.len() - 1
            }
        };
        levels[level].push((lo, hi));
        arcs[i].level = level;
    }
    levels.len()
}

/// For every node, the arcs attached to it ordered left to right:
/// arcs going left (inner ones first), then arcs going right (outer ones
/// first). This keeps nested arcs from crossing each other.
fn port_order(arcs: &[Arc], n: usize) -> Vec<Vec<usize>> {
    let mut ports: Vec<Vec<usize>> = vec![vec![]; n];
    for (i, arc) in arcs.iter().enumerate() {
        ports[arc.from].push(i);
        ports[arc.to].push(i);
    }
    for (node, list) in ports.iter_mut().enumerate() {
        list.sort_by_key(|&i| {
            let goes_left = arcs[i].lo() < node;
            let level = arcs[i].level;
            if goes_left { (0, level, Reverse(0)) } else { (1, 0, Reverse(level)) }
        });
    }
    ports
}

fn render(graph: &Graph, cycles: &[Chain]) -> String {
    let nodes = graph.nodes();
    let n = nodes.len();
    if n == 0 {
        return String::from("(empty graph)\n");
    }

    let cycle_edges: HashSet<(usize, usize)> = cycles.iter().flat_map(|c| c.edges()).collect();
    let cycle_nodes: HashSet<usize> = cycle_edges.iter().flat_map(|&(u, v)| [u, v]).collect();

    let mut above = vec![];
    let mut below = vec![];
    let mut seen = HashSet::new();
    for node in nodes {
        for &(to, _) in &node.connections {
            let from = node.node_id;
            if from == to || to >= n || !seen.insert((from, to)) {
                continue;
            }
            let color = if cycle_edges.contains(&(from, to)) {
                CYCLE
            } else {
                PALETTE[(seen.len() - 1) % PALETTE.len()]
            };
            let arc = Arc { from, to, level: 0, color };
            if from < to {
                above.push(arc);
            } else {
                below.push(arc);
            }
        }
    }

    let top = assign_levels(&mut above);
    let bottom = assign_levels(&mut below);
    let ports_above = port_order(&above, n);
    let ports_below = port_order(&below, n);

    // Horizontal layout: each box is wide enough for its label and for one
    // port every two columns.
    let labels: Vec<String> = nodes.iter().map(|node| node.node_id.to_string()).collect();
    let inner: Vec<usize> = (0..n)
        .map(|i| {
            let ports = ports_above[i].len().max(ports_below[i].len());
            (labels[i].len() + 2).max(2 * ports + 1)
        })
        .collect();
    let mut x = vec![1];
    for i in 1..n {
        x.push(x[i - 1] + inner[i - 1] + 2 + GAP);
    }
    let width = x[n - 1] + inner[n - 1] + 3;

    let box_top = top;
    let box_bottom = top + 2;
    let mut canvas = Canvas::new(width, top + 3 + bottom);

    for i in 0..n {
        let color = cycle_nodes.contains(&i).then_some(CYCLE);
        let border = "─".repeat(inner[i]);
        canvas.text(box_top, x[i], &format!("┌{border}┐"), color);
        canvas.text(box_top + 1, x[i], &format!("│{}│", " ".repeat(inner[i])), color);
        canvas.text(box_bottom, x[i], &format!("└{border}┘"), color);
        let pad = (inner[i] - labels[i].len()) / 2;
        canvas.text(box_top + 1, x[i] + 1 + pad, &labels[i], color.or(Some(BOLD)));
    }

    let port_col = |ports: &[Vec<usize>], node: usize, arc: usize| {
        let list = &ports[node];
        let k = list.iter().position(|&a| a == arc).unwrap();
        let offset = (inner[node] - (2 * list.len() - 1)) / 2;
        x[node] + 1 + offset + 2 * k
    };

    for (arcs, ports, is_above) in [(&above, &ports_above, true), (&below, &ports_below, false)] {
        for (i, arc) in arcs.iter().enumerate() {
            let from_col = port_col(ports, arc.from, i);
            let to_col = port_col(ports, arc.to, i);
            let (a, b) = (from_col.min(to_col), from_col.max(to_col));

            let (row, box_row, toward_box) = if is_above {
                (top - 1 - arc.level, box_top, DOWN)
            } else {
                (box_bottom + 1 + arc.level, box_bottom, UP)
            };

            canvas.line(row, a, toward_box | RIGHT, arc.color);
            canvas.line(row, b, toward_box | LEFT, arc.color);
            for col in a + 1..b {
                canvas.line(row, col, LEFT | RIGHT, arc.color);
            }
            for r in row.min(box_row) + 1..row.max(box_row) {
                canvas.line(r, a, UP | DOWN, arc.color);
                canvas.line(r, b, UP | DOWN, arc.color);
            }

            let (source, target) = if is_above { ('┴', '▼') } else { ('┬', '▲') };
            canvas.put(box_row, from_col, source, Some(arc.color));
            canvas.put(box_row, to_col, target, Some(arc.color));
        }
    }

    let mut out = canvas.render();
    out.push_str(&format!("\n▼ ▲ edge target   {CYCLE}red{RESET} cycle edge\n"));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strip_ansi(s: &str) -> String {
        let mut out = String::new();
        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            if c == '\x1b' {
                chars.by_ref().take_while(|c| *c != 'm').for_each(drop);
            } else {
                out.push(c);
            }
        }
        out
    }

    #[test]
    fn empty_graph() {
        assert_eq!(render(&Graph::new(), &[]), "(empty graph)\n");
    }

    #[test]
    fn nodes_without_edges() {
        let mut graph = Graph::new();
        graph.add_node(vec![]);
        graph.add_node(vec![]);
        let out = strip_ansi(&render(&graph, &[]));
        assert!(out.starts_with(" ┌───┐   ┌───┐\n │ 0 │   │ 1 │\n └───┘   └───┘\n"));
    }

    #[test]
    fn dense_graph_with_multi_digit_ids() {
        let mut graph = Graph::new();
        for i in 0..12 {
            graph.add_node((0..12).filter(|&j| j != i).collect());
        }
        let out = strip_ansi(&render(&graph, &[]));
        let (out, _legend) = out.split_once("

").unwrap();
        assert!(out.contains("11"));
        assert_eq!(out.matches('▼').count(), 66);
        assert_eq!(out.matches('▲').count(), 66);
    }
}
